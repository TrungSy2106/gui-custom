use super::*;

impl ColorDialog {
    pub(super) fn index_view(&mut self, ui: &mut egui::Ui) {
        text(ui.painter(), 20.0, 74.0, "Color Index (ACI):");
        ui.painter()
            .rect_filled(rect(14.0, 90.0, 452.0, 167.0), 0.0, Color32::from_gray(244));
        for column in 0..24_u8 {
            for row in 0..10_u8 {
                let shade = [8, 6, 4, 2, 0, 1, 3, 5, 7, 9][row as usize];
                let y = if row < 5 {
                    94.0 + row as f32 * 15.0
                } else {
                    178.0 + (row - 5) as f32 * 15.0
                };
                self.swatch(
                    ui,
                    rect(18.0 + column as f32 * 18.5, y, 18.5, 15.0),
                    10 + column * 10 + shade,
                );
            }
        }
        for (start, count, y) in [(1_u8, 9, 288.0), (250, 6, 320.0)] {
            for offset in 0..count {
                self.swatch(
                    ui,
                    rect(20.0 + offset as f32 * 24.0, y, 24.0, 24.0),
                    start + offset,
                );
            }
        }
        for (x, label, index) in [(259.0, "ByLayer", 256), (361.0, "ByBlock", 0)] {
            if button(ui, rect(x, 287.0, 95.0, 24.0), label) {
                self.select_index(index);
            }
        }
    }

    fn swatch(&mut self, ui: &egui::Ui, area: Rect, index: u8) {
        let p = ui.painter();
        let area = p.round_rect_to_pixels(area);
        let response = ui
            .interact(area, egui::Id::new(("aci", index)), egui::Sense::click())
            .on_hover_text(index.to_string());
        let pixel = 1.0 / ui.ctx().pixels_per_point();
        solid_rect(p, area, Color32::WHITE);
        let fill = Rect::from_min_max(area.min, area.max - vec2(pixel, pixel));
        solid_rect(p, fill, palette::aci(index));
        if response.hovered() || self.selected == Some(index as u16) {
            p.rect_stroke(fill.shrink(pixel * 0.5), 0.0, stroke(pixel, Color32::BLACK));
            p.rect_stroke(fill.shrink(pixel * 1.5), 0.0, stroke(pixel, Color32::WHITE));
        }
        if response.clicked() {
            self.select_index(index as u16);
        }
    }

    pub(super) fn select_index(&mut self, index: u16) {
        self.selected = Some(index);
        if (1..=255).contains(&index) {
            let c = palette::aci(index as u8);
            self.hsl = palette::from_rgb([c.r(), c.g(), c.b()]);
        }
        self.input = self.selection_text();
    }

    pub(super) fn selection_text(&self) -> String {
        match (self.index_mode, self.selected) {
            (true, Some(0)) => "BYBLOCK".into(),
            (true, Some(256)) => "BYLAYER".into(),
            (true, Some(index)) => index.to_string(),
            _ => palette::rgb_text(palette::color(self.hsl)),
        }
    }
}
