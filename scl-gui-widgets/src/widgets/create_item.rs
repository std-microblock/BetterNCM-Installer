use druid::{
    piet::{PaintBrush, Text, TextLayoutBuilder},
    widget::LabelText,
    *,
};

use crate::theme::color as theme;

/// 一个左侧有一个加号的可点击项目组件
pub struct CreateItem<T> {
    name: Option<druid::piet::PietTextLayout>,
    text: LabelText<T>,
}

impl<T> CreateItem<T> {
    /// 给予文字作为参数创建组件，文字将会显示在图标的右侧
    pub fn new(text: impl Into<LabelText<T>>) -> CreateItem<T> {
        CreateItem {
            name: None,
            text: text.into(),
        }
    }
}

impl<T: Data> Widget<T> for CreateItem<T> {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut T, _env: &Env) {}

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &T, _env: &Env) {}

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &T, _data: &T, _env: &Env) {}

    fn layout(&mut self, _ctx: &mut LayoutCtx, bc: &BoxConstraints, _data: &T, _env: &Env) -> Size {
        bc.constrain((150., 50.))
    }

    fn paint(&mut self, ctx: &mut PaintCtx, _data: &T, env: &Env) {
        let size = ctx.size();
        let is_hot = ctx.is_hot();
        let is_active = ctx.is_active();
        if is_hot {
            ctx.fill(
                size.to_rect(),
                &PaintBrush::Color(if is_active {
                    env.get(crate::theme::color::base::LOW)
                } else {
                    env.get(crate::theme::color::list::LIST_LOW)
                }),
            )
        }

        let text = ctx.text();
        let font_color = env.get(theme::typography::FONT_COLOR);
        if self.name.is_none() {
            let t = text
                .new_text_layout(self.text.display_text())
                .text_color(font_color.to_owned())
                .font(env.get(theme::typography::BASE).family, 13.)
                .build()
                .unwrap();
            self.name = Some(t);
        }
        let mut path = druid::kurbo::BezPath::new();
        path.move_to((26., 13.));
        path.line_to((26., 37.));
        path.move_to((14., 25.));
        path.line_to((38., 25.));
        ctx.stroke(path, &PaintBrush::Color(font_color), 4.);
        if let Some(name) = &self.name {
            ctx.draw_text(name, (50., 17.))
        }
    }
}
