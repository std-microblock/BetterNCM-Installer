//! 一个开始游戏按钮，修改自 [`druid::widget::Button`]

use druid::{
    widget::{prelude::*, Click, ControllerHost, Label, LabelText},
    Affine, Data, Insets, LocalizedString, Vec2,
};

use super::label;
use crate::theme::color as theme;

// the minimum padding added to a button.
// NOTE: these values are chosen to match the existing look of TextBox; these
// should be reevaluated at some point.
const LABEL_INSETS: Insets = Insets::uniform_xy(8., 2.);

/// A button with a text label.
pub struct StartGameButton<T> {
    label: Label<T>,
    sub_label: Label<T>,
    label_size: Size,
    sub_label_size: Size,
    accent: bool,
}

impl<T: Data> StartGameButton<T> {
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
    pub fn new(text: impl Into<LabelText<T>>) -> StartGameButton<T> {
        StartGameButton::from_label(
            label::new(text)
                .with_font(theme::typography::CAPTION)
                .with_line_break_mode(druid::widget::LineBreaking::Clip),
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
    pub fn from_label(sub_label: Label<T>) -> StartGameButton<T> {
        StartGameButton {
            label: label::new(
                LocalizedString::new("net.stevexmh.scl.start-game").with_placeholder("启动游戏"),
            )
            .with_font(theme::typography::BODY),
            sub_label: sub_label.with_line_break_mode(druid::widget::LineBreaking::Clip),
            label_size: Size::ZERO,
            sub_label_size: Size::ZERO,
            accent: false,
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
        StartGameButton::new(text)
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
        });
        self.sub_label.set_text_color(if self.accent {
            theme::border::EDGE_HIGHTLIGHT
        } else {
            theme::typography::FONT_COLOR
        });
    }

    /// Builder style to use accent color as button color
    pub fn with_accent(mut self, value: bool) -> Self {
        self.set_accent(value);
        self
    }
}

impl<T: Data> Widget<T> for StartGameButton<T> {
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
        }
        self.label.lifecycle(ctx, event, data, env);
        self.sub_label.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
        self.label.update(ctx, old_data, data, env);
        self.sub_label.update(ctx, old_data, data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        bc.debug_check("Button");
        let padding = Size::new(LABEL_INSETS.x_value(), LABEL_INSETS.y_value());
        let label_bc = bc.shrink(padding);
        let label_bc = label_bc.shrink((0., label_bc.max().height / 2.)).loosen();
        self.label_size = self.label.layout(ctx, &label_bc, data, env);
        self.sub_label_size = self.sub_label.layout(ctx, &label_bc, data, env);
        // HACK: to make sure we look okay at default sizes when beside a textbox,
        // we make sure we will have at least the same height as the default textbox.
        let min_height = 32.;
        let baseline = self.label.baseline_offset();
        ctx.set_baseline_offset(baseline + LABEL_INSETS.y1 + env.get(theme::typography::BODY).size);

        bc.constrain(Size::new(
            self.label_size.width + padding.width,
            (self.label_size.height + padding.height).max(min_height),
        ))
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        let is_hot = ctx.is_hot();
        let is_active = ctx.is_active();
        let is_disabled = ctx.is_disabled();
        let size = ctx.size();

        super::common::print_common_button(
            (self.accent, false, is_active, is_hot, is_disabled),
            ctx,
            size,
            env,
        );

        let label_offset: Vec2 = (size.to_vec2() - self.label_size.to_vec2()) / 2.0;
        let label_offset = label_offset - Vec2::new(0., self.label_size.height / 2.);

        ctx.with_save(|ctx| {
            ctx.transform(Affine::translate(label_offset));
            self.label.paint(ctx, data, env);
        });

        let label_offset: Vec2 = (size.to_vec2() - self.sub_label_size.to_vec2()) / 2.0;
        let label_offset = label_offset + Vec2::new(0., self.sub_label_size.height / 2.);

        ctx.with_save(|ctx| {
            ctx.clip(size.to_rect());
            ctx.transform(Affine::translate(label_offset));
            self.sub_label.paint(ctx, data, env);
        });
    }
}
