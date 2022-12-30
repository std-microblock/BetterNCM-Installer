//! 一个列表项目按钮组件，修改自 [`druid`]

use druid::{
    piet::PaintBrush,
    widget::{prelude::*, Click, ControllerHost, Label, LabelText},
    Affine, Data, Insets,
};

use super::{label, Icon};
use crate::theme::color as theme;

// the minimum padding added to a button.
// NOTE: these values are chosen to match the existing look of TextBox; these
// should be reevaluated at some point.
const LABEL_INSETS: Insets = Insets::uniform_xy(8., 2.);

/// A button with a text label.
pub struct ListItem<T> {
    label: Label<T>,
    label_size: Size,
    icon: Option<Icon<T>>,
    accent: bool,
    disabled: bool,
}

impl<T: Data> ListItem<T> {
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
    pub fn new(text: impl Into<LabelText<T>>) -> ListItem<T> {
        ListItem::from_label(
            label::new(text)
                .with_text_alignment(druid::TextAlignment::Start)
                .with_font(theme::typography::BODY),
        )
    }

    /// Create a new button with the provided [`Label`].
    ///
    /// Use the [`.on_click`] method to provide a closure to be called when the
    /// button is clicked.
    ///
    /// # Examples
    ///
    /// ```
    /// use druid::Color;
    /// use druid::widget::{Button, Label};
    ///
    /// let button = Button::from_label(Label::new("Increment").with_text_color(Color::grey(0.5))).on_click(|_ctx, data: &mut u32, _env| {
    ///     *data += 1;
    /// });
    /// ```
    ///
    /// [`Label`]: struct.Label.html
    /// [`.on_click`]: #method.on_click
    pub fn from_label(label: Label<T>) -> ListItem<T> {
        ListItem {
            label,
            label_size: Size::ZERO,
            icon: None,
            accent: false,
            disabled: false,
        }
    }

    /// Construct a new dynamic button.
    ///
    /// The contents of this button are generated from the data using a closure.
    ///
    /// This is provided as a convenience; a closure can also be passed to [`new`],
    /// but due to limitations of the implementation of that method, the types in
    /// the closure need to be annotated, which is not true for this method.
    ///
    /// # Examples
    ///
    /// The following are equivalent.
    ///
    /// ```
    /// use druid::Env;
    /// use druid::widget::Button;
    /// let button1: Button<u32> = Button::new(|data: &u32, _: &Env| format!("total is {}", data));
    /// let button2: Button<u32> = Button::dynamic(|data, _| format!("total is {}", data));
    /// ```
    ///
    /// [`new`]: #method.new
    pub fn dynamic(text: impl Fn(&T, &Env) -> String + 'static) -> Self {
        let text: LabelText<T> = text.into();
        ListItem::new(text)
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
        self.label.set_text_color(if self.accent {
            theme::chrome::WHITE_HIGH
        } else {
            theme::typography::FONT_COLOR
        })
    }

    /// 设置一个图标
    pub fn set_icon(&mut self, icon: Option<Icon<T>>) {
        self.icon = icon;
    }

    /// 以 Builder 方式设置一个图标
    pub fn with_icon(mut self, icon: Icon<T>) -> Self {
        self.set_icon(Some(icon));
        self
    }

    /// Builder style to use accent color as button color
    pub fn with_accent(mut self, value: bool) -> Self {
        self.set_accent(value);
        self
    }

    /// 以 Builder 方式设置组件禁用状态
    pub fn set_disabled(&mut self, value: bool) {
        self.disabled = value;
    }

    /// 以 Builder 方式设置组件禁用状态
    pub fn with_disabled(mut self, value: bool) -> Self {
        self.set_disabled(value);
        self
    }
}

impl<T: Data> Widget<T> for ListItem<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, _data: &mut T, _env: &Env) {
        match event {
            Event::MouseDown(_) => {
                if !ctx.is_disabled() {
                    ctx.set_active(true);
                    ctx.request_paint();
                }
            }
            Event::MouseUp(_) => {
                if ctx.is_active() && !ctx.is_disabled() {
                    ctx.request_paint();
                }
                ctx.set_active(false);
            }
            _ => (),
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        if let LifeCycle::HotChanged(_) | LifeCycle::DisabledChanged(_) = event {
            ctx.request_paint();
        }
        self.label.lifecycle(ctx, event, data, env)
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
        self.label.update(ctx, old_data, data, env)
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        bc.debug_check("ListItem");
        // HACK: to make sure we look okay at default sizes when beside a textbox,
        // we make sure we will have at least the same height as the default textbox.
        let min_height = 32.;
        let padding = Size::new(LABEL_INSETS.x_value(), LABEL_INSETS.y_value());
        let label_bc = BoxConstraints::new(
            Size::new(bc.min().width, bc.min().height.max(min_height)),
            Size::new(bc.max().width, bc.max().height.max(min_height)),
        );
        let label_bc = label_bc.shrink(padding).loosen();
        self.label_size = self.label.layout(ctx, &label_bc, data, env);
        let baseline = self.label.baseline_offset();
        ctx.set_baseline_offset(baseline + LABEL_INSETS.y1);

        bc.constrain(Size::new(
            self.label_size.width + padding.width,
            (self.label_size.height + padding.height).max(min_height),
        ))
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        let size = ctx.size();
        let is_hot = ctx.is_hot();
        let is_active = ctx.is_active();

        if is_hot && !self.disabled {
            ctx.fill(
                size.to_rect(),
                &PaintBrush::Color(if is_active {
                    env.get(theme::base::LOW)
                } else {
                    env.get(theme::list::LIST_LOW)
                }),
            )
        }

        let mut label_offset = (size.to_vec2() - self.label_size.to_vec2()) / 2.0;
        label_offset.x = 0.;

        ctx.with_save(|ctx| {
            ctx.transform(Affine::translate(label_offset));
            self.label.paint(ctx, data, env);
        });
    }
}
