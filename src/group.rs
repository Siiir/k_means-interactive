use palette::IntoColor;

pub fn color(rel_group_num: f32) -> slint::Color {
    color_with_alpha(rel_group_num, 1.)
}

pub fn color_with_alpha(rel_group_num: f32, alpha: f32) -> slint::Color {
    let color = palette::Hsl::new(360. * rel_group_num, 0.9, 0.5);
    let color: palette::Srgb = color.into_color();
    let color = slint::Color::from_argb_f32(alpha, color.red, color.green, color.blue);
    color
}
