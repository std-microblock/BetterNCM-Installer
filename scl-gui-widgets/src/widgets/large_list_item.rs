use druid::{
    kurbo::{BezPath, Shape},
    piet::{PaintBrush, TextStorage},
    widget::{Click, ControllerHost, LabelText},
    Affine, BoxConstraints, Data, Event, LifeCycle, Point, RenderContext, Size, Widget, WidgetPod,
};

use super::label;
use crate::theme::{
    color::{
        base, list,
        main::IS_DARK,
        typography::{BODY, CAPTION_ALT},
    },
    icons::IconKeyPair,
};

/// 一个提供显示图标、主标签和副标签的大列表项目组件
pub struct LargeListItem<D> {
    icon_key: IconKeyPair,
    icon_path: BezPath,
    text: WidgetPod<D, Box<dyn Widget<D>>>,
    desc: WidgetPod<D, Box<dyn Widget<D>>>,
}

impl<D: Data> LargeListItem<D> {
    /// 根据所给的图标键组 [`crate::theme::icons::IconKeyPair`] 、主标签文本和副标签文本创建此组件
    pub fn new(
        icon_key: IconKeyPair,
        text: impl Into<LabelText<D>>,
        desc: impl Into<LabelText<D>>,
    ) -> LargeListItem<D> {
        LargeListItem {
            icon_key,
            icon_path: BezPath::new(),
            text: WidgetPod::new(Box::new(
                label::new(text.into())
                    .with_text_size(14.)
                    .with_text_color(base::MEDIUM)
                    .with_font(BODY)
                    .with_line_break_mode(druid::widget::LineBreaking::WordWrap),
            )),
            desc: WidgetPod::new(Box::new(
                label::new(desc.into())
                    .with_text_color(base::MEDIUM)
                    .with_font(CAPTION_ALT)
                    .with_line_break_mode(druid::widget::LineBreaking::WordWrap),
            )),
        }
    }

    /// Provide a closure to be called when this button is clicked.
    pub fn on_click(
        self,
        f: impl Fn(&mut druid::EventCtx, &mut D, &druid::Env) + 'static,
    ) -> ControllerHost<Self, Click<D>> {
        ControllerHost::new(self, Click::new(f))
    }

    fn reload_icon(&mut self, env: &druid::Env) {
        self.icon_path = BezPath::from_svg(env.get(&self.icon_key.0).as_str()).unwrap_or_default();
    }
}

impl<D: Data> Widget<D> for LargeListItem<D> {
    fn event(&mut self, ctx: &mut druid::EventCtx, event: &Event, data: &mut D, env: &druid::Env) {
        match event {
            Event::MouseDown(_) => {
                ctx.set_active(true);
                ctx.request_paint();
            }
            Event::MouseUp(_) => {
                if ctx.is_active() {
                    ctx.set_active(false);
                    ctx.request_paint();
                }
            }
            _ => (),
        }
        self.text.event(ctx, event, data, env);
        self.desc.event(ctx, event, data, env);
    }

    fn lifecycle(
        &mut self,
        ctx: &mut druid::LifeCycleCtx,
        event: &druid::LifeCycle,
        data: &D,
        env: &druid::Env,
    ) {
        if let LifeCycle::WidgetAdded = event {
            self.reload_icon(env);
        } else if let LifeCycle::HotChanged(_) = event {
            ctx.request_paint();
        }
        self.text.lifecycle(ctx, event, data, env);
        self.desc.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut druid::UpdateCtx, _old_data: &D, _data: &D, env: &druid::Env) {
        if ctx.env_key_changed(&self.icon_key.0) {
            self.reload_icon(env);
        }
        if ctx.env_key_changed(&self.icon_key.1) | ctx.env_key_changed(&IS_DARK) {
            ctx.request_paint();
        }
    }

    fn layout(
        &mut self,
        ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        data: &D,
        env: &druid::Env,
    ) -> druid::Size {
        bc.debug_check("LoginMethodItem");
        let text_bc =
            BoxConstraints::new(Size::ZERO, Size::new(bc.max().width - 60., bc.max().height));
        let text_size = self.text.layout(ctx, &text_bc, data, env);
        let desc_size = self.desc.layout(ctx, &text_bc, data, env);
        let text_height = text_size.height;
        let desc_height = desc_size.height;
        let height = (desc_height + text_height + 27.).max(60.);
        let start_pos = (height - text_height - desc_height) / 2.;
        self.text.set_origin(ctx, Point::new(60., start_pos));
        self.desc
            .set_origin(ctx, Point::new(60., start_pos + text_height));
        bc.constrain((0., height))
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &D, env: &druid::Env) {
        let icon_brush = PaintBrush::Color(if env.get(IS_DARK) {
            env.get(&self.icon_key.2)
        } else {
            env.get(&self.icon_key.1)
        });
        let size = ctx.size();
        let is_hot = ctx.is_hot();
        let is_active = ctx.is_active();

        if is_hot {
            ctx.fill(
                size.to_rect(),
                &PaintBrush::Color(if is_active {
                    env.get(base::LOW)
                } else {
                    env.get(list::LIST_LOW)
                }),
            )
        }
        let icon_size = druid::Size::new(60., size.height);
        ctx.with_save(|ctx| {
            ctx.transform(Affine::translate(
                ((icon_size - self.icon_path.bounding_box().size()) / 2.).to_vec2(),
            ));
            ctx.fill_even_odd(&self.icon_path, &icon_brush)
        });
        self.text.paint(ctx, data, env);
        self.desc.paint(ctx, data, env);
    }
}
