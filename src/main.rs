#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
#![feature(fs_try_exists)]
#![feature(rustc_attrs)]
#[rustc_box]
mod ncm_utils;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process;
use std::process::Command;
use std::time::Duration;
use std::{env, os::windows::process::CommandExt};

use anyhow::Context;
use anyhow::Result;
use druid::commands::CLOSE_ALL_WINDOWS;
use druid::widget::Checkbox;
use druid::widget::{Flex, Label, ProgressBar};
use druid::Color;
use druid::ExtEventSink;
use druid::{
    AppLauncher, Data, FontDescriptor, FontWeight, Lens, Widget, WidgetExt as _, WindowDesc,
};
use ncm_utils::{get_ncm_version, is_vc_redist_14_x86_installed};
use semver::Version;
use winreg::enums::HKEY_CURRENT_USER;
use winreg::enums::HKEY_LOCAL_MACHINE;
use winreg::RegKey;

use scl_gui_widgets::{
    widget_ext::WidgetExt,
    widgets::{Button, WindowWidget, QUERY_CLOSE_WINDOW},
};

use crate::ncm_utils::get_ncm_install_path;

#[derive(Debug, Clone, PartialEq)]
pub enum AdaptedVersionResult {
    Version(Version),
    NoAdaptedVersion,
}

#[derive(Debug, Clone, Data, Lens)]
struct AppData {
    progress: f64,
    prerelease: bool,
    #[data(eq)]
    latest_version: Option<AdaptedVersionResult>,
    old_version: bool,
    new_version: bool,
    #[data(eq)]
    installer_version: Version,

    #[data(eq)]
    tips_string: String,
    #[data(eq)]
    ncm_install_path: Option<PathBuf>,
    #[data(eq)]
    latest_download_url: Option<String>,
    #[data(eq)]
    ncm_version: Option<Version>,
}

fn config_path() -> String {
    String::from(
        dirs::home_dir()
            .unwrap()
            .as_os_str()
            .to_str()
            .expect("Covert error"),
    ) + "\\betterncm\\"
}

fn get_adapted_betterncm_version(
    ncm_version_: Option<Version>,
    event_sink: ExtEventSink,
    channel: String,
) -> anyhow::Result<(), Box<dyn std::error::Error>> {
    if let Some(ncm_ver) = ncm_version_ {
        use serde_json::Value;
        let releases = tinyget::get(if ncm_ver.major == 3 {
            "https://gitee.com/microblock/better-ncm-v2-data/raw/master/betterncm/betterncm3.json"
        } else {
            "https://gitee.com/microblock/better-ncm-v2-data/raw/master/betterncm/betterncm1.json"
        })
        .with_header("User-Agent", "BetterNCM Installer/1.0.3")
        .send()?;

        let releases = releases.as_str()?;

        let releases: Value = serde_json::from_str(releases)?;

        let adapted_versions = releases[channel]
            .as_object()
            .context("Invalid JSON")?
            .clone();
        for (version_req, val) in adapted_versions.iter() {
            if semver::VersionReq::parse(version_req)
                .context("Failed to parse version req")?
                .matches(&ncm_ver)
            {
                let latest_version = Some(AdaptedVersionResult::Version(
                    Version::parse(val["version"].to_owned().as_str().unwrap()).unwrap(),
                ));
                let latest_url = Some(val["url"].clone().as_str().unwrap().to_string());

                event_sink.add_idle_callback(move |data: &mut AppData| {
                    data.latest_version = latest_version;
                    data.latest_download_url = latest_url;
                });
                return Ok(());
            }
        }
    }

    event_sink.add_idle_callback(move |data: &mut AppData| {
        data.latest_version = Some(AdaptedVersionResult::NoAdaptedVersion);
    });

    Ok(())
}

fn main() -> Result<()> {
    let main_window = WindowDesc::new(ui_builder())
        .window_size((400., 310.))
        .resizable(false)
        .show_titlebar(false)
        .title("BetterNCM Installer");

    let mut data = AppData {
        prerelease: false,
        progress: 0.,
        latest_version: None,
        old_version: if let Ok(path) = get_ncm_install_path() {
            path.join("cloudmusicn.exe").exists()
        } else {
            false
        },
        new_version: if let Ok(path) = get_ncm_install_path() {
            path.join("msimg32.dll").exists()
        } else {
            false
        },
        latest_download_url: None,
        installer_version: Version::parse(env!("CARGO_PKG_VERSION"))?,
        ncm_install_path: if let Ok(path) = get_ncm_install_path() {
            Some(path)
        } else {
            None
        },
        ncm_version: get_ncm_version().ok(),
        tips_string: String::new(),
    };
    if let Some(v) = &data.ncm_version {
        if v < &Version::new(2, 10, 2) {
            data.tips_string = "您的网易云版本太低，请更新".to_string();
        }
    }
    let launcher = AppLauncher::with_window(main_window);

    let event_sink = launcher.get_external_handle();

    let ncm_version_ = data.ncm_version.clone();

    std::thread::spawn(move || {
        let _ = get_adapted_betterncm_version(ncm_version_, event_sink, "versions".to_string());
    });

    launcher
        .configure_env(|env, _| {
            scl_gui_widgets::theme::color::set_color_to_env(
                env,
                scl_gui_widgets::theme::color::Theme::Dark,
            );
        })
        .launch(data)?;
    Ok(())
}

fn get_ncm_localdata_path() -> String {
    let appdata = env::var("APPDATA").unwrap();
    let ncmdata = Path::new(&appdata);
    ncmdata
        .parent()
        .unwrap()
        .join("Local")
        .join("Netease")
        .join("CloudMusic")
        .to_str()
        .unwrap()
        .to_string()
}

fn set_noproxy_localdata() -> anyhow::Result<()> {
    fs::write(
        get_ncm_localdata_path() + "/localdata",
        include_bytes!("localdata/localdata_noproxy"),
    )?;
    Ok(())
}

fn ui_builder() -> impl Widget<AppData> {
    let title = Label::new("BetterNCM Installer".to_string()).with_font(
        FontDescriptor::default()
            .with_size(20.)
            .with_weight(FontWeight::BOLD),
    );

    let installer_version_label = Flex::row()
        .with_child(Label::new("BetterNCM Installer 版本: ").with_text_color(Color::grey(0.7)))
        .with_child(
            Label::new(|data: &AppData, _env: &_| -> String { data.installer_version.to_string() })
                .with_font(
                    FontDescriptor::default()
                        .with_size(17.)
                        .with_weight(FontWeight::SEMI_BOLD),
                ),
        );

    let latest_version_label = Flex::row()
        .with_child(Label::new("适配 BetterNCM 版本: ").with_text_color(Color::grey(0.7)))
        .with_child(
            Label::new(|data: &AppData, _env: &_| -> String {
                match &data.latest_version {
                    Some(AdaptedVersionResult::Version(version)) => version.to_string(),
                    Some(AdaptedVersionResult::NoAdaptedVersion) => "未适配".to_string(),
                    None => String::from("获取中..."),
                }
            })
            .with_font(
                FontDescriptor::default()
                    .with_size(17.)
                    .with_weight(FontWeight::SEMI_BOLD),
            ),
        );

    let local_version_label = Flex::row().with_child(
        Label::new(|data: &AppData, _env: &_| -> String {
            match data.old_version {
                true => String::from("检测到老版本BetterNCM 请先卸载"),
                false => String::from(""),
            }
        })
        .with_font(
            FontDescriptor::default()
                .with_size(17.)
                .with_weight(FontWeight::SEMI_BOLD),
        ),
    );

    let install_path_label = Flex::row()
        .with_child(Label::new("网易云版本: ").with_text_color(Color::grey(0.7)))
        .with_child(
            Label::new(|data: &AppData, _env: &_| -> String {
                match &data.ncm_version {
                    Some(ver) => format!("{ver}"),
                    None => "未安装".to_string(),
                }
            })
            .with_font(
                FontDescriptor::default()
                    .with_size(17.)
                    .with_weight(FontWeight::SEMI_BOLD),
            ),
        );

    let checker_prerelease = Checkbox::new("测试通道")
        .on_change(|ctx, _old, new, _env| {
            let sink = ctx.get_external_handle();
            let channel = if *new { "test" } else { "versions" };
            ctx.get_external_handle()
                .add_idle_callback(move |data: &mut AppData| {
                    data.latest_version = None;
                    data.tips_string = "".into();
                    let ncm_version_ = data.ncm_version.clone();
                    std::thread::spawn(move || {
                        let _ =
                            get_adapted_betterncm_version(ncm_version_, sink, channel.to_string());
                    });
                });
        })
        .lens(AppData::prerelease);

    let button_install = Button::new("安装")
        .disabled_if(|data: &AppData, _env: &_| {
            data.latest_version.is_none()
                || data.latest_version == Some(AdaptedVersionResult::NoAdaptedVersion)
                || data.old_version
                || data.new_version
        })
        .on_click(|ctx, data, _env| {
            let event_sink = ctx.get_external_handle();
            let url: String = data.latest_download_url.as_ref().unwrap().clone();
            std::thread::spawn(move || {
                fn add_exclude_from_wd() -> anyhow::Result<()> {
                    Command::new("powershell.exe")
                        .arg("-Command")
                        .arg(format!(
                            "Add-MpPreference -ExclusionPath \"{}\"",
                            get_ncm_install_path()?
                                .to_str()
                                .context("Failed to get ncm install path")?
                        ))
                        .spawn()?
                        .wait()?;

                    Ok(())
                }

                let _ = add_exclude_from_wd();

                let _ = std::fs::remove_file("betterncm.dll");

                download_file(&url, "betterncm.dll", event_sink.to_owned());
                install_vc_redist_14_x86(event_sink.to_owned());

                event_sink.add_idle_callback(move |data: &mut AppData| {
                    data.tips_string = "正在安装 BetterNCM…".into();
                });

                Command::new("taskkill.exe")
                    .args(["/f", "/im", "cloudmusic.exe"])
                    .creation_flags(0x08000000)
                    .spawn()?
                    .wait()?;

                std::thread::sleep(Duration::from_millis(300));

                std::fs::copy("betterncm.dll", get_ncm_install_path()?.join("msimg32.dll"))
                    .unwrap();

                event_sink.add_idle_callback(move |data: &mut AppData| {
                    data.tips_string = "安装成功！".into();
                    data.new_version = if let Ok(path) = get_ncm_install_path() {
                        path.join("msimg32.dll").exists()
                    } else {
                        false
                    };
                });

                Command::new(get_ncm_install_path()?.join("cloudmusic.exe"))
                    .current_dir(get_ncm_install_path()?)
                    .spawn()?;
                anyhow::Ok(())
            });
        });

    let button_reinstall = Button::new("重装/更新")
        .disabled_if(|data: &AppData, _env: &_| {
            data.latest_version.is_none()
                || data.latest_version == Some(AdaptedVersionResult::NoAdaptedVersion)
                || data.old_version
                || !data.new_version
        })
        .on_click(|ctx, data, _env| {
            let event_sink = ctx.get_external_handle();
            let url: String = data.latest_download_url.as_ref().unwrap().clone();
            std::thread::spawn(move || {
                let _ = std::fs::remove_file("betterncm.dll");

                download_file(&url, "betterncm.dll", event_sink.to_owned());
                install_vc_redist_14_x86(event_sink.to_owned());

                event_sink.add_idle_callback(move |data: &mut AppData| {
                    data.tips_string = "正在升级/重新安装 BetterNCM…".into();
                });

                Command::new("taskkill.exe")
                    .args(["/f", "/im", "cloudmusic.exe"])
                    .creation_flags(0x08000000)
                    .spawn()?
                    .wait()?;

                std::thread::sleep(Duration::from_millis(300));

                std::fs::copy("betterncm.dll", get_ncm_install_path()?.join("msimg32.dll"))
                    .unwrap();

                event_sink.add_idle_callback(move |data: &mut AppData| {
                    data.tips_string = "升级/重新安装成功！".into();
                    data.new_version = if let Ok(path) = get_ncm_install_path() {
                        path.join("msimg32.dll").exists()
                    } else {
                        false
                    };
                });

                Command::new(get_ncm_install_path()?.join("cloudmusic.exe"))
                    .current_dir(get_ncm_install_path()?)
                    .spawn()?;
                anyhow::Ok(())
            });
        });

    let button_uninstall = Button::new("卸载")
        .disabled_if(|data: &AppData, _env: &_| data.old_version || !data.new_version)
        .on_click(|ctx, _data, _env| {
            let event_sink = ctx.get_external_handle();
            std::thread::spawn(move || {
                event_sink.add_idle_callback(move |data: &mut AppData| {
                    data.tips_string = "正在卸载 BetterNCM…".into();
                });
                Command::new("taskkill.exe")
                    .args(["/f", "/im", "cloudmusic.exe"])
                    .creation_flags(0x08000000)
                    .spawn()?
                    .wait()?;
                Command::new("taskkill.exe")
                    .args(["/f", "/im", "cloudmusicn.exe"])
                    .creation_flags(0x08000000)
                    .spawn()?
                    .wait()?;
                fs::remove_file(get_ncm_install_path()?.join("msimg32.dll"))?;

                event_sink.add_idle_callback(move |data: &mut AppData| {
                    data.new_version = if let Ok(path) = get_ncm_install_path() {
                        path.join("msimg32.dll").exists()
                    } else {
                        false
                    };
                    data.tips_string = "卸载完成！".into();
                });

                process::Command::new(get_ncm_install_path()?.join("cloudmusic.exe"))
                    .current_dir(get_ncm_install_path()?)
                    .spawn()?;
                anyhow::Ok(())
            });
        });

    let button_uninstall_old = Button::new("卸载老版本")
        .disabled_if(|data: &AppData, _env: &_| !data.old_version)
        .on_click(|_ctx, data, _env| {
            let mut ins = || {
                fs::remove_dir_all(config_path())?;
                Command::new("taskkill.exe")
                    .args(["/f", "/im", "cloudmusic.exe"])
                    .creation_flags(0x08000000)
                    .spawn()?
                    .wait()?;
                Command::new("taskkill.exe")
                    .args(["/f", "/im", "cloudmusicn.exe"])
                    .creation_flags(0x08000000)
                    .spawn()?
                    .wait()?;
                fs::remove_file(get_ncm_install_path()?.join("cloudmusic.exe"))?;

                fs::rename(
                    get_ncm_install_path()?.join("cloudmusicn.exe"),
                    get_ncm_install_path()?.join("cloudmusic.exe"),
                )?;

                set_noproxy_localdata()?;

                data.old_version = if let Ok(path) = get_ncm_install_path() {
                    path.join("cloudmusicn.exe").exists()
                } else {
                    false
                };

                process::Command::new(get_ncm_install_path()?.join("cloudmusic.exe"))
                    .current_dir(get_ncm_install_path()?)
                    .spawn()?;
                anyhow::Ok(())
            };
            ins().unwrap();
        });

    let button_set_path = Button::new("修改数据地址为 C:/betterncm")
        .on_click(|_ctx, _data, _env| {
            let hkcu = RegKey::predef(HKEY_CURRENT_USER);
            let (env, _) = hkcu.create_subkey("Environment").unwrap();
            env.set_value("BETTERNCM_PROFILE", &"C:\\betterncm")
                .unwrap();

            let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
            let (env, _) = hklm
                .create_subkey("System\\CurrentControlSet\\Control\\Session Manager\\Environment")
                .unwrap();
            env.set_value("BETTERNCM_PROFILE", &"C:\\betterncm")
                .unwrap();
        })
        .disabled_if(|_data, _env| {
            let get_profile = || {
                let hkcu = RegKey::predef(HKEY_CURRENT_USER);
                let (env, _) = hkcu.create_subkey("Environment")?;
                let profile: String = env.get_value("BETTERNCM_PROFILE")?;
                Ok(profile)
            };
            let profile: anyhow::Result<String> = get_profile();

            if let Ok(path) = profile {
                path != *"C:\\betterncm"
            } else {
                true
            }
        });

    let progress_bar = ProgressBar::new().lens(AppData::progress).expand_width();

    WindowWidget::new(
        "BetterNCM Installer",
        Flex::column()
            .with_child(title)
            .with_child(installer_version_label)
            .with_child(latest_version_label)
            .with_child(install_path_label)
            .with_child(local_version_label)
            .with_spacer(5.)
            .with_child(Label::new(|data: &AppData, _env: &_| -> String {
                data.tips_string.clone()
            }))
            .with_flex_spacer(1.)
            .with_child(checker_prerelease)
            .with_spacer(5.)
            .with_child(
                Flex::row()
                    .with_flex_child(button_install.expand_width(), 1.)
                    .with_spacer(5.)
                    .with_flex_child(button_reinstall.expand_width(), 1.)
                    .with_spacer(5.)
                    .with_flex_child(button_uninstall.expand_width(), 1.)
                    .with_spacer(5.)
                    .with_flex_child(button_uninstall_old.expand_width(), 1.),
            )
            .with_spacer(5.)
            .with_child(button_set_path.expand_width())
            .with_spacer(5.)
            .with_child(progress_bar)
            .cross_axis_alignment(druid::widget::CrossAxisAlignment::Start)
            .padding(10.),
    )
    .on_notify(QUERY_CLOSE_WINDOW, |ctx, _, _| {
        ctx.submit_command(CLOSE_ALL_WINDOWS);
    })
}

fn download_file(url: &str, path: &str, event_sink: druid::ExtEventSink) {
    let tip_str = format!("正在下载: {path}");
    event_sink.add_idle_callback(move |data: &mut AppData| {
        data.tips_string = tip_str;
    });
    use std::fs::File;
    use std::io::Write;

    let res = tinyget::get(url)
        .with_header(
            "User-Agent",
            &format!("BetterNCM Installer/{};", env!("CARGO_PKG_VERSION")),
        )
        .send_lazy()
        .unwrap();

    let file_size = res
        .headers
        .get("content-length")
        .map(|x| x.as_str().parse::<usize>())
        .unwrap_or(Ok(0))
        .unwrap_or(0);

    event_sink.add_idle_callback(move |data: &mut AppData| {
        data.tips_string = "正在下载…".into();
    });

    let mut file = File::create(path)
        .or(Err(format!("Failed to create file '{path}'")))
        .unwrap();

    let mut buf = Vec::with_capacity(file_size);
    let mut tip_str = "正在下载…".to_string();
    for data in res {
        let (byte, length) = data.unwrap();
        buf.reserve(length);
        buf.push(byte);

        let progress = buf.len() as f64 / file_size as f64;
        let percent_progress = ((progress * 100.).floor() as u32).min(100).max(0);
        let new_tip_str = format!("正在下载：{path}（{percent_progress}%）");
        if tip_str != new_tip_str {
            tip_str = new_tip_str.to_owned();
            event_sink.add_idle_callback(move |data: &mut AppData| {
                data.tips_string = new_tip_str;
                data.progress = progress;
            });
        }
    }

    file.write_all(&buf).unwrap();

    event_sink.add_idle_callback(move |data: &mut AppData| {
        data.tips_string = "".to_string();
    });
}

pub fn install_vc_redist_14_x86(event_sink: druid::ExtEventSink) {
    if is_vc_redist_14_x86_installed() {
        return;
    }
    // https://aka.ms/vs/17/release/VC_redist.x86.exe
    // Install: /install /passive /norestart
    // SilentInstall: /install /quiet /norestart

    download_file(
        "https://aka.ms/vs/17/release/VC_redist.x86.exe",
        "VC_redist.x86.exe",
        event_sink.to_owned(),
    );

    event_sink.add_idle_callback(move |data: &mut AppData| {
        data.tips_string = "正在安装 VC 运行时…".into();
        data.progress = 1.;
    });

    assert!(
        Command::new("VC_redist.x86.exe")
            .args(["/install", "/quiet", "/norestart"])
            .creation_flags(0x08000000)
            .status()
            .unwrap()
            .success(),
        "VC Redist 运行时安装失败！"
    );
}
