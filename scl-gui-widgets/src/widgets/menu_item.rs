use druid::{
    kurbo::{BezPath, RoundedRect},
    piet::{PaintBrush, PietTextLayout, StrokeStyle, Text, TextLayoutBuilder},
    widget::{Label, LabelText},
    Color, Data, RenderContext, Widget, WidgetPod,
};

use crate::widgets::label;

/// 一个左侧显示内容，右侧显示勾选框或文本内容的组件
///
/// 如果数据源是 [`bool`] 类型的话，组件右侧会变成勾选框，且在没有被禁用的情况下可以开关并更新数据
///
/// 如果数据源是 [`String`] 类型的话，组件右侧会变成普通文本并显示数据的文本
pub struct MenuItem<D> {
    title: WidgetPod<D, Label<D>>,
    desc_text: Option<PietTextLayout>,
}

const LEFT_RIGHT_PADDING: f64 = 5.;

impl<D: Data> MenuItem<D> {
    /// 以左侧标签文本为参数创建此组件
    pub fn new(title: impl Into<LabelText<D>>) -> Self {
        MenuItem {
            title: WidgetPod::new(label::new(title)),
            desc_text: None,
        }
    }
}

/// 默认实现：右侧是一个箭头
impl Widget<()> for MenuItem<()> {
    fn event(
        &mut self,
        _ctx: &mut druid::EventCtx,
        _event: &druid::Event,
        _data: &mut (),
        _env: &druid::Env,
    ) {
        todo!()
    }

    fn lifecycle(
        &mut self,
        _ctx: &mut druid::LifeCycleCtx,
        _event: &druid::LifeCycle,
        _data: &(),
        _env: &druid::Env,
    ) {
        todo!()
    }

    fn update(
        &mut self,
        _ctx: &mut druid::UpdateCtx,
        _old_data: &(),
        _data: &(),
        _env: &druid::Env,
    ) {
        todo!()
    }

    fn layout(
        &mut self,
        _ctx: &mut druid::LayoutCtx,
        _bc: &druid::BoxConstraints,
        _data: &(),
        _env: &druid::Env,
    ) -> druid::Size {
        todo!()
    }

    fn paint(&mut self, _ctx: &mut druid::PaintCtx, _data: &(), _env: &druid::Env) {
        todo!()
    }
}

/// 如果传入的是布尔值，则右侧显示为勾选框
impl Widget<bool> for MenuItem<bool> {
    fn event(
        &mut self,
        ctx: &mut druid::EventCtx,
        event: &druid::Event,
        data: &mut bool,
        env: &druid::Env,
    ) {
        if let druid::Event::MouseDown(e) = event {
            if e.buttons.contains(druid::MouseButton::Left) && !ctx.is_disabled() {
                *data = !*data;
                ctx.request_paint();
            }
        }
        self.title.event(ctx, event, data, env);
    }

    fn lifecycle(
        &mut self,
        ctx: &mut druid::LifeCycleCtx,
        event: &druid::LifeCycle,
        data: &bool,
        env: &druid::Env,
    ) {
        if let druid::LifeCycle::HotChanged(_) = event {
            ctx.request_paint();
        }
        if let druid::LifeCycle::FocusChanged(_) = event {
            ctx.request_paint();
        }
        self.title.lifecycle(ctx, event, data, env);
    }

    fn update(
        &mut self,
        ctx: &mut druid::UpdateCtx,
        _old_data: &bool,
        data: &bool,
        env: &druid::Env,
    ) {
        if ctx.env_key_changed(&crate::theme::color::base::LOW)
            || ctx.env_key_changed(&crate::theme::color::list::LIST_LOW)
        {
            ctx.request_paint();
        }
        self.title.update(ctx, data, env);
    }

    fn layout(
        &mut self,
        ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        data: &bool,
        env: &druid::Env,
    ) -> druid::Size {
        let _ = self.title.layout(ctx, bc, data, env);
        self.title
            .set_origin(ctx, (8. + LEFT_RIGHT_PADDING, 7.).into());
        bc.constrain((100., 32.))
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &bool, env: &druid::Env) {
        let is_active = ctx.is_active();
        let is_disabled = ctx.is_disabled();
        let r = ctx.size().to_rect();

        if ctx.is_hot() && !is_disabled {
            ctx.fill(
                r,
                &PaintBrush::Color(if is_active {
                    env.get(crate::theme::color::base::LOW)
                } else {
                    env.get(crate::theme::color::list::LIST_LOW)
                }),
            );
        }

        self.title.paint(ctx, data, env);

        if *data {
            let rr = RoundedRect::from_origin_size(
                (r.width() - 32. - LEFT_RIGHT_PADDING + 6.5, 6.5),
                (19., 19.),
                3.5,
            );
            // ctx.is_disabled() 21.69%
            ctx.stroke(
                rr,
                &PaintBrush::Color(env.get(crate::theme::color::main::SECONDARY)),
                1.,
            );
            let rr = RoundedRect::from_origin_size(
                (r.width() - 32. - LEFT_RIGHT_PADDING + 7., 7.),
                (18., 18.),
                3.,
            );
            ctx.fill(
                rr,
                &PaintBrush::Color(env.get(crate::theme::color::main::PRIMARY)),
            );
            let mut check_path = BezPath::new();
            check_path.move_to((r.width() - 32. - LEFT_RIGHT_PADDING + 11.5, 16.5));
            check_path.line_to((r.width() - 32. - LEFT_RIGHT_PADDING + 14.5, 19.5));
            check_path.line_to((r.width() - 32. - LEFT_RIGHT_PADDING + 20.5, 13.5));
            ctx.stroke_styled(
                &check_path,
                &PaintBrush::Color(Color::WHITE),
                1.,
                &StrokeStyle {
                    line_join: druid::piet::LineJoin::Round,
                    line_cap: druid::piet::LineCap::Round,
                    ..Default::default()
                },
            );
        } else {
            let srr = RoundedRect::from_origin_size(
                (r.width() - 32. - LEFT_RIGHT_PADDING + 6.5, 6.5),
                (19., 19.),
                3.,
            );
            let frr = RoundedRect::from_origin_size(
                (r.width() - 32. - LEFT_RIGHT_PADDING + 7., 7.),
                (18., 18.),
                3.,
            );
            if env.get(crate::theme::color::main::IS_DARK) {
                ctx.stroke(srr, &PaintBrush::Color(Color::WHITE.with_alpha(0.5442)), 1.);
                ctx.stroke(frr, &PaintBrush::Color(Color::BLACK.with_alpha(0.1)), 1.);
            } else {
                ctx.stroke(srr, &PaintBrush::Color(Color::BLACK.with_alpha(0.4458)), 1.);
                ctx.stroke(frr, &PaintBrush::Color(Color::BLACK.with_alpha(0.4458)), 1.);
            }
        }
    }
}

/// 如果传入的是字符串，则右侧显示其为副标签
impl Widget<String> for MenuItem<String> {
    fn event(
        &mut self,
        ctx: &mut druid::EventCtx,
        event: &druid::Event,
        data: &mut String,
        env: &druid::Env,
    ) {
        self.title.event(ctx, event, data, env);
    }

    fn lifecycle(
        &mut self,
        ctx: &mut druid::LifeCycleCtx,
        event: &druid::LifeCycle,
        data: &String,
        env: &druid::Env,
    ) {
        if let druid::LifeCycle::HotChanged(_) = event {
            ctx.request_paint();
        }
        if let druid::LifeCycle::FocusChanged(_) = event {
            ctx.request_paint();
        }
        self.title.lifecycle(ctx, event, data, env);
    }

    fn update(
        &mut self,
        ctx: &mut druid::UpdateCtx,
        old_data: &String,
        data: &String,
        env: &druid::Env,
    ) {
        if !old_data.same(data) {
            self.desc_text = None;
            ctx.request_paint();
        }
        if ctx.env_key_changed(&crate::theme::color::base::MEDIUM)
            || ctx.env_key_changed(&crate::theme::color::typography::BODY)
        {
            ctx.request_paint();
        }
        if ctx.env_key_changed(&crate::theme::color::base::LOW)
            || ctx.env_key_changed(&crate::theme::color::list::LIST_LOW)
        {
            ctx.request_paint();
        }
        self.title.update(ctx, data, env);
    }

    fn layout(
        &mut self,
        ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        data: &String,
        env: &druid::Env,
    ) -> druid::Size {
        let _ = self.title.layout(ctx, bc, data, env);
        self.title
            .set_origin(ctx, (8. + LEFT_RIGHT_PADDING, 7.).into());
        self.desc_text = None;
        bc.constrain((100., 32.))
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &String, env: &druid::Env) {
        let is_active = ctx.is_active();
        let r = ctx.size().to_rect();

        if ctx.is_hot() {
            ctx.fill(
                r,
                &PaintBrush::Color(if is_active {
                    env.get(crate::theme::color::base::LOW)
                } else {
                    env.get(crate::theme::color::list::LIST_LOW)
                }),
            );
        }

        self.title.paint(ctx, data, env);

        if self.desc_text.is_none() {
            let font = env.get(crate::theme::color::typography::BODY);
            self.desc_text = Some(
                ctx.text()
                    .new_text_layout(data.to_owned())
                    .alignment(druid::TextAlignment::End)
                    .max_width(r.width() - 16.)
                    .font(font.family, 13.)
                    .text_color(env.get(crate::theme::color::base::MEDIUM))
                    .build()
                    .unwrap(),
            );
        }

        if data.is_empty() {
            let mut check_path = BezPath::new();
            check_path.move_to((r.width() - 32. - LEFT_RIGHT_PADDING + 14., 13.));
            check_path.line_to((r.width() - 32. - LEFT_RIGHT_PADDING + 17.5, 16.5));
            check_path.line_to((r.width() - 32. - LEFT_RIGHT_PADDING + 14., 20.));
            ctx.stroke_styled(
                &check_path,
                &PaintBrush::Color(Color::WHITE),
                1.,
                &StrokeStyle {
                    line_join: druid::piet::LineJoin::Round,
                    line_cap: druid::piet::LineCap::Round,
                    ..Default::default()
                },
            );
        } else if let Some(desc_text) = &self.desc_text {
            #[cfg(windows)]
            ctx.draw_text(desc_text, (0., 7.));
            #[cfg(not(windows))]
            {
                use druid::piet::TextLayout;
                ctx.draw_text(desc_text, (r.width() - 14. - desc_text.size().width, 7.))
            }
        }
    }
}
