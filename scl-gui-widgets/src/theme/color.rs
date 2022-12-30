//! 根据 Windows UI （原 Fluent Design）规范制成的颜色和字体模块
//!
//! 详情请参考 <https://www.figma.com/community/file/989931624019688277>

/// 基本颜色组合，通常用在背景上显示内容（文字图标等）
pub mod base {
    use druid::{Color, Key};
    /// 默认情况下 亮色模式为 `#000000 - 20%` 黑暗模式为 `#FFFFFF - 20%`
    pub const LOW: Key<Color> = Key::new("net.stevexmh.scl.fluent.color.light.base.low");
    /// 默认情况下 亮色模式为 `#000000 - 40%` 黑暗模式为 `#FFFFFF - 40%`
    pub const MEDIUM_LOW: Key<Color> =
        Key::new("net.stevexmh.scl.fluent.color.light.base.medium-low");
    /// 默认情况下 亮色模式为 `#000000 - 60%` 黑暗模式为 `#FFFFFF - 60%`
    pub const MEDIUM: Key<Color> = Key::new("net.stevexmh.scl.fluent.color.light.base.medium");
    /// 默认情况下 亮色模式为 `#000000 - 80%` 黑暗模式为 `#FFFFFF - 80%`
    pub const MEDIUM_HIGH: Key<Color> =
        Key::new("net.stevexmh.scl.fluent.color.light.base.medium-high");
    /// 默认情况下 亮色模式为 `#000000 - 100%` 黑暗模式为 `#FFFFFF - 100%`
    pub const HIGH: Key<Color> = Key::new("net.stevexmh.scl.fluent.color.light.base.high");
}

/// 次颜色组合，通常用在背景显示，颜色组合与 [`crate::theme::color::base`] 相反
pub mod alt {
    use druid::{Color, Key};
    /// 默认情况下 亮色模式为 `#FFFFFF - 20%` 黑暗模式为 `#000000 - 20%`
    pub const LOW: Key<Color> = Key::new("net.stevexmh.scl.fluent.color.light.alt.low");
    /// 默认情况下 亮色模式为 `#FFFFFF - 40%` 黑暗模式为 `#000000 - 40%`
    pub const MEDIUM_LOW: Key<Color> =
        Key::new("net.stevexmh.scl.fluent.color.light.alt.medium-low");
    /// 默认情况下 亮色模式为 `#FFFFFF - 60%` 黑暗模式为 `#000000 - 60%`
    pub const MEDIUM: Key<Color> = Key::new("net.stevexmh.scl.fluent.color.light.alt.medium");
    /// 默认情况下 亮色模式为 `#FFFFFF - 80%` 黑暗模式为 `#000000 - 80%`
    pub const MEDIUM_HIGH: Key<Color> =
        Key::new("net.stevexmh.scl.fluent.color.light.alt.medium-high");
    /// 默认情况下 亮色模式为 `#FFFFFF - 100%` 黑暗模式为 `#000000 - 100%`
    pub const HIGH: Key<Color> = Key::new("net.stevexmh.scl.fluent.color.light.alt.high");
}

/// 外壳颜色组合，此处的部分颜色是实色而非 [`crate::theme::color::base`] 和 [`crate::theme::color::alt`] 的半透明颜色组
pub mod chrome {
    use druid::{Color, Key};
    /// 默认情况下 亮色模式为 `#FFFFFF` 黑暗模式为 `#FFFFFF`
    pub const WHITE_HIGH: Key<Color> = Key::new("net.stevexmh.scl.fluent.color.chrome.white-high");
    /// 默认情况下 亮色模式为 `#F2F2F2` 黑暗模式为 `#2B2B2B`
    pub const MEDIUM_LOW: Key<Color> = Key::new("net.stevexmh.scl.fluent.color.chrome.medium-low");
    /// 默认情况下 亮色模式为 `#F2F2F2` 黑暗模式为 `#171717`
    pub const LOW: Key<Color> = Key::new("net.stevexmh.scl.fluent.color.chrome.low");
    /// 默认情况下 亮色模式为 `#CCCCCC` 黑暗模式为 `#333333`
    pub const DISABLED_HIGH: Key<Color> =
        Key::new("net.stevexmh.scl.fluent.color.chrome.disabled-high");
    /// 默认情况下 亮色模式为 `#E6E6E6` 黑暗模式为 `#1F1F1F`
    pub const MEDIUM: Key<Color> = Key::new("net.stevexmh.scl.fluent.color.chrome.medium");
    /// 默认情况下 亮色模式为 `#CCCCCC` 黑暗模式为 `#767676`
    pub const HIGH: Key<Color> = Key::new("net.stevexmh.scl.fluent.color.chrome.high");
    /// 默认情况下 亮色模式为 `#CCCCCC` 黑暗模式为 `#767676`
    pub const BLACK_LOW: Key<Color> = Key::new("net.stevexmh.scl.fluent.color.chrome.black-low");
    /// 默认情况下 亮色模式为 `#000000 - 40%` 黑暗模式为 `#000000 - 40%`
    pub const BLACK_MEDIUM_LOW: Key<Color> =
        Key::new("net.stevexmh.scl.fluent.color.chrome.black-medium-low");
    /// 默认情况下 亮色模式为 `#7A7A7A` 黑暗模式为 `#858585`
    pub const DISABLED_LOW: Key<Color> =
        Key::new("net.stevexmh.scl.fluent.color.chrome.disabled-low");
    /// 默认情况下 亮色模式为 `#000000 - 80%` 黑暗模式为 `#000000 - 80%`
    pub const BLACK_MEDIUM: Key<Color> =
        Key::new("net.stevexmh.scl.fluent.color.chrome.black-medium");
    /// 默认情况下 亮色模式为 `#000000 - 40%` 黑暗模式为 `#F2F2F2`
    pub const ALT_LOW: Key<Color> = Key::new("net.stevexmh.scl.fluent.color.chrome.alt-low");
    /// 默认情况下 亮色模式为 `#000000` 黑暗模式为 `#000000`
    pub const BLACK_HIGH: Key<Color> = Key::new("net.stevexmh.scl.fluent.color.chrome.black-high");
    /// 默认情况下 亮色模式为 `#FFFFFF` 黑暗模式为 `#FFFFFF`
    pub const WHITE: Key<Color> = Key::new("net.stevexmh.scl.fluent.color.chrome.white");
}

/// 列表颜色组，不知道为什么微软要单独分这么一组颜色出来
pub mod list {
    use druid::{Color, Key};
    /// 默认情况下 亮色模式为 `#0078D4 - 70%` 黑暗模式为 `#0078D4 - 90%`
    pub const ACCENT_HIGH: Key<Color> =
        Key::new("net.stevexmh.scl.fluent.color.chrome.accent-high");
    /// 默认情况下 亮色模式为 `#0078D4 - 60%` 黑暗模式为 `#0078D4 - 80%`
    pub const ACCENT_MEDIUM: Key<Color> =
        Key::new("net.stevexmh.scl.fluent.color.chrome.accent-medium");
    /// 默认情况下 亮色模式为 `#0078D4 - 40%` 黑暗模式为 `#0078D4 - 60%`
    pub const ACCENT_LOW: Key<Color> = Key::new("net.stevexmh.scl.fluent.color.chrome.accent-low");
    /// 默认情况下 亮色模式为 `#000000 - 10%` 黑暗模式为 `#000000 - 10%`
    pub const LIST_LOW: Key<Color> = Key::new("net.stevexmh.scl.fluent.color.chrome.list-low");
    /// 默认情况下 亮色模式为 `#000000 - 20%` 黑暗模式为 `#000000 - 20%`
    pub const LIST_MEDIUM: Key<Color> =
        Key::new("net.stevexmh.scl.fluent.color.chrome.list-medium");
}

/// 边框颜色组
pub mod border {
    use druid::{Color, Key};
    /// 默认情况下 亮色模式为 `#FFFFFF - 60%` 黑暗模式为 `#FFFFFF - 5%`
    pub const EDGE_HIGHTLIGHT: Key<Color> =
        Key::new("net.stevexmh.scl.fluent.color.chrome.edge-hightlight");
    /// 默认情况下 亮色模式为 `#000000 - 14%` 黑暗模式为 `#000000 - 36%`
    pub const TRANSIENT: Key<Color> = Key::new("net.stevexmh.scl.fluent.color.chrome.transient");
}

/// 主要颜色组
pub mod accent {
    use druid::{Color, Key};
    /// 默认情况下 亮色模式为 `#0078D4` 黑暗模式为 `#0078D4`
    pub const ACCENT: Key<Color> = Key::new("net.stevexmh.scl.fluent.color.chrome.accent");
    /// 默认情况下 亮色模式为 `#429CE3` 黑暗模式为 `#429CE3`
    pub const ACCENT_1: Key<Color> = Key::new("net.stevexmh.scl.fluent.color.chrome.accent-1");
    /// 默认情况下 亮色模式为 `#005A9E` 黑暗模式为 `#005A9E`
    pub const ACCENT_DARK_1: Key<Color> =
        Key::new("net.stevexmh.scl.fluent.color.chrome.accent-dark-1");
    /// 默认情况下 亮色模式为 `#FF7233` 黑暗模式为 `#FF7233`
    pub const ACCENT_LIGHT_1: Key<Color> =
        Key::new("net.stevexmh.scl.fluent.color.chrome.accent-light-1");
}

/// 字体组
pub mod typography {
    use druid::{Color, FontDescriptor, Key};
    /// 标准字体颜色，默认情况下 亮色模式为 `#201F1E` 黑暗模式为 `#FFFFFF`
    pub const FONT_COLOR: Key<Color> =
        Key::new("net.stevexmh.scl.fluent.color.typography.font-color");

    /// 副标题字体，默认情况下大小为 34px 字重为 Light
    pub const SUBHEADER: Key<FontDescriptor> =
        Key::new("net.stevexmh.scl.fluent.color.typography.subheader");
    /// 大标题字体，默认情况下大小为 46px 字重为 Light
    pub const HEADER: Key<FontDescriptor> =
        Key::new("net.stevexmh.scl.fluent.color.typography.header");

    /// 标题字体，默认情况下大小为 24px 字重为 Semi Light
    pub const TITLE: Key<FontDescriptor> =
        Key::new("net.stevexmh.scl.fluent.color.typography.title");

    /// 标题字体，默认情况下大小为 12px 字重为 Regular
    pub const CAPTION: Key<FontDescriptor> =
        Key::new("net.stevexmh.scl.fluent.color.typography.caption");
    /// 替代标题字体，默认情况下大小为 13px 字重为 Regular
    pub const CAPTION_ALT: Key<FontDescriptor> =
        Key::new("net.stevexmh.scl.fluent.color.typography.caption-alt");
    /// 正文字体，默认情况下大小为 14px 字重为 Regular
    pub const BODY: Key<FontDescriptor> = Key::new("net.stevexmh.scl.fluent.color.typography.body");
    /// 替代副标题字体，默认情况下大小为 18px 字重为 Regular
    pub const SUBTITLE_ALT: Key<FontDescriptor> =
        Key::new("net.stevexmh.scl.fluent.color.typography.subtitle-alt");
    /// 副标题字体，默认情况下大小为 20px 字重为 Regular
    pub const SUBTITLE: Key<FontDescriptor> =
        Key::new("net.stevexmh.scl.fluent.color.typography.subtitle");

    /// 基本字体，默认情况下大小为 14px 字重为 Semi Bold
    pub const BASE: Key<FontDescriptor> = Key::new("net.stevexmh.scl.fluent.color.typography.base");
    /// 替代基本字体，默认情况下大小为 14px 字重为 Bold
    pub const BASE_ALT: Key<FontDescriptor> =
        Key::new("net.stevexmh.scl.fluent.color.typography.base-alt");
}

/// 为应用特别设定的几个键，和 Windows UI 无关
pub mod main {
    use druid::{Color, Key};
    /// 默认情况下 亮色模式为 `#F74C00` 黑暗模式为 `#F74C00`
    pub const PRIMARY: Key<Color> = Key::new("net.stevexmh.scl.primary");
    /// 默认情况下 亮色模式为 `#CC3F00` 黑暗模式为 `#CC3F00`
    pub const SECONDARY: Key<Color> = Key::new("net.stevexmh.scl.secondary");
    /// 默认情况下 亮色模式为 `#FFFFFF - 0%` 黑暗模式为 `#FFFFFF - 0%`
    pub const TITLE_BAR: Key<Color> = Key::new("net.stevexmh.scl.titlebar");
    /// 是否处于黑暗模式下，默认为 `false`
    pub const IS_DARK: Key<bool> = Key::new("net.stevexmh.scl.is-dark");
}

use serde::{Deserialize, Serialize};

/// 页面主题颜色，有亮色和暗色区分
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum Theme {
    /// 亮色模式
    Light,
    /// 暗色模式
    Dark,
}

impl Default for Theme {
    fn default() -> Self {
        Self::Light
    }
}

use druid::{Color, Env, FontDescriptor, FontFamily, FontStyle, FontWeight};

#[cfg(target_os = "windows")]
fn get_font() -> FontFamily {
    use druid::piet::{D2DText, DwriteFactory, Text};
    let dwrite = DwriteFactory::new().unwrap();
    let mut text = D2DText::new_with_shared_fonts(dwrite, None);
    text.font_family("微软雅黑")
        .or_else(|| text.font_family("Segoe WPC"))
        .or_else(|| text.font_family("Segoe UI"))
        .or_else(|| text.font_family("Garamond"))
        .unwrap_or(FontFamily::SYSTEM_UI)
}

#[cfg(not(target_os = "windows"))]
fn get_font() -> FontFamily {
    FontFamily::SYSTEM_UI
}

fn set_light_theme(env: &mut Env, _theme: Theme, font: FontFamily) {
    // Light Base
    env.set(base::LOW, Color::BLACK.with_alpha(0.2));
    env.set(base::MEDIUM_LOW, Color::BLACK.with_alpha(0.4));
    env.set(base::MEDIUM, Color::BLACK.with_alpha(0.6));
    env.set(base::MEDIUM_HIGH, Color::BLACK.with_alpha(0.8));
    env.set(base::HIGH, Color::BLACK);
    // Light Alt
    env.set(alt::LOW, Color::WHITE.with_alpha(0.2));
    env.set(alt::MEDIUM_LOW, Color::WHITE.with_alpha(0.4));
    env.set(alt::MEDIUM, Color::WHITE.with_alpha(0.6));
    env.set(alt::MEDIUM_HIGH, Color::WHITE.with_alpha(0.8));
    env.set(alt::HIGH, Color::WHITE);
    // Chrome
    env.set(chrome::WHITE_HIGH, Color::Rgba32(0xFFFFFFFF));
    env.set(chrome::MEDIUM_LOW, Color::Rgba32(0xF2F2F2FF));
    env.set(chrome::LOW, Color::Rgba32(0xF2F2F2FF));
    env.set(chrome::DISABLED_HIGH, Color::Rgba32(0xCCCCCCFF));
    env.set(chrome::MEDIUM, Color::Rgba32(0xE6E6E6FF));
    env.set(chrome::HIGH, Color::Rgba32(0xCCCCCCFF));
    env.set(chrome::BLACK_LOW, Color::BLACK.with_alpha(0.2));
    env.set(chrome::BLACK_MEDIUM_LOW, Color::BLACK.with_alpha(0.4));
    env.set(chrome::DISABLED_LOW, Color::Rgba32(0x7A7A7AFF));
    env.set(chrome::BLACK_MEDIUM, Color::BLACK.with_alpha(0.8));
    env.set(chrome::ALT_LOW, Color::BLACK.with_alpha(0.4));
    env.set(chrome::BLACK_HIGH, Color::BLACK);
    env.set(chrome::WHITE, Color::WHITE);
    // List
    env.set(list::ACCENT_HIGH, Color::Rgba32(0x0078D4FF).with_alpha(0.2));
    env.set(
        list::ACCENT_MEDIUM,
        Color::Rgba32(0x0078D4FF).with_alpha(0.4),
    );
    env.set(list::ACCENT_LOW, Color::Rgba32(0x0078D4FF).with_alpha(0.6));
    env.set(list::LIST_LOW, Color::BLACK.with_alpha(0.1));
    env.set(list::LIST_MEDIUM, Color::BLACK.with_alpha(0.2));
    // Border
    env.set(border::EDGE_HIGHTLIGHT, Color::WHITE.with_alpha(0.6));
    env.set(border::TRANSIENT, Color::BLACK.with_alpha(0.14));
    // Accent
    env.set(accent::ACCENT, Color::Rgba32(0x0078D4FF));
    env.set(accent::ACCENT_1, Color::Rgba32(0x429CE3FF));
    env.set(accent::ACCENT_DARK_1, Color::Rgba32(0x005A9EFF));
    env.set(accent::ACCENT_LIGHT_1, Color::Rgba32(0xFF7233FF));
    // Main
    env.set(main::PRIMARY, Color::Rgba32(0xF74C00FF));
    env.set(main::SECONDARY, Color::Rgba32(0xCC3F00FF));
    env.set(main::TITLE_BAR, Color::Rgba32(0xFFFFFF00));
    // Typography
    env.set(typography::FONT_COLOR, Color::Rgba32(0x201F1EFF));

    env.set(
        typography::SUBHEADER,
        FontDescriptor::new(font.to_owned())
            .with_size(34.)
            .with_weight(FontWeight::EXTRA_LIGHT),
    );
    env.set(
        typography::HEADER,
        FontDescriptor::new(font.to_owned())
            .with_size(46.)
            .with_weight(FontWeight::EXTRA_LIGHT),
    );

    env.set(
        typography::TITLE,
        FontDescriptor::new(font.to_owned())
            .with_size(24.)
            .with_weight(FontWeight::EXTRA_LIGHT),
    );

    env.set(
        typography::CAPTION,
        FontDescriptor::new(font.to_owned())
            .with_size(12.)
            .with_weight(FontWeight::LIGHT),
    );
    env.set(
        typography::CAPTION_ALT,
        FontDescriptor::new(font.to_owned())
            .with_size(13.)
            .with_weight(FontWeight::LIGHT),
    );
    env.set(
        typography::BODY,
        FontDescriptor::new(font.to_owned())
            .with_size(14.)
            .with_weight(FontWeight::REGULAR),
    );
    env.set(
        typography::SUBTITLE_ALT,
        FontDescriptor::new(font.to_owned())
            .with_size(18.)
            .with_weight(FontWeight::REGULAR),
    );
    env.set(
        typography::SUBTITLE,
        FontDescriptor::new(font.to_owned())
            .with_size(20.)
            .with_weight(FontWeight::REGULAR),
    );

    env.set(
        typography::BASE,
        FontDescriptor::new(font.to_owned())
            .with_size(14.)
            .with_weight(FontWeight::REGULAR),
    );
    env.set(
        typography::BASE_ALT,
        FontDescriptor::new(font.to_owned())
            .with_size(14.)
            .with_weight(FontWeight::SEMI_BOLD),
    );
    // Druid Fonts
    env.set(
        druid::theme::UI_FONT,
        FontDescriptor::new(font.to_owned())
            .with_size(15.)
            .with_weight(FontWeight::REGULAR),
    );
    env.set(
        druid::theme::UI_FONT_BOLD,
        FontDescriptor::new(font.to_owned())
            .with_size(15.)
            .with_weight(FontWeight::BOLD),
    );
    env.set(
        druid::theme::UI_FONT_ITALIC,
        FontDescriptor::new(font)
            .with_size(15.)
            .with_style(FontStyle::Italic)
            .with_weight(FontWeight::REGULAR),
    );
    // Others
    env.set(
        druid::theme::WINDOW_BACKGROUND_COLOR,
        Color::Rgba32(0xF3F3F3FF),
    );
}

fn set_dark_theme(env: &mut Env, _theme: Theme, font: FontFamily) {
    // Light Base
    env.set(base::LOW, Color::WHITE.with_alpha(0.2));
    env.set(base::MEDIUM_LOW, Color::WHITE.with_alpha(0.4));
    env.set(base::MEDIUM, Color::WHITE.with_alpha(0.6));
    env.set(base::MEDIUM_HIGH, Color::WHITE.with_alpha(0.8));
    env.set(base::HIGH, Color::WHITE);
    // Light Alt
    env.set(alt::LOW, Color::BLACK.with_alpha(0.2));
    env.set(alt::MEDIUM_LOW, Color::BLACK.with_alpha(0.4));
    env.set(alt::MEDIUM, Color::BLACK.with_alpha(0.6));
    env.set(alt::MEDIUM_HIGH, Color::BLACK.with_alpha(0.8));
    env.set(alt::HIGH, Color::BLACK);
    // Chrome
    env.set(chrome::LOW, Color::Rgba32(0x373737FF));
    env.set(chrome::MEDIUM_LOW, Color::Rgba32(0x2B2B2BFF));
    env.set(chrome::MEDIUM, Color::Rgba32(0x1F1F1FFF));
    env.set(chrome::HIGH, Color::Rgba32(0x767676FF));
    env.set(chrome::ALT_LOW, Color::Rgba32(0xF2F2F2FF));
    env.set(chrome::DISABLED_LOW, Color::Rgba32(0x858585FF));
    env.set(chrome::DISABLED_HIGH, Color::Rgba32(0x333333FF));
    env.set(chrome::BLACK_LOW, Color::BLACK.with_alpha(0.2));
    env.set(chrome::WHITE_HIGH, Color::Rgba32(0xFFFFFFFF));
    env.set(chrome::BLACK_MEDIUM_LOW, Color::BLACK.with_alpha(0.4));
    env.set(chrome::BLACK_MEDIUM, Color::BLACK.with_alpha(0.8));
    env.set(chrome::BLACK_HIGH, Color::BLACK);
    env.set(chrome::WHITE, Color::WHITE);
    // List
    env.set(list::ACCENT_HIGH, Color::Rgba32(0x0078D4FF).with_alpha(0.2));
    env.set(
        list::ACCENT_MEDIUM,
        Color::Rgba32(0x0078D4FF).with_alpha(0.4),
    );
    env.set(list::ACCENT_LOW, Color::Rgba32(0x0078D4FF).with_alpha(0.6));
    env.set(list::LIST_LOW, Color::Rgba32(0x3c3c3cFF));
    env.set(list::LIST_MEDIUM, Color::BLACK.with_alpha(0.2));
    // Border
    env.set(border::EDGE_HIGHTLIGHT, Color::WHITE.with_alpha(0.6));
    env.set(border::TRANSIENT, Color::BLACK.with_alpha(0.14));
    // Accent
    env.set(accent::ACCENT, Color::Rgba32(0x0078D4FF));
    env.set(accent::ACCENT_1, Color::Rgba32(0x429CE3FF));
    env.set(accent::ACCENT_DARK_1, Color::Rgba32(0x005A9EFF));
    env.set(accent::ACCENT_LIGHT_1, Color::Rgba32(0xFF7233FF));
    // Main
    env.set(main::PRIMARY, Color::Rgba32(0xF74C00FF));
    env.set(main::SECONDARY, Color::Rgba32(0xCC3F00FF));
    env.set(main::TITLE_BAR, Color::Rgba32(0x00000000));
    // Typography
    env.set(typography::FONT_COLOR, Color::Rgba32(0xFFFFFFFF));

    env.set(
        typography::SUBHEADER,
        FontDescriptor::new(font.to_owned())
            .with_size(34.)
            .with_weight(FontWeight::EXTRA_LIGHT),
    );
    env.set(
        typography::HEADER,
        FontDescriptor::new(font.to_owned())
            .with_size(46.)
            .with_weight(FontWeight::EXTRA_LIGHT),
    );

    env.set(
        typography::TITLE,
        FontDescriptor::new(font.to_owned())
            .with_size(24.)
            .with_weight(FontWeight::EXTRA_LIGHT),
    );

    env.set(
        typography::CAPTION,
        FontDescriptor::new(font.to_owned())
            .with_size(12.)
            .with_weight(FontWeight::LIGHT),
    );
    env.set(
        typography::CAPTION_ALT,
        FontDescriptor::new(font.to_owned())
            .with_size(13.)
            .with_weight(FontWeight::LIGHT),
    );
    env.set(
        typography::BODY,
        FontDescriptor::new(font.to_owned())
            .with_size(14.)
            .with_weight(FontWeight::REGULAR),
    );
    env.set(
        typography::SUBTITLE_ALT,
        FontDescriptor::new(font.to_owned())
            .with_size(18.)
            .with_weight(FontWeight::REGULAR),
    );
    env.set(
        typography::SUBTITLE,
        FontDescriptor::new(font.to_owned())
            .with_size(20.)
            .with_weight(FontWeight::REGULAR),
    );

    env.set(
        typography::BASE,
        FontDescriptor::new(font.to_owned())
            .with_size(14.)
            .with_weight(FontWeight::REGULAR),
    );
    env.set(
        typography::BASE_ALT,
        FontDescriptor::new(font.to_owned())
            .with_size(14.)
            .with_weight(FontWeight::SEMI_BOLD),
    );
    // Druid Fonts
    env.set(
        druid::theme::UI_FONT,
        FontDescriptor::new(font.to_owned())
            .with_size(15.)
            .with_weight(FontWeight::REGULAR),
    );
    env.set(
        druid::theme::UI_FONT_BOLD,
        FontDescriptor::new(font.to_owned())
            .with_size(15.)
            .with_weight(FontWeight::BOLD),
    );
    env.set(
        druid::theme::UI_FONT_ITALIC,
        FontDescriptor::new(font)
            .with_size(15.)
            .with_style(FontStyle::Italic)
            .with_weight(FontWeight::REGULAR),
    );
    // Others
    env.set(
        druid::theme::WINDOW_BACKGROUND_COLOR,
        Color::Rgba32(0x202020FF),
    );
}

/// 为所给的 [`Env`] 设置组件库的默认配色
///
/// 必须在显示窗口前设置完毕，否则会因为颜色缺失报错
pub fn set_color_to_env(env: &mut Env, theme: Theme) {
    let font = get_font();
    env.set(main::IS_DARK, theme == Theme::Dark);
    match theme {
        Theme::Light => set_light_theme(env, theme, font),
        Theme::Dark => set_dark_theme(env, theme, font),
    }
}
