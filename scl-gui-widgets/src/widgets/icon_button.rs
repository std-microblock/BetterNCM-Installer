//! 一个图标按钮，修改自 [`druid::widget::Button`]

use druid::{
    kurbo::{BezPath, Shape},
    piet::{PaintBrush, TextStorage},
    widget::{prelude::*, Click, ControllerHost},
    Affine, Data,
};

use crate::theme::{color as theme, icons::IconKeyPair};

type DynamicIconCallback<T> = Option<Box<dyn Fn(&T, &Env) -> IconKeyPair>>;

/// A button with a text label.
pub struct IconButton<T> {
    icon_key: IconKeyPair,
    icon_dyn: DynamicIconCallback<T>,
    icon_path: BezPath,
    flat: bool,
    accent: bool,
}

impl<T: Data> IconButton<T> {
    /// Create a new button with a text label.
    ///
    /// Use the [`.on_click`] method to provide a closure to be called when the
    /// button is clicked.
    ///
    /// # Examples
    ///
    /// ```
    /// use druid::widget::Button;
    ///
    /// let button = Button::new("Increment").on_click(|_ctx, data: &mut u32, _env| {
    ///     *data += 1;
    /// });
    /// ```
    ///
    /// [`.on_click`]: #method.on_click
    pub fn dynamic(icon_key: impl Fn(&T, &Env) -> IconKeyPair + 'static) -> IconButton<T> {
        IconButton {
            icon_key: crate::theme::icons::EMPTY,
            icon_dyn: Some(Box::new(icon_key)),
            icon_path: BezPath::new(),
            accent: false,
            flat: false,
        }
    }

    /// 根据提供的图标键组对创建一个图标按钮
    pub fn new(icon_key: IconKeyPair) -> IconButton<T> {
        IconButton {
            icon_key,
            icon_dyn: None,
            icon_path: BezPath::new(),
            accent: false,
            flat: false,
        }
    }

    /// Provide a closure to be called when this button is clicked.
    pub fn on_click(
        self,
        f: impl Fn(&mut EventCtx, &mut T, &Env) + 'static,
    ) -> ControllerHost<Self, Click<T>> {
        ControllerHost::new(self, Click::new(f))
    }

    /// Use accent color as button color
    pub fn set_accent(&mut self, value: bool) {
        self.accent = value;
    }

    /// Builder style to use accent color as button color
    pub fn with_accent(mut self, value: bool) -> Self {
        self.set_accent(value);
        self
    }

    /// Use no color as button color
    pub fn set_flat(&mut self, value: bool) {
        self.flat = value;
    }

    /// Builder style to use no color as button color
    pub fn with_flat(mut self, value: bool) -> Self {
        self.set_flat(value);
        self
    }

    fn reload_icon(&mut self, env: &Env) {
        self.icon_path = BezPath::from_svg(env.get(&self.icon_key.0).as_str()).unwrap_or_default();
    }
}

impl<T: Data> Widget<T> for IconButton<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, _data: &mut T, _env: &Env) {
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
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        if let LifeCycle::HotChanged(_) = event {
            ctx.request_paint();
        } else if let LifeCycle::WidgetAdded = event {
            if let Some(icon_callback) = &self.icon_dyn {
                self.icon_key = icon_callback(data, env);
            }
            self.reload_icon(env);
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
        if let Some(icon_callback) = &self.icon_dyn {
            if !data.same(old_data) || ctx.env_changed() {
                self.icon_key = icon_callback(data, env);
                self.reload_icon(env);
            }
        } else if ctx.env_key_changed(&self.icon_key.0) {
            self.reload_icon(env);
        }
    }

    fn layout(&mut self, _: &mut LayoutCtx, bc: &BoxConstraints, _: &T, _: &Env) -> Size {
        bc.debug_check("IconButton");
        bc.constrain((32., 32.))
    }

    fn paint(&mut self, ctx: &mut PaintCtx, _: &T, env: &Env) {
        let is_hot = ctx.is_hot();
        let is_active = ctx.is_active();
        let is_disabled = ctx.is_disabled();
        let size = ctx.size();

        super::common::print_common_button(
            (self.accent, self.flat, is_active, is_hot, is_disabled),
            ctx,
            size,
            env,
        );

        ctx.with_save(move |ctx| {
            ctx.transform(Affine::translate(
                ((size - self.icon_path.bounding_box().size()) / 2.).to_vec2(),
            ));
            ctx.fill_even_odd(
                &self.icon_path,
                &if self.accent {
                    PaintBrush::Color(env.get(theme::chrome::WHITE))
                } else {
                    PaintBrush::Color(env.get(theme::base::HIGH))
                },
            )
        });
    }
}
