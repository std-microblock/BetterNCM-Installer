use druid::{
    kurbo::{BezPath, Shape},
    piet::{PaintBrush, TextStorage},
    Affine, ArcStr, Color, Data, Env, Key, LifeCycle, RenderContext, Widget,
};

use crate::theme::icons::IconKeyPair;

/// 一个图标组件，可以静态显示一个图标，也可以根据数据更新回调返回的图标键组 [`crate::theme::icons::IconKeyPair`] 显示矢量路径填充的图标
pub struct Icon<T> {
    callback: Option<fn(&T, &Env) -> IconKeyPair>,
    icon_key: Key<ArcStr>,
    icon_color_key: (Key<Color>, Key<Color>),
    icon_path: BezPath,
    brush: PaintBrush,
}

impl<T> Icon<T> {
    /// 根据图标键组 [`crate::theme::icons::IconKeyPair`] 创建一个静态的图标组件
    pub fn new(icon_key_pair: IconKeyPair) -> Self {
        Self {
            icon_key: icon_key_pair.0,
            icon_color_key: (icon_key_pair.1, icon_key_pair.2),
            icon_path: BezPath::new(),
            brush: PaintBrush::Color(Color::BLACK),
            callback: None,
        }
    }

    /// 根据数据更新回调返回的图标键组 [`crate::theme::icons::IconKeyPair`] 创建一个动态的图标组件
    pub fn dynamic(callback: fn(&T, &Env) -> IconKeyPair) -> Self {
        Self {
            icon_key: crate::theme::icons::EMPTY.0,
            icon_color_key: (crate::theme::icons::EMPTY.1, crate::theme::icons::EMPTY.2),
            icon_path: BezPath::new(),
            brush: PaintBrush::Color(Color::BLACK),
            callback: Some(callback),
        }
    }

    /// Use accent color as button color
    pub fn set_brush(&mut self, value: PaintBrush) {
        self.brush = value;
    }

    /// Builder style to use accent color as button color
    pub fn with_brush(mut self, value: PaintBrush) -> Self {
        self.set_brush(value);
        self
    }

    fn reload_icon(&mut self, data: &T, env: &Env) {
        if let Some(callback) = self.callback {
            let key_pair = callback(data, env);
            self.icon_key = key_pair.0;
            self.icon_color_key = (key_pair.1, key_pair.2)
        }
        self.icon_path = BezPath::from_svg(env.get(&self.icon_key).as_str()).unwrap_or_default();
        if env.get(crate::theme::color::main::IS_DARK) {
            self.set_brush(PaintBrush::Color(env.get(&self.icon_color_key.0)));
        } else {
            self.set_brush(PaintBrush::Color(env.get(&self.icon_color_key.1)));
        }
    }
}

impl<T: Data> Widget<T> for Icon<T> {
    fn event(
        &mut self,
        _ctx: &mut druid::EventCtx,
        _event: &druid::Event,
        _data: &mut T,
        _env: &druid::Env,
    ) {
    }

    fn lifecycle(
        &mut self,
        _ctx: &mut druid::LifeCycleCtx,
        event: &druid::LifeCycle,
        data: &T,
        env: &druid::Env,
    ) {
        if let LifeCycle::WidgetAdded = event {
            self.reload_icon(data, env);
        }
    }

    fn update(&mut self, ctx: &mut druid::UpdateCtx, _old_data: &T, data: &T, env: &druid::Env) {
        if ctx.env_key_changed(&self.icon_key) {
            self.reload_icon(data, env);
        }
    }

    fn layout(
        &mut self,
        _ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        _data: &T,
        _env: &druid::Env,
    ) -> druid::Size {
        bc.debug_check("Icon");
        bc.constrain((32., 32.))
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, _data: &T, _env: &druid::Env) {
        let size = ctx.size();
        ctx.with_save(|ctx| {
            ctx.transform(Affine::translate(
                ((size - self.icon_path.bounding_box().size()) / 2.)
                    .to_vec2()
                    .round(),
            ));
            ctx.fill_even_odd(&self.icon_path, &self.brush)
        })
    }
}
