use crate::dashboard::ServerRequest;
use alvr_gui_common::tr;
use eframe::egui::Ui;

pub fn debug_tab_ui(ui: &mut Ui) -> Option<ServerRequest> {
    let mut request = None;

    ui.label(
        tr(
            "Recording from ALVR using the buttons below is not suitable for capturing gameplay.
For that, use other means of recording, for example through headset or desktop VR output.",
        )
        .into_owned(),
    );

    ui.columns(4, |ui| {
        if ui[0].button(tr("Capture frame").as_ref()).clicked() {
            request = Some(ServerRequest::CaptureFrame);
        }

        if ui[1].button(tr("Insert IDR").as_ref()).clicked() {
            request = Some(ServerRequest::InsertIdr);
        }

        if ui[2].button(tr("Start recording").as_ref()).clicked() {
            request = Some(ServerRequest::StartRecording);
        }

        if ui[3].button(tr("Stop recording").as_ref()).clicked() {
            request = Some(ServerRequest::StopRecording);
        }
    });

    request
}
