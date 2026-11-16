use crate::{palette, widgets::*};
use eframe::egui::{self, pos2, vec2, Align2, Color32, FontFamily, FontId, Rect};

#[path = "index.rs"]
mod index;

pub const SIZE: [f32; 2] = [482.0, 465.0];

// Chỉ giữ trạng thái để tương tác trong preview.
pub struct ColorDialog {
    spectrum: egui::TextureHandle,
    hsl: [f32; 3],
    input: String,
    selected: Option<u16>,
    index_mode: bool,
    rgb_mode: bool,
}

impl ColorDialog {
    pub fn new(ctx: &egui::Context) -> Self {
        Self {
            spectrum: ctx.load_texture(
                "spectrum",
                palette::spectrum(),
                egui::TextureOptions::NEAREST,
            ),
            hsl: palette::from_rgb([82, 101, 173]),
            input: "82,101,173".into(),
            selected: None,
            index_mode: true,
            rgb_mode: false,
        }
    }

    fn show(&mut self, ui: &mut egui::Ui) {
        panel(
            ui.painter(),
            rect(0.0, 0.0, SIZE[0], SIZE[1]),
            Color32::from_gray(240),
            10.0,
        );
        self.header(ui);
        self.tabs(ui);
        if self.index_mode {
            self.index_view(ui);
        } else {
            self.true_color(ui);
        }
        self.color_input(ui);
        preview(
            ui.painter(),
            if self.index_mode && matches!(self.selected, Some(0 | 256)) {
                Color32::WHITE
            } else {
                palette::color(self.hsl)
            },
        );
        for (x, title) in [(159.0, "OK"), (263.0, "Cancel"), (367.0, "Help")] {
            // Các nút chỉ phản hồi hover/bấm trong bản preview.
            button(ui, rect(x, 428.0, 97.0, 26.0), title);
        }
    }

    fn header(&self, ui: &mut egui::Ui) {
        ui.painter().rect_filled(
            rect(1.0, 1.0, 480.0, 35.0),
            egui::Rounding {
                nw: 9.0,
                ne: 9.0,
                sw: 0.0,
                se: 0.0,
            },
            Color32::from_gray(243),
        );
        ui.painter().text(
            pos2(12.0, 8.0),
            Align2::LEFT_TOP,
            "Select Color",
            FontId::new(14.0, FontFamily::Name("title".into())),
            Color32::BLACK,
        );
        let close = rect(438.0, 1.0, 43.0, 34.0);
        let response = ui.interact(close, egui::Id::new("close"), egui::Sense::click());
        if response.hovered() {
            ui.painter()
                .rect_filled(close, 3.0, Color32::from_rgb(232, 17, 35));
        }
        centered(ui.painter(), close, "×");
        if response.clicked() {
            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
        }
        if ui
            .interact(
                rect(0.0, 0.0, 438.0, 35.0),
                egui::Id::new("title"),
                egui::Sense::drag(),
            )
            .drag_started()
        {
            ui.ctx().send_viewport_cmd(egui::ViewportCommand::StartDrag);
        }
    }

    fn tabs(&mut self, ui: &mut egui::Ui) {
        let tabs = [
            (8.0, 118.0, "Index Color"),
            (126.0, 124.0, "True Color"),
            (250.0, 115.0, "Color Books"),
        ];
        for (i, &(x, width, title)) in tabs.iter().enumerate().take(2) {
            if ui
                .interact(
                    rect(x, 49.0, width, 24.0),
                    egui::Id::new(title),
                    egui::Sense::click(),
                )
                .clicked()
                && self.index_mode != (i == 0)
            {
                self.index_mode = i == 0;
                self.input = self.selection_text();
            }
        }
        let p = ui.painter();
        for &(x, width, _) in &tabs {
            panel(p, rect(x, 51.0, width, 22.0), Color32::from_gray(243), 0.0);
        }
        panel(p, rect(6.0, 72.0, 466.0, 344.0), PANEL, 0.0);
        let active = usize::from(!self.index_mode);
        let (x, width, _) = tabs[active];
        panel(p, rect(x, 49.0, width, 24.0), PANEL, 0.0);
        p.line_segment(
            [pos2(x + 0.5, 73.0), pos2(x + width - 0.5, 73.0)],
            stroke(2.0, PANEL),
        );
        for (i, &(x, width, title)) in tabs.iter().enumerate() {
            centered(
                p,
                rect(x, if i == active { 49.0 } else { 51.0 }, width, 22.0),
                title,
            );
        }
    }

    fn true_color(&mut self, ui: &mut egui::Ui) {
        text(ui.painter(), 344.0, 78.0, "Color model:");
        ui.allocate_new_ui(
            egui::UiBuilder::new().max_rect(rect(344.0, 96.0, 117.0, 24.0)),
            |ui| {
                egui::ComboBox::from_id_salt("model")
                    .width(117.0)
                    .selected_text(if self.rgb_mode { "RGB" } else { "HSL" })
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.rgb_mode, false, "HSL");
                        ui.selectable_value(&mut self.rgb_mode, true, "RGB");
                    });
            },
        );
        if self.rgb_mode {
            self.rgb_view(ui);
        } else {
            self.hsl_view(ui);
        }
        let p = ui.painter();
        p.line_segment(
            [pos2(335.0, 86.0), pos2(335.0, 404.0)],
            stroke(2.0, Color32::WHITE),
        );
        text(p, 344.0, 134.0, "True Color Stored");
        text(p, 344.0, 150.0, "as RGB:");
        for (i, label) in ["Red:", "Green:", "Blue:"].iter().enumerate() {
            let y = 180.0 + i as f32 * 20.0;
            text(p, 360.0, y, label);
            text(p, 416.0, y, &self.rgb()[i].to_string());
        }
    }

    fn hsl_view(&mut self, ui: &mut egui::Ui) {
        let mut changed = false;
        for (i, (x, label)) in [(20.0, "Hue:"), (98.0, "Saturation:"), (256.0, "Luminance:")]
            .into_iter()
            .enumerate()
        {
            text(ui.painter(), x, 76.0, label);
            changed |= number(
                ui,
                [x, 95.0],
                i,
                &mut self.hsl[i],
                if i == 0 { 360.0 } else { 100.0 },
            );
        }
        let area = rect(21.0, 131.0, 250.0, 224.0);
        let light = rect(284.0, 136.0, 36.0, 218.0);
        if let Some(t) = drag(ui, area, "spectrum") {
            self.hsl[0] = t.x * 360.0;
            self.hsl[1] = (1.0 - t.y) * 100.0;
            changed = true;
        }
        if let Some(t) = drag(ui, light, "luminance") {
            self.hsl[2] = (1.0 - t.y) * 100.0;
            changed = true;
        }
        if changed {
            self.sync_input();
        }
        let p = ui.painter();
        p.image(
            self.spectrum.id(),
            area,
            rect(0.0, 0.0, 1.0, 1.0),
            Color32::WHITE,
        );
        let mid = palette::color([self.hsl[0], self.hsl[1], 50.0]);
        gradient(
            p,
            rect(light.left(), light.top(), light.width(), 109.0),
            [Color32::WHITE, Color32::WHITE, mid, mid],
        );
        gradient(
            p,
            rect(light.left(), light.top() + 109.0, light.width(), 109.0),
            [mid, mid, Color32::BLACK, Color32::BLACK],
        );
        let center = area.min
            + vec2(
                self.hsl[0] / 360.0 * 249.0,
                (1.0 - self.hsl[1] / 100.0) * 223.0,
            );
        let crosshair = p.with_clip_rect(area);
        for (a, b) in [
            ([-16.0, 0.0], [-2.0, 0.0]),
            ([2.0, 0.0], [16.0, 0.0]),
            ([0.0, -16.0], [0.0, -2.0]),
            ([0.0, 2.0], [0.0, 16.0]),
        ] {
            crosshair.line_segment(
                [center + vec2(a[0], a[1]), center + vec2(b[0], b[1])],
                stroke(3.0, Color32::BLACK),
            );
        }
        let y = light.top() + (1.0 - self.hsl[2] / 100.0) * 217.0;
        marker(p, rect(282.0, y - 4.0, 40.0, 8.0), palette::color(self.hsl));
    }

    fn rgb_view(&mut self, ui: &mut egui::Ui) {
        let mut values = self.rgb().map(|v| v as f32);
        let rows = [
            ("Red:", 76.0, 94.0, 130.0),
            ("Green:", 166.0, 186.0, 220.0),
            ("Blue:", 258.0, 276.0, 308.0),
        ];
        let mut changed = false;
        for (i, &(label, y, field, bar)) in rows.iter().enumerate() {
            text(ui.painter(), 20.0, y, label);
            changed |= number(ui, [20.0, field], i, &mut values[i], 255.0);
            if let Some(t) = drag(ui, rect(21.0, bar, 294.0, 30.0), ("rgb", i)) {
                values[i] = (t.x * 255.0).round();
                changed = true;
            }
        }
        if changed {
            self.hsl = palette::from_rgb(values.map(|v| v.round() as u8));
            self.sync_input();
        }
        let rgb = self.rgb();
        for (i, &(_, _, _, y)) in rows.iter().enumerate() {
            let mut left = rgb;
            let mut right = rgb;
            left[i] = 0;
            right[i] = 255;
            let [a, b] = [left, right].map(|[r, g, b]| Color32::from_rgb(r, g, b));
            gradient(ui.painter(), rect(21.0, y, 294.0, 30.0), [a, b, b, a]);
            let x = 21.0 + rgb[i] as f32 / 255.0 * 293.0;
            marker(
                ui.painter(),
                rect(x - 3.0, y - 2.0, 7.0, 34.0),
                palette::color(self.hsl),
            );
        }
    }

    fn color_input(&mut self, ui: &mut egui::Ui) {
        text(
            ui.painter(),
            20.0,
            358.0,
            if self.index_mode {
                "Color:"
            } else {
                "RGB Color:"
            },
        );
        if input(ui, rect(20.0, 376.0, 252.0, 24.0), &mut self.input) {
            if self.index_mode {
                let index = match self.input.trim().to_ascii_uppercase().as_str() {
                    "BYLAYER" => Some(256),
                    "BYBLOCK" => Some(0),
                    value => value.parse::<u16>().ok().filter(|v| *v <= 256),
                };
                if let Some(index) = index {
                    self.select_index(index);
                    return;
                }
            }
            if let Some(rgb) = palette::parse_rgb(&self.input) {
                self.hsl = palette::from_rgb(rgb);
                self.selected = None;
            }
        }
    }

    fn rgb(&self) -> [u8; 3] {
        let c = palette::color(self.hsl);
        [c.r(), c.g(), c.b()]
    }

    fn sync_input(&mut self) {
        self.selected = None;
        self.input = palette::rgb_text(palette::color(self.hsl));
    }
}

impl eframe::App for ColorDialog {
    fn clear_color(&self, _: &egui::Visuals) -> [f32; 4] {
        [0.0; 4]
    }

    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(egui::Frame::none())
            .show(ctx, |ui| self.show(ui));
    }
}
