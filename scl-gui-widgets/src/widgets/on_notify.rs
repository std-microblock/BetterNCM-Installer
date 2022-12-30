use druid::{widget::prelude::*, Selector, WidgetPod};

type OnNotifyHandler<CT, WT> = Box<dyn Fn(&mut EventCtx, &CT, &mut WT)>;

/// 一个组件，当子组件向上抛出指定的 [`druid::Notification`] 时触发回调
///
/// 推荐使用 [`crate::widget_ext::WidgetExt::on_notify`] 来创建本组件于一个组件上
pub struct OnNotify<CT, WT> {
    selector: Selector<CT>,
    handler: OnNotifyHandler<CT, WT>,
    inner: WidgetPod<WT, Box<dyn Widget<WT>>>,
}

impl<CT, WT> OnNotify<CT, WT> {
    /// 一个组件，当子组件向上抛出指定的 [`druid::Notification`] 时触发回调
    ///
    /// 推荐使用 [`crate::widget_ext::WidgetExt::on_notify`] 来创建本组件于一个组件上
    pub fn new(
        selector: Selector<CT>,
        handler: impl Fn(&mut EventCtx, &CT, &mut WT) + 'static,
        inner: impl Widget<WT> + 'static,
    ) -> Self {
        Self {
            selector,
            handler: Box::new(handler),
            inner: WidgetPod::new(Box::new(inner)),
        }
    }
}

impl<WT: Data, CT: 'static> Widget<WT> for OnNotify<CT, WT> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut WT, env: &Env) {
        if let Event::Notification(c) = event {
            if let Some(ct) = c.get(self.selector) {
                (self.handler)(ctx, ct, data);
            }
        }
        self.inner.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &WT, env: &Env) {
        self.inner.lifecycle(ctx, event, data, env)
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &WT, data: &WT, env: &Env) {
        self.inner.update(ctx, data, env)
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &WT, env: &Env) -> Size {
        self.inner.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &WT, env: &Env) {
        self.inner.paint(ctx, data, env)
    }
}
