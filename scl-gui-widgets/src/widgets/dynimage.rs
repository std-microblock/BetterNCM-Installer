use druid::{ImageBuf, RenderContext, Widget};

/// 一个可以动态改变的位图组件，可以接收 [`druid::ImageBuf`] 数据来改变显示的图片
#[derive(Clone, Default)]
pub struct DynImage(Option<druid::piet::PietImage>);

impl Widget<ImageBuf> for DynImage {
    fn event(
        &mut self,
        _: &mut druid::EventCtx,
        _: &druid::Event,
        _: &mut ImageBuf,
        _: &druid::Env,
    ) {
    }

    fn lifecycle(
        &mut self,
        _: &mut druid::LifeCycleCtx,
        _: &druid::LifeCycle,
        _: &ImageBuf,
        _: &druid::Env,
    ) {
    }

    fn update(&mut self, _: &mut druid::UpdateCtx, _: &ImageBuf, _: &ImageBuf, _: &druid::Env) {
        self.0 = None;
    }

    fn layout(
        &mut self,
        _: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        data: &ImageBuf,
        _: &druid::Env,
    ) -> druid::Size {
        if data.height() * data.width() == 0 {
            bc.constrain((0., 0.))
        } else {
            bc.constrain_aspect_ratio(
                data.height() as f64 / data.width() as f64,
                data.width() as f64,
            )
        }
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &ImageBuf, _env: &druid::Env) {
        if self.0.is_none() {
            self.0 = Some(data.to_image(ctx.render_ctx));
        }
        if let Some(img) = &self.0 {
            ctx.with_save(|ctx| {
                let rect = ctx.size().to_rect();
                ctx.clip(rect.to_rounded_rect(5.));
                ctx.draw_image(img, rect, druid::piet::InterpolationMode::Bilinear);
            });
        }
    }
}
