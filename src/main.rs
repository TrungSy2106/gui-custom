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
            Ok(Box::new(dialog::ColorDialog::new(&cc.egui_ctx)))
        }),
    )
}

fn load_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    let windows = std::env::var_os("WINDIR").unwrap_or_else(|| "C:/Windows".into());
    let directory = std::path::PathBuf::from(windows).join("Fonts");
    for (name, file) in [("body", "arial.ttf"), ("title", "segoeui.ttf")] {
        if let Ok(bytes) = std::fs::read(directory.join(file)) {
            fonts
                .font_data
                .insert(name.into(), egui::FontData::from_owned(bytes));
            fonts
                .families
                .insert(egui::FontFamily::Name(name.into()), vec![name.into()]);
        } else {
            fonts.families.insert(
                egui::FontFamily::Name(name.into()),
                fonts.families[&egui::FontFamily::Proportional].clone(),
            );
        }
    }
    ctx.set_fonts(fonts);
}
