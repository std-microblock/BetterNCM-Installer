use std::collections::BTreeMap;

use druid::{
    piet::{TextLayout, *},
    *,
};
use scl_gui_animation::Spring;

/// 用于增加新进度的指令，需要传递一个独一无二的编号作为此进度的唯一编号
pub const NEW_PROGRESS: Selector<usize> = Selector::new("net.stevexmh.scl.progress.new");
/// 用于删除进度的指令，需要传递此进度的唯一编号
pub const REMOVE_PROGRESS: Selector<usize> = Selector::new("net.stevexmh.scl.progress.rm");
/// 用于给进度设置主要信息，需要传递此进度的唯一编号
pub const SET_MESSAGE: Selector<(usize, String)> =
    Selector::new("net.stevexmh.scl.progress.set-msg");
/// 用于给进度设置次要信息，需要传递此进度的唯一编号和其需要展示的信息
pub const SET_SUB_MESSAGE: Selector<(usize, String)> =
    Selector::new("net.stevexmh.scl.progress.set-sub-msg");
/// 用于给进度设置最大进度值，需要传递此进度的唯一编号和其进度最大值
pub const SET_MAX_PROGRESS: Selector<(usize, f64)> =
    Selector::new("net.stevexmh.scl.progress.set-max");
/// 用于给进度增加/减少最大进度值，需要传递此进度的唯一编号和其进度最大值变化值
pub const ADD_MAX_PROGRESS: Selector<(usize, f64)> =
    Selector::new("net.stevexmh.scl.progress.add-max");
/// 用于给进度设置进度值，需要传递此进度的唯一编号和其进度值
pub const SET_PROGRESS: Selector<(usize, f64)> = Selector::new("net.stevexmh.scl.progress.set");
/// 用于给进度增加/减少进度值，需要传递此进度的唯一编号和其进度变化值
pub const ADD_PROGRESS: Selector<(usize, f64)> = Selector::new("net.stevexmh.scl.progress.add");
/// 用于给进度隐藏进度值，需要传递此进度的唯一编号
pub const HIDE_PROGRESS: Selector<usize> = Selector::new("net.stevexmh.scl.progress.hide");

struct ProgressState {
    max_progress: f64,
    progress: f64,
    current_progress: f64,
    indeterminate: bool,
    msg: String,
    msg_layout: Option<PietTextLayout>,
    sub_msg: String,
    sub_msg_layout: Option<PietTextLayout>,
}

/// 一个进度显示组件，用于搭配 SCL Core 的进度报告功能显示正在处理的项目进度
///
/// 在没有进度时会隐藏，当存在进度时将会占用一点位置来显示进度。
/// 此时如果鼠标悬浮在这个区域则会放大进度组件以查看正在处理的所有进度。
pub struct ProgressOverlay<T> {
    progress_expanding: bool,
    progress_height_spring: Spring,
    progress_map: BTreeMap<usize, ProgressState>,

    inner: WidgetPod<T, Box<dyn Widget<T>>>,
}

impl<T> ProgressOverlay<T> {
    /// 以一个将会显示进度的组件为参数创建此组件
    pub fn new(inner: impl Widget<T> + 'static) -> Self {
        Self {
            progress_expanding: false,
            progress_height_spring: Spring::new(0.),
            progress_map: BTreeMap::new(),
            inner: WidgetPod::new(Box::new(inner)),
        }
    }

    fn should_display(&self) -> bool {
        !self.progress_map.is_empty()
    }

    fn should_update_progress(&self) -> bool {
        if self.should_display() {
            for p in self.progress_map.values() {
                if p.current_progress != p.progress {
                    return true;
                }
            }
        }
        false
    }

    fn paint_progress(&mut self, ctx: &mut PaintCtx, _data: &T, env: &Env) {
        ctx.with_save(|ctx| {
            // Envs
            let bg = env
                .get(druid::theme::WINDOW_BACKGROUND_COLOR)
                .with_alpha(1.);
            let base_low = env.get(crate::theme::color::base::LOW);
            let base_high = env.get(crate::theme::color::base::HIGH);
            let accent_sec = env.get(crate::theme::color::main::SECONDARY);
            let base_font = env.get(crate::theme::color::typography::BASE).family;

            let linear_brush = PaintBrush::Linear(LinearGradient::new(
                UnitPoint::LEFT,
                UnitPoint::RIGHT,
                (bg.to_owned().with_alpha(0.), bg.to_owned()),
            ));

            let width = ctx.size().width;
            let height = ctx.size().height;
            let mut current_height = height - self.progress_height_spring.position();
            let rect = Rect::new(0., current_height, width, height);
            ctx.fill(rect, &bg);
            let rect = Rect::new(0., current_height, width, current_height + 1.);
            ctx.fill(rect, &base_low);

            for (i, (_, item)) in self.progress_map.iter_mut().enumerate() {
                if item.progress.round() == item.max_progress.round()
                    && item.msg.is_empty()
                    && item.sub_msg.is_empty()
                {
                    continue;
                }
                item.current_progress = item.current_progress
                    + ((item.progress / item.max_progress) - item.current_progress) / 7.;
                let text = ctx.text();
                if item.msg_layout.is_none() {
                    item.msg_layout = text
                        .new_text_layout(item.msg.to_owned())
                        .alignment(TextAlignment::Start)
                        .text_color(base_high.to_owned())
                        .font(base_font.to_owned(), 12.)
                        .build()
                        .ok();
                }
                let layout_height = item
                    .msg_layout
                    .as_ref()
                    .map(|x| x.size().height)
                    .unwrap_or(0.);
                if item.sub_msg_layout.is_none() {
                    item.sub_msg_layout = text
                        .new_text_layout(if item.sub_msg.is_empty() {
                            if item.progress.round() != item.max_progress.round() {
                                format!("{}/{}", item.progress.round(), item.max_progress.round())
                            } else {
                                "".into()
                            }
                        } else {
                            item.sub_msg.to_owned()
                        })
                        .alignment(TextAlignment::Start)
                        .text_color(base_high.to_owned())
                        .font(base_font.to_owned(), 12.)
                        .build()
                        .ok();
                }
                let right_layout_width = item
                    .sub_msg_layout
                    .as_ref()
                    .map(|x| x.size().width)
                    .unwrap_or(0.);
                ctx.with_save(|ctx| {
                    ctx.clip(Rect::new(
                        12.,
                        current_height + 12.,
                        width - right_layout_width - 20.,
                        current_height + 12. + layout_height,
                    ));
                    if let Some(layout) = &item.msg_layout {
                        ctx.draw_text(layout, (12., current_height + 12.));
                    }
                });
                ctx.fill(
                    Rect::new(
                        width - right_layout_width - 20.,
                        current_height + 12.,
                        width - right_layout_width - 30.,
                        current_height + 12. + layout_height,
                    ),
                    &linear_brush,
                );
                if let Some(layout) = &item.sub_msg_layout {
                    ctx.draw_text(
                        layout,
                        (width - 20. - right_layout_width, current_height + 12.),
                    );
                }
                if !item.indeterminate {
                    let progress_bar_rect =
                        Rect::from_origin_size((10., current_height + 35.), (width - 20., 4.));
                    ctx.fill(
                        progress_bar_rect.to_rounded_rect(progress_bar_rect.height() / 2.),
                        &base_low,
                    );
                    let progress_bar_current_rect = Rect::from_origin_size(
                        progress_bar_rect.origin(),
                        (
                            progress_bar_rect.width() * item.current_progress.clamp(0., 1.),
                            progress_bar_rect.height(),
                        ),
                    );
                    ctx.fill(
                        progress_bar_current_rect.to_rounded_rect(progress_bar_rect.height() / 2.),
                        &accent_sec,
                    );
                }
                if i > 0 && current_height > height - 100. {
                    ctx.fill(
                        Rect::from_origin_size((0., current_height), (width, 50.)),
                        &bg.with_alpha((current_height - (height - 100.)) * 0.4 / 20.),
                    );
                    break;
                }
                current_height += 30.;
                if item.progress > 0. && item.progress <= 1. {
                    current_height += 20.;
                }
            }
        })
    }
}

impl<T: Data> Widget<T> for ProgressOverlay<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        match event {
            Event::MouseMove(m) => {
                let size = ctx.size();
                let width = size.width;
                let height = size.height;
                let m_pos_x = m.pos.x;
                let m_pos_y = m.pos.y;
                let old_progress_expanding = self.progress_expanding;
                self.progress_expanding = self.should_display()
                    && m_pos_y > height - 50.
                    && m_pos_y < height
                    && m_pos_x > 0.
                    && m_pos_x < width;
                if old_progress_expanding != self.progress_expanding {
                    self.progress_height_spring
                        .set_target(if self.progress_expanding {
                            height
                        } else if self.should_display() {
                            50.
                        } else {
                            0.
                        });
                    ctx.request_anim_frame();
                }
            }
            Event::AnimFrame(_) => {
                if self.should_update_progress() || !self.progress_height_spring.arrived() {
                    ctx.request_paint();
                    ctx.request_anim_frame();
                    ctx.request_layout();
                }
            }
            Event::Command(cmd) => {
                if let Some(&p) = cmd.get(NEW_PROGRESS) {
                    self.progress_height_spring
                        .set_target(if self.progress_map.len() > 1 {
                            if self.progress_expanding {
                                ctx.size().height
                            } else {
                                50.
                            }
                        } else {
                            50.
                        });
                    self.progress_map.insert(
                        p,
                        ProgressState {
                            max_progress: 0.,
                            progress: 0.,
                            current_progress: 0.,
                            indeterminate: true,
                            msg: String::new(),
                            msg_layout: None,
                            sub_msg: String::new(),
                            sub_msg_layout: None,
                        },
                    );
                    ctx.request_anim_frame();
                } else if let Some(&p) = cmd.get(REMOVE_PROGRESS) {
                    self.progress_map.remove(&p);
                    if self.progress_map.is_empty() {
                        self.progress_height_spring.set_target(0.);
                    }
                    ctx.request_anim_frame();
                } else if let Some(p) = cmd.get(SET_PROGRESS) {
                    if let Some(v) = self.progress_map.get_mut(&p.0) {
                        v.progress = p.1;
                        if v.indeterminate {
                            v.indeterminate = false;
                            v.current_progress = v.progress;
                        }
                        if v.sub_msg.is_empty() {
                            v.sub_msg_layout = None;
                        }
                        ctx.request_anim_frame();
                    }
                } else if let Some(p) = cmd.get(ADD_PROGRESS) {
                    if let Some(v) = self.progress_map.get_mut(&p.0) {
                        v.progress += p.1;
                        v.indeterminate = false;
                        if v.sub_msg.is_empty() {
                            v.sub_msg_layout = None;
                        }
                        ctx.request_anim_frame();
                    }
                } else if let Some(p) = cmd.get(SET_MAX_PROGRESS) {
                    if let Some(v) = self.progress_map.get_mut(&p.0) {
                        v.max_progress = p.1;
                        v.indeterminate = false;
                        if v.sub_msg.is_empty() {
                            v.sub_msg_layout = None;
                        }
                        ctx.request_anim_frame();
                    }
                } else if let Some(p) = cmd.get(ADD_MAX_PROGRESS) {
                    if let Some(v) = self.progress_map.get_mut(&p.0) {
                        v.max_progress += p.1;
                        v.indeterminate = false;
                        if v.sub_msg.is_empty() {
                            v.sub_msg_layout = None;
                        }
                        ctx.request_anim_frame();
                    }
                } else if let Some(p) = cmd.get(HIDE_PROGRESS) {
                    if let Some(v) = self.progress_map.get_mut(p) {
                        v.indeterminate = true;
                        ctx.request_anim_frame();
                    }
                } else if let Some(p) = cmd.get(SET_MESSAGE) {
                    if let Some(v) = self.progress_map.get_mut(&p.0) {
                        v.msg = p.1.to_owned();
                        v.msg_layout = None;
                        ctx.request_anim_frame();
                    }
                } else if let Some(p) = cmd.get(SET_SUB_MESSAGE) {
                    if let Some(v) = self.progress_map.get_mut(&p.0) {
                        v.sub_msg = p.1.to_owned();
                        v.sub_msg_layout = None;
                        ctx.request_anim_frame();
                    }
                }
            }
            _ => {}
        }
        self.inner.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        self.inner.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &T, data: &T, env: &Env) {
        self.inner.update(ctx, data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        let progress_area_height = self
            .progress_height_spring
            .position_rounded()
            .clamp(0., 50.);
        let mut size = if progress_area_height == 0. {
            self.inner.layout(ctx, bc, data, env)
        } else {
            self.inner
                .layout(ctx, &bc.shrink((0., progress_area_height)), data, env)
        };
        size.height += progress_area_height;
        bc.constrain(size)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        ctx.with_save(|ctx| self.inner.paint(ctx, data, env));
        self.paint_progress(ctx, data, env);
    }
}
