use eframe::egui::{ecolor::Hsva, Color32, ColorImage};

pub fn spectrum() -> ColorImage {
    let mut image = ColorImage::new([250, 224], Color32::BLACK);
    for y in 0..224 {
        let saturation = 1.0 - y as f32 / 223.0;
        for x in 0..250 {
            let [r, g, b] = Hsva::new(x as f32 / 249.0, 1.0, 1.0, 1.0).to_rgb();
            let channel = |v: f32| (128.0 + (v * 255.0 - 128.0) * saturation).round() as u8;
            image[(x, y)] = Color32::from_rgb(channel(r), channel(g), channel(b));
        }
    }
    image
}

// HSL dùng độ cho hue, phần trăm cho saturation và lightness.
pub fn color([hue, saturation, lightness]: [f32; 3]) -> Color32 {
    let [r, g, b] = Hsva::new(hue / 360.0, 1.0, 1.0, 1.0).to_rgb();
    let s = saturation / 100.0;
    let l = lightness / 100.0;
    let chroma = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let channel = |v: f32| ((v * chroma + l - chroma / 2.0) * 255.0).round() as u8;
    Color32::from_rgb(channel(r), channel(g), channel(b))
}

pub fn from_rgb([r, g, b]: [u8; 3]) -> [f32; 3] {
    let [r, g, b] = [r, g, b].map(|v| v as f32 / 255.0);
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;
    let lightness = (max + min) / 2.0;
    if delta == 0.0 {
        return [0.0, 0.0, lightness * 100.0];
    }
    let hue = if max == r {
        ((g - b) / delta).rem_euclid(6.0)
    } else if max == g {
        (b - r) / delta + 2.0
    } else {
        (r - g) / delta + 4.0
    };
    [
        hue * 60.0,
        delta / (1.0 - (2.0 * lightness - 1.0).abs()) * 100.0,
        lightness * 100.0,
    ]
}

pub fn rgb_text(color: Color32) -> String {
    format!("{},{},{}", color.r(), color.g(), color.b())
}

pub fn parse_rgb(text: &str) -> Option<[u8; 3]> {
    let parts: Vec<_> = text.split(',').map(str::trim).collect();
    if parts.len() != 3 {
        return None;
    }
    Some([
        parts[0].parse().ok()?,
        parts[1].parse().ok()?,
        parts[2].parse().ok()?,
    ])
}

pub fn luminance(hue: f32, saturation: f32) -> ColorImage {
    let mut image = ColorImage::new([36, 218], Color32::BLACK);
    for y in 0..218 {
        let color = color([hue, saturation, (1.0 - y as f32 / 217.0) * 100.0]);
        for x in 0..36 {
            image[(x, y)] = color;
        }
    }
    image
}

// Bảng ACI theo các ô màu trong ảnh tham chiếu.
pub fn aci(index: u8) -> Color32 {
    if index < 10 {
        let [r, g, b] = [
            [255, 255, 255],
            [255, 0, 0],
            [255, 255, 0],
            [0, 255, 0],
            [0, 255, 255],
            [0, 0, 255],
            [255, 0, 255],
            [255, 255, 255],
            [128, 128, 128],
            [192, 192, 192],
        ][index as usize];
        return Color32::from_rgb(r, g, b);
    }
    if index >= 250 {
        return Color32::from_gray([51, 91, 132, 173, 214, 255][(index - 250) as usize]);
    }
    let hue = (index - 10) / 10;
    let shade = (index - 10) % 10;
    let max: u16 = [255, 204, 153, 127, 76][shade as usize / 2];
    let min = if shade % 2 == 1 { max / 2 } else { 0 };
    let rising = min + (max - min) * (hue % 4) as u16 / 4;
    let falling = min + (max - min) * (4 - hue % 4) as u16 / 4;
    let [r, g, b] = match hue / 4 {
        0 => [max, rising, min],
        1 => [falling, max, min],
        2 => [min, max, rising],
        3 => [min, falling, max],
        4 => [rising, min, max],
        _ => [max, min, falling],
    };
    Color32::from_rgb(r as u8, g as u8, b as u8)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rgb_hsl_round_trip() {
        for rgb in [
            [82, 101, 173],
            [255, 0, 0],
            [0, 255, 0],
            [0, 0, 255],
            [0, 0, 0],
            [255, 255, 255],
            [128, 128, 128],
            [1, 254, 127],
        ] {
            let result = color(from_rgb(rgb));
            assert_eq!([result.r(), result.g(), result.b()], rgb);
        }
        assert_eq!(color([360.0, 100.0, 50.0]), Color32::RED);
    }

    #[test]
    fn rgb_input_validation() {
        assert_eq!(parse_rgb("82, 101,173"), Some([82, 101, 173]));
        for text in ["256,0,0", "-1,0,0", "1,2", "1,2,3,4", "a,b,c", ""] {
            assert_eq!(parse_rgb(text), None);
        }
    }

    #[test]
    fn aci_matches_reference_swatches() {
        for (index, rgb) in [
            (1, [255, 0, 0]),
            (9, [192, 192, 192]),
            (18, [76, 0, 0]),
            (28, [76, 19, 0]),
            (30, [255, 127, 0]),
            (11, [255, 127, 127]),
            (23, [204, 127, 102]),
            (249, [76, 38, 47]),
            (250, [51, 51, 51]),
            (255, [255, 255, 255]),
        ] {
            let c = aci(index);
            assert_eq!([c.r(), c.g(), c.b()], rgb, "ACI {index}");
        }
    }
}
