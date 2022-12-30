//! 自制密码输入框

use druid::{piet::*, text::TextComponent, Rect, Widget};

/// 自制密码输入框，可以简单的输入密码和粘贴密码
pub struct PasswordBox {
    cursor: Option<usize>,
    mask_text: Option<druid::piet::PietTextLayout>,
}

impl PasswordBox {
    /// 创建一个空密码框
    pub const fn new() -> Self {
        Self {
            cursor: None,
            mask_text: None,
        }
    }
}

impl Default for PasswordBox {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget<String> for PasswordBox {
    fn event(
        &mut self,
        ctx: &mut druid::EventCtx,
        event: &druid::Event,
        data: &mut String,
        env: &druid::Env,
    ) {
        match event {
            druid::Event::KeyDown(k) => {
                if ctx.is_focused() && !k.is_composing {
                    let is_ctrl_pressed = {
                        #[cfg(target_os = "macos")]
                        {
                            k.mods.meta() // 对应 Mac 上的 Command 键
                        }
                        #[cfg(not(target_os = "macos"))]
                        {
                            k.mods.ctrl()
                        }
                    };
                    if let Some(cursor) = &mut self.cursor {
                        match k.code {
                            druid::Code::Backspace => {
                                if *cursor > 0 {
                                    if is_ctrl_pressed {
                                        for _ in 0..*cursor {
                                            data.remove(0);
                                        }
                                        *cursor = 0;
                                        ctx.request_paint();
                                    } else if let Some((ci, _)) =
                                        data.char_indices().nth(*cursor - 1)
                                    {
                                        *cursor -= 1;
                                        data.remove(ci);
                                        ctx.request_paint();
                                    }
                                }
                            }
                            druid::Code::ArrowLeft => {
                                if *cursor > 0 {
                                    if let Some((ci, _)) = data.char_indices().nth(*cursor - 1) {
                                        *cursor = ci;
                                        ctx.request_paint();
                                    }
                                }
                            }
                            druid::Code::ArrowRight => {
                                if *cursor < data.chars().count() {
                                    *cursor += 1;
                                    ctx.request_paint();
                                }
                            }
                            druid::Code::ArrowUp => {
                                *cursor = 0;
                                ctx.request_paint();
                            }
                            druid::Code::ArrowDown => {
                                *cursor = data.chars().count();
                                ctx.request_paint();
                            }
                            druid::Code::Tab => {
                                if k.mods.shift() {
                                    ctx.focus_prev();
                                } else {
                                    ctx.focus_next();
                                }
                                ctx.request_paint();
                            }
                            druid::Code::Enter => {}
                            code => {
                                if is_ctrl_pressed && code == druid::Code::KeyV {
                                    // 黏贴
                                    ctx.submit_command(druid::commands::PASTE.to(ctx.window_id()));
                                } else if !is_ctrl_pressed && !k.mods.alt() {
                                    if let druid::KbKey::Character(c) = &k.key {
                                        if *cursor > 0 {
                                            if let Some((ci, ch)) =
                                                data.char_indices().nth(*cursor - 1)
                                            {
                                                data.insert_str(ci + ch.len_utf8(), c.as_str());
                                            } else {
                                                data.insert_str(0, c.as_str())
                                            }
                                        } else {
                                            data.insert_str(0, c.as_str())
                                        }
                                        *cursor += c.chars().count();
                                        ctx.request_paint();
                                    }
                                }
                            }
                        }
                    }
                }
            }
            druid::Event::MouseDown(m) => {
                ctx.set_focus(ctx.widget_id());
                if let Some(mask_text) = &self.mask_text {
                    let textbox_insets = env.get(druid::theme::TEXTBOX_INSETS);
                    let cursor_index =
                        ((m.pos.x - textbox_insets.x0) / mask_text.size().width).max(0.);
                    self.cursor = Some((cursor_index as usize).min(data.chars().count()));
                    ctx.request_paint();
                    ctx.request_focus();
                }
            }
            druid::Event::MouseMove(_) => {
                ctx.set_cursor(&druid::Cursor::IBeam);
            }
            druid::Event::Paste(item) if ctx.is_focused() => {
                if let Some(cursor) = &mut self.cursor {
                    if let Some(c) = item.get_string() {
                        if *cursor > 0 {
                            if let Some((ci, ch)) = data.char_indices().nth(*cursor - 1) {
                                data.insert_str(ci + ch.len_utf8(), c.as_str());
                            } else {
                                data.insert_str(0, c.as_str())
                            }
                        } else {
                            data.insert_str(0, c.as_str())
                        }
                        *cursor += c.chars().count();
                        ctx.request_paint();
                    }
                }
            }
            druid::Event::Notification(cmd) => {
                if cmd.is(TextComponent::TAB) {
                    ctx.focus_next();
                    ctx.request_paint();
                    ctx.set_handled();
                }
            }
            _ => {}
        }
    }

    fn lifecycle(
        &mut self,
        ctx: &mut druid::LifeCycleCtx,
        event: &druid::LifeCycle,
        data: &String,
        _env: &druid::Env,
    ) {
        // todo!()

        if let druid::LifeCycle::FocusChanged(f) = event {
            if *f {
                self.cursor = Some(data.chars().count());
            } else {
                self.cursor = None;
            }
            ctx.request_paint();
        } else if let druid::LifeCycle::BuildFocusChain = event {
            ctx.register_for_focus();
        }
    }

    fn update(
        &mut self,
        _ctx: &mut druid::UpdateCtx,
        _old_data: &String,
        data: &String,
        _env: &druid::Env,
    ) {
        if let Some(cursor) = &mut self.cursor {
            *cursor = (*cursor).min(data.chars().count());
        }
    }

    fn layout(
        &mut self,
        _ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        _data: &String,
        _env: &druid::Env,
    ) -> druid::Size {
        bc.constrain((100., 32.))
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &String, env: &druid::Env) {
        if self.mask_text.is_none() {
            self.mask_text = ctx
                .text()
                .new_text_layout("•")
                .text_color(env.get(druid::theme::TEXT_COLOR))
                .font(
                    env.get(crate::theme::color::typography::CAPTION).family,
                    16.,
                )
                .build()
                .ok();
        }

        let size = ctx.size();
        let background_color = env.get(druid::theme::BACKGROUND_LIGHT);
        let border_width = env.get(druid::theme::TEXTBOX_BORDER_WIDTH);
        let pass_len = data.chars().count();

        let is_focused = ctx.is_focused();

        let border_color = if is_focused {
            env.get(druid::theme::PRIMARY_LIGHT)
        } else {
            env.get(druid::theme::BORDER_DARK)
        };

        // Paint the background
        let clip_rect = size
            .to_rect()
            .inset(-border_width / 2.0)
            .to_rounded_rect(env.get(druid::theme::TEXTBOX_BORDER_RADIUS));

        ctx.fill(clip_rect, &background_color);

        if let Some(mask_text) = &self.mask_text {
            let mask_size = mask_text.size();
            let cursor_color = env.get(druid::theme::CURSOR_COLOR);
            let textbox_insets = env.get(druid::theme::TEXTBOX_INSETS);

            ctx.with_save(|ctx| {
                ctx.clip(clip_rect);

                for c in 0..pass_len {
                    ctx.draw_text(
                        mask_text,
                        (
                            textbox_insets.x0 + (c as f64) * mask_size.width,
                            textbox_insets.y0,
                        ),
                    );
                }

                if let Some(cursor) = &self.cursor {
                    let cursor_rect = Rect::from_origin_size(
                        (
                            (*cursor as f64) * mask_size.width + textbox_insets.x0,
                            textbox_insets.y0,
                        ),
                        (1., mask_size.height),
                    );

                    ctx.fill(cursor_rect, &cursor_color);
                }
            })
        }

        // Paint the border
        ctx.stroke(clip_rect, &border_color, border_width);
    }
}
