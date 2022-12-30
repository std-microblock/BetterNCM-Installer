//! 一个组件的模板代码
//!
//! 你可以复制黏贴这个文件来快速制作一个自定义组件

use druid::{
    BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, Size,
    UpdateCtx, Widget,
};

pub struct TemplateWidget;

impl TemplateWidget {
    #[allow(dead_code)]
    pub fn new() -> TemplateWidget {
        TemplateWidget
    }
}

impl<T: Data> Widget<T> for TemplateWidget {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut T, _env: &Env) {}

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &T, _env: &Env) {}

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &T, _data: &T, _env: &Env) {}

    fn layout(&mut self, _ctx: &mut LayoutCtx, bc: &BoxConstraints, _data: &T, _env: &Env) -> Size {
        bc.debug_check("Template");
        bc.max()
    }

    fn paint(&mut self, _ctx: &mut PaintCtx, _data: &T, _env: &Env) {}
}
