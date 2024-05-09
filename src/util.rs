pub fn loosely_parse_i32(string: &str) -> i32 {
    match string.parse::<i32>() {
        Ok(int) => int,
        Err(e) => {
            use std::num::IntErrorKind as IEK;
            match e.kind() {
                IEK::PosOverflow => i32::MAX,
                _ => i32::MIN,
            }
        }
    }
}
/// More slint aligned version of the inner function.
pub fn loosely_parse_int(string: slint::SharedString) -> i32 {
    loosely_parse_i32(&string)
}
// pub fn palette_color(rel_group_num: f32) -> slint::Color {
//     if rel_group_num >= 0.0 && rel_group_num < 1.0 {
//         // Convert the relative group number to a hue value in degrees
//         // 360 degrees represent the full spectrum of colors in the HSL model
//         let hue = rel_group_num * 360.0;

//         // Fixed saturation and lightness to ensure the color is neither too light nor too dark
//         let saturation = 0.9; // 90% saturation
//         let lightness = 0.5; // 50% lightness

//         // Using slint's Color::from_hsl which expects (hue, saturation, lightness)
//         slint::Color(hue, saturation, lightness)
//     } else {
//         panic!("`rel_group_num` out of range. It must be within [0, 1).")
//     }
// }
