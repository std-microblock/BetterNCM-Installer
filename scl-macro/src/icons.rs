use proc_macro::TokenStream;
use quote::*;
use syn::{parse::*, *};

#[derive(Debug, Clone)]
pub struct Icon {
    pub name: Ident,
    pub light_color: u32,
    pub dark_color: u32,
    pub svg_path: LitStr,
}

#[derive(Debug, Clone)]
pub struct IconsInput {
    pub icons: Vec<Icon>,
}

impl Parse for IconsInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut result = Self {
            icons: Vec::with_capacity(32),
        };
        while !input.is_empty() {
            let icon_name: Ident = input.parse()?;
            if input.peek(LitInt) {
                let light_color = input.parse::<LitInt>()?.base10_parse()?;
                if input.peek(LitInt) {
                    let dark_color = input.parse::<LitInt>()?.base10_parse()?;
                    let svg_path: LitStr = input.parse()?;
                    result.icons.push(Icon {
                        name: icon_name,
                        light_color,
                        dark_color,
                        svg_path,
                    });
                } else if input.peek(LitStr) {
                    let svg_path: LitStr = input.parse()?;
                    result.icons.push(Icon {
                        name: icon_name,
                        light_color,
                        dark_color: light_color,
                        svg_path,
                    });
                } else {
                    return Err(syn::Error::new(
                        input.span(),
                        "not a correct icon dark color value (u32) or svg path (&str)",
                    ));
                }
            } else if input.peek(LitStr) {
                let svg_path: LitStr = input.parse()?;
                result.icons.push(Icon {
                    name: icon_name,
                    light_color: 0x000000FF,
                    dark_color: 0xFFFFFFFF,
                    svg_path,
                });
            } else {
                return Err(syn::Error::new(
                    input.span(),
                    "not a correct icon light color value (u32) or svg path (&str)",
                ));
            }
            input.parse::<Token![;]>()?;
        }
        Ok(result)
    }
}

pub(crate) fn impl_icons_def(input: IconsInput) -> TokenStream {
    let mut tokens = quote! {};
    tokens.append_all(input.icons.iter().map(|icon| {
        let icon_func_name = icon.name.to_owned();
        let icon_key_pair_name = format_ident!("{}", icon.name.to_owned().to_string().to_uppercase());
        let icon_path_key_name = format_ident!("{}", format!("{}_PATH", icon_func_name).to_uppercase());
        let icon_path_key = format!("net.stevexmh.scl-macro.icon.{}", icon_func_name).to_lowercase();
        let icon_light_color_key_name = format_ident!("{}", format!("{}_LIGHT_COLOR", icon_func_name).to_uppercase());
        let icon_light_color_key = format!("net.stevexmh.scl-macro.icon-color.light.{}", icon_func_name).to_lowercase();
        let icon_dark_color_key_name = format_ident!("{}", format!("{}_DARK_COLOR", icon_func_name).to_uppercase());
        let icon_dark_color_key = format!("net.stevexmh.scl-macro.icon-color.dark.{}", icon_func_name).to_lowercase();
        let default_light_color = icon.light_color;
        let default_dark_color = icon.dark_color;
        let svg_path = icon.svg_path.to_owned();
        quote! {
            /// 该图标对应的绘图路径键，可通过 [`druid::Env`] 取得
            const #icon_path_key_name: IconPathKey = druid::Key::new(#icon_path_key);
            /// 该图标对应的亮色模式的填充颜色键，可通过 [`druid::Env`] 取得
            const #icon_light_color_key_name: IconColorKey = druid::Key::new(#icon_light_color_key);
            /// 该图标对应的暗色模式的填充颜色键，可通过 [`druid::Env`] 取得
            const #icon_dark_color_key_name: IconColorKey = druid::Key::new(#icon_dark_color_key);
            /// 该图标对应的图标键对，可以用来提供给部分图标组件
            pub const #icon_key_pair_name: IconKeyPair = (#icon_path_key_name, #icon_light_color_key_name, #icon_dark_color_key_name);

            /// 返回一个 [`IconData`] 用于给 [`druid::Env`] 设置图标的信息
            fn #icon_func_name () -> IconData {
                IconData(#default_light_color, #default_dark_color, #svg_path.into())
            }
        }
    }));
    let mut icon_theme_config_tokens = quote! {};
    icon_theme_config_tokens.append_all(input.icons.iter().map(|icon| {
        let icon_config_key = icon.name.to_owned();
        quote! {
            /// 图标的主题配置
            #[serde(skip_serializing_if = "Option::is_none")]
            pub #icon_config_key: Option<IconData>,
        }
    }));
    let icon_theme_config_tokens = quote! {
        #[derive(Debug, Clone, druid::Data, druid::Lens, serde::Deserialize, serde::Serialize, PartialEq, Eq, Default)]
        #[serde(default)]
        #[allow(non_camel_case_types)]
        /// 过程宏生成的图标主题配置结构，已实现各种需要的特质
        pub struct IconThemeConfig {
            #icon_theme_config_tokens
        }
    };
    let mut set_icon_to_env_tokens = quote! {};
    set_icon_to_env_tokens.append_all(input.icons.iter().map(|icon| {
        let icon_func_name = icon.name.to_owned();
        let _icon_key_pair_name =
            format_ident!("{}", icon.name.to_owned().to_string().to_uppercase());
        let icon_path_key_name =
            format_ident!("{}", format!("{}_PATH", icon_func_name).to_uppercase());
        let _icon_path_key = format!("net.stevexmh.scl.icon.{}", icon_func_name).to_lowercase();
        let icon_light_color_key_name = format_ident!(
            "{}",
            format!("{}_LIGHT_COLOR", icon_func_name).to_uppercase()
        );
        let _icon_light_color_key =
            format!("net.stevexmh.scl.icon-color.light.{}", icon_func_name).to_lowercase();
        let icon_dark_color_key_name = format_ident!(
            "{}",
            format!("{}_DARK_COLOR", icon_func_name).to_uppercase()
        );
        let _icon_dark_color_key =
            format!("net.stevexmh.scl.icon-color.dark.{}", icon_func_name).to_lowercase();
        let default_light_color = icon.light_color;
        let default_dark_color = icon.dark_color;
        let svg_path = icon.svg_path.to_owned();
        quote! {
            if let Some(icon) = &icon_theme.#icon_func_name {
                icon_env.set(
                    #icon_path_key_name,
                    icon.2.to_string()
                );
                icon_env.set(
                    #icon_light_color_key_name,
                    druid::Color::Rgba32(icon.0)
                );
                icon_env.set(
                    #icon_dark_color_key_name,
                    druid::Color::Rgba32(icon.1)
                );
            } else {
                icon_env.set(
                    #icon_path_key_name,
                    #svg_path.to_string()
                );
                icon_env.set(
                    #icon_light_color_key_name,
                    druid::Color::Rgba32(#default_light_color)
                );
                icon_env.set(
                    #icon_dark_color_key_name,
                    druid::Color::Rgba32(#default_dark_color)
                );
            }
        }
    }));
    let set_icon_to_env_tokens = quote! {
        /// 过程宏生成的图标主题设置函数
        pub fn set_icon_to_env(icon_env: &mut druid::Env, icon_theme: &IconThemeConfig) {
            #set_icon_to_env_tokens
        }
    };
    let tokens = quote! {
        #tokens
        #icon_theme_config_tokens
        #set_icon_to_env_tokens
    };
    tokens.into()
}
