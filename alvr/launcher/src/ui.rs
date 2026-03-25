use crate::{InstallationInfo, Progress, ReleaseChannelsInfo, UiMessage, WorkerMessage, actions};
use alvr_gui_common::{Language, ModalButton, current_language, set_current_language, tr};
use eframe::{
    egui::{
        self, Button, CentralPanel, ComboBox, Context, Frame, Grid, Layout, ProgressBar, RichText,
        Ui, ViewportCommand,
    },
    emath::Align,
    epaint::Color32,
};
use std::{
    mem,
    sync::mpsc::{Receiver, Sender},
};

enum State {
    Default,
    Installing(Progress),
    Error(String),
}

#[derive(Default)]
enum PopupType {
    #[default]
    None,
    DeleteInstallation(String),
    EditVersion(String),
    AddVersion {
        version_selection: Version,
        session_version_selection: Option<String>,
    },
}

#[derive(Clone, PartialEq, Eq)]
enum ReleaseChannelType {
    Stable,
    Nightly,
}

#[derive(Clone, PartialEq, Eq)]
struct Version {
    string: String,
    release_channel: ReleaseChannelType,
}

pub struct Launcher {
    worker_message_receiver: Receiver<WorkerMessage>,
    ui_message_sender: Sender<UiMessage>,
    language: Language,
    state: State,
    release_channels_info: Option<ReleaseChannelsInfo>,
    installations: Vec<InstallationInfo>,
    popup: PopupType,
}

impl Launcher {
    pub fn new(
        cc: &eframe::CreationContext,
        worker_message_receiver: Receiver<WorkerMessage>,
        ui_message_sender: Sender<UiMessage>,
    ) -> Self {
        alvr_gui_common::theme::set_theme(&cc.egui_ctx);

        Self {
            worker_message_receiver,
            ui_message_sender,
            language: current_language(),
            state: State::Default,
            release_channels_info: None,
            installations: actions::get_installations(),
            popup: PopupType::None,
        }
    }

    fn version_popup(
        &self,
        ctx: &Context,
        mut version: Version,
        mut session_version: Option<String>,
    ) -> PopupType {
        let response = alvr_gui_common::modal(
            ctx,
            "Add version",
            {
                // Safety: unwrap is safe because the "Add release" button is available after populating the release_channels_info.
                let release_channels_info = self.release_channels_info.as_ref().unwrap();
                Some(|ui: &mut Ui| {
                    let version_str = version.string.clone();
                    let versions: Vec<_> = match &version.release_channel {
                        ReleaseChannelType::Stable => release_channels_info
                            .stable
                            .iter()
                            .map(|release| Version {
                                string: release.version.clone(),
                                release_channel: ReleaseChannelType::Stable,
                            })
                            .collect(),

                        ReleaseChannelType::Nightly => release_channels_info
                            .nightly
                            .iter()
                            .map(|release| Version {
                                string: release.version.clone(),
                                release_channel: ReleaseChannelType::Nightly,
                            })
                            .collect(),
                    };
                    let installations_with_session: Vec<_> = self
                        .installations
                        .iter()
                        .filter(|installation| installation.has_session_json)
                        .map(|installation| installation.version.clone())
                        .collect();

                    Grid::new("add-version-grid").num_columns(2).show(ui, |ui| {
                        ui.label(tr("Channel").as_ref());
                        ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                            let channel_str = match version.release_channel {
                                ReleaseChannelType::Stable => tr("Stable").into_owned(),
                                ReleaseChannelType::Nightly => tr("Nightly").into_owned(),
                            };

                            ComboBox::from_id_salt("channel")
                                .selected_text(channel_str)
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(
                                        &mut version,
                                        Version {
                                            string: release_channels_info.stable[0].version.clone(),
                                            release_channel: ReleaseChannelType::Stable,
                                        },
                                        tr("Stable").as_ref(),
                                    );
                                    ui.selectable_value(
                                        &mut version,
                                        Version {
                                            string: release_channels_info.nightly[0]
                                                .version
                                                .clone(),
                                            release_channel: ReleaseChannelType::Nightly,
                                        },
                                        tr("Nightly").as_ref(),
                                    );
                                })
                        });
                        ui.end_row();

                        ui.label(tr("Version").as_ref());
                        ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                            ComboBox::from_id_salt("version")
                                .selected_text(version_str)
                                .show_ui(ui, |ui| {
                                    for ver in versions {
                                        ui.selectable_value(&mut version, ver.clone(), ver.string);
                                    }
                                })
                        });
                        ui.end_row();

                        if cfg!(windows) {
                            ui.label(tr("Copy session from:").as_ref());
                            ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                                ComboBox::from_id_salt("session")
                                    .selected_text(
                                        session_version
                                            .clone()
                                            .unwrap_or_else(|| tr("None").into_owned()),
                                    )
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(
                                            &mut session_version,
                                            None,
                                            tr("None").as_ref(),
                                        );
                                        for ver_str in installations_with_session {
                                            ui.selectable_value(
                                                &mut session_version,
                                                Some(ver_str.clone()),
                                                ver_str,
                                            );
                                        }
                                    })
                            });
                            ui.end_row();
                        }
                    });
                })
            },
            &[ModalButton::Cancel, ModalButton::Custom("Install".into())],
            None,
            self.language,
        );

        match response {
            Some(ModalButton::Cancel) => PopupType::None,
            Some(ModalButton::Custom(_)) => {
                let release_info = match &version.release_channel {
                    ReleaseChannelType::Stable => self
                        .release_channels_info
                        .as_ref()
                        .unwrap()
                        .stable
                        .iter()
                        .find(|release| release.version == version.string)
                        .unwrap()
                        .clone(),
                    ReleaseChannelType::Nightly => self
                        .release_channels_info
                        .as_ref()
                        .unwrap()
                        .nightly
                        .iter()
                        .find(|release| release.version == version.string)
                        .unwrap()
                        .clone(),
                };

                self.ui_message_sender
                    .send(UiMessage::InstallServer {
                        release_info,
                        session_version,
                    })
                    .ok();

                PopupType::None
            }
            _ => PopupType::AddVersion {
                version_selection: version,
                session_version_selection: session_version,
            },
        }
    }

    fn edit_popup(&self, ctx: &Context, version: String) -> PopupType {
        let mut delete_version = false;
        let response = alvr_gui_common::modal(
            ctx,
            "Edit version",
            Some(|ui: &mut Ui| {
                ui.with_layout(Layout::top_down_justified(Align::Center), |ui| {
                    delete_version = ui.button(tr("Delete version").as_ref()).clicked();
                });
            }),
            &[ModalButton::Close],
            None,
            self.language,
        );

        if delete_version {
            PopupType::DeleteInstallation(version)
        } else if matches!(response, Some(ModalButton::Close)) {
            PopupType::None
        } else {
            PopupType::EditVersion(version)
        }
    }

    fn delete_popup(&mut self, ctx: &Context, version: String) -> PopupType {
        let response = alvr_gui_common::modal(
            ctx,
            "Are you sure?",
            Some({
                let version = version.clone();
                move |ui: &mut Ui| {
                    ui.with_layout(Layout::top_down(Align::Center), |ui| {
                        let message = if matches!(current_language(), Language::Chinese) {
                            format!("这将永久删除版本 {version}")
                        } else {
                            format!("This will permanently delete version {version}")
                        };
                        ui.label(message);
                    });
                }
            }),
            &[
                ModalButton::Cancel,
                ModalButton::Custom("Delete version".into()),
            ],
            None,
            self.language,
        );

        match response {
            Some(ModalButton::Cancel) => PopupType::None,
            Some(ModalButton::Custom(_)) => {
                if let Err(e) = actions::delete_installation(&version) {
                    self.state = State::Error(if matches!(self.language, Language::Chinese) {
                        format!("删除版本失败：{e}")
                    } else {
                        format!("Failed to delete version: {e}")
                    });
                }

                self.installations = actions::get_installations();

                PopupType::None
            }
            _ => PopupType::DeleteInstallation(version),
        }
    }
}

impl eframe::App for Launcher {
    fn update(&mut self, ctx: &Context, _: &mut eframe::Frame) {
        set_current_language(self.language);
        let previous_language = self.language;

        while let Ok(msg) = self.worker_message_receiver.try_recv() {
            match msg {
                WorkerMessage::ReleaseChannelsInfo(data) => self.release_channels_info = Some(data),
                WorkerMessage::ProgressUpdate(progress) => {
                    self.state = State::Installing(progress);
                }
                WorkerMessage::Done => {
                    // Refresh installations
                    self.installations = actions::get_installations();
                    self.state = State::Default;
                }
                WorkerMessage::Error(e) => self.state = State::Error(e),
            }
        }

        CentralPanel::default().show(ctx, |ui| match &self.state {
            State::Default => {
                ui.with_layout(Layout::top_down(Align::Center), |ui| {
                    ui.label(
                        RichText::new(tr("ALVR Launcher").into_owned())
                            .size(25.0)
                            .strong(),
                    );
                    ui.horizontal(|ui| {
                        ui.label(tr("Language").as_ref());
                        ComboBox::from_id_salt("launcher-language")
                            .selected_text(self.language.label())
                            .show_ui(ui, |ui| {
                                for language in Language::ALL {
                                    ui.selectable_value(
                                        &mut self.language,
                                        language,
                                        language.label(),
                                    );
                                }
                            });
                    });
                    set_current_language(self.language);

                    if self.language != previous_language {
                        ctx.send_viewport_cmd(ViewportCommand::Title(
                            tr("ALVR Launcher").into_owned(),
                        ));
                    }

                    ui.label(match &self.release_channels_info {
                        Some(data) if matches!(self.language, Language::Chinese) => {
                            format!("最新稳定版：{}", data.stable[0].version)
                        }
                        Some(data) => format!("Latest stable release: {}", data.stable[0].version),
                        None => tr("Fetching latest release...").into_owned(),
                    });

                    for installation in &self.installations {
                        let path = actions::installations_dir().join(&installation.version);

                        Frame::group(ui.style())
                            .fill(alvr_gui_common::theme::SECTION_BG)
                            .inner_margin(egui::vec2(10.0, 5.0))
                            .show(ui, |ui| {
                                Grid::new(&installation.version)
                                    .num_columns(2)
                                    .show(ui, |ui| {
                                        ui.label(&installation.version);
                                        ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                                            if ui.button(tr("Edit").as_ref()).clicked() {
                                                self.popup = PopupType::EditVersion(
                                                    installation.version.clone(),
                                                );
                                            }

                                            if ui.button(tr("Open directory").as_ref()).clicked() {
                                                open::that_in_background(path);
                                            }

                                            let release_info = self
                                                .release_channels_info
                                                .as_ref()
                                                .and_then(|info| {
                                                    actions::get_release(
                                                        info,
                                                        &installation.version,
                                                    )
                                                });
                                            if ui
                                                .add_enabled(
                                                    release_info.is_some()
                                                        || installation.is_apk_downloaded,
                                                    Button::new(tr("Install APK").as_ref()),
                                                )
                                                .clicked()
                                            {
                                                if let Some(release_info) = release_info {
                                                    self.ui_message_sender
                                                        .send(UiMessage::InstallClient(
                                                            release_info,
                                                        ))
                                                        .ok();
                                                } else {
                                                    self.state = State::Error(
                                                        tr("Failed to get release info")
                                                            .into_owned(),
                                                    );
                                                }
                                            };

                                            if ui.button(tr("Launch").as_ref()).clicked() {
                                                match actions::launch_dashboard(
                                                    &installation.version,
                                                ) {
                                                    Ok(()) => {
                                                        self.ui_message_sender
                                                            .send(UiMessage::Quit)
                                                            .ok();
                                                        ctx.send_viewport_cmd(
                                                            ViewportCommand::Close,
                                                        );
                                                    }
                                                    Err(e) => {
                                                        self.state = State::Error(e.to_string());
                                                    }
                                                }
                                            }
                                        })
                                    })
                            });
                    }

                    if ui
                        .add_enabled(
                            self.release_channels_info.is_some(),
                            Button::new(tr("Add version").as_ref()),
                        )
                        .clicked()
                    {
                        self.popup = PopupType::AddVersion {
                            version_selection: Version {
                                string: self.release_channels_info.as_ref().unwrap().stable[0]
                                    .version
                                    .clone(),
                                release_channel: ReleaseChannelType::Stable,
                            },
                            session_version_selection: None,
                        };
                    }

                    let popup = match mem::take(&mut self.popup) {
                        PopupType::AddVersion {
                            version_selection,
                            session_version_selection,
                        } => self.version_popup(ctx, version_selection, session_version_selection),
                        PopupType::EditVersion(version) => self.edit_popup(ctx, version),
                        PopupType::DeleteInstallation(version) => self.delete_popup(ctx, version),
                        PopupType::None => PopupType::None,
                    };
                    self.popup = popup;
                });
            }
            State::Installing(progress) => {
                ui.with_layout(Layout::top_down(Align::Center), |ui| {
                    ui.label(&progress.message);
                    ui.add(ProgressBar::new(progress.progress).animate(true));
                });
            }
            State::Error(e) => {
                let e = e.clone(); // Avoid borrowing issues with the closure for the layout
                ui.with_layout(Layout::top_down(Align::Center), |ui| {
                    ui.colored_label(Color32::LIGHT_RED, tr("Error!").as_ref());
                    ui.label(e);

                    if ui.button(tr("Close").as_ref()).clicked() {
                        self.state = State::Default;
                    }
                });
            }
        });

        if ctx.input(|i| i.viewport().close_requested()) {
            self.ui_message_sender.send(UiMessage::Quit).ok();
        }
    }
}
