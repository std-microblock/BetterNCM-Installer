use druid::{widget::Controller, *};

type PressKeyAction<T> = Box<dyn Fn(&mut EventCtx, &mut T, &Env)>;

/// 一个控制器，当子组件被聚焦且按下指定按键时触发回调
pub struct PressKey<T> {
    is_key_down: bool,
    key_code: Code,
    /// A closure that will be invoked when the child widget is focused and the target key is pressed.
    action: PressKeyAction<T>,
}

impl<T> PressKey<T> {
    /// 根据需要按下的按键和需要触发的回调函数创建此控制器
    pub fn new(key_code: Code, action: impl Fn(&mut EventCtx, &mut T, &Env) + 'static) -> Self {
        PressKey {
            action: Box::new(action),
            key_code,
            is_key_down: false,
        }
    }
}

impl<T, W: Widget<T>> Controller<T, W> for PressKey<T> {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        if ctx.is_focused() && !ctx.is_disabled() {
            if let Event::KeyDown(key) = event {
                if key.code == self.key_code {
                    self.is_key_down = true;
                }
            } else if let Event::KeyUp(key) = event {
                if key.code == self.key_code && self.is_key_down {
                    (self.action)(ctx, data, env);
                    self.is_key_down = false;
                }
            }
        } else {
            self.is_key_down = false;
        }
        child.event(ctx, event, data, env)
    }
}
