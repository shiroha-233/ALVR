use crate::{Language, tr};
use egui::{Align, Align2, Context, Layout, Ui, Window};
use std::fmt::{self, Display, Formatter};

#[derive(Clone, PartialEq)]
pub enum ModalButton {
    Ok,
    Cancel,
    Close,
    Custom(String),
}

impl Display for ModalButton {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ModalButton::Ok => write!(f, "OK"),
            ModalButton::Cancel => write!(f, "Cancel"),
            ModalButton::Close => write!(f, "Close"),
            ModalButton::Custom(text) => write!(f, "{text}"),
        }
    }
}

impl ModalButton {
    pub fn label(&self, language: Language) -> String {
        match self {
            Self::Ok => language.translate("OK").into_owned(),
            Self::Cancel => language.translate("Cancel").into_owned(),
            Self::Close => language.translate("Close").into_owned(),
            Self::Custom(text) => language.translate(text).into_owned(),
        }
    }
}

pub fn modal(
    context: &Context,
    title: &str,
    content: Option<impl FnOnce(&mut Ui)>,
    buttons: &[ModalButton],
    width: Option<f32>,
    language: Language,
) -> Option<ModalButton> {
    let mut response = None;

    let mut window = Window::new(tr(title).into_owned())
        .anchor(Align2::CENTER_CENTER, (0.0, 0.0))
        .collapsible(false)
        .resizable(false);

    if let Some(w) = width {
        window = window.min_width(w).max_width(w);
    }

    window.show(context, |ui| {
        ui.vertical_centered_justified(|ui| {
            if let Some(content) = content {
                ui.add_space(10.0);
                content(ui);
                ui.add_space(10.0);
            }

            ui.columns(buttons.len(), |cols| {
                for (idx, response_type) in buttons.iter().enumerate() {
                    cols[idx].with_layout(Layout::top_down_justified(Align::Center), |ui| {
                        if ui.button(response_type.label(language)).clicked() {
                            response = Some(response_type.clone());
                        }
                    });
                }
            });
        });
    });

    response
}
