use druid::widget::{prelude::*, Controller};

type OnChangeCallback<T> = Box<dyn Fn(&mut EventCtx, &T, &mut T, &Env)>;

/// 一个控制器，当数据流发生更新时触发回调
///
/// 推荐使用 [`crate::widget_ext::WidgetExt::on_change`] 来创建本控制器于一个组件上
pub struct OnChange<T>(OnChangeCallback<T>);

impl<T> OnChange<T> {
    /// 根据所给回调创建此控制器
    ///
    /// 推荐使用 [`crate::widget_ext::WidgetExt::on_change`] 来创建本控制器于一个组件上
    pub fn new(f: impl Fn(&mut EventCtx, &T, &mut T, &Env) + 'static) -> Self {
        Self(Box::new(f))
    }
}

impl<T: Data, W: Widget<T>> Controller<T, W> for OnChange<T> {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        let old_data = data.clone();
        child.event(ctx, event, data, env);
        if !old_data.same(data) && !ctx.is_disabled() {
            (self.0)(ctx, &old_data, data, env);
        }
    }
}
