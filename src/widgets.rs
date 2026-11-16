use eframe::egui::{self, pos2, vec2, Align2, Color32, FontFamily, FontId, Painter, Rect, Stroke};

pub const PANEL: Color32 = Color32::from_rgb(248, 248, 248);

pub fn rect(x: f32, y: f32, width: f32, height: f32) -> Rect {
    Rect::from_min_size(pos2(x, y), vec2(width, height))
}

pub fn stroke(width: f32, color: Color32) -> Stroke {
    Stroke::new(width, color)
}

fn font() -> FontId {
    FontId::new(13.0, FontFamily::Name("body".into()))
}

pub fn text(p: &Painter, x: f32, y: f32, value: &str) {
    p.text(pos2(x, y), Align2::LEFT_TOP, value, font(), Color32::BLACK);
}

pub fn centered(p: &Painter, area: Rect, value: &str) {
    p.text(
        area.center(),
        Align2::CENTER_CENTER,
        value,
        font(),
        Color32::BLACK,
    );
}

pub fn panel(p: &Painter, area: Rect, color: Color32, radius: f32) {
    p.rect(area, radius, color, stroke(1.0, Color32::from_gray(210)));
}

pub fn button(ui: &egui::Ui, area: Rect, title: &str) -> bool {
    let response = ui.interact(
        area,
        ui.id()
            .with((title, area.min.x.to_bits(), area.min.y.to_bits())),
        egui::Sense::click(),
    );
    let fill = if response.is_pointer_button_down_on() {
        Color32::from_rgb(204, 228, 247)
    } else if response.hovered() {
        Color32::from_rgb(229, 241, 251)
    } else {
        Color32::from_gray(253)
    };
    panel(ui.painter(), area, fill, 3.0);
    centered(ui.painter(), area, title);
    response.clicked()
}

pub fn input(ui: &mut egui::Ui, area: Rect, value: &mut String) -> bool {
    panel(ui.painter(), area, Color32::WHITE, 0.0);
    ui.put(
        area,
        egui::TextEdit::singleline(value)
            .font(font())
            .text_color(Color32::BLACK)
            .margin(vec2(2.0, 3.0))
            .frame(false),
    )
    .changed()
}

pub fn number(ui: &mut egui::Ui, [x, y]: [f32; 2], id: usize, value: &mut f32, max: f32) -> bool {
    panel(ui.painter(), rect(x, y, 61.0, 23.0), Color32::WHITE, 0.0);
    let mut changed = ui
        .push_id(id, |ui| {
            ui.style_mut().override_font_id = Some(font());
            ui.visuals_mut().widgets.inactive.weak_bg_fill = Color32::WHITE;
            ui.put(
                rect(x + 1.0, y + 1.0, 39.0, 21.0),
                egui::DragValue::new(value)
                    .range(0.0..=max)
                    .speed(1.0)
                    .fixed_decimals(0),
            )
            .changed()
        })
        .inner;
    for (offset, step) in [(2.0, 1.0), (12.0, -1.0)] {
        let area = rect(x + 41.0, y + offset, 18.0, 9.0);
        if button(ui, area, "") {
            *value = (value.round() + step).clamp(0.0, max);
            changed = true;
        }
        let c = area.center();
        ui.painter().add(egui::Shape::convex_polygon(
            vec![
                c + vec2(-3.0, step * 1.5),
                c + vec2(3.0, step * 1.5),
                c + vec2(0.0, -step * 2.0),
            ],
            Color32::from_gray(55),
            Stroke::NONE,
        ));
    }
    changed
}

// Tọa độ chuột quy về 0..1, đủ cho tương tác preview.
pub fn drag(ui: &egui::Ui, area: Rect, id: impl std::hash::Hash) -> Option<egui::Vec2> {
    let response = ui.interact(area, egui::Id::new(id), egui::Sense::click_and_drag());
    if response.clicked() || response.is_pointer_button_down_on() {
        response.interact_pointer_pos().map(|p| {
            let t = (p - area.min) / (area.size() - vec2(1.0, 1.0));
            vec2(t.x.clamp(0.0, 1.0), t.y.clamp(0.0, 1.0))
        })
    } else {
        None
    }
}

pub fn gradient(p: &Painter, area: Rect, colors: [Color32; 4]) {
    let mut mesh = egui::Mesh::default();
    for (point, color) in [
        area.left_top(),
        area.right_top(),
        area.right_bottom(),
        area.left_bottom(),
    ]
    .into_iter()
    .zip(colors)
    {
        mesh.colored_vertex(point, color);
    }
    mesh.add_triangle(0, 1, 2);
    mesh.add_triangle(0, 2, 3);
    p.add(mesh);
}

pub fn marker(p: &Painter, area: Rect, color: Color32) {
    p.rect(area, 0.0, Color32::WHITE, stroke(1.0, Color32::BLACK));
    p.rect_filled(area.shrink(2.0), 0.0, color);
}

pub fn preview(p: &Painter, color: Color32) {
    panel(p, rect(392.0, 324.0, 42.0, 42.0), Color32::WHITE, 0.0);
    panel(p, rect(348.0, 338.0, 70.0, 64.0), Color32::WHITE, 0.0);
    p.rect_filled(rect(350.0, 340.0, 66.0, 60.0), 0.0, color);
}
