use druid::{kurbo::BezPath, piet::PaintBrush, widget::LabelText, *};

use super::label;
use crate::theme::color::{base, list, typography::BODY};

/// 一个可以点击折叠展开的文件夹列表组件
pub struct FolderList<T> {
    opened: bool,
    hot: bool,
    inner: WidgetPod<T, Box<dyn Widget<T>>>,
    text: WidgetPod<T, Box<dyn Widget<T>>>,
}

impl<T: Data> FolderList<T> {
    /// 一个可以点击折叠展开的文件夹列表组件，提供需要折叠展开的组件和说明文本信息
    pub fn new<W: Widget<T> + 'static>(inner: W, text: impl Into<LabelText<T>>) -> FolderList<T> {
        FolderList {
            opened: false,
            hot: false,
            text: WidgetPod::new(
                label::new(text.into())
                    .with_text_size(14.)
                    .with_text_color(base::MEDIUM)
                    .with_font(BODY)
                    .padding((40., 0., 0., 0.))
                    .align_vertical(druid::UnitPoint::LEFT)
                    .boxed(),
            ),
            inner: WidgetPod::new(inner.boxed()),
        }
    }
}

impl<T: Data> Widget<T> for FolderList<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        let should_spread = match event {
            Event::MouseDown(m) => {
                if m.pos.y < 40. {
                    ctx.set_active(true);
                    ctx.request_paint();
                    false
                } else {
                    true
                }
            }
            Event::MouseUp(m) => {
                if ctx.is_active() {
                    ctx.set_active(false);
                    ctx.request_paint();
                    if m.pos.y < 40. {
                        self.opened = !self.opened;
                        ctx.request_layout();
                        false
                    } else {
                        true
                    }
                } else {
                    true
                }
            }
            Event::MouseMove(m) => {
                if self.hot != (m.pos.y < 40.) {
                    self.hot = m.pos.y < 40.;
                    ctx.request_paint();
                    true
                } else {
                    true
                }
            }
            _ => true,
        };
        self.text.event(ctx, event, data, env);
        if should_spread && (event.should_propagate_to_hidden() || self.opened) {
            self.inner.event(ctx, event, data, env);
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        if let LifeCycle::HotChanged(_) = event {
            ctx.request_paint();
        }
        self.text.lifecycle(ctx, event, data, env);
        if event.should_propagate_to_hidden() || self.opened {
            self.inner.lifecycle(ctx, event, data, env);
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &T, data: &T, env: &Env) {
        self.text.update(ctx, data, env);
        self.inner.update(ctx, data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        bc.debug_check("FolderList");
        let text_bc =
            BoxConstraints::new((bc.min().width, 40.).into(), (bc.max().width, 40.).into());
        self.text.layout(ctx, &text_bc, data, env);
        if self.opened {
            let list_bc = bc.shrink((40., 40.));
            let size = self.inner.layout(ctx, &list_bc, data, env);
            self.inner.set_origin(ctx, (40., 40.).into());
            bc.constrain(size + (40., 40.).into())
        } else {
            bc.constrain((0., 40.))
        }
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        let size = ctx.size();
        let is_hot = self.hot && ctx.is_hot();
        let is_active = ctx.is_active();
        let text_size = Size::new(size.width, 40.);

        if is_hot {
            ctx.fill(
                text_size.to_rect(),
                &PaintBrush::Color(if is_active {
                    env.get(base::LOW)
                } else {
                    env.get(list::LIST_LOW)
                }),
            )
        }

        self.text.paint(ctx, data, env);
        let mut switch_icon = BezPath::new();
        if self.opened {
            switch_icon.move_to((14.5, 22.8));
            switch_icon.line_to((20., 17.3));
            switch_icon.line_to((25.5, 22.8));
            ctx.stroke(&switch_icon, &PaintBrush::Color(env.get(base::MEDIUM)), 1.);
            self.inner.paint(ctx, data, env);
        } else {
            switch_icon.move_to((14.5, 17.3));
            switch_icon.line_to((20., 22.8));
            switch_icon.line_to((25.5, 17.3));
            ctx.stroke(&switch_icon, &PaintBrush::Color(env.get(base::MEDIUM)), 1.);
        }
    }
}
