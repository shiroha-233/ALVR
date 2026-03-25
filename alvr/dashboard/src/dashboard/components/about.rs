use alvr_common::ALVR_VERSION;
use alvr_gui_common::{Language, current_language, theme, tr};
use eframe::egui::{Frame, RichText, ScrollArea, Ui};

pub fn about_tab_ui(ui: &mut Ui) {
    let title = if matches!(current_language(), Language::Chinese) {
        format!("ALVR 串流端 v{}", *ALVR_VERSION)
    } else {
        format!("ALVR streamer v{}", *ALVR_VERSION)
    };

    ui.label(RichText::new(title).size(30.0));
    ui.add_space(10.0);
    ui.hyperlink_to(
        tr("Visit us on GitHub").into_owned(),
        "https://github.com/alvr-org/ALVR",
    );
    ui.hyperlink_to(
        tr("Join us on Discord").into_owned(),
        "https://discord.gg/ALVR",
    );
    ui.hyperlink_to(
        tr("Latest release").into_owned(),
        "https://github.com/alvr-org/ALVR/releases/latest",
    );
    ui.hyperlink_to(
        tr("Donate to ALVR on Open Collective").into_owned(),
        "https://opencollective.com/alvr",
    );
    ui.add_space(10.0);
    ui.label(tr("License:").as_ref());
    Frame::group(ui.style())
        .fill(theme::DARKER_BG)
        .inner_margin(theme::FRAME_PADDING)
        .show(ui, |ui| {
            ScrollArea::new([false, true])
                .id_salt("license_scroll")
                .show(ui, |ui| ui.label(include_str!("../../../../../LICENSE")))
        });
}
