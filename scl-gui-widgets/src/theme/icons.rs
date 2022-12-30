//! 图标模块，提供了显示图标所需要的东西

use druid::{ArcStr, Color, Data, Key};
use serde::{Deserialize, Serialize};

/// 一个存放明色，暗色，填充路径字符串的类型
#[derive(Debug, Clone, Data, PartialEq, Eq, Deserialize, Serialize)]
pub struct IconData(pub u32, pub u32, pub String);
/// 一个存放了图标颜色的键类型
pub type IconColorKey = Key<Color>;
/// 一个存放了图标填充路径的键类型
pub type IconPathKey = Key<ArcStr>;
/// 一个存放明色，暗色，填充路径字符串的键组合类型
pub type IconKeyPair = (IconPathKey, IconColorKey, IconColorKey);

impl From<String> for IconData {
    fn from(v: String) -> Self {
        Self(0x000000FF, 0xFFFFFFFF, v)
    }
}

impl From<&str> for IconData {
    fn from(v: &str) -> Self {
        Self(0x000000FF, 0xFFFFFFFF, v.into())
    }
}

impl From<(u32, String)> for IconData {
    fn from(v: (u32, String)) -> Self {
        Self(v.0, v.0, v.1)
    }
}

impl From<(u32, &str)> for IconData {
    fn from(v: (u32, &str)) -> Self {
        Self(v.0, v.0, v.1.into())
    }
}

scl_macro::icons_def! {
    empty "";
}
