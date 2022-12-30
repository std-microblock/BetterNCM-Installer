use druid::{
    widget::{prelude::*, Controller},
    Selector,
};

type OnCmdHandler<CT, WT> = Box<dyn Fn(&mut EventCtx, &CT, &mut WT)>;

/// 一个控制器，当接收到特定的 [`druid::Command`] 时触发回调
///
/// 推荐使用 [`crate::widget_ext::WidgetExt::on_command`] 来创建本控制器于一个组件上
pub struct OnCmd<CT, WT> {
    selector: Selector<CT>,
    handler: OnCmdHandler<CT, WT>,
}

impl<CT, WT> OnCmd<CT, WT> {
    /// 根据所给指令 [`druid::Command`] 和回调创建此控制器
    ///
    /// 推荐使用 [`crate::widget_ext::WidgetExt::on_command`] 来创建本控制器于一个组件上
    pub fn new(
        selector: Selector<CT>,
        handler: impl Fn(&mut EventCtx, &CT, &mut WT) + 'static,
    ) -> Self {
        Self {
            selector,
            handler: Box::new(handler),
        }
    }
}

impl<WT: Data, W: Widget<WT>, CT: 'static> Controller<WT, W> for OnCmd<CT, WT> {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut WT,
        env: &Env,
    ) {
        match event {
            Event::Command(c) if c.is(self.selector) => {
                (self.handler)(ctx, c.get_unchecked(self.selector), data);
            }
            _ => {}
        }
        child.event(ctx, event, data, env);
    }
}
