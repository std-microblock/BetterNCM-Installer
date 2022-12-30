//! 一个中间件组件，用于拦截数据流并进行处理

use druid::{Data, Widget, WidgetPod};

/// 一个中间件组件，用于拦截数据流并进行处理
///
/// 和 [`druid::Lens`] 不一样的地方是，它不强制你去设置数据，这允许你在无法转换数据的情况下保持子代数据不变或父代数据不变。
pub struct Store<F, T> {
    inner: WidgetPod<T, Box<dyn Widget<T>>>,
    pre_data_processor: fn(&F, &mut T),
    post_data_processor: fn(&T, &mut F),
    post_data: T,
}

impl<F: Data, T: Data> Store<F, T> {
    /// 创建一个中间件组件，需要提供将被传递数据的组件和相互转换的回调函数，并提供一个默认的转换后的值
    pub fn new(
        inner: impl Widget<T> + 'static,
        pre_data_processor: fn(&F, &mut T),
        post_data_processor: fn(&T, &mut F),
        initial_post_data: T,
    ) -> Self {
        Self {
            inner: WidgetPod::new(inner).boxed(),
            pre_data_processor,
            post_data_processor,
            post_data: initial_post_data,
        }
    }
}

impl<F: Data, T: Data + Default> Store<F, T> {
    /// 创建一个中间件组件，需要提供将被传递数据的组件和相互转换的回调函数，默认值会使用其转换后类型的 [`Default::default`] 默认值
    #[inline]
    pub fn new_default(
        inner: impl Widget<T> + 'static,
        pre_data_processor: fn(&F, &mut T),
        post_data_processor: fn(&T, &mut F),
    ) -> Self {
        Self::new(inner, pre_data_processor, post_data_processor, T::default())
    }
}

impl<F: Data, T: Data> Widget<F> for Store<F, T> {
    fn event(
        &mut self,
        ctx: &mut druid::EventCtx,
        event: &druid::Event,
        data: &mut F,
        env: &druid::Env,
    ) {
        println!("Event Called");
        let mut post_data = self.post_data.to_owned();
        self.inner.event(ctx, event, &mut post_data, env);
        if !post_data.same(&self.post_data) {
            self.post_data = post_data;
            (self.post_data_processor)(&self.post_data, data);
            println!("Post Data Updated");
            ctx.request_update();
        }
    }

    fn lifecycle(
        &mut self,
        ctx: &mut druid::LifeCycleCtx,
        event: &druid::LifeCycle,
        _data: &F,
        env: &druid::Env,
    ) {
        println!("Lifecycle Called");
        self.inner.lifecycle(ctx, event, &self.post_data, env)
    }

    fn update(&mut self, ctx: &mut druid::UpdateCtx, _old_data: &F, data: &F, env: &druid::Env) {
        let mut post_data = self.post_data.to_owned();
        (self.pre_data_processor)(data, &mut post_data);
        if !post_data.same(&self.post_data) {
            println!("Update Called");
            self.inner.update(ctx, &self.post_data, env);
        }
    }

    fn layout(
        &mut self,
        ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        _data: &F,
        env: &druid::Env,
    ) -> druid::Size {
        println!("Layout Called");
        self.inner.layout(ctx, bc, &self.post_data, env)
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, _data: &F, env: &druid::Env) {
        println!("Paint Called");
        self.inner.paint(ctx, &self.post_data, env)
    }
}
