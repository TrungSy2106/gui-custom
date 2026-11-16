use super::*;

fn frame(ctx: &egui::Context, dialog: &mut ColorDialog, events: Vec<egui::Event>) {
    let input = egui::RawInput {
        screen_rect: Some(rect(0.0, 0.0, SIZE[0], SIZE[1])),
        events,
        ..Default::default()
    };
    let _ = ctx.run(input, |ctx| {
        egui::CentralPanel::default()
            .frame(egui::Frame::none())
            .show(ctx, |ui| dialog.content(ui));
    });
}

fn pointer(x: f32, y: f32, pressed: bool) -> egui::Event {
    egui::Event::PointerButton {
        pos: pos2(x, y),
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: egui::Modifiers::NONE,
    }
}

fn click(ctx: &egui::Context, dialog: &mut ColorDialog, x: f32, y: f32) {
    frame(
        ctx,
        dialog,
        vec![
            egui::Event::PointerMoved(pos2(x, y)),
            pointer(x, y, true),
            pointer(x, y, false),
        ],
    );
}

#[test]
fn choose_drag_and_edit_color() {
    let ctx = egui::Context::default();
    crate::load_fonts(&ctx);
    let mut dialog = ColorDialog::new(&ctx);
    dialog.index_mode = false;
    frame(&ctx, &mut dialog, vec![]);

    // Một click nhanh vẫn đổi màu, dù press/release đến trong cùng frame.
    frame(
        &ctx,
        &mut dialog,
        vec![
            egui::Event::PointerMoved(pos2(21.0, 131.0)),
            pointer(21.0, 131.0, true),
            pointer(21.0, 131.0, false),
        ],
    );
    assert_eq!(dialog.hsl[0..2], [0.0, 100.0]);
    assert_eq!(dialog.rgb_input, "255,0,0");

    // Kéo thanh sáng ra ngoài vẫn giới hạn ở màu đen/trắng.
    frame(
        &ctx,
        &mut dialog,
        vec![
            egui::Event::PointerMoved(pos2(300.0, 245.0)),
            pointer(300.0, 245.0, true),
        ],
    );
    frame(
        &ctx,
        &mut dialog,
        vec![egui::Event::PointerMoved(pos2(300.0, 420.0))],
    );
    assert_eq!(dialog.rgb_input, "0,0,0");
    frame(
        &ctx,
        &mut dialog,
        vec![egui::Event::PointerMoved(pos2(300.0, 100.0))],
    );
    assert_eq!(dialog.rgb_input, "255,255,255");
    frame(&ctx, &mut dialog, vec![pointer(300.0, 100.0, false)]);

    ctx.memory_mut(|memory| memory.request_focus(egui::Id::new("rgb_input")));
    dialog.rgb_input.clear();
    frame(&ctx, &mut dialog, vec![egui::Event::Text("0,255,0".into())]);
    assert_eq!(palette::color(dialog.hsl), Color32::GREEN);
    assert_eq!(dialog.cached_hs, [120.0, 100.0]);

    let before = dialog.hsl;
    frame(
        &ctx,
        &mut dialog,
        vec![
            egui::Event::PointerMoved(pos2(149.0, 112.0)),
            pointer(149.0, 112.0, true),
            pointer(149.0, 112.0, false),
        ],
    );
    assert_eq!(dialog.hsl[0], before[0]);
    assert_eq!(dialog.hsl[1], 99.0);
}

#[test]
fn rgb_sliders_preserve_other_channels_and_sync_inputs() {
    let ctx = egui::Context::default();
    crate::load_fonts(&ctx);
    let mut dialog = ColorDialog::new(&ctx);
    dialog.index_mode = false;
    dialog.rgb_mode = true;
    frame(&ctx, &mut dialog, vec![]);
    assert_eq!(dialog.rgb_input, "82,101,173");

    for (y, expected) in [
        (145.0, "255,101,173"),
        (235.0, "255,255,173"),
        (323.0, "255,255,255"),
    ] {
        frame(
            &ctx,
            &mut dialog,
            vec![
                egui::Event::PointerMoved(pos2(314.0, y)),
                pointer(314.0, y, true),
                pointer(314.0, y, false),
            ],
        );
        assert_eq!(dialog.rgb_input, expected);
    }

    // Kéo ra ngoài thanh vẫn giới hạn kênh đang chọn ở 0..255.
    frame(
        &ctx,
        &mut dialog,
        vec![
            egui::Event::PointerMoved(pos2(100.0, 145.0)),
            pointer(100.0, 145.0, true),
        ],
    );
    frame(
        &ctx,
        &mut dialog,
        vec![egui::Event::PointerMoved(pos2(-20.0, 145.0))],
    );
    assert_eq!(dialog.rgb_input, "0,255,255");
    frame(&ctx, &mut dialog, vec![pointer(-20.0, 145.0, false)]);

    // Nút giảm Green chỉ giảm Green.
    frame(
        &ctx,
        &mut dialog,
        vec![
            egui::Event::PointerMoved(pos2(70.0, 203.0)),
            pointer(70.0, 203.0, true),
            pointer(70.0, 203.0, false),
        ],
    );
    assert_eq!(dialog.rgb_input, "0,254,255");

    ctx.memory_mut(|memory| memory.request_focus(egui::Id::new("rgb_input")));
    dialog.rgb_input.clear();
    frame(
        &ctx,
        &mut dialog,
        vec![egui::Event::Text("82,101,173".into())],
    );
    assert_eq!(palette::color(dialog.hsl), Color32::from_rgb(82, 101, 173));
    for rgb_mode in [false, true] {
        dialog.rgb_mode = rgb_mode;
        frame(&ctx, &mut dialog, vec![]);
        assert_eq!(dialog.rgb_input, "82,101,173");
    }
}

#[test]
fn index_palette_special_modes_and_tab_sync() {
    let ctx = egui::Context::default();
    crate::load_fonts(&ctx);
    let mut dialog = ColorDialog::new(&ctx);
    frame(&ctx, &mut dialog, vec![]);

    // Hai khối màu, màu cơ bản và thang xám đều chọn được đúng mã ACI.
    for (x, y, index) in [
        (26.0, 100.0, 18),
        (26.0, 160.0, 10),
        (26.0, 184.0, 11),
        (26.0, 244.0, 19),
        (80.0, 300.0, 3),
        (32.0, 332.0, 250),
        (152.0, 332.0, 255),
    ] {
        click(&ctx, &mut dialog, x, y);
        assert_eq!(dialog.index.selected, Some(index));
        assert_eq!(palette::color(dialog.hsl), palette::aci(index as u8));
    }
    click(&ctx, &mut dialog, 300.0, 299.0);
    assert_eq!(dialog.index.selected, Some(256));
    click(&ctx, &mut dialog, 410.0, 299.0);
    assert_eq!(dialog.index.selected, Some(0));

    click(&ctx, &mut dialog, 50.0, 388.0);
    // Chọn toàn bộ nội dung bằng phím tắt rồi nhập chỉ số.
    frame(
        &ctx,
        &mut dialog,
        vec![
            egui::Event::Key {
                key: egui::Key::A,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers {
                    ctrl: true,
                    command: true,
                    ..Default::default()
                },
            },
            egui::Event::Text("140".into()),
        ],
    );
    assert_eq!(dialog.index.selected, Some(140));
    let selected = palette::aci(140);
    click(&ctx, &mut dialog, 180.0, 60.0);
    assert!(!dialog.index_mode);
    assert_eq!(palette::color(dialog.hsl), selected);

    ctx.memory_mut(|memory| memory.request_focus(egui::Id::new("rgb_input")));
    dialog.rgb_input.clear();
    frame(&ctx, &mut dialog, vec![egui::Event::Text("1,2,3".into())]);
    click(&ctx, &mut dialog, 65.0, 60.0);
    assert!(dialog.index_mode);
    assert_eq!(dialog.index.selected, None);
    assert_eq!(dialog.rgb_input, "1,2,3");
    click(&ctx, &mut dialog, 300.0, 60.0);
    assert!(dialog.index_mode); // Color Books chưa chuyển trang.
}
