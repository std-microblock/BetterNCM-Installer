//! 一个标签创建模块，会使用 SCL 的主题（字体、大小等）来创建标签

use druid::{
    widget::{Label, LabelText},
    Data, Env,
};

use crate::theme::color as theme;

/// 以默认主题字体 [`crate::theme::color::typography::BASE`] 和颜色 [`crate::theme::color::typography::FONT_COLOR`] 创建一个标签组件 [`druid::widget::Label`]
#[inline]
pub fn new<T: Data>(text: impl Into<LabelText<T>>) -> Label<T> {
    Label::new(text)
        .with_font(theme::typography::BASE)
        .with_text_size(13.)
        .with_text_color(theme::typography::FONT_COLOR)
        .with_text_alignment(druid::TextAlignment::Start)
        .with_line_break_mode(druid::widget::LineBreaking::WordWrap)
}

/// 以默认主题字体 [`crate::theme::color::typography::BASE`] 和颜色 [`crate::theme::color::typography::FONT_COLOR`] 创建一个动态标签组件 [`druid::widget::Label`]
#[inline]
pub fn dynamic<T: Data>(text: impl Fn(&T, &Env) -> String + 'static) -> Label<T> {
    let text: LabelText<T> = text.into();
    new(text)
}
