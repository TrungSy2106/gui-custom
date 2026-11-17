mod dialog;
mod palette;
mod widgets;

use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(dialog::SIZE)
            .with_resizable(false)
            .with_decorations(false)
            .with_transparent(true),
        ..Default::default()
    };

    eframe::run_native(
        "Select Color",
        options,
        Box::new(|cc| {
            load_fonts(&cc.egui_ctx);
            cc.egui_ctx.set_visuals(egui::Visuals::light());
            cc.egui_ctx.style_mut(|style| {
                style.override_font_id = Some(egui::FontId::new(
                    13.0,
                    egui::FontFamily::Name("body".into()),
                ));
            });
            Ok(Box::new(dialog::ColorDialog::new(&cc.egui_ctx)))
        }),
    )
}

fn load_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    let windows = std::env::var_os("WINDIR").unwrap_or_else(|| "C:/Windows".into());
    let directory = std::path::PathBuf::from(windows).join("Fonts");
    for (name, file) in [("body", "micross.ttf"), ("title", "segoeui.ttf")] {
        let mut family = fonts.families[&egui::FontFamily::Proportional].clone();
        if let Ok(bytes) = std::fs::read(directory.join(file)) {
            fonts
                .font_data
                .insert(name.into(), egui::FontData::from_owned(bytes));
            family.insert(0, name.into());
        }
        fonts
            .families
            .insert(egui::FontFamily::Name(name.into()), family);
    }
    fonts.families.insert(
        egui::FontFamily::Proportional,
        fonts.families[&egui::FontFamily::Name("body".into())].clone(),
    );
    ctx.set_fonts(fonts);
}
