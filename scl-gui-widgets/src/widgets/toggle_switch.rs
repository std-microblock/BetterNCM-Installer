use druid::{piet::*, *};

/// 一个仿 WinUI 3 的开关组件
pub struct ToggleSwitch {
    current_thumb_extension: f64,
    current_thumb_radius: f64,
    current_thumb_position: f64,
    dragging_thumb_position: f64,
}

impl ToggleSwitch {
    /// 创建一个开关组件
    pub fn new() -> Self {
        Self::default()
    }

    fn is_hovering_thumb(&self, x: f64, y: f64) -> bool {
        let thumb_origin_pos = (4. + 4. + 6. + self.current_thumb_position, 6. + 4. + 6.);
        let thumb_radius = 6f64;
        let distance = (x - thumb_origin_pos.0).powi(2) + (y - thumb_origin_pos.1).powi(2);
        distance <= thumb_radius.powi(2)
    }
}

impl Default for ToggleSwitch {
    fn default() -> Self {
        Self {
            dragging_thumb_position: f64::NAN,
            current_thumb_position: 0.,
            current_thumb_radius: 0.,
            current_thumb_extension: 0.,
        }
    }
}

impl Widget<bool> for ToggleSwitch {
    fn event(&mut self, ctx: &mut EventCtx, evt: &Event, data: &mut bool, _env: &Env) {
        match evt {
            Event::MouseDown(m) => {
                if !ctx.is_disabled() {
                    ctx.set_active(true);
                    if self.is_hovering_thumb(m.pos.x, m.pos.y) {
                        self.dragging_thumb_position = m.pos.x;
                        ctx.request_anim_frame();
                    }
                }
            }
            Event::MouseUp(m) => {
                if !ctx.is_disabled() {
                    ctx.set_active(false);
                    let _last_data = *data;
                    if self.dragging_thumb_position.is_nan()
                        || (self.dragging_thumb_position - m.pos.x).abs() < f64::EPSILON
                    {
                        *data = !*data;
                    } else {
                        *data =
                            self.current_thumb_position > 10. - self.current_thumb_extension / 2.;
                    }
                    ctx.request_anim_frame();
                    self.dragging_thumb_position = f64::NAN;
                }
            }
            Event::MouseMove(m) => {
                if !ctx.is_disabled() && ctx.is_hot() && !self.dragging_thumb_position.is_nan() {
                    if *data {
                        self.current_thumb_position = 20. - self.current_thumb_extension + m.pos.x
                            - self.dragging_thumb_position;
                    } else {
                        self.current_thumb_position = m.pos.x - self.dragging_thumb_position;
                    }
                    self.current_thumb_position = self
                        .current_thumb_position
                        .clamp(0., 20. - self.current_thumb_extension);
                }
            }
            Event::AnimFrame(_) => {
                let mut should_animate = false;
                if self.dragging_thumb_position.is_nan() {
                    if ctx.is_active() {
                        let target_position = if *data {
                            20. - self.current_thumb_extension
                        } else {
                            0.
                        };
                        self.current_thumb_position +=
                            (target_position - self.current_thumb_position) / 4.;
                        should_animate |=
                            (target_position - self.current_thumb_position).abs() > f64::EPSILON;
                        self.current_thumb_extension += (3. - self.current_thumb_extension) / 4.;
                        should_animate |= (3. - self.current_thumb_extension).abs() > f64::EPSILON;
                    } else if ctx.is_hot() {
                        self.current_thumb_radius += (1. - self.current_thumb_radius) / 4.;
                        should_animate |= (1. - self.current_thumb_radius).abs() > f64::EPSILON;
                        ctx.request_paint();
                    } else {
                        self.current_thumb_radius -= self.current_thumb_radius / 4.;
                        should_animate |= self.current_thumb_radius.abs() > f64::EPSILON;
                    }
                    let target_position = if *data {
                        20. - self.current_thumb_extension
                    } else {
                        0.
                    };
                    self.current_thumb_position +=
                        (target_position - self.current_thumb_position) / 4.;
                    should_animate |=
                        (target_position - self.current_thumb_position).abs() > f64::EPSILON;
                    self.current_thumb_extension -= self.current_thumb_extension / 4.;
                    should_animate |= self.current_thumb_extension > f64::EPSILON;
                } else {
                    self.current_thumb_extension += (3. - self.current_thumb_extension) / 4.;
                    should_animate |= (3. - self.current_thumb_extension).abs() > f64::EPSILON;
                }
                ctx.request_paint();
                if should_animate {
                    ctx.request_anim_frame();
                }
            }
            _ => {}
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &bool, _env: &Env) {
        if let LifeCycle::WidgetAdded = event {
            self.current_thumb_position = if *data {
                20. - self.current_thumb_extension
            } else {
                0.
            };
        } else if let LifeCycle::HotChanged(_) = event {
            if !ctx.is_disabled() {
                ctx.request_anim_frame();
            }
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &bool, _data: &bool, _env: &Env) {
        ctx.request_anim_frame();
    }

    fn layout(
        &mut self,
        _ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &bool,
        _env: &Env,
    ) -> Size {
        bc.debug_check("ToggleSwitch");
        bc.constrain((48., 32.))
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &bool, env: &Env) {
        let disabled = ctx.is_disabled();
        let outline = Rect::from_origin_size((4. + 0.5, 6. + 0.5), (38. + 1., 18. + 1.));
        let outline = outline.to_rounded_rect(outline.height() / 2.);

        ctx.fill(
            outline,
            &if disabled {
                if *data {
                    PaintBrush::Color(env.get(crate::theme::color::base::LOW))
                } else {
                    PaintBrush::Color(Color::TRANSPARENT)
                }
            } else if *data {
                PaintBrush::Color(env.get(crate::theme::color::main::PRIMARY))
            } else {
                PaintBrush::Color(env.get(crate::theme::color::base::LOW))
            },
        );
        ctx.stroke(
            outline,
            &if disabled {
                PaintBrush::Color(env.get(crate::theme::color::base::LOW))
            } else if *data {
                PaintBrush::Color(env.get(crate::theme::color::main::PRIMARY))
            } else {
                PaintBrush::Color(env.get(crate::theme::color::base::MEDIUM))
            },
            1.,
        );
        let thumb_outline = Rect::from_origin_size(
            (
                4. + 4. + self.current_thumb_position - self.current_thumb_radius / 2.,
                6. + 4. - self.current_thumb_radius / 2.,
            ),
            (
                12. + self.current_thumb_radius + self.current_thumb_extension,
                12. + self.current_thumb_radius,
            ),
        );
        let thumb_outline = thumb_outline.to_rounded_rect(thumb_outline.height() / 2.);
        ctx.fill(
            thumb_outline,
            &if disabled {
                PaintBrush::Color(env.get(crate::theme::color::base::MEDIUM_LOW))
            } else if *data {
                PaintBrush::Color(env.get(crate::theme::color::alt::HIGH))
            } else {
                PaintBrush::Color(env.get(crate::theme::color::chrome::ALT_LOW))
            },
        );
        if !disabled {
            ctx.stroke(
                thumb_outline,
                &PaintBrush::Color(env.get(crate::theme::color::base::LOW)),
                1.,
            );
        }
    }
}
