use druid::{
    kurbo::{BezPath, Shape},
    piet::{PaintBrush, TextStorage},
    widget::{Click, ControllerHost, LabelText},
    Affine, BoxConstraints, Data, Env, Event, LifeCycle, RenderContext, Widget, WidgetExt,
    WidgetPod,
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

/// 一个左侧有图标和说明信息，右侧有副文本信息的可点击项组件
pub struct DownloadModuleItem<D> {
    icon_key: IconKeyPair,
    icon_path: BezPath,
    text: WidgetPod<D, Box<dyn Widget<D>>>,
    desc: WidgetPod<D, Box<dyn Widget<D>>>,
}

impl<D: Data> DownloadModuleItem<D> {
    /// 根据所给的图标对（通过 `scl-macros` 来生成图标）创建组件
    pub fn new(
        icon_key: IconKeyPair,
        text: impl Into<LabelText<D>>,
        desc: impl Into<LabelText<D>>,
    ) -> Self {
        Self {
            icon_key,
            icon_path: BezPath::new(),
            text: WidgetPod::new(Box::new(
                label::new(text)
                    .with_text_size(14.)
                    .with_text_color(base::MEDIUM)
                    .with_font(BODY)
                    .align_vertical(druid::UnitPoint::LEFT),
            )),
            desc: WidgetPod::new(Box::new(
                label::new(desc)
                    .with_text_color(base::MEDIUM)
                    .with_font(CAPTION_ALT)
                    .align_vertical(druid::UnitPoint::RIGHT),
            )),
        }
    }

    /// 根据所给的图标对（通过 `scl-macros` 来生成图标）创建组件，但可以是动态文字
    pub fn dynamic(
        icon_key: IconKeyPair,
        text: impl Fn(&D, &Env) -> String + 'static,
        desc: impl Fn(&D, &Env) -> String + 'static,
    ) -> Self {
        Self {
            icon_key,
            icon_path: BezPath::new(),
            text: WidgetPod::new(Box::new(
                label::dynamic(text)
                    .with_text_size(14.)
                    .with_text_color(base::MEDIUM)
                    .with_font(BODY)
                    .with_line_break_mode(druid::widget::LineBreaking::Clip)
                    .align_vertical(druid::UnitPoint::LEFT),
            )),
            desc: WidgetPod::new(Box::new(
                label::dynamic(desc)
                    .with_text_color(base::MEDIUM)
                    .with_font(CAPTION_ALT)
                    .with_line_break_mode(druid::widget::LineBreaking::Clip)
                    .align_vertical(druid::UnitPoint::RIGHT),
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

impl<D: Data> Widget<D> for DownloadModuleItem<D> {
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

    fn update(&mut self, ctx: &mut druid::UpdateCtx, old_data: &D, data: &D, env: &druid::Env) {
        if ctx.env_key_changed(&self.icon_key.0) {
            self.reload_icon(env);
        }
        if ctx.env_key_changed(&self.icon_key.1) {
            ctx.request_paint();
        }
        if ctx.has_requested_update() || !old_data.same(data) {
            self.text.update(ctx, data, env);
            self.desc.update(ctx, data, env);
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
        let min_height = 40.;
        let min_bc = BoxConstraints::new(
            (bc.min().width, min_height).into(),
            (bc.max().width, min_height).into(),
        );
        let text_bc = min_bc.shrink((40., 0.));
        let desc_bc = min_bc.shrink((50., 0.));
        let _text_size = self.text.layout(ctx, &text_bc, data, env);
        let _desc_size = self.desc.layout(ctx, &desc_bc, data, env);
        self.text.set_origin(ctx, (40., 0.).into());
        self.desc.set_origin(ctx, (40., 0.).into());
        min_bc.constrain((0., min_height))
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &D, env: &druid::Env) {
        let icon_brush = PaintBrush::Color(env.get(if env.get(IS_DARK) {
            &self.icon_key.2
        } else {
            &self.icon_key.1
        }));
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
        let icon_size = druid::Size::new(size.height, size.height);
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
