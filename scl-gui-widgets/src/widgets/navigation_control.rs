//! 导航控件条，用于设置页面的分页

use druid::{
    piet::{Text, TextLayout, TextLayoutBuilder},
    BoxConstraints, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx,
    RenderContext, Size, UpdateCtx, Widget,
};
use scl_gui_animation::Spring;

const FONT_SIZE: f64 = 14.;
const BUTTON_PADDING: f64 = 12.;
const NAV_HEIGHT: f64 = 40.;

#[derive(Debug, PartialEq, Eq)]
enum SpringState {
    Init,
    Static,
    UpdateBoth,
    UpdateStart,
    UpdateEnd,
}

/// 一个导航分页组件，提供了类似 WinUI 3 中的 [Pivot](https://docs.microsoft.com/en-us/windows/apps/design/controls/pivot) 组件
///
/// 可以通过增加页面和传递对应 [`usize`] 值的数据源来切换页面索引，之后你可以使用 [`druid::widget::ViewSwitcher`] 来显示你需要的页面
pub struct NavigationControl {
    pages: Vec<String>,
    pages_text: Vec<Option<druid::piet::PietTextLayout>>,
    hovering_page: Option<usize>,
    hover_bar_start_spring: Spring,
    hover_bar_end_spring: Spring,
    response_timer: druid::TimerToken,
    spring_state: SpringState,
}

impl NavigationControl {
    /// 创建一个没有任何页面的导航分页组件
    pub fn new() -> Self {
        Self::default()
    }

    /// 为这个导航组件增加一个页面，每页的索引值由添加顺序从 0 开始递增
    pub fn add_page(&mut self, page_name: String) {
        self.pages.push(page_name);
        self.pages_text.push(None);
    }

    /// 以 Builder 模式为这个导航组件增加一个页面，每页的索引值由添加顺序从 0 开始递增
    pub fn with_page(mut self, page_name: String) -> Self {
        self.add_page(page_name);
        self
    }
}

impl Default for NavigationControl {
    fn default() -> Self {
        Self {
            pages: Vec::with_capacity(8),
            pages_text: Vec::with_capacity(8),
            hovering_page: None,
            hover_bar_start_spring: Spring::new(0.),
            hover_bar_end_spring: Spring::new(0.),
            response_timer: druid::TimerToken::INVALID,
            spring_state: SpringState::Init,
        }
    }
}

impl Widget<usize> for NavigationControl {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut usize, _env: &Env) {
        match event {
            Event::MouseMove(m) => {
                if ctx.is_hot() {
                    // 检测按钮距离，悬浮合适的按钮
                    let mut pos_x = m.pos.x;
                    let mut checked = false;
                    let last_page = self.hovering_page;
                    for (index, btn) in self.pages_text.iter().enumerate() {
                        if let Some(btn) = btn {
                            let button_width = btn.size().width + BUTTON_PADDING * 2.;
                            if pos_x < button_width {
                                checked = true;
                                self.hovering_page = Some(index);
                                break;
                            }
                            pos_x -= button_width;
                        }
                    }
                    if !checked {
                        if let Some(last_page_index) = last_page {
                            self.pages_text[last_page_index] = None;
                        }
                        self.hovering_page = None;
                    }
                    if last_page != self.hovering_page {
                        if let Some(last_page_index) = last_page {
                            self.pages_text[last_page_index] = None;
                        }
                        if let Some(last_page_index) = self.hovering_page {
                            self.pages_text[last_page_index] = None;
                        }
                        ctx.request_paint();
                    }
                } else if let Some(page_index) = self.hovering_page.take() {
                    self.pages_text[page_index] = None;
                    ctx.request_paint();
                }
            }
            Event::MouseDown(_) => {
                ctx.set_active(true);
                if let Some(page_index) = self.hovering_page {
                    self.pages_text[page_index] = None;
                    ctx.request_paint();
                }
            }
            Event::MouseUp(_) => {
                if ctx.is_active() {
                    ctx.set_active(false);
                    if let Some(page_index) = self.hovering_page {
                        if *data > page_index {
                            self.spring_state = SpringState::UpdateStart;
                        } else {
                            self.spring_state = SpringState::UpdateEnd;
                        }
                        *data = page_index;
                        self.pages_text[page_index] = None;
                        self.response_timer =
                            ctx.request_timer(std::time::Duration::from_millis(150));
                        ctx.request_anim_frame();
                    }
                }
            }
            Event::Timer(t) => {
                if &self.response_timer == t {
                    self.spring_state = SpringState::UpdateBoth;
                    ctx.request_anim_frame();
                }
            }
            Event::AnimFrame(_) => {
                ctx.request_paint();
                if self.spring_state != SpringState::Static
                    || !(self.hover_bar_end_spring.arrived()
                        && self.hover_bar_start_spring.arrived())
                {
                    ctx.request_anim_frame();
                }
            }
            _ => {}
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, _data: &usize, _env: &Env) {
        if let LifeCycle::HotChanged(is_hot) = event {
            if !*is_hot {
                if let Some(page_index) = self.hovering_page.take() {
                    self.pages_text[page_index] = None;
                    ctx.request_paint();
                }
            }
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &usize, data: &usize, _env: &Env) {
        if ctx.env_key_changed(&crate::theme::color::base::HIGH)
            || ctx.env_key_changed(&crate::theme::color::base::MEDIUM_HIGH)
            || ctx.env_key_changed(&crate::theme::color::base::MEDIUM)
        {
            for text in self.pages_text.iter_mut() {
                *text = None;
            }
            ctx.request_paint();
        }
        if old_data != data {
            self.pages_text[*old_data] = None;
            self.pages_text[*data] = None;
            ctx.request_paint();
        }
    }

    fn layout(
        &mut self,
        _ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &usize,
        _env: &Env,
    ) -> Size {
        bc.debug_check("NavigationControl");
        bc.constrain((100., NAV_HEIGHT))
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &usize, env: &Env) {
        let primary = env.get(crate::theme::color::main::PRIMARY);
        let font = env.get(crate::theme::color::typography::BODY);
        let hot_color = env.get(crate::theme::color::base::HIGH);
        let active_color = env.get(crate::theme::color::base::MEDIUM_HIGH);
        let normal_color = env.get(crate::theme::color::base::MEDIUM);
        let is_active = ctx.is_active();
        let t = ctx.text();
        let mut start_pos = 0.;
        for (index, text) in &mut self.pages_text.iter_mut().enumerate() {
            if text.is_none() {
                if let Some(raw_text) = self.pages.get(index) {
                    *text = Some(
                        t.new_text_layout(raw_text.to_owned())
                            .text_color(if *data == index {
                                hot_color.to_owned()
                            } else if let Some(s) = self.hovering_page {
                                if s == index {
                                    if is_active {
                                        active_color.to_owned()
                                    } else {
                                        hot_color.to_owned()
                                    }
                                } else {
                                    normal_color.to_owned()
                                }
                            } else {
                                normal_color.to_owned()
                            })
                            .font(font.family.to_owned(), FONT_SIZE)
                            .build()
                            .unwrap(),
                    );
                }
            }
        }
        for (index, text) in self.pages_text.iter().enumerate() {
            if let Some(text) = text {
                let text_size = text.size();
                let button_width = text_size.width + BUTTON_PADDING * 2.;

                if index == *data {
                    match self.spring_state {
                        SpringState::Static => {}
                        SpringState::Init => {
                            self.hover_bar_start_spring =
                                Spring::new(start_pos + BUTTON_PADDING).with_damper(0.8);
                            self.hover_bar_end_spring =
                                Spring::new(start_pos + BUTTON_PADDING + text_size.width)
                                    .with_damper(0.8);
                            self.spring_state = SpringState::Static;
                        }
                        SpringState::UpdateBoth => {
                            if self.hover_bar_start_spring.target() != start_pos + BUTTON_PADDING {
                                self.hover_bar_start_spring
                                    .set_target(start_pos + BUTTON_PADDING);
                            }
                            if self.hover_bar_end_spring.target()
                                != start_pos + BUTTON_PADDING + text_size.width
                            {
                                self.hover_bar_end_spring
                                    .set_target(start_pos + BUTTON_PADDING + text_size.width);
                            }
                            self.spring_state = SpringState::Static;
                        }
                        SpringState::UpdateStart => {
                            self.hover_bar_start_spring
                                .set_target(start_pos + BUTTON_PADDING);
                            self.spring_state = SpringState::Static;
                        }
                        SpringState::UpdateEnd => {
                            self.hover_bar_end_spring
                                .set_target(start_pos + BUTTON_PADDING + text_size.width);
                            self.spring_state = SpringState::Static;
                        }
                    }
                }

                ctx.draw_text(
                    text,
                    (
                        start_pos + BUTTON_PADDING,
                        (NAV_HEIGHT - text_size.height) / 2.,
                    ),
                );
                start_pos += button_width;
            }
        }
        // 滑块条
        let slider_rect = druid::Rect::new(
            self.hover_bar_start_spring.position_rounded(),
            NAV_HEIGHT - 6.,
            self.hover_bar_end_spring.position_rounded(),
            NAV_HEIGHT - 4.,
        );
        let slider_rect = slider_rect.to_rounded_rect(2.);
        ctx.fill(slider_rect, &primary);
    }
}
