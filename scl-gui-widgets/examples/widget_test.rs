//! 一个简短的组件例程
//!
//! 方便用来做控件测试
//!

use druid::{
    commands::QUIT_APP,
    widget::{Flex, TextBox},
    AppLauncher, Insets, Target, Widget, WidgetExt, WindowDesc,
};
use scl_gui_widgets::{
    widgets::{PasswordBox, WindowWidget, QUERY_CLOSE_WINDOW},
    WidgetExt as _,
};

type AppState = String;

fn ui_builder() -> impl Widget<AppState> {
    Flex::column()
        .with_child(TextBox::new().expand_width())
        .with_spacer(5.)
        .with_child(PasswordBox::new().expand_width())
        .padding(5.)
}

fn main() {
    AppLauncher::with_window({
        let win = WindowDesc::new(WindowWidget::new("Widget Test", ui_builder()).on_notify(
            QUERY_CLOSE_WINDOW,
            |ctx, _, _| {
                ctx.submit_command(QUIT_APP.to(Target::Window(ctx.window_id())));
            },
        ))
        .title("Widget Test")
        .window_size((300., 120.));

        #[cfg(any(target_os = "windows", target_os = "linux"))]
        {
            win.show_titlebar(false).transparent(true)
        }
        #[cfg(not(any(target_os = "windows", target_os = "linux")))]
        {
            win
        }
    })
    .configure_env(|env, _| {
        scl_gui_widgets::theme::color::set_color_to_env(
            env,
            scl_gui_widgets::theme::color::Theme::Dark, // 更换此处来切换黑白主题（或者自己触发 Command）
        );
        // 设置 Druid 自带的主题的大小以匹配 WinUI 3 的风格
        env.set(
            druid::theme::TEXTBOX_INSETS,
            Insets::new(12.0, 6.0, 12.0, 6.0),
        );
        env.set(
            druid::theme::BACKGROUND_LIGHT,
            env.get(scl_gui_widgets::theme::color::alt::HIGH),
        );
        env.set(
            druid::theme::BACKGROUND_DARK,
            env.get(scl_gui_widgets::theme::color::alt::HIGH),
        );
        env.set(
            druid::theme::SELECTED_TEXT_BACKGROUND_COLOR,
            env.get(scl_gui_widgets::theme::color::accent::ACCENT),
        );
        env.set(
            druid::theme::CURSOR_COLOR,
            env.get(scl_gui_widgets::theme::color::base::HIGH),
        );
        env.set(druid::theme::TEXTBOX_BORDER_WIDTH, 1.);
        env.set(druid::theme::TEXTBOX_BORDER_RADIUS, 4.);
    })
    .launch("password!1234".into())
    .unwrap();
}
