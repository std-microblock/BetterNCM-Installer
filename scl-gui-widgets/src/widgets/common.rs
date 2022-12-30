//! 一些通用的东西

use druid::{
    piet::{GradientStop, PaintBrush},
    Env, LinearGradient, PaintCtx, RenderContext, Size, UnitPoint,
};

use crate::theme::color as theme;

pub fn print_common_button(
    (is_accent, is_flat, is_active, is_hot, is_disabled): (bool, bool, bool, bool, bool),
    ctx: &mut PaintCtx,
    size: Size,
    env: &Env,
) {
    let border_width = 1.;
    let rounded_rect = size.to_rect().to_rounded_rect(5.);
    let border_rounded_rect = size
        .to_rect()
        .inflate(border_width / -2.0, border_width / -2.0)
        .to_rounded_rect(5. - border_width / 2.0);

    let bg_gradient = if ctx.is_disabled() || is_disabled {
        PaintBrush::Color(env.get(theme::base::LOW))
    } else if is_accent {
        if is_active {
            PaintBrush::Color(env.get(theme::accent::ACCENT_DARK_1))
        } else if is_hot {
            PaintBrush::Color(env.get(theme::accent::ACCENT_LIGHT_1))
        } else {
            PaintBrush::Color(env.get(theme::accent::ACCENT))
        }
    } else if is_flat {
        if is_active {
            PaintBrush::Color(env.get(theme::base::LOW))
        } else if is_hot {
            PaintBrush::Color(env.get(theme::list::LIST_LOW))
        } else {
            PaintBrush::Color(druid::Color::Rgba32(0x303030ff))
        }
    } else if is_active {
        PaintBrush::Color(env.get(theme::base::MEDIUM_LOW))
    } else if is_hot {
        PaintBrush::Color(env.get(theme::list::LIST_LOW))
    } else {
        PaintBrush::Color(env.get(theme::base::LOW))
    };
    ctx.fill(rounded_rect, &bg_gradient);
    if !is_flat {
        let gradient_stops = vec![
            GradientStop {
                pos: 0.9067,
                color: druid::Color::Rgba32(0xFFFFFF14),
            },
            GradientStop {
                pos: 1.,
                color: druid::Color::Rgba32(0x00000066),
            },
        ];
        let border_gradient = PaintBrush::Linear(LinearGradient::new(
            UnitPoint::TOP,
            UnitPoint::BOTTOM,
            gradient_stops,
        ));
        ctx.stroke(border_rounded_rect, &border_gradient, border_width);
    }
}
