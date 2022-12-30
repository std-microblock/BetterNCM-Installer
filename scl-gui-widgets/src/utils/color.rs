//! 一些用于颜色计算的模块

use druid::Color;

/// 根据底色计算出最适合观察的对比色，通常用于文字显示上
pub fn get_contrast_yiq(color: Color) -> Color {
    let (red, green, blue, alpha) = color.as_rgba();
    let yiq = 255. * ((red * 299.) + (green * 587.) + (blue * 114.)) / 1000.;
    if yiq >= 128. {
        Color::BLACK.with_alpha(alpha)
    } else {
        Color::WHITE.with_alpha(alpha)
    }
}

/// 保留透明度，反转颜色
pub fn invert_color(color: Color) -> Color {
    let (red, green, blue, alpha) = color.as_rgba();
    Color::rgba(1. - red, 1. - green, 1. - blue, alpha)
}

/// 将颜色转换成黑白色
pub fn gray_color(color: Color) -> Color {
    let (red, green, blue, alpha) = color.as_rgba();
    let gray = (red + green + blue) / 3.;
    Color::rgba(gray, gray, gray, alpha)
}

/// 混合颜色
pub fn mix_color(base: Color, add: Color) -> Color {
    let (bg_r, bg_g, bg_b, bg_a) = base.as_rgba();
    let (fg_r, fg_g, fg_b, fg_a) = add.as_rgba();
    let bg_r_a = bg_r * bg_a;
    let bg_g_a = bg_g * bg_a;
    let bg_b_a = bg_b * bg_a;
    let fg_r_a = fg_r * fg_a;
    let fg_g_a = fg_g * fg_a;
    let fg_b_a = fg_b * fg_a;
    let col_a = fg_a + bg_a * (1. - fg_a);
    let col_r_a = fg_r_a + bg_r_a * (1. - fg_a);
    let col_g_a = fg_g_a + bg_g_a * (1. - fg_a);
    let col_b_a = fg_b_a + bg_b_a * (1. - fg_a);
    Color::rgba(col_r_a / col_a, col_g_a / col_a, col_b_a / col_a, col_a)
}
