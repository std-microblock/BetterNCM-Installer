use druid::{commands::CONFIGURE_WINDOW, kurbo::BezPath, piet::*, *};

use crate::{theme::color as theme, utils::color::get_contrast_yiq};

/// 请求关闭窗口的 [`druid::Notification`]
pub const QUERY_CLOSE_WINDOW: Selector = Selector::new("net.stevexmh.scl.query-close-window");

/// 窗口的返回按钮被点击时抛出的 [`druid::Notification`]
pub const BACK_PAGE_CLICKED: Selector = Selector::new("net.stevexmh.scl.back-page-clicked");

/// 请求设置窗口图片背景的 [`druid::Command`]
pub const SET_BACKGROUND_IMAGE: Selector<druid::ImageBuf> =
    Selector::new("net.stevexmh.scl.set-bg-img");

/// 请求清除窗口图片背景的 [`druid::Command`]
pub const CLEAR_BACKGROUND_IMAGE: Selector = Selector::new("net.stevexmh.scl.clear-bg-img");

/// 请求启用窗口返回按钮的 [`druid::Command`] ，参数为是否启用/禁用
pub const ENABLE_BACK_PAGE: Selector<bool> = Selector::new("net.stevexmh.scl.enable-back-page");

/// 一个用于小型窗口的仿 Windows 窗口组件，推荐创建含有此组件的窗口时将 [`druid::WindowDesc`] 中
/// [`druid::WindowDesc::show_titlebar`] 设置为 `false`，效果最佳。
///
/// 一旦使用了这个组件，任何关闭窗口的通知都将被捕获且失效，需要自行另外接收
/// [`QUERY_CLOSE_WINDOW`] 通知以自行确认是否关闭。
///
/// 如果要使用返回按钮，先使用 [`ENABLE_BACK_PAGE`] 启用后再接收 [`BACK_PAGE_CLICKED`]
/// 通知以处理操作，推荐配合 [`crate::widgets::PageSwitcher`] 效果最佳。
pub struct WindowWidget<T> {
    title: String,

    show_back_btn: bool,
    inner: WidgetPod<T, Box<dyn Widget<T>>>,
    hovering: u8, // 0: nothing, 1: close btn, 2: min btn, 3: title bar
    down_hovering: u8,

    // Background Image
    background_image: Option<druid::piet::PietImage>,
    background_image_data: Option<druid::ImageBuf>,
}

impl<T: Data> WindowWidget<T> {
    /// 创建一个带标题的窗口，并将要显示的组件包裹在内
    pub fn new(title: impl Into<String>, inner: impl Widget<T> + 'static) -> Self {
        Self {
            title: title.into(),

            show_back_btn: false,
            inner: WidgetPod::new(Box::new(inner)),
            hovering: 0,
            down_hovering: u8::MAX,
            background_image: None,
            background_image_data: None,
        }
    }

    fn paint_no_shadow(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env, window_rect: &Rect) {
        let hovering = self.hovering;
        #[allow(unused_variables)]
        let size = window_rect.size();

        let title_bar_color = env.get(theme::main::TITLE_BAR);
        let title_bar_text_color = get_contrast_yiq(title_bar_color.to_owned()).with_alpha(1.);
        #[allow(unused_variables, clippy::redundant_clone)]
        let title_bar_hover_color = title_bar_text_color.to_owned().with_alpha(0.2);

        if let Some(img_buf) = &self.background_image_data.take() {
            if self.background_image.is_none() {
                self.background_image = Some(
                    ctx.make_image(
                        img_buf.width(),
                        img_buf.height(),
                        img_buf.raw_pixels(),
                        img_buf.format(),
                    )
                    .unwrap(),
                );
            }
        } else {
            ctx.fill(
                window_rect,
                &PaintBrush::Color(env.get(druid::theme::WINDOW_BACKGROUND_COLOR)),
            );
        }

        let mut title_rect = window_rect.to_owned();
        title_rect.y1 = title_rect.y0 + TITLE_HEIGHT;

        if let Some(img) = &self.background_image {
            ctx.draw_image(
                img,
                window_rect.to_owned(),
                druid::piet::InterpolationMode::Bilinear,
            );
        } else {
            ctx.fill(
                window_rect,
                &PaintBrush::Color(
                    env.get(druid::theme::WINDOW_BACKGROUND_COLOR)
                        .with_alpha(0.25),
                ),
            );
        }

        #[cfg(target_os = "macos")]
        let _ = self.title.to_owned(); // 为了过 clippy（逃）

        // Title Bar
        ctx.with_save(|ctx| {
            ctx.clip(window_rect);

            ctx.fill(title_rect, &PaintBrush::Color(title_bar_color.to_owned()));

            ctx.transform(Affine::translate((title_rect.x0, title_rect.y0)));
            // Close Button
            #[cfg(any(target_os = "windows", target_os = "linux"))]
            let label_layout = {
                if hovering == 1 {
                    ctx.fill(
                        Rect::new(
                            size.width - CONTROL_BUTTON_WIDTH,
                            0.,
                            size.width,
                            TITLE_HEIGHT,
                        ),
                        &PaintBrush::Color(Color::RED),
                    );
                }
                let mut path = BezPath::new();
                path.move_to((
                    size.width - CONTROL_BUTTON_WIDTH * 0.5 - 5.,
                    TITLE_HEIGHT * 0.5 - 5.,
                ));
                path.line_to((
                    size.width - CONTROL_BUTTON_WIDTH * 0.5 + 5.,
                    TITLE_HEIGHT * 0.5 + 5.,
                ));
                path.move_to((
                    size.width - CONTROL_BUTTON_WIDTH * 0.5 + 5.,
                    TITLE_HEIGHT * 0.5 - 5.,
                ));
                path.line_to((
                    size.width - CONTROL_BUTTON_WIDTH * 0.5 - 5.,
                    TITLE_HEIGHT * 0.5 + 5.,
                ));
                if hovering == 1 {
                    ctx.stroke(path, &PaintBrush::Color(Color::WHITE), 1.);
                } else {
                    ctx.stroke(
                        path,
                        &PaintBrush::Color(title_bar_text_color.to_owned()),
                        1.,
                    );
                }
                // Minisize Button
                if hovering == 2 {
                    ctx.fill(
                        Rect::new(
                            size.width - CONTROL_BUTTON_WIDTH * 2.,
                            0.,
                            size.width - CONTROL_BUTTON_WIDTH,
                            TITLE_HEIGHT,
                        ),
                        &PaintBrush::Color(title_bar_hover_color.to_owned()),
                    );
                }
                let mut path = BezPath::new();
                path.move_to((
                    size.width - CONTROL_BUTTON_WIDTH * 1.5 - 5.,
                    TITLE_HEIGHT * 0.5 - 0.5,
                ));
                path.line_to((
                    size.width - CONTROL_BUTTON_WIDTH * 1.5 + 5.,
                    TITLE_HEIGHT * 0.5 - 0.5,
                ));
                ctx.stroke(
                    path,
                    &PaintBrush::Color(title_bar_text_color.to_owned()),
                    1.,
                );
                // Title
                let text = ctx.text();
                let laybel_font = env.get(theme::typography::CAPTION);
                text.new_text_layout(self.title.to_owned())
                    .alignment(TextAlignment::Start)
                    .font(laybel_font.family, laybel_font.size)
                    .text_color(title_bar_text_color.to_owned())
                    .build()
                    .unwrap()
            };
            #[cfg(any(target_os = "windows", target_os = "linux"))]
            // Back button
            if self.show_back_btn {
                if hovering == 4 {
                    ctx.fill(
                        Rect::new(0., 0., CONTROL_BUTTON_WIDTH, TITLE_HEIGHT),
                        &PaintBrush::Color(title_bar_hover_color.to_owned()),
                    );
                }
                let mut path = BezPath::new();
                path.move_to((CONTROL_BUTTON_WIDTH * 0.5 + 5.25, TITLE_HEIGHT * 0.5 - 0.5));
                path.line_to((CONTROL_BUTTON_WIDTH * 0.5 - 5.25, TITLE_HEIGHT * 0.5 - 0.5));
                path.move_to((
                    CONTROL_BUTTON_WIDTH * 0.5 - 0.5,
                    TITLE_HEIGHT * 0.5 + 5. - 0.5,
                ));
                path.line_to((CONTROL_BUTTON_WIDTH * 0.5 - 5.25, TITLE_HEIGHT * 0.5 - 0.5));
                path.line_to((CONTROL_BUTTON_WIDTH * 0.5 - 0.5, TITLE_HEIGHT * 0.5 - 5.));
                ctx.stroke(path, &PaintBrush::Color(title_bar_text_color), 1.);
                #[cfg(any(windows, unix))]
                ctx.draw_text(
                    &label_layout,
                    (42., (TITLE_HEIGHT - label_layout.size().height) / 2.),
                );
            } else {
                #[cfg(any(windows, unix))]
                ctx.draw_text(
                    &label_layout,
                    (10., (TITLE_HEIGHT - label_layout.size().height) / 2.),
                );
            }
            #[cfg(target_os = "macos")]
            if self.show_back_btn {
                use druid::kurbo::Circle;
                // if hovering == 4 {
                //     ctx.fill(
                //         Rect::new(
                //             size.width - CONTROL_BUTTON_WIDTH,
                //             0.,
                //             size.width,
                //             TITLE_HEIGHT,
                //         ),
                //         &PaintBrush::Color(title_bar_hover_color.to_owned()),
                //     );
                // }

                let button_center_x = 34.;
                let button_center_y = 14.;

                ctx.fill(
                    Circle::new((button_center_x, button_center_y), 6.),
                    &PaintBrush::Color(Color::Rgba32(0xE3E3E3FF)),
                );
                ctx.stroke(
                    Circle::new((button_center_x, button_center_y), 5.75),
                    &PaintBrush::Color(Color::BLACK.with_alpha(0.12)),
                    0.5,
                );

                if hovering == 4 {
                    let mut path = BezPath::new();
                    path.move_to((button_center_x - 6. + 7., button_center_y - 6. + 3.5));
                    path.line_to((button_center_x - 6. + 4.5, button_center_y - 6. + 6.));
                    path.line_to((button_center_x - 6. + 7., button_center_y - 6. + 8.5));
                    ctx.stroke(path, &PaintBrush::Color(Color::BLACK.with_alpha(0.5)), 1.);
                }

                // let mut path = BezPath::new();
                // path.move_to((
                //     size.width - CONTROL_BUTTON_WIDTH + CONTROL_BUTTON_WIDTH * 0.5 + 5.25,
                //     TITLE_HEIGHT * 0.5 - 0.5,
                // ));
                // path.line_to((
                //     size.width - CONTROL_BUTTON_WIDTH + CONTROL_BUTTON_WIDTH * 0.5 - 5.25,
                //     TITLE_HEIGHT * 0.5 - 0.5,
                // ));
                // path.move_to((
                //     size.width - CONTROL_BUTTON_WIDTH + CONTROL_BUTTON_WIDTH * 0.5 - 0.5,
                //     TITLE_HEIGHT * 0.5 + 5. - 0.5,
                // ));
                // path.line_to((
                //     size.width - CONTROL_BUTTON_WIDTH + CONTROL_BUTTON_WIDTH * 0.5 - 5.25,
                //     TITLE_HEIGHT * 0.5 - 0.5,
                // ));
                // path.line_to((
                //     size.width - CONTROL_BUTTON_WIDTH + CONTROL_BUTTON_WIDTH * 0.5 - 0.5,
                //     TITLE_HEIGHT * 0.5 - 5.,
                // ));
                // ctx.stroke(path, &PaintBrush::Color(title_bar_text_color), 1.);
            }
        });

        // Child
        ctx.with_save(|ctx| {
            let mut inner_rect = window_rect.to_owned();
            inner_rect.y0 += TITLE_HEIGHT;
            ctx.clip(inner_rect);
            self.inner.paint(ctx, data, env)
        })
    }
}

#[cfg(not(target_os = "macos"))]
const CONTROL_BUTTON_WIDTH: f64 = 32f64;
#[cfg(not(target_os = "macos"))]
const TITLE_HEIGHT: f64 = 32f64;
#[cfg(target_os = "macos")]
const TITLE_HEIGHT: f64 = 28f64;

impl<T: Data> Widget<T> for WindowWidget<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        if let Event::MouseMove(m) = event {
            let width = ctx.size().width;
            let old_hovering = self.hovering;
            let m_pos_x = m.pos.x;
            let m_pos_y = m.pos.y;
            #[cfg(not(target_os = "macos"))]
            {
                self.hovering = if !(0. ..=TITLE_HEIGHT).contains(&m_pos_y)
                    || m_pos_x < 0.
                    || m_pos_x > width
                {
                    0
                } else if m_pos_x > width - CONTROL_BUTTON_WIDTH {
                    1
                } else if m_pos_x > width - CONTROL_BUTTON_WIDTH * 2. {
                    2
                } else if self.show_back_btn && m_pos_x < CONTROL_BUTTON_WIDTH {
                    4
                } else {
                    3
                };
            }
            #[cfg(target_os = "macos")]
            {
                let button_center_x = 34.;
                let button_center_y = 14.;
                self.hovering =
                    if !(0. ..=TITLE_HEIGHT).contains(&m_pos_y) || m_pos_x < 0. || m_pos_x > width {
                        0
                    } else if (button_center_x - m_pos_x).powi(2)
                        + (button_center_y - m_pos_y).powi(2)
                        <= 64.
                    {
                        4
                    } else {
                        3
                    }
            }
            if old_hovering != self.hovering {
                ctx.request_paint();
                ctx.request_anim_frame();
            }
            ctx.window().handle_titlebar(self.hovering == 3);
        } else if let Event::MouseDown(_m) = event {
            self.down_hovering = self.hovering;
            let should_active = self.hovering == 1 || self.hovering == 2;
            ctx.set_active(should_active);
            if should_active {
                ctx.set_focus(ctx.widget_id());
                ctx.set_handled();
            }
        } else if let Event::MouseUp(_m) = event {
            if self.down_hovering == self.hovering {
                match self.hovering {
                    1 => {
                        ctx.submit_notification(QUERY_CLOSE_WINDOW);
                        ctx.set_handled();
                    }
                    2 => {
                        ctx.submit_command(CONFIGURE_WINDOW.with(
                            WindowConfig::default().set_window_state(WindowState::Minimized),
                        ));
                        ctx.set_handled();
                    }
                    4 if self.show_back_btn => {
                        ctx.submit_notification(BACK_PAGE_CLICKED);
                        ctx.set_handled();
                    }
                    _ => ctx.set_active(false),
                }
            }
            self.down_hovering = u8::MAX;
        } else if let Event::WindowSize(_) = event {
            ctx.request_paint();
        } else if let Event::WindowCloseRequested = event {
            ctx.submit_notification(QUERY_CLOSE_WINDOW);
            ctx.set_handled();
        } else if let Event::WindowConnected = event {
            #[cfg(target_os = "windows")]
            if let RawWindowHandle::Win32(handle) = ctx.window().raw_window_handle() {
                unsafe {
                    use std::{ffi::OsStr, os::windows::ffi::OsStrExt};

                    use raw_window_handle_5::Win32WindowHandle;
                    use winapi::{
                        shared::{
                            minwindef::{LPARAM, *},
                            windef::*,
                        },
                        um::winuser::{LoadImageW, SendMessageW, IMAGE_ICON, WM_SETICON},
                    };
                    let small_icon = LoadImageW(
                        handle.hinstance as _,
                        OsStr::new("ICON")
                            .encode_wide()
                            .chain(Some(0))
                            .collect::<Vec<u16>>()
                            .as_ptr(),
                        IMAGE_ICON,
                        16,
                        16,
                        0,
                    );
                    let big_icon = LoadImageW(
                        handle.hinstance as _,
                        OsStr::new("ICON")
                            .encode_wide()
                            .chain(Some(0))
                            .collect::<Vec<u16>>()
                            .as_ptr(),
                        IMAGE_ICON,
                        32,
                        32,
                        0,
                    );
                    SendMessageW(handle.hwnd as _, WM_SETICON, 0, small_icon as LPARAM);
                    SendMessageW(handle.hwnd as _, WM_SETICON, 1, big_icon as LPARAM);
                    struct WinHandle {
                        handle: HWND,
                        instance: HINSTANCE,
                    }
                    unsafe impl HasRawWindowHandle for WinHandle {
                        fn raw_window_handle(&self) -> RawWindowHandle {
                            let mut handle = Win32WindowHandle::empty();
                            handle.hwnd = self.handle as _;
                            handle.hinstance = self.instance as _;
                            RawWindowHandle::Win32(handle)
                        }
                    }
                    window_shadows::set_shadow(
                        WinHandle {
                            handle: handle.hwnd as _,
                            instance: handle.hinstance as _,
                        },
                        true,
                    )
                    .unwrap_or_default();
                }
            }
            #[cfg(target_os = "macos")]
            if let RawWindowHandle::AppKit(handle) = ctx.window().raw_window_handle() {
                unsafe {
                    use cocoa::{
                        appkit::{NSControl, NSWindow, NSWindowStyleMask},
                        base::{id, NO, YES},
                    };
                    use objc::*;
                    let nsv = objc::rc::StrongPtr::new(handle.ns_view as *mut _);
                    let win: id = msg_send![*nsv, window];
                    if !win.is_null() {
                        win.setTitlebarAppearsTransparent_(cocoa::base::YES);
                        let mut mask = win.styleMask();
                        mask |= NSWindowStyleMask::NSFullSizeContentViewWindowMask;
                        win.setStyleMask_(mask);
                        win.setTitleVisibility_(
                            cocoa::appkit::NSWindowTitleVisibility::NSWindowTitleHidden,
                        );
                        let btn = win.standardWindowButton_(
                            cocoa::appkit::NSWindowButton::NSWindowFullScreenButton,
                        );

                        btn.setEnabled_(NO);
                        let _: id = msg_send![btn, setHidden: YES];

                        let btn = win.standardWindowButton_(
                            cocoa::appkit::NSWindowButton::NSWindowMiniaturizeButton,
                        );

                        btn.setEnabled_(NO);
                        let _: id = msg_send![btn, setHidden: YES];

                        let btn = win.standardWindowButton_(
                            cocoa::appkit::NSWindowButton::NSWindowZoomButton,
                        );

                        btn.setEnabled_(NO);
                        let _: id = msg_send![btn, setHidden: YES];

                        let color =
                            get_contrast_yiq(env.get(druid::theme::WINDOW_BACKGROUND_COLOR));
                        let (r, g, b, a) = color.as_rgba();
                        let color: id = msg_send![class!(NSColor), colorWithSRGBRed: 1. - r green: 1. - g blue: 1. - b alpha: a];

                        win.setBackgroundColor_(color);
                    }
                }
            }
        } else if let Event::Command(cmd) = event {
            if let Some(img_buf) = cmd.get(SET_BACKGROUND_IMAGE) {
                self.background_image = None;
                self.background_image_data = Some(img_buf.to_owned());
            } else if let Some(enable) = cmd.get(ENABLE_BACK_PAGE) {
                self.show_back_btn = *enable;
                ctx.request_update();
                ctx.request_paint();
                ctx.request_layout();
            } else if cmd.get(CLEAR_BACKGROUND_IMAGE).is_some() {
                self.background_image = None;
                self.background_image_data = None;
            }
        }
        self.inner.event(ctx, event, data, env)
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        bc.debug_check("Window");
        let wbc = *bc;
        let child_bc = wbc.shrink((0., TITLE_HEIGHT));
        let mut size = self.inner.layout(ctx, &child_bc, data, env);
        self.inner.set_origin(ctx, (0., TITLE_HEIGHT).into());
        size.height += TITLE_HEIGHT;
        bc.constrain(size)
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        self.inner.lifecycle(ctx, event, data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        self.paint_no_shadow(ctx, data, env, &ctx.size().to_rect().round())
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &T, data: &T, env: &Env) {
        self.inner.update(ctx, data, env);
        #[cfg(target_os = "macos")]
        if ctx.env_key_changed(&druid::theme::WINDOW_BACKGROUND_COLOR) {
            if let RawWindowHandle::AppKit(handle) = ctx.window().raw_window_handle() {
                unsafe {
                    use cocoa::{appkit::NSWindow, base::id};
                    use objc::*;
                    let nsv = objc::rc::StrongPtr::new(handle.ns_view as *mut _);
                    let win: id = msg_send![*nsv, window];
                    if !win.is_null() {
                        let color =
                            get_contrast_yiq(env.get(druid::theme::WINDOW_BACKGROUND_COLOR));
                        let (r, g, b, a) = color.as_rgba();
                        let color: id = msg_send![class!(NSColor), colorWithSRGBRed: 1. - r green: 1. - g blue: 1. - b alpha: a];

                        // let nsa: id = msg_send![class!(NSAppearance), currentDrawingAppearance];
                        // let name: id = msg_send![nsa, name];
                        // let name = NSString::UTF8String(name);
                        // let name = std::ffi::CStr::from_ptr(name);
                        // let name = name.to_str().unwrap_or_default().to_string();

                        // struct MacWindowHandle {
                        //     nsw: *mut libc::c_void,
                        //     nsv: *mut libc::c_void
                        // }

                        // unsafe impl raw_window_handle_5::HasRawWindowHandle for MacWindowHandle {
                        //     fn raw_window_handle(&self) -> raw_window_handle_5::RawWindowHandle {
                        //         let mut h = raw_window_handle_5::AppKitWindowHandle::empty();
                        //         h.ns_view = self.nsv;
                        //         h.ns_window = self.nsw;
                        //         raw_window_handle_5::RawWindowHandle::AppKit(h)
                        //     }
                        // }

                        // let _ = window_vibrancy::apply_vibrancy(MacWindowHandle {
                        //     nsv: handle.ns_view,
                        //     nsw: win as *mut _,
                        // }, window_vibrancy::NSVisualEffectMaterial::Sidebar);

                        // if &name == "darkAqra" {}

                        win.setBackgroundColor_(color);
                    }
                }
            }
        }
        #[cfg(target_os = "windows")]
        if ctx.env_key_changed(&druid::theme::WINDOW_BACKGROUND_COLOR) {
            ctx.request_paint();
        }
    }
}
