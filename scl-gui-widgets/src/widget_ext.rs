use druid::{
    widget::{prelude::*, ControllerHost},
    Selector, WidgetExt as _,
};

use crate::widgets::{OnChange, OnCmd, OnNotify, ShowIf, ShowIfCallback};

/// 一些常用的组件扩展特质
pub trait WidgetExt<T: Data>: Widget<T> + Sized + 'static {
    /// 用当该组件接收到指定的 [`druid::Command`] 时产生回调的控制器 [`crate::widgets::OnCmd`] 包裹当前组件
    fn on_command<CT: 'static>(
        self,
        selector: Selector<CT>,
        handler: impl Fn(&mut EventCtx, &CT, &mut T) + 'static,
    ) -> ControllerHost<Self, OnCmd<CT, T>> {
        self.controller(OnCmd::new(selector, handler))
    }

    /// 用当该组件接收到指定的 [`druid::Notification`] 时产生回调的控制器 [`crate::widgets::OnNotify`] 包裹当前组件
    fn on_notify<CT: 'static>(
        self,
        selector: Selector<CT>,
        handler: impl Fn(&mut EventCtx, &CT, &mut T) + 'static,
    ) -> OnNotify<CT, T> {
        OnNotify::new(selector, handler, self)
    }

    /// Calls the function when data changes **in a child widget**
    ///
    /// `&T` is the old data and `&mut T` is the new data
    fn on_change(
        self,
        f: impl Fn(&mut EventCtx, &T, &mut T, &Env) + 'static,
    ) -> ControllerHost<Self, OnChange<T>> {
        self.controller(OnChange::new(f))
    }

    /// 当数据变更时触发回调，如果为 `true` 则显示包裹起来的组件，否则隐藏
    ///
    /// 且隐藏时不会占用布局体积
    fn show_if(self, f: ShowIfCallback<T>) -> ShowIf<T> {
        ShowIf::new(self, f)
    }
}

impl<T: Data, W: Widget<T> + 'static> WidgetExt<T> for W {}
