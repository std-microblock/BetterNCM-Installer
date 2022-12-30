use druid::{Data, Env, EventCtx, UpdateCtx, Widget, WidgetPod};

/// 一个针对 [`ShowIf`] 触发的回调，根据其返回值决定是否显示组件
pub type ShowIfCallback<T> = fn(&T, &Env) -> bool;

/// 一个组件，当回调返回 `true` 时会显示其包裹的组件，反之则隐藏且不占用布局控件
///
/// 推荐使用 [`crate::widget_ext::WidgetExt::show_if`] 来创建本组件于一个组件上
pub struct ShowIf<T> {
    inner: WidgetPod<T, Box<dyn Widget<T>>>,
    showed: bool,
    callback: ShowIfCallback<T>,
}

impl<T> ShowIf<T> {
    /// 一个组件，当回调返回 `true` 时会显示其包裹的组件，反之则隐藏且不占用布局控件
    ///
    /// 推荐使用 [`crate::widget_ext::WidgetExt::show_if`] 来创建本组件于一个组件上
    pub fn new(inner: impl Widget<T> + 'static, callback: ShowIfCallback<T>) -> Self {
        Self {
            inner: WidgetPod::new(Box::new(inner)),
            showed: false,
            callback,
        }
    }
}

impl<T: Data> Widget<T> for ShowIf<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &druid::Event, data: &mut T, env: &Env) {
        let should_show = (self.callback)(data, env);
        if self.showed != should_show {
            ctx.children_changed();
            self.showed = should_show;
        } else if should_show {
            self.inner.event(ctx, event, data, env);
        }
    }

    fn lifecycle(
        &mut self,
        ctx: &mut druid::LifeCycleCtx,
        event: &druid::LifeCycle,
        data: &T,
        env: &Env,
    ) {
        let should_show = (self.callback)(data, env);
        if let druid::LifeCycle::WidgetAdded = event {
            self.inner.lifecycle(ctx, event, data, env);
        } else if self.showed != should_show {
            ctx.children_changed();
            self.showed = should_show;
        } else if should_show {
            self.inner.lifecycle(ctx, event, data, env);
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &T, data: &T, env: &Env) {
        let should_show = (self.callback)(data, env);
        if self.showed != should_show {
            ctx.children_changed();
            self.showed = should_show;
            if should_show {
                self.inner.update(ctx, data, env);
            }
        } else if self.showed {
            self.inner.update(ctx, data, env);
        }
    }

    fn layout(
        &mut self,
        ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        data: &T,
        env: &Env,
    ) -> druid::Size {
        let should_show = (self.callback)(data, env);
        if should_show {
            self.inner.layout(ctx, bc, data, env)
        } else {
            bc.constrain((0., 0.))
        }
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &T, env: &Env) {
        let should_show = (self.callback)(data, env);
        if should_show {
            self.inner.paint(ctx, data, env)
        }
    }
}
