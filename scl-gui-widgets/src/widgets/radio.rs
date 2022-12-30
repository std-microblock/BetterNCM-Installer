//! 一个单选框组件，修改自 [`druid::widget::Radio`] [`druid::widget::RadioGroup`]

use druid::{
    debug_state::DebugState,
    kurbo::Circle,
    theme,
    widget::{prelude::*, Axis, CrossAxisAlignment, Flex, Label, LabelText},
    Data,
};
use tracing::{instrument, trace};

const DEFAULT_RADIO_RADIUS: f64 = (20.0 - 1.) / 2.;
const INNER_CIRCLE_RADIUS: f64 = 6.0;
/// A group of radio buttons
#[derive(Debug, Clone)]
pub struct RadioGroup;

impl RadioGroup {
    /// Given a vector of `(label_text, enum_variant)` tuples, create a group of Radio buttons
    /// along the vertical axis.
    pub fn column<T: Data + PartialEq>(
        variants: impl IntoIterator<Item = (impl Into<LabelText<T>> + 'static, T)>,
    ) -> impl Widget<T> {
        RadioGroup::for_axis(Axis::Vertical, variants)
    }

    /// Given a vector of `(label_text, enum_variant)` tuples, create a group of Radio buttons
    /// along the horizontal axis.
    pub fn row<T: Data + PartialEq>(
        variants: impl IntoIterator<Item = (impl Into<LabelText<T>> + 'static, T)>,
    ) -> impl Widget<T> {
        RadioGroup::for_axis(Axis::Horizontal, variants)
    }

    /// Given a vector of `(label_text, enum_variant)` tuples, create a group of Radio buttons
    /// along the specified axis.
    pub fn for_axis<T: Data + PartialEq>(
        axis: Axis,
        variants: impl IntoIterator<Item = (impl Into<LabelText<T>> + 'static, T)>,
    ) -> impl Widget<T> {
        let mut col = Flex::for_axis(axis).cross_axis_alignment(CrossAxisAlignment::Start);
        for (label, variant) in variants.into_iter() {
            let radio = Radio::new(label, variant);
            col.add_child(radio);
        }
        col
    }
}

/// A single radio button
pub struct Radio<T> {
    variant: T,
    inner_circle_target_radius: f64,
    inner_circle_current_radius: f64,
    label_height: f64,
    child_label: Label<T>,
}

impl<T: Data> Radio<T> {
    /// Create a lone Radio button from label text and an enum variant
    pub fn new(label: impl Into<LabelText<T>>, variant: T) -> Radio<T> {
        Radio {
            variant,
            label_height: 0.,
            inner_circle_target_radius: 0.,
            inner_circle_current_radius: 0.,
            child_label: crate::widgets::label::new(label),
        }
    }
}

impl<T: Data + PartialEq> Widget<T> for Radio<T> {
    #[instrument(name = "Radio", level = "trace", skip(self, ctx, event, data, _env))]
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, _env: &Env) {
        match event {
            Event::MouseDown(_) => {
                if !ctx.is_disabled() {
                    if data == &self.variant {
                        self.inner_circle_target_radius = INNER_CIRCLE_RADIUS - 1.;
                    } else {
                        self.inner_circle_target_radius = INNER_CIRCLE_RADIUS;
                    }
                    ctx.set_active(true);
                    ctx.request_anim_frame();
                    ctx.request_paint();
                    trace!("Radio button {:?} pressed", ctx.widget_id());
                }
            }
            Event::MouseUp(_) => {
                if ctx.is_active() && !ctx.is_disabled() && ctx.is_hot() {
                    if ctx.is_active() {
                        *data = self.variant.clone();
                    }
                    if ctx.is_hot() {
                        self.inner_circle_target_radius = INNER_CIRCLE_RADIUS;
                    }
                    ctx.request_anim_frame();
                    ctx.request_paint();
                    trace!("Radio button {:?} released", ctx.widget_id());
                }
                ctx.set_active(false);
            }
            Event::AnimFrame(_) => {
                self.inner_circle_current_radius +=
                    (self.inner_circle_target_radius - self.inner_circle_current_radius) / 3.;
                if (self.inner_circle_target_radius - self.inner_circle_current_radius).abs()
                    > f64::EPSILON
                {
                    ctx.request_anim_frame();
                }
                ctx.request_paint();
            }
            _ => (),
        }
    }

    #[instrument(name = "Radio", level = "trace", skip(self, ctx, event, data, env))]
    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        self.child_label.lifecycle(ctx, event, data, env);
        if let LifeCycle::HotChanged(_) | LifeCycle::DisabledChanged(_) = event {
            let target_radius = if data == &self.variant {
                if ctx.is_active() {
                    INNER_CIRCLE_RADIUS + 0.5
                } else {
                    INNER_CIRCLE_RADIUS
                }
            } else if ctx.is_active() {
                INNER_CIRCLE_RADIUS
            } else {
                0.
            };
            if (self.inner_circle_target_radius - target_radius).abs() > f64::EPSILON {
                self.inner_circle_target_radius = target_radius;
                ctx.request_anim_frame();
            }
            ctx.request_paint();
        } else if let LifeCycle::WidgetAdded = event {
            self.inner_circle_target_radius = if data == &self.variant {
                INNER_CIRCLE_RADIUS
            } else {
                0.
            };
            self.inner_circle_current_radius = self.inner_circle_target_radius;
        }
    }

    #[instrument(name = "Radio", level = "trace", skip(self, ctx, old_data, data, env))]
    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
        self.child_label.update(ctx, old_data, data, env);
        if !old_data.same(data) {
            let target_radius = if data == &self.variant {
                if ctx.is_active() {
                    INNER_CIRCLE_RADIUS + 0.5
                } else {
                    INNER_CIRCLE_RADIUS
                }
            } else if ctx.is_active() {
                INNER_CIRCLE_RADIUS
            } else {
                0.
            };
            if (self.inner_circle_target_radius - target_radius).abs() > f64::EPSILON {
                self.inner_circle_target_radius = target_radius;
                ctx.request_anim_frame();
            }
            ctx.request_paint();
        }
    }

    #[instrument(name = "Radio", level = "trace", skip(self, ctx, bc, data, env))]
    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        bc.debug_check("Radio");
        let radio_diam = env.get(theme::BASIC_WIDGET_HEIGHT);
        let x_padding = env.get(theme::WIDGET_CONTROL_COMPONENT_PADDING);

        let label_size = self.child_label.layout(
            ctx,
            &bc.shrink((
                ((DEFAULT_RADIO_RADIUS + x_padding) * 2. + x_padding).max(32.),
                0.,
            )),
            data,
            env,
        );

        self.label_height = label_size.height;

        let desired_size = Size::new(
            label_size.width + radio_diam + x_padding,
            radio_diam.max(label_size.height).max(32.),
        );
        let size = bc.constrain(desired_size);
        trace!("Computed size: {}", size);
        size
    }

    #[instrument(name = "Radio", level = "trace", skip(self, ctx, data, env))]
    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        let size = ctx.size();
        let is_dark = env.get(crate::theme::color::main::IS_DARK);
        let x_padding = env.get(theme::WIDGET_CONTROL_COMPONENT_PADDING);

        let circle_x = (size.height / 2.).min(DEFAULT_RADIO_RADIUS) + x_padding;
        let circle_y = size.height / 2.;

        let circle = Circle::new((circle_x, circle_y), DEFAULT_RADIO_RADIUS);

        // Paint the background

        let is_matched = data == &self.variant;

        let background_color = if is_matched {
            env.get(crate::theme::color::accent::ACCENT)
        } else if ctx.is_active() {
            env.get(crate::theme::color::base::MEDIUM_LOW)
        } else if ctx.is_hot() {
            env.get(crate::theme::color::base::LOW)
        } else {
            env.get(crate::theme::color::alt::LOW)
        };

        ctx.fill(circle, &background_color);

        let border_color = if is_dark {
            if is_matched {
                env.get(crate::theme::color::accent::ACCENT)
            } else if ctx.is_active() {
                env.get(crate::theme::color::base::MEDIUM)
            } else {
                env.get(crate::theme::color::base::LOW)
            }
        } else if is_matched {
            env.get(crate::theme::color::accent::ACCENT)
        } else if ctx.is_active() {
            env.get(crate::theme::color::base::MEDIUM_HIGH)
        } else {
            env.get(crate::theme::color::base::MEDIUM_LOW)
        };

        ctx.stroke(circle, &border_color, 1.);

        let inner_circle = Circle::new((circle_x, circle_y), self.inner_circle_current_radius);

        ctx.fill(inner_circle, &env.get(crate::theme::color::alt::HIGH));

        // Paint the text label
        self.child_label.draw_at(
            ctx,
            (
                (circle_x + INNER_CIRCLE_RADIUS + x_padding * 2.).max(32.),
                (size.height - self.label_height) / 2.,
            ),
        );
    }

    fn debug_state(&self, data: &T) -> DebugState {
        let value_text = if *data == self.variant {
            format!("[X] {}", self.child_label.text())
        } else {
            self.child_label.text().to_string()
        };
        DebugState {
            display_name: self.short_type_name().to_string(),
            main_value: value_text,
            ..Default::default()
        }
    }
}
