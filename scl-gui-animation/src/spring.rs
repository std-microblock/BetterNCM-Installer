//! 一个弹簧算法类

use std::{f64::consts::E, time::Instant};

type Num = f64;

#[inline]
/// Fast rounding for x <= 2^23.
/// This is orders of magnitude faster than built-in rounding intrinsic.
///
/// Source: <https://stackoverflow.com/a/42386149/585725>
pub fn round(mut x: Num) -> Num {
    x += 12582912.0;
    x -= 12582912.0;
    x
}

/// 一个一维弹簧，可用于动画用途
///
/// 优点是调用次数无关，可以在任意时间计算出当前时间的弹簧末端位置，非常方便
pub struct Spring {
    start_time: Instant,
    damper: Num,
    velocity: Num,
    speed: Num,
    target: Num,
    position: Num,
}

impl Spring {
    /// 根据初始位置创建一个新的弹簧
    pub fn new(start_position: Num) -> Self {
        Self {
            start_time: Instant::now(),
            position: start_position,
            damper: 0.95,
            velocity: 0.,
            speed: 1.,
            target: start_position,
        }
    }

    fn position_velocity(&mut self) -> (Num, Num) {
        let x = self.start_time.elapsed().as_secs_f64();
        let c0 = self.position - self.target;
        if self.speed == 0. {
            (self.position, 0.)
        } else if self.damper < 1. {
            let c = (1. - self.damper.powi(2)).sqrt();
            let c1 = (self.velocity / self.speed + self.damper * c0) / c;
            let co = (c * self.speed * x).cos();
            let si = (c * self.speed * x).sin();
            let e = E.powf(self.damper * self.speed * x);
            (
                self.target + (c0 * co + c1 * si) / e,
                self.speed * ((c * c1 - self.damper * c0) * co - (c * c0 + self.damper * c1) * si)
                    / e,
            )
        } else {
            let c1 = self.velocity / self.speed + c0;
            let e = E.powf(self.speed * x);
            (
                self.target + (c0 + c1 * self.speed * x) / e,
                self.speed * (c1 - c0 - c1 * self.speed * x) / e,
            )
        }
    }

    /// 判断弹簧是否已到达目标位置
    ///
    /// 既速度为零的情况下已到达目标位置
    pub fn arrived(&mut self) -> bool {
        let (pos, vel) = self.position_velocity();
        (round(pos * 10.) - round(self.target * 10.)).abs() < f64::EPSILON && round(vel * 10.) == 0.
    }

    /// 获得当前弹簧所在位置
    ///
    /// 会因为调用的时间不同而变化
    pub fn position(&mut self) -> Num {
        let r = self.position_velocity();
        self.position = r.0;
        self.velocity = r.1;
        r.0
    }

    /// 获得当前弹簧所在位置，但是会四舍五入成整数
    pub fn position_rounded(&mut self) -> Num {
        let r = self.position_velocity();
        self.position = r.0;
        self.velocity = r.1;
        round(r.0)
    }

    /// 获取当前弹簧的移动速度
    ///
    /// 会因为调用的时间不同而变化
    pub fn velocity(&mut self) -> Num {
        let r = self.position_velocity();
        self.position = r.0;
        self.velocity = r.1;
        r.1
    }

    /// 获取当前弹簧的加速度
    ///
    /// 会因为调用的时间不同而变化
    pub fn acceleration(&self) -> Num {
        let x = self.start_time.elapsed().as_secs_f64();
        let c0 = self.position - x;
        if self.speed == 0. {
            0.
        } else if self.damper < 1. {
            let c = (1. - self.damper.powi(2)).sqrt();
            let c1 = (self.velocity / self.speed + self.damper * c0) / c;
            self.speed.powi(2)
                * ((self.damper.powi(2) * c0 - 2. * c * self.damper * c1 - c.powi(2) * c0)
                    * (c * self.speed * x).cos()
                    + (self.damper * self.damper * c1 + 2. * c * self.damper * c0 - c.powi(2) * c1)
                        * (c * self.speed * x).cos())
                / E.powf(self.damper * self.speed * x)
        } else {
            let c1 = self.velocity / self.speed + c0;
            self.speed.powi(2) * (c0 - 2. * c1 + c1 * self.speed * x) / E.powf(self.speed * x)
        }
    }

    fn reset_time(&mut self) {
        self.start_time = Instant::now();
    }

    /// 设置当前位置，注意这不是目标位置，如果当前位置不是目标位置则会发生移动
    pub fn set_position(&mut self, value: Num) {
        let r = self.position_velocity();
        self.position = value;
        self.velocity = r.1;
        self.reset_time();
    }

    /// 设置当前速度，此时弹簧会立刻改变速度
    pub fn set_velocity(&mut self, value: Num) {
        let r = self.position_velocity();
        self.position = r.0;
        self.velocity = value;
        self.reset_time();
    }

    /// Builder 模式的 [`Spring::set_velocity`]
    pub fn with_velocity(mut self, value: Num) -> Self {
        self.set_velocity(value);
        self
    }

    /// 设置弹簧的阻尼
    ///
    /// 如果取值在 `[0.0 - 1.0)` 之间则会有回弹效果
    ///
    /// 如果大于等于 `1.0` 则会缓慢移动到目标位置而没有回弹效果
    pub fn set_damper(&mut self, value: Num) {
        let r = self.position_velocity();
        self.position = r.0;
        self.velocity = r.1;
        self.damper = value;
        self.reset_time();
    }

    /// Builder 模式的 [`Spring::set_damper`]
    pub fn with_damper(mut self, value: Num) -> Self {
        self.set_damper(value);
        self
    }

    /// 设置弹簧的运行速度，数值越大弹簧的速度就越快
    pub fn set_speed(&mut self, value: Num) {
        let r = self.position_velocity();
        self.position = r.0;
        self.velocity = r.1;
        self.speed = value;
        self.reset_time();
    }

    /// 返回弹簧的目标位置
    pub fn target(&self) -> Num {
        self.target
    }

    /// 设置弹簧的目标位置，此时弹簧会立刻开始变换
    pub fn set_target(&mut self, value: Num) {
        let r = self.position_velocity();
        self.position = r.0;
        self.velocity = r.1;
        self.target = value;
        self.reset_time();
    }
}

/// 一个二维弹簧，由两个一维弹簧组成
pub struct Spring2D {
    sx: Spring,
    sy: Spring,
}

impl Spring2D {
    /// 根据初始位置创建一个新的弹簧
    pub fn new(start_pos: (Num, Num)) -> Self {
        Self {
            sx: Spring::new(start_pos.0),
            sy: Spring::new(start_pos.1),
        }
    }

    /// 获得当前弹簧所在位置
    ///
    /// 会因为调用的时间不同而变化
    pub fn position(&mut self) -> (Num, Num) {
        (self.sx.position(), self.sy.position())
    }

    /// 获得当前弹簧所在位置，但是会四舍五入成整数
    pub fn position_rounded(&mut self) -> (Num, Num) {
        (self.sx.position_rounded(), self.sy.position_rounded())
    }

    /// 获取当前弹簧的移动速度
    ///
    /// 会因为调用的时间不同而变化
    pub fn velocity(&mut self) -> (Num, Num) {
        (self.sx.velocity(), self.sy.velocity())
    }

    /// 获取当前弹簧的阻尼
    pub fn damper(&self) -> Num {
        self.sx.damper
    }

    /// 获取弹簧的运行速度
    pub fn speed(&self) -> Num {
        (self.sx.speed.powi(2) + self.sy.speed.powi(2)).sqrt()
    }

    /// 获取弹簧的目标位置
    pub fn target(&self) -> (Num, Num) {
        (self.sx.target, self.sy.target)
    }

    /// 设置当前位置，注意这不是目标位置，如果当前位置不是目标位置则会发生移动
    pub fn set_position(&mut self, value: (Num, Num)) {
        self.sx.position = value.0;
        self.sy.position = value.1;
    }

    /// 设置当前速度，此时弹簧会立刻改变速度
    pub fn set_velocity(&mut self, value: (Num, Num)) {
        self.sx.velocity = value.0;
        self.sy.velocity = value.1;
    }

    /// 设置弹簧的阻尼
    ///
    /// 如果取值在 `[0.0 - 1.0)` 之间则会有回弹效果
    ///
    /// 如果大于等于 `1.0` 则会缓慢移动到目标位置而没有回弹效果
    pub fn set_damper(&mut self, value: Num) {
        self.sx.damper = value;
        self.sy.damper = value;
    }

    /// 设置弹簧的运行速度，数值越大弹簧的速度就越快
    pub fn set_speed(&mut self, value: Num) {
        self.sx.set_speed(value);
        self.sy.set_speed(value);
    }

    /// 设置弹簧的目标位置，此时弹簧会立刻开始变换
    pub fn set_target(&mut self, value: (Num, Num)) {
        self.sx.set_target(value.0);
        self.sy.set_target(value.1);
    }

    /// 判断弹簧是否已到达目标位置
    ///
    /// 既速度为零的情况下已到达目标位置
    pub fn arrived(&mut self) -> bool {
        self.sx.arrived() && self.sy.arrived()
    }
}

impl From<f64> for Spring {
    fn from(p: f64) -> Self {
        Self::new(p)
    }
}
