use super::*;

pub(super) struct IndexPicker {
    pub(super) selected: Option<u16>,
    input: String,
}

impl IndexPicker {
    pub(super) fn new(color: Color32) -> Self {
        Self {
            selected: None,
            input: palette::rgb_text(color),
        }
    }

    pub(super) fn sync(&mut self, color: Color32) {
        self.selected = None;
        self.input = palette::rgb_text(color);
    }

    pub(super) fn show(&mut self, ui: &mut egui::Ui, color: Color32) -> Option<[u8; 3]> {
        text(ui.painter(), 20.0, 74.0, "Color Index (ACI):");
        ui.painter()
            .rect_filled(rect(14.0, 90.0, 452.0, 167.0), 0.0, Color32::from_gray(244));
        let mut picked = None;
        for column in 0..24_u8 {
            for row in 0..10_u8 {
                let shade = if row < 5 {
                    8 - row * 2
                } else {
                    1 + (row - 5) * 2
                };
                let index = 10 + column * 10 + shade;
                let y = if row < 5 {
                    94.0 + row as f32 * 15.0
                } else {
                    178.0 + (row - 5) as f32 * 15.0
                };
                let cell = rect(18.0 + column as f32 * 18.5, y, 18.5, 15.0);
                if self.swatch(ui, cell, index) {
                    picked = Some(index as u16);
                }
            }
        }
        for (start, count, y) in [(1_u8, 9, 288.0), (250, 6, 320.0)] {
            ui.painter().rect_filled(
                rect(16.0, y - 4.0, count as f32 * 24.0 + 8.0, 31.0),
                0.0,
                Color32::from_gray(244),
            );
            for offset in 0..count {
                if self.swatch(
                    ui,
                    rect(20.0 + offset as f32 * 24.0, y, 24.0, 24.0),
                    start + offset,
                ) {
                    picked = Some((start + offset) as u16);
                }
            }
        }
        for (x, title, index) in [(259.0, "ByLayer", 256), (361.0, "ByBlock", 0)] {
            let area = rect(x, 287.0, 95.0, 24.0);
            let response = ui.interact(area, egui::Id::new(title), egui::Sense::click());
            ui.painter().rect(
                area,
                3.0,
                if response.hovered() {
                    Color32::from_rgb(229, 241, 251)
                } else {
                    Color32::from_gray(253)
                },
                Stroke::new(1.0_f32, Color32::from_gray(199)),
            );
            centered(ui.painter(), area, title);
            if response.clicked() {
                picked = Some(index);
            }
        }
        let mut changed = false;
        if let Some(index) = picked {
            self.selected = Some(index);
            self.input = self.selection_text(color);
            changed = true;
        }

        text(ui.painter(), 20.0, 358.0, "Color:");
        let area = rect(20.0, 376.0, 252.0, 24.0);
        ui.painter()
            .rect(area, 0.0, Color32::WHITE, Stroke::new(1.0_f32, BORDER));
        let response = ui.put(
            area,
            egui::TextEdit::singleline(&mut self.input)
                .id(egui::Id::new("index_input"))
                .font(font())
                .text_color(Color32::BLACK)
                .margin(vec2(2.0, 3.0))
                .frame(false),
        );
        let mut rgb = None;
        let valid = if let Some(index) = parse_index(&self.input) {
            if response.changed() {
                self.selected = Some(index);
                changed = true;
            }
            true
        } else if let Some(value) = palette::parse_rgb(&self.input) {
            if response.changed() {
                self.selected = None;
                rgb = Some(value);
            }
            true
        } else {
            false
        };
        if response.lost_focus() {
            self.input = self.selection_text(
                rgb.map(|[r, g, b]| Color32::from_rgb(r, g, b))
                    .unwrap_or(color),
            );
        }
        ui.painter().line_segment(
            [area.left_bottom(), area.right_bottom()],
            Stroke::new(
                1.0_f32,
                if !valid {
                    Color32::RED
                } else if response.has_focus() {
                    Color32::from_rgb(0, 99, 177)
                } else {
                    Color32::from_gray(140)
                },
            ),
        );
        if changed {
            if let Some(index @ 1..=255) = self.selected {
                let c = palette::aci(index as u8);
                rgb = Some([c.r(), c.g(), c.b()]);
            }
            ui.ctx().request_repaint();
        }

        let preview = match self.selected {
            Some(0 | 256) => Color32::WHITE,
            Some(index) => palette::aci(index as u8),
            None => rgb
                .map(|[r, g, b]| Color32::from_rgb(r, g, b))
                .unwrap_or(color),
        };
        let p = ui.painter();
        let outline = Stroke::new(1.0_f32, Color32::from_rgb(170, 173, 186));
        p.rect(rect(392.0, 324.0, 42.0, 42.0), 0.0, Color32::WHITE, outline);
        p.rect(rect(348.0, 338.0, 70.0, 64.0), 0.0, Color32::WHITE, outline);
        p.rect_filled(rect(350.0, 340.0, 66.0, 60.0), 0.0, preview);
        rgb
    }

    fn swatch(&self, ui: &egui::Ui, area: Rect, index: u8) -> bool {
        let response = ui
            .interact(area, egui::Id::new(("aci", index)), egui::Sense::click())
            .on_hover_text(index.to_string());
        let p = ui.painter();
        p.rect_filled(area.shrink(0.5), 0.0, palette::aci(index));
        if response.hovered() || self.selected == Some(index as u16) {
            p.rect_stroke(area.shrink(1.0), 0.0, Stroke::new(1.0_f32, Color32::BLACK));
            p.rect_stroke(area.shrink(2.0), 0.0, Stroke::new(1.0_f32, Color32::WHITE));
        }
        response.clicked()
    }

    fn selection_text(&self, color: Color32) -> String {
        match self.selected {
            Some(0) => "BYBLOCK".into(),
            Some(256) => "BYLAYER".into(),
            Some(index) => index.to_string(),
            None => palette::rgb_text(color),
        }
    }
}

fn parse_index(input: &str) -> Option<u16> {
    match input.trim().to_ascii_uppercase().as_str() {
        "BYLAYER" => Some(256),
        "BYBLOCK" => Some(0),
        value => value.parse::<u16>().ok().filter(|index| *index <= 256),
    }
}
