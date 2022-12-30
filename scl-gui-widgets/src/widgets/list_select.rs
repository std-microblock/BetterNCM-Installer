//! 一个列表项目组件，修改自 [`druid`]

use druid::{
    keyboard_types::Key,
    theme,
    widget::{Controller, CrossAxisAlignment, Flex, Label, LabelText},
    BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, LinearGradient,
    PaintCtx, RenderContext, Size, UnitPoint, UpdateCtx, Widget,
};

// added padding between the edges of the widget and the text.
const LABEL_X_PADDING: f64 = 8.0;

/// Builds a simple list selection widget, for selecting a single value out of a list.
pub struct ListSelect<T> {
    /// Internal widget data.
    widget: Flex<T>,
    /// A controller handling item selection.
    controller: ListSelectController<T>,
}

impl<T: Data> ListSelect<T> {
    /// Given a vector of `(label_text, enum_variant)` tuples, create a list of items to select from
    pub fn new(
        values: impl IntoIterator<Item = (impl Into<LabelText<T>> + 'static, T)>,
    ) -> ListSelect<T> {
        let mut col = Flex::column().cross_axis_alignment(CrossAxisAlignment::Fill);
        let mut variants = Vec::new();
        for (index, (label, variant)) in values.into_iter().enumerate() {
            variants.insert(index, variant.clone());
            col.add_child(ListItem::new(label, variant));
        }

        ListSelect {
            widget: col,
            controller: ListSelectController {
                variants,
                action: None,
            },
        }
    }

    /// Provide a closure to be called when an item is selected.
    pub fn on_select(self, f: impl Fn(&mut EventCtx, &mut T, &Env) + 'static) -> ListSelect<T> {
        let widget = self.widget;
        let ListSelectController { variants, .. } = self.controller;

        ListSelect {
            widget,
            controller: ListSelectController {
                variants,
                action: Some(Box::new(f)),
            },
        }
    }
}

impl<T: Data> Widget<T> for ListSelect<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        self.controller
            .event(&mut self.widget, ctx, event, data, env)
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        self.controller
            .lifecycle(&mut self.widget, ctx, event, data, env)
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
        self.controller
            .update(&mut self.widget, ctx, old_data, data, env)
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        self.widget.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        self.widget.paint(ctx, data, env)
    }
}

type ListSelectAction<T> = Box<dyn Fn(&mut EventCtx, &mut T, &Env) + 'static>;

// A Controller to handle arrow key in the list selection widget.
struct ListSelectController<T> {
    variants: Vec<T>,
    action: Option<ListSelectAction<T>>,
}

impl<T: Data> ListSelectController<T> {
    fn change_index(&self, data: &mut T, next_else_previous: bool) {
        if let Some(mut index) = self.variants.iter().position(|variant| variant.same(data)) {
            if next_else_previous {
                index += 1
            } else {
                index = index.saturating_sub(1);
            }
            if let Some(new_data) = self.variants.get(index) {
                *data = (*new_data).clone();
            }
        }
    }
}

impl<T: Data> Controller<T, Flex<T>> for ListSelectController<T> {
    fn event(
        &mut self,
        child: &mut Flex<T>,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut T,
        env: &Env,
    ) {
        let mut selected = false;

        if let Event::MouseDown(_) = event {
            ctx.request_focus();
        }
        if let Event::MouseUp(_) = event {
            selected = ctx.is_hot() && ctx.has_focus();
        }
        if let Event::KeyDown(key_event) = event {
            match key_event.key {
                Key::ArrowUp => {
                    selected = true;
                    self.change_index(data, false);
                    ctx.request_update();
                }
                Key::ArrowDown => {
                    selected = true;
                    self.change_index(data, true);
                    ctx.request_update();
                }
                _ => {}
            }
        } else {
            child.event(ctx, event, data, env)
        }

        // fire the callback if a valid index was selected
        if selected {
            if let Some(cb) = &self.action {
                cb(ctx, data, env);
            }
        }
    }
}

/// A single list item.
pub struct ListItem<T> {
    // Ultimately this shall be able to display either a label, a label with an icon, or a single icon
    variant: T,
    child_label: Label<T>,
    label_y: f64,
}

impl<T: Data> ListItem<T> {
    /// Create a single ListItem from label text and an enum variant
    pub fn new(label: impl Into<LabelText<T>>, variant: T) -> ListItem<T> {
        ListItem {
            variant,
            child_label: Label::new(label),
            label_y: 0.0,
        }
    }
}

impl<T: Data> Widget<T> for ListItem<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, _env: &Env) {
        match event {
            Event::MouseDown(_) => {
                ctx.set_active(true);
                ctx.request_paint();
            }
            Event::MouseUp(_) => {
                if ctx.is_active() {
                    ctx.set_active(false);
                    if ctx.is_hot() {
                        *data = self.variant.clone();
                    }
                    ctx.request_paint();
                }
            }
            _ => (),
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        self.child_label.lifecycle(ctx, event, data, env);
        if let LifeCycle::HotChanged(_) = event {
            ctx.request_paint();
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
        self.child_label.update(ctx, old_data, data, env);
        if !old_data.same(data) {
            ctx.request_paint();
        }
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        let label_size = self.child_label.layout(ctx, &bc.loosen(), data, env);
        let height = (env.get(theme::BASIC_WIDGET_HEIGHT)
            + env.get(theme::WIDGET_PADDING_VERTICAL))
        .max(label_size.height);
        self.label_y = (height - label_size.height) / 2.0;
        bc.constrain(Size::new(label_size.width + LABEL_X_PADDING * 2.0, height))
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        let border_width = 1.0;
        let rect = ctx.size().to_rect().inset(-border_width / 2.0);

        // Paint the data in the primary color if we are selected
        if data.same(&self.variant) {
            let background_gradient = LinearGradient::new(
                UnitPoint::TOP,
                UnitPoint::BOTTOM,
                (env.get(theme::PRIMARY_LIGHT), env.get(theme::PRIMARY_DARK)),
            );
            ctx.fill(rect, &background_gradient);
        } else if ctx.is_active() {
            let background_gradient = LinearGradient::new(
                UnitPoint::TOP,
                UnitPoint::BOTTOM,
                (
                    env.get(theme::BACKGROUND_LIGHT),
                    env.get(theme::BACKGROUND_DARK),
                ),
            );
            ctx.fill(rect, &background_gradient);
        }

        // Paint a light rectangle around the item if hot
        if ctx.is_hot() {
            ctx.stroke(rect, &env.get(theme::BORDER_LIGHT), 1.);
        }

        // Paint the text label
        self.child_label
            .draw_at(ctx, (LABEL_X_PADDING, self.label_y));
    }
}
