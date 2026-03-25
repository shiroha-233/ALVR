use crate::dashboard::ServerRequest;
use alvr_gui_common::tr;
use eframe::{
    egui::{Button, Label, Layout, RichText, Ui},
    emath::Align,
};

pub enum SetupWizardRequest {
    ServerRequest(ServerRequest),
    Close { finished: bool },
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Page {
    Welcome = 0,
    ResetSettings = 1,
    HardwareRequirements = 2,
    SoftwareRequirements = 3,
    Firewall = 4,
    Recommendations = 5,
    Finished = 6,
}

fn index_to_page(index: usize) -> Page {
    match index {
        0 => Page::Welcome,
        1 => Page::ResetSettings,
        2 => Page::HardwareRequirements,
        3 => Page::SoftwareRequirements,
        4 => Page::Firewall,
        5 => Page::Recommendations,
        6 => Page::Finished,
        _ => panic!("Invalid page index"),
    }
}

fn page_content(
    ui: &mut Ui,
    subtitle: &str,
    paragraph: &str,
    interactible_content: impl FnMut(&mut Ui),
) {
    ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
        ui.add_space(60.0);
        ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
            ui.add_space(60.0);
            ui.with_layout(Layout::top_down(Align::LEFT), |ui| {
                ui.add_space(15.0);
                ui.heading(RichText::new(subtitle).size(20.0));
                ui.add(Label::new(RichText::new(paragraph).size(14.0)).wrap());
                ui.add_space(30.0);
                ui.vertical_centered(interactible_content);
            });
        })
    });
}

pub struct SetupWizard {
    page: Page,
}

impl SetupWizard {
    pub fn new() -> Self {
        Self {
            page: Page::Welcome,
        }
    }

    pub fn ui(&mut self, ui: &mut Ui) -> Option<SetupWizardRequest> {
        let mut request = None;

        ui.horizontal(|ui| {
            ui.add_space(60.0);
            ui.vertical(|ui| {
                ui.add_space(30.0);
                ui.heading(RichText::new(tr("Welcome to ALVR").into_owned()).size(30.0));
                ui.add_space(5.0);
            });
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                ui.add_space(15.0);
                if ui.button("❌").clicked() {
                    request = Some(SetupWizardRequest::Close { finished: false });
                }
            })
        });
        ui.separator();
        match &self.page {
            Page::Welcome => page_content(
                ui,
                tr("This setup wizard will help you setup ALVR.").as_ref(),
                "",
                |_| (),
            ),
            Page::ResetSettings => page_content(
                ui,
                tr("Reset settings").as_ref(),
                tr("It is recommended to reset your settings everytime you update ALVR.").as_ref(),
                |ui| {
                    if ui.button(tr("Reset settings").as_ref()).clicked() {
                        request = Some(SetupWizardRequest::ServerRequest(
                            ServerRequest::UpdateSession(Box::default()),
                        ));
                    }
                },
            ),
            Page::HardwareRequirements => page_content(
                ui,
                tr("Hardware requirements").as_ref(),
                tr(
                    "ALVR requires a dedicated and recent graphics card. Low-end Intel integrated graphics may fail to work.
Make sure you have at least one output audio device.",
                )
                .as_ref(),
                |_| (),
            ),
            Page::SoftwareRequirements => {
                let paragraph = if cfg!(windows) {
                    tr(
                        "To stream the headset microphone on Windows you need to install Virtual Audio Cable, VB-Cable, Voicemeeter",
                    )
                    .into_owned()
                } else if cfg!(target_os = "linux") {
                    tr(
                        "You need the PipeWire (0.3.49+ version) audio system to be able to stream audio and use microphone.",
                    )
                    .into_owned()
                } else {
                    tr("Unsupported OS").into_owned()
                };

                page_content(
                    ui,
                    tr("Software requirements").as_ref(),
                    &paragraph,
                    #[allow(unused_variables)]
                    |ui| {
                        #[cfg(windows)]
                        if ui.button(tr("Download Virtual Audio Cable (Lite)").as_ref()).clicked() {
                            ui.ctx().open_url(eframe::egui::OpenUrl::same_tab(
                                "https://software.muzychenko.net/freeware/vac470lite.zip",
                            ));
                        }
                    },
                )
            }
            Page::Firewall => page_content(
                ui,
                tr("Firewall").as_ref(),
                tr(
                    "To communicate with the headset, some firewall rules need to be set.
This requires administrator rights!",
                )
                .as_ref(),
                |ui| {
                    if ui.button(tr("Add firewall rules").as_ref()).clicked() {
                        request = Some(SetupWizardRequest::ServerRequest(
                            ServerRequest::AddFirewallRules,
                        ));
                    }
                },
            ),
            Page::Recommendations => page_content(
                ui,
                tr("Recommendations").as_ref(),
                tr(
                    "ALVR supports multiple types of PC hardware and headsets but not all might work correctly with default settings. Please try tweaking different settings like resolution, bitrate, encoder and others if your ALVR experience is not optimal.",
                )
                .as_ref(),
                |_| (),
            ),
            Page::Finished => page_content(
                ui,
                tr("Finished").as_ref(),
                tr(r#"You can always restart this setup wizard from the "Installation" tab on the left."#).as_ref(),
                |_| (),
            ),
        };

        ui.with_layout(Layout::bottom_up(Align::RIGHT), |ui| {
            ui.add_space(30.0);
            ui.horizontal(|ui| {
                ui.add_space(15.0);
                if self.page == Page::Finished {
                    if ui.button(tr("Finish").as_ref()).clicked() {
                        request = Some(SetupWizardRequest::Close { finished: true });
                    }
                } else if ui.button(tr("Next").as_ref()).clicked() {
                    self.page = index_to_page(self.page as usize + 1);
                }
                if ui
                    .add_visible(self.page != Page::Welcome, Button::new(tr("Back").as_ref()))
                    .clicked()
                {
                    self.page = index_to_page(self.page as usize - 1);
                }
            });
            ui.separator();
        });

        request
    }
}
