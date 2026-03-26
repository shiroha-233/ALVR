#![allow(
    non_upper_case_globals,
    non_snake_case,
    clippy::missing_safety_doc,
    clippy::unseparated_literal_suffix
)]

mod c_api;
mod connection;
mod logging_backend;
mod sockets;
mod statistics;
mod storage;

#[cfg(target_os = "android")]
mod audio;

pub mod video_decoder;

use alvr_common::{
    ConnectionState, Fov, LifecycleState, Pose, ViewParams, dbg_client_core, error,
    glam::{Quat, UVec2, Vec2, Vec3},
    info,
    parking_lot::{Mutex, RwLock},
    warn,
};
use alvr_packets::{
    BatteryInfo, ButtonEntry, ClientControlPacket, RealTimeConfig, StreamConfig, TrackingData,
};
use alvr_session::CodecType;
use alvr_system_info::Platform;
use connection::{ConnectionContext, DecoderCallback};
use std::{
    collections::{HashSet, VecDeque},
    sync::{Arc, LazyLock},
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};
use storage::Config;

pub use logging_backend::init_logging;

const YVR_VIEW_LOG_INTERVAL: Duration = Duration::from_millis(1000);
static LAST_YVR_VIEW_LOG: LazyLock<Mutex<Instant>> = LazyLock::new(|| Mutex::new(Instant::now()));

fn should_log_yvr_view_params() -> bool {
    let now = Instant::now();
    let mut last_log = LAST_YVR_VIEW_LOG.lock();

    if *last_log + YVR_VIEW_LOG_INTERVAL < now {
        *last_log = now;

        true
    } else {
        false
    }
}

fn format_view_params(view: ViewParams) -> String {
    format!(
        "pos=[{:.4},{:.4},{:.4}] q=[{:.4},{:.4},{:.4},{:.4}] fov=[l:{:.1} r:{:.1} u:{:.1} d:{:.1}]",
        view.pose.position.x,
        view.pose.position.y,
        view.pose.position.z,
        view.pose.orientation.x,
        view.pose.orientation.y,
        view.pose.orientation.z,
        view.pose.orientation.w,
        view.fov.left.to_degrees(),
        view.fov.right.to_degrees(),
        view.fov.up.to_degrees(),
        view.fov.down.to_degrees(),
    )
}

pub enum ClientCoreEvent {
    UpdateHudMessage(String),
    StreamingStarted(Box<StreamConfig>),
    StreamingStopped,
    Haptics {
        device_id: u64,
        duration: Duration,
        frequency: f32,
        amplitude: f32,
    },
    // Note: All subsequent DecoderConfig events should be ignored until reconnection
    DecoderConfig {
        codec: CodecType,
        config_nal: Vec<u8>,
    },
    RealTimeConfig(RealTimeConfig),
}

// Note: this struct may change without breaking network protocol changes
#[derive(Clone)]
pub struct ClientCapabilities {
    pub platform: Platform,
    pub default_view_resolution: UVec2,
    pub max_view_resolution: UVec2,
    pub refresh_rates: Vec<f32>,
    pub foveated_encoding: bool,
    pub encoder_high_profile: bool,
    pub encoder_10_bits: bool,
    pub encoder_av1: bool,
    pub prefer_10bit: bool,
    pub preferred_encoding_gamma: f32,
    pub prefer_hdr: bool,
}

pub struct ClientCoreContext {
    platform: Platform,
    lifecycle_state: Arc<RwLock<LifecycleState>>,
    event_queue: Arc<Mutex<VecDeque<ClientCoreEvent>>>,
    connection_context: Arc<ConnectionContext>,
    connection_thread: Arc<Mutex<Option<JoinHandle<()>>>>,
    last_good_global_view_params: Mutex<[ViewParams; 2]>,
}

impl ClientCoreContext {
    pub fn new(capabilities: ClientCapabilities) -> Self {
        dbg_client_core!("Create");

        // Make sure to reset config in case of version compat mismatch.
        if Config::load().protocol_id != alvr_common::protocol_id() {
            // NB: Config::default() sets the current protocol ID
            Config::default().store();
        }

        #[cfg(target_os = "android")]
        {
            dbg_client_core!("Getting permissions");
            alvr_system_info::try_get_permission(alvr_system_info::MICROPHONE_PERMISSION);
            alvr_system_info::set_wifi_lock(true);
        }

        let platform = capabilities.platform;

        let lifecycle_state = Arc::new(RwLock::new(LifecycleState::Idle));
        let event_queue = Arc::new(Mutex::new(VecDeque::new()));
        let connection_context = Arc::new(ConnectionContext::default());
        let connection_thread = thread::spawn({
            let lifecycle_state = Arc::clone(&lifecycle_state);
            let connection_context = Arc::clone(&connection_context);
            let event_queue = Arc::clone(&event_queue);
            move || {
                connection::connection_lifecycle_loop(
                    capabilities,
                    connection_context,
                    lifecycle_state,
                    event_queue,
                )
            }
        });

        Self {
            platform,
            lifecycle_state,
            event_queue,
            connection_context,
            connection_thread: Arc::new(Mutex::new(Some(connection_thread))),
            last_good_global_view_params: Mutex::new([ViewParams::DUMMY; 2]),
        }
    }

    pub fn resume(&self) {
        dbg_client_core!("resume");

        *self.lifecycle_state.write() = LifecycleState::Resumed;
    }

    pub fn pause(&self) {
        dbg_client_core!("pause");

        let mut connection_state_lock = self.connection_context.state.write();

        *self.lifecycle_state.write() = LifecycleState::Idle;

        // We want to shutdown streaming when pausing.
        if *connection_state_lock != ConnectionState::Disconnected {
            alvr_common::wait_rwlock(
                &self.connection_context.disconnected_notif,
                &mut connection_state_lock,
            );
        }
    }

    pub fn poll_event(&self) -> Option<ClientCoreEvent> {
        dbg_client_core!("poll_event");

        self.event_queue.lock().pop_front()
    }

    pub fn send_battery(&self, device_id: u64, gauge_value: f32, is_plugged: bool) {
        dbg_client_core!("send_battery");

        if let Some(sender) = &mut *self.connection_context.control_sender.lock() {
            sender
                .send(&ClientControlPacket::Battery(BatteryInfo {
                    device_id,
                    gauge_value,
                    is_plugged,
                }))
                .ok();
        }
    }

    pub fn send_playspace(&self, area: Option<Vec2>) {
        dbg_client_core!("send_playspace");

        if let Some(sender) = &mut *self.connection_context.control_sender.lock() {
            sender.send(&ClientControlPacket::PlayspaceSync(area)).ok();
        }
    }

    pub fn send_active_interaction_profile(
        &self,
        device_id: u64,
        profile_id: u64,
        input_ids: HashSet<u64>,
    ) {
        dbg_client_core!("send_active_interaction_profile");

        if let Some(sender) = &mut *self.connection_context.control_sender.lock() {
            sender
                .send(&ClientControlPacket::ActiveInteractionProfile {
                    device_id,
                    profile_id,
                    input_ids,
                })
                .ok();
        }
    }

    pub fn send_buttons(&self, entries: Vec<ButtonEntry>) {
        dbg_client_core!("send_buttons");

        if let Some(sender) = &mut *self.connection_context.control_sender.lock() {
            sender.send(&ClientControlPacket::Buttons(entries)).ok();
        }
    }

    // These must be in its local space, as if the head pose is in the origin.
    pub fn send_view_params(&self, views: [ViewParams; 2]) {
        dbg_client_core!("send_view_params");

        let views_openvr = [
            canted_view_to_proportional_circumscribed_orthogonal(views[0], 1.0),
            canted_view_to_proportional_circumscribed_orthogonal(views[1], 1.0),
        ];

        if self.platform.is_yvr() && should_log_yvr_view_params() {
            info!(
                "YVR TRACE client local views: raw_l={} raw_r={} openvr_l={} openvr_r={}",
                format_view_params(views[0]),
                format_view_params(views[1]),
                format_view_params(views_openvr[0]),
                format_view_params(views_openvr[1]),
            );
        }

        if let Some(sender) = &mut *self.connection_context.control_sender.lock() {
            sender
                .send(&ClientControlPacket::LocalViewParams(views_openvr))
                .ok();
        }
    }

    pub fn send_tracking(&self, data: TrackingData) {
        dbg_client_core!("send_tracking");

        if let Some(sender) = &mut *self.connection_context.tracking_sender.lock() {
            sender.send_header(&data).ok();

            if let Some(stats) = &mut *self.connection_context.statistics_manager.lock() {
                stats.report_input_acquired(data.poll_timestamp);
            }
        }
    }

    pub fn send_proximity_state(&self, headset_is_worn: bool) {
        if let Some(sender) = &mut *self.connection_context.control_sender.lock() {
            sender
                .send(&ClientControlPacket::ProximityState(headset_is_worn))
                .ok();
        }
    }

    pub fn get_total_prediction_offset(&self) -> Duration {
        dbg_client_core!("get_total_prediction_offset");

        if let Some(stats) = &*self.connection_context.statistics_manager.lock() {
            Duration::min(
                stats.average_total_pipeline_latency(),
                *self.connection_context.max_prediction.read(),
            )
        } else {
            Duration::ZERO
        }
    }

    /// The callback should return true if the frame was successfully submitted to the decoder
    pub fn set_decoder_input_callback(&self, callback: Box<DecoderCallback>) {
        dbg_client_core!("set_decoder_input_callback");

        *self.connection_context.decoder_callback.lock() = Some(callback);

        if let Some(sender) = &mut *self.connection_context.control_sender.lock() {
            sender.send(&ClientControlPacket::RequestIdr).ok();
        }
    }

    pub fn report_frame_decoded(&self, timestamp: Duration) {
        dbg_client_core!("report_frame_decoded");

        if let Some(stats) = &mut *self.connection_context.statistics_manager.lock() {
            stats.report_frame_decoded(timestamp);
        }
    }

    pub fn report_fatal_decoder_error(&self, error: &str) {
        error!("Fatal decoder error, restarting connection: {error}");

        // The connection loop observes changes on this value
        *self.connection_context.state.write() = ConnectionState::Disconnecting;
    }

    pub fn report_compositor_start(&self, timestamp: Duration) -> [ViewParams; 2] {
        dbg_client_core!("report_compositor_start");

        if let Some(stats) = &mut *self.connection_context.statistics_manager.lock() {
            stats.report_compositor_start(timestamp);
        }

        let global_view_params_lock = &mut *self.last_good_global_view_params.lock();
        for (ts, params) in &*self.connection_context.global_view_params_queue.lock() {
            if *ts == timestamp {
                *global_view_params_lock = *params;
                break;
            }
        }

        *global_view_params_lock
    }

    pub fn report_submit(&self, timestamp: Duration, vsync_queue: Duration) {
        dbg_client_core!("report_submit");

        if let Some(stats) = &mut *self.connection_context.statistics_manager.lock() {
            stats.report_submit(timestamp, vsync_queue);

            if let Some(sender) = &mut *self.connection_context.statistics_sender.lock() {
                if let Some(stats) = stats.summary(timestamp) {
                    sender.send_header(&stats).ok();
                } else {
                    warn!("Statistics summary not ready!");
                }
            }
        }
    }

    pub fn platform(&self) -> Platform {
        self.platform
    }
}

impl Drop for ClientCoreContext {
    fn drop(&mut self) {
        dbg_client_core!("Drop");

        *self.lifecycle_state.write() = LifecycleState::ShuttingDown;

        if let Some(thread) = self.connection_thread.lock().take() {
            thread.join().ok();
        }

        #[cfg(target_os = "android")]
        alvr_system_info::set_wifi_lock(false);
    }
}

fn canted_view_to_proportional_circumscribed_orthogonal(
    view_canted: ViewParams,
    fov_post_scale: f32,
) -> ViewParams {
    let viewpose_orth = Pose {
        orientation: Quat::IDENTITY,
        position: view_canted.pose.position,
    };

    let v0 = Vec3::new(view_canted.fov.left, view_canted.fov.down, -1.0);
    let v1 = Vec3::new(view_canted.fov.right, view_canted.fov.down, -1.0);
    let v2 = Vec3::new(view_canted.fov.right, view_canted.fov.up, -1.0);
    let v3 = Vec3::new(view_canted.fov.left, view_canted.fov.up, -1.0);

    let w0 = view_canted.pose.orientation * v0;
    let w1 = view_canted.pose.orientation * v1;
    let w2 = view_canted.pose.orientation * v2;
    let w3 = view_canted.pose.orientation * v3;

    let pt0 = Vec2::new(w0.x * (-1.0 / w0.z), w0.y * (-1.0 / w0.z));
    let pt1 = Vec2::new(w1.x * (-1.0 / w1.z), w1.y * (-1.0 / w1.z));
    let pt2 = Vec2::new(w2.x * (-1.0 / w2.z), w2.y * (-1.0 / w2.z));
    let pt3 = Vec2::new(w3.x * (-1.0 / w3.z), w3.y * (-1.0 / w3.z));

    let pts_x = [pt0.x, pt1.x, pt2.x, pt3.x];
    let pts_y = [pt0.y, pt1.y, pt2.y, pt3.y];
    let inscribed_left = pts_x.iter().fold(f32::INFINITY, |a, &b| a.min(b));
    let inscribed_right = pts_x.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
    let inscribed_up = pts_y.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
    let inscribed_down = pts_y.iter().fold(f32::INFINITY, |a, &b| a.min(b));

    let fov_orth = Fov {
        left: inscribed_left,
        right: inscribed_right,
        up: inscribed_up,
        down: inscribed_down,
    };

    let fov_orth_width = fov_orth.right.abs() + fov_orth.left.abs();
    let fov_orth_height = fov_orth.up.abs() + fov_orth.down.abs();
    let fov_orig_width = view_canted.fov.right.abs() + view_canted.fov.left.abs();
    let fov_orig_height = view_canted.fov.up.abs() + view_canted.fov.down.abs();
    let scales = [
        fov_orth_width / fov_orig_width,
        fov_orth_height / fov_orig_height,
    ];

    let fov_inscribe_scale = scales
        .iter()
        .fold(f32::NEG_INFINITY, |a, &b| a.max(b))
        .max(1.0);
    let fov_orth_corrected = Fov {
        left: view_canted.fov.left * fov_inscribe_scale * fov_post_scale,
        right: view_canted.fov.right * fov_inscribe_scale * fov_post_scale,
        up: view_canted.fov.up * fov_inscribe_scale * fov_post_scale,
        down: view_canted.fov.down * fov_inscribe_scale * fov_post_scale,
    };

    ViewParams {
        pose: viewpose_orth,
        fov: fov_orth_corrected,
    }
}
