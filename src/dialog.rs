use crate::palette;
use eframe::egui::{self, pos2, vec2, Align2, Color32, FontFamily, FontId, Painter, Rect, Stroke};

pub const SIZE: [f32; 2] = [482.0, 465.0];
const PANEL: Color32 = Color32::from_rgb(248, 248, 248);
const BORDER: Color32 = Color32::from_rgb(224, 224, 224);

#[path = "index.rs"]
mod index;

pub struct ColorDialog {
    spectrum: egui::TextureHandle,
    luminance: egui::TextureHandle,
    hsl: [f32; 3],
    cached_hs: [f32; 2],
    rgb_input: String,
    rgb_mode: bool,
    index_mode: bool,
    index: index::IndexPicker,
}

impl ColorDialog {
    pub fn new(ctx: &egui::Context) -> Self {
        let hsl = palette::from_rgb([82, 101, 173]);
        Self {
            spectrum: ctx.load_texture(
                "spectrum",
                crate::palette::spectrum(),
                egui::TextureOptions::NEAREST,
            ),
            luminance: ctx.load_texture(
                "luminance",
                palette::luminance(hsl[0], hsl[1]),
                egui::TextureOptions::NEAREST,
            ),
            hsl,
            cached_hs: [hsl[0], hsl[1]],
            rgb_input: "82,101,173".into(),
            rgb_mode: false,
            index_mode: true,
            index: index::IndexPicker::new(palette::color(hsl)),
        }
    }

    fn content(&mut self, ui: &mut egui::Ui) {
        let p = ui.painter().clone();
        p.rect(
            rect(0.0, 0.0, 482.0, 465.0),
            10.0,
            Color32::from_gray(240),
            Stroke::new(1.0_f32, Color32::from_gray(190)),
        );
        self.header(ui);
        self.tabs(ui);
        if self.index_mode {
            if let Some(rgb) = self.index.show(ui, palette::color(self.hsl)) {
                self.hsl = palette::from_rgb(rgb);
                self.rgb_input = palette::rgb_text(palette::color(self.hsl));
            }
        } else {
            self.fields(ui);
            if self.rgb_mode {
                self.rgb_picker(ui);
            } else {
                self.picker(ui);
            }
            self.rgb_field(ui);
            self.details(&p);
        }
        self.footer(ui);
    }

    fn header(&self, ui: &egui::Ui) {
        let p = ui.painter();
        p.rect_filled(
            rect(1.0, 1.0, 480.0, 35.0),
            egui::Rounding {
                nw: 9.0,
                ne: 9.0,
                sw: 0.0,
                se: 0.0,
            },
            Color32::from_gray(243),
        );
        p.text(
            pos2(12.0, 8.0),
            Align2::LEFT_TOP,
            "Select Color",
            FontId::new(14.0, FontFamily::Name("title".into())),
            Color32::BLACK,
        );

        let close = rect(438.0, 1.0, 43.0, 34.0);
        let response = ui.interact(close, egui::Id::new("close"), egui::Sense::click());
        if response.hovered() {
            p.rect_filled(close.shrink(1.0), 3.0, Color32::from_rgb(232, 17, 35));
        }
        let ink = if response.hovered() {
            Color32::WHITE
        } else {
            Color32::BLACK
        };
        p.line_segment(
            [pos2(455.0, 12.0), pos2(466.0, 23.0)],
            Stroke::new(1.0_f32, ink),
        );
        p.line_segment(
            [pos2(466.0, 12.0), pos2(455.0, 23.0)],
            Stroke::new(1.0_f32, ink),
        );
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

    fn tabs(&mut self, ui: &egui::Ui) {
        let tabs = [
            (8.0, 118.0, "Index Color"),
            (126.0, 124.0, "True Color"),
            (250.0, 115.0, "Color Books"),
        ];
        for (tab, &(x, width, title)) in tabs.iter().enumerate().take(2) {
            if ui
                .interact(
                    rect(x, 49.0, width, 24.0),
                    egui::Id::new(title),
                    egui::Sense::click(),
                )
                .clicked()
            {
                self.index_mode = tab == 0;
                ui.memory_mut(|memory| {
                    memory.surrender_focus(egui::Id::new(if self.index_mode {
                        "rgb_input"
                    } else {
                        "index_input"
                    }))
                });
            }
        }
        let p = ui.painter();
        for &(x, width, _) in &tabs {
            p.rect(
                rect(x, 51.0, width, 22.0),
                0.0,
                Color32::from_gray(243),
                Stroke::new(1.0_f32, BORDER),
            );
        }
        p.rect(
            rect(6.0, 72.0, 466.0, 344.0),
            0.0,
            PANEL,
            Stroke::new(1.0_f32, BORDER),
        );
        let (x, width, _) = tabs[if self.index_mode { 0 } else { 1 }];
        p.rect(
            rect(x, 49.0, width, 24.0),
            0.0,
            PANEL,
            Stroke::new(1.0_f32, BORDER),
        );
        p.line_segment(
            [pos2(x + 0.5, 73.0), pos2(x + width - 0.5, 73.0)],
            Stroke::new(2.0_f32, PANEL),
        );
        for (tab, &(x, width, title)) in tabs.iter().enumerate() {
            let y = if (tab == 0 && self.index_mode) || (tab == 1 && !self.index_mode) {
                49.0
            } else {
                51.0
            };
            centered(p, rect(x, y, width, 22.0), title);
        }
    }

    fn fields(&mut self, ui: &mut egui::Ui) {
        text(ui.painter(), 344.0, 78.0, "Color model:");
        ui.allocate_new_ui(
            egui::UiBuilder::new().max_rect(rect(344.0, 96.0, 117.0, 24.0)),
            |ui| {
                egui::ComboBox::from_id_salt("color_model")
                    .selected_text(if self.rgb_mode { "RGB" } else { "HSL" })
                    .width(117.0)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.rgb_mode, false, "HSL");
                        ui.selectable_value(&mut self.rgb_mode, true, "RGB");
                    });
            },
        );
        if self.rgb_mode {
            return;
        }
        let mut values = self.hsl;
        let labels = ["Hue:", "Saturation:", "Luminance:"];
        let mut changed = false;
        for (index, x) in [20.0, 98.0, 256.0].into_iter().enumerate() {
            text(ui.painter(), x, 76.0, labels[index]);
            let max = if index == 0 { 360.0 } else { 100.0 };
            changed |= number_field(
                ui,
                rect(x, 95.0, 61.0, 23.0),
                index,
                &mut values[index],
                max,
            );
        }
        if changed {
            self.hsl = values;
            self.sync_rgb();
        }
    }

    fn picker(&mut self, ui: &mut egui::Ui) {
        let spectrum = rect(21.0, 131.0, 250.0, 224.0);
        let lightness = rect(284.0, 136.0, 36.0, 218.0);
        let response = ui.interact(
            spectrum,
            egui::Id::new("spectrum"),
            egui::Sense::click_and_drag(),
        );
        if response.clicked() || response.is_pointer_button_down_on() {
            if let Some(point) = response.interact_pointer_pos() {
                self.hsl[0] = ((point.x - spectrum.left()) / (spectrum.width() - 1.0))
                    .clamp(0.0, 1.0)
                    * 360.0;
                self.hsl[1] = (1.0 - (point.y - spectrum.top()) / (spectrum.height() - 1.0))
                    .clamp(0.0, 1.0)
                    * 100.0;
                self.sync_rgb();
            }
        }
        let response = ui.interact(
            lightness.expand2(vec2(3.0, 0.0)),
            egui::Id::new("lightness"),
            egui::Sense::click_and_drag(),
        );
        if response.clicked() || response.is_pointer_button_down_on() {
            if let Some(point) = response.interact_pointer_pos() {
                self.hsl[2] = (1.0 - (point.y - lightness.top()) / (lightness.height() - 1.0))
                    .clamp(0.0, 1.0)
                    * 100.0;
                self.sync_rgb();
            }
        }

        self.update_luminance();
        self.paint_hsl(ui.painter(), spectrum, lightness);
    }

    fn rgb_picker(&mut self, ui: &mut egui::Ui) {
        let color = palette::color(self.hsl);
        let mut values = [color.r() as f32, color.g() as f32, color.b() as f32];
        let rows = [
            ("Red:", 76.0, 94.0, 130.0),
            ("Green:", 166.0, 186.0, 220.0),
            ("Blue:", 258.0, 276.0, 308.0),
        ];
        let mut changed = false;
        for (channel, &(label, label_y, field_y, bar_y)) in rows.iter().enumerate() {
            text(ui.painter(), 20.0, label_y, label);
            changed |= number_field(
                ui,
                rect(20.0, field_y, 61.0, 23.0),
                channel,
                &mut values[channel],
                255.0,
            );
            let bar = rect(21.0, bar_y, 294.0, 30.0);
            let response = ui.interact(
                bar.expand2(vec2(0.0, 3.0)),
                egui::Id::new(("rgb_bar", channel)),
                egui::Sense::click_and_drag(),
            );
            if response.clicked() || response.is_pointer_button_down_on() {
                if let Some(point) = response.interact_pointer_pos() {
                    values[channel] = ((point.x - bar.left()) / (bar.width() - 1.0) * 255.0)
                        .round()
                        .clamp(0.0, 255.0);
                    changed = true;
                }
            }
        }
        if changed {
            self.hsl = palette::from_rgb(values.map(|v| v.round() as u8));
            self.sync_rgb();
            ui.ctx().request_repaint();
        }

        // Mỗi thanh chỉ thay đổi một kênh, giữ nguyên hai kênh còn lại.
        let rgb = values.map(|v| v.round() as u8);
        for (channel, &(_, _, _, bar_y)) in rows.iter().enumerate() {
            let bar = rect(21.0, bar_y, 294.0, 30.0);
            let mut left = rgb;
            let mut right = rgb;
            left[channel] = 0;
            right[channel] = 255;
            let p = ui.painter();
            p.rect_filled(bar.expand2(vec2(10.0, 4.0)), 0.0, Color32::from_gray(244));
            let mut mesh = egui::Mesh::default();
            let start = Color32::from_rgb(left[0], left[1], left[2]);
            let end = Color32::from_rgb(right[0], right[1], right[2]);
            mesh.colored_vertex(bar.left_top(), start);
            mesh.colored_vertex(bar.right_top(), end);
            mesh.colored_vertex(bar.right_bottom(), end);
            mesh.colored_vertex(bar.left_bottom(), start);
            mesh.add_triangle(0, 1, 2);
            mesh.add_triangle(0, 2, 3);
            p.add(mesh);
            let x = bar.left() + rgb[channel] as f32 / 255.0 * (bar.width() - 1.0);
            let marker = rect(x - 3.0, bar.top() - 2.0, 7.0, bar.height() + 4.0);
            p.rect(
                marker,
                0.0,
                Color32::WHITE,
                Stroke::new(1.0_f32, Color32::from_rgb(25, 32, 54)),
            );
            p.rect_filled(marker.shrink(2.0), 0.0, palette::color(self.hsl));
        }
    }

    fn rgb_field(&mut self, ui: &mut egui::Ui) {
        text(ui.painter(), 20.0, 358.0, "RGB Color:");
        let input = rect(20.0, 376.0, 252.0, 24.0);
        ui.painter()
            .rect(input, 0.0, Color32::WHITE, Stroke::new(1.0_f32, BORDER));
        let response = ui.put(
            input,
            egui::TextEdit::singleline(&mut self.rgb_input)
                .id(egui::Id::new("rgb_input"))
                .font(font())
                .text_color(Color32::BLACK)
                .margin(vec2(2.0, 3.0))
                .frame(false),
        );
        if response.changed() {
            if let Some(rgb) = palette::parse_rgb(&self.rgb_input) {
                self.hsl = palette::from_rgb(rgb);
                self.index.sync(palette::color(self.hsl));
            }
        }
        if response.lost_focus() {
            self.sync_rgb();
        }
        let valid = palette::parse_rgb(&self.rgb_input).is_some();
        ui.painter().line_segment(
            [input.left_bottom(), input.right_bottom()],
            Stroke::new(
                if response.has_focus() || !valid {
                    2.0_f32
                } else {
                    1.0_f32
                },
                if !valid {
                    Color32::RED
                } else if response.has_focus() {
                    Color32::from_rgb(0, 99, 177)
                } else {
                    Color32::from_gray(140)
                },
            ),
        );
        self.update_luminance();
    }

    fn update_luminance(&mut self) {
        let hs = [self.hsl[0], self.hsl[1]];
        if hs != self.cached_hs {
            self.luminance.set(
                palette::luminance(hs[0], hs[1]),
                egui::TextureOptions::NEAREST,
            );
            self.cached_hs = hs;
        }
    }

    fn paint_hsl(&self, p: &Painter, spectrum: Rect, lightness: Rect) {
        let uv = rect(0.0, 0.0, 1.0, 1.0);
        p.image(
            self.spectrum.id(),
            rect(21.0, 131.0, 250.0, 224.0),
            uv,
            Color32::WHITE,
        );
        p.rect_filled(
            rect(282.0, 130.0, 42.0, 242.0),
            0.0,
            Color32::from_gray(244),
        );
        p.image(
            self.luminance.id(),
            rect(284.0, 136.0, 36.0, 218.0),
            uv,
            Color32::WHITE,
        );
        let center = pos2(
            spectrum.left() + self.hsl[0] / 360.0 * (spectrum.width() - 1.0),
            spectrum.top() + (1.0 - self.hsl[1] / 100.0) * (spectrum.height() - 1.0),
        );
        let crosshair = p.with_clip_rect(spectrum);
        for (a, b) in [
            ([0.0, -16.0], [0.0, -2.0]),
            ([0.0, 2.0], [0.0, 16.0]),
            ([-16.0, 0.0], [-2.0, 0.0]),
            ([2.0, 0.0], [16.0, 0.0]),
        ] {
            crosshair.line_segment(
                [center + vec2(a[0], a[1]), center + vec2(b[0], b[1])],
                Stroke::new(3.0_f32, Color32::BLACK),
            );
        }
        let marker_y = lightness.top() + (1.0 - self.hsl[2] / 100.0) * (lightness.height() - 1.0);
        p.rect(
            rect(282.0, marker_y - 4.0, 40.0, 8.0),
            0.0,
            Color32::WHITE,
            Stroke::new(1.0_f32, Color32::BLACK),
        );
        p.rect_filled(
            rect(284.0, marker_y - 2.0, 36.0, 4.0),
            0.0,
            palette::color(self.hsl),
        );
    }

    fn sync_rgb(&mut self) {
        self.rgb_input = palette::rgb_text(palette::color(self.hsl));
        self.index.sync(palette::color(self.hsl));
    }

    fn details(&self, p: &Painter) {
        p.line_segment(
            [pos2(335.0, 86.0), pos2(335.0, 404.0)],
            Stroke::new(2.0_f32, Color32::WHITE),
        );
        text(p, 344.0, 134.0, "True Color Stored");
        text(p, 344.0, 150.0, "as RGB:");
        let color = palette::color(self.hsl);
        for (y, label, value) in [
            (180.0, "Red:", color.r()),
            (200.0, "Green:", color.g()),
            (220.0, "Blue:", color.b()),
        ] {
            text(p, 360.0, y, label);
            text(p, 416.0, y, &value.to_string());
        }
        let outline = Stroke::new(1.0_f32, Color32::from_rgb(170, 173, 186));
        p.rect(rect(392.0, 324.0, 42.0, 42.0), 0.0, Color32::WHITE, outline);
        p.rect(rect(348.0, 338.0, 70.0, 64.0), 0.0, Color32::WHITE, outline);
        p.rect_filled(rect(350.0, 340.0, 66.0, 60.0), 0.0, color);
    }

    fn footer(&self, ui: &egui::Ui) {
        let p = ui.painter();
        for (x, title) in [(159.0, "OK"), (263.0, "Cancel"), (367.0, "Help")] {
            let response = ui.interact(
                rect(x, 428.0, 97.0, 26.0),
                egui::Id::new(title),
                egui::Sense::click(),
            );
            if response.clicked() && title != "Help" {
                ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
            }
            p.rect(
                rect(x, 428.0, 97.0, 26.0),
                3.0,
                if response.hovered() {
                    Color32::from_rgb(229, 241, 251)
                } else {
                    Color32::from_rgb(253, 253, 253)
                },
                Stroke::new(1.0_f32, Color32::from_gray(199)),
            );
            centered(p, rect(x, 428.0, 97.0, 26.0), title);
        }
        p.line_segment(
            [pos2(404.0, 446.5), pos2(413.0, 446.5)],
            Stroke::new(0.7_f32, Color32::BLACK),
        );
    }
}

impl eframe::App for ColorDialog {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        [0.0; 4]
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(egui::Frame::none())
            .show(ctx, |ui| self.content(ui));
    }
}

// Các helper dùng chung; mọi tọa độ phía trên tính từ góc trái dialog.
fn rect(x: f32, y: f32, width: f32, height: f32) -> Rect {
    Rect::from_min_size(pos2(x, y), vec2(width, height))
}

fn font() -> FontId {
    FontId::new(13.0, FontFamily::Name("body".into()))
}

fn text(p: &Painter, x: f32, y: f32, value: &str) {
    p.text(pos2(x, y), Align2::LEFT_TOP, value, font(), Color32::BLACK);
}

fn centered(p: &Painter, area: Rect, value: &str) {
    p.text(
        area.center(),
        Align2::CENTER_CENTER,
        value,
        font(),
        Color32::BLACK,
    );
}

fn number_field(ui: &mut egui::Ui, area: Rect, id: usize, value: &mut f32, max: f32) -> bool {
    let p = ui.painter().clone();
    p.rect(
        area,
        0.0,
        Color32::WHITE,
        Stroke::new(1.0_f32, Color32::from_gray(171)),
    );
    let mut changed = false;
    ui.scope(|ui| {
        ui.style_mut().override_font_id = Some(font());
        let visuals = ui.visuals_mut();
        visuals.override_text_color = Some(Color32::BLACK);
        for widget in [
            &mut visuals.widgets.inactive,
            &mut visuals.widgets.hovered,
            &mut visuals.widgets.active,
        ] {
            widget.bg_fill = Color32::WHITE;
            widget.weak_bg_fill = Color32::WHITE;
            widget.bg_stroke = Stroke::NONE;
        }
        changed |= ui
            .put(
                rect(
                    area.left() + 1.0,
                    area.top() + 1.0,
                    area.width() - 22.0,
                    area.height() - 2.0,
                ),
                egui::DragValue::new(value)
                    .range(0.0..=max)
                    .speed(1.0)
                    .fixed_decimals(0),
            )
            .changed();
    });
    for (offset, up) in [(2.0, true), (12.0, false)] {
        let button = rect(area.right() - 20.0, area.top() + offset, 18.0, 9.0);
        p.rect(
            button,
            0.0,
            Color32::from_gray(244),
            Stroke::new(1.0_f32, Color32::from_gray(188)),
        );
        if ui
            .interact(
                button,
                egui::Id::new(("step", id, up)),
                egui::Sense::click(),
            )
            .clicked()
        {
            *value = (value.round() + if up { 1.0 } else { -1.0 }).clamp(0.0, max);
            changed = true;
        }
        let c = button.center();
        let direction = if up { 1.0 } else { -1.0 };
        p.add(egui::Shape::convex_polygon(
            vec![
                c + vec2(-3.0, direction * 1.5),
                c + vec2(3.0, direction * 1.5),
                c + vec2(0.0, -direction * 2.0),
            ],
            Color32::from_gray(55),
            Stroke::NONE,
        ));
    }
    changed
}

#[cfg(test)]
#[path = "dialog_tests.rs"]
mod tests;
