#![doc = include_str!("../README.md")]
#![forbid(missing_docs)]

use proc_macro::TokenStream;
use syn::*;

mod icons;
use icons::*;

/// 使用简易的语法定义图标们
/// 中间的颜色值为可选，分别为亮色主题色和暗色主题色
/// ```
/// use druid::{ArcStr, Color, Data, Key};
/// use serde::{Deserialize, Serialize};
/// /// 一个存放明色，暗色，填充路径字符串的类型
/// #[derive(Debug, Clone, Data, PartialEq, Eq, Deserialize, Serialize)]
/// pub struct IconData(pub u32, pub u32, pub String);
/// /// 一个存放了图标颜色的键类型
/// pub type IconColorKey = Key<Color>;
/// /// 一个存放了图标填充路径的键类型
/// pub type IconPathKey = Key<ArcStr>;
/// /// 一个存放明色，暗色，填充路径字符串的键组合类型
/// pub type IconKeyPair = (IconPathKey, IconColorKey, IconColorKey);
/// scl_macro::icons_def! {
///     test_two_color 0x0000FFFF 0x00FF00FF "test_two_color";
///     test_one_color 0x0000FFFF "test_one_color";
///     test "test";
/// }
/// ```
#[proc_macro]
pub fn icons_def(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as IconsInput);
    icons::impl_icons_def(input)
}
