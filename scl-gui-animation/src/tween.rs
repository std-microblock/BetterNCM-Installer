//! 各种动画曲线算法

/// Circ 算法滑入，输入输出均为 [0.0 - 1.0]
pub fn ease_in_circ(x: f64) -> f64 {
    1. - (1. - x.powi(2)).sqrt()
}

/// Circ 算法滑出，输入输出均为 [0.0 - 1.0]
pub fn ease_out_circ(x: f64) -> f64 {
    (1. - (x - 1.).powi(2)).sqrt()
}

/// Circ 算法滑入滑出，输入输出均为 [0.0 - 1.0]
pub fn ease_inout_circ(x: f64) -> f64 {
    if x < 0.5 {
        ease_in_circ(x * 2.) / 2.
    } else {
        ease_out_circ((0.5 - x) * 2.) / 2. + 0.5
    }
}

/// Expo 算法滑入，输入输出均为 [0.0 - 1.0]
pub fn ease_in_expo(x: f64) -> f64 {
    if x == 0. {
        0.
    } else {
        (2f64).powf(10. * x - 10.)
    }
}

/// Expo 算法滑出，输入输出均为 [0.0 - 1.0]
pub fn ease_out_expo(x: f64) -> f64 {
    if x >= 1. {
        1.
    } else {
        1. - (2f64).powf(-10. * x)
    }
}
