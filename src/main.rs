#![windows_subsystem = "windows"]
#![feature(path_try_exists)]
extern crate winreg;
use core::fmt;
use std::env;
use std::fmt::Display;
use std::fs;
use std::io::Error;
use std::io::ErrorKind;
use std::path::Path;
use std::process;
use std::process::Command;
use winreg::enums::*;
use winreg::RegKey;

use druid::widget::{Button, Flex, Label, ProgressBar};
use druid::{
    AppLauncher, Data, FontDescriptor, FontWeight, Lens, PlatformError, Widget,
    WidgetExt, WindowDesc,
};

#[derive(Eq, Ord, Clone, Copy, Data, Debug)]
struct Version {
    major: i32,
    minor: i32,
    patch: i32,
}

impl Version {
    fn from_version_string(version: &str) -> Version {
        let mut version_parts = version.split(".");
        let major = version_parts.next().unwrap().parse::<i32>().unwrap();
        let minor = version_parts.next().unwrap().parse::<i32>().unwrap();
        let patch = version_parts.next().unwrap().parse::<i32>().unwrap();
        Version {
            major,
            minor,
            patch,
        }
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl PartialEq for Version {
    fn eq(&self, other: &Version) -> bool {
        self.major == other.major && self.minor == other.minor && self.patch == other.patch
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Version) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Data, Lens)]
struct AppData {
    progress: f64,

    latest_version: Option<Version>,
    local_version: Option<Version>,
    installer_version: Version,

    #[data(eq)]
    tips_string: String,
    #[data(eq)]
    ncm_install_path: Option<String>,
    #[data(eq)]
    latest_download_url: Option<String>,
}

fn config_path() -> String {
    String::from(
        std::env::home_dir()
            .unwrap()
            .as_os_str()
            .to_str()
            .expect("Covert error"),
    ) + "\\betterncm\\"
}

#[tokio::main]
async fn main() -> Result<(), PlatformError> {
    println!("Async main called");
    let main_window = WindowDesc::new(ui_builder())
        .window_size((400., 280.))
        .resizable(false)
        .title("BetterNCM Installer");
    let mut data = AppData {
        progress: 0.,
        latest_version: None,
        local_version: None,
        latest_download_url: None,
        installer_version: Version::from_version_string(env!("CARGO_PKG_VERSION")),
        ncm_install_path: if let Ok(path) = get_ncm_install_path() {
            Some(path)
        } else {
            None
        },
        tips_string: String::new(),
    };
    let launcher = AppLauncher::with_window(main_window);

    let event_sink = launcher.get_external_handle();

    tokio::spawn(async move {
        use serde_json::{Value};
        let client = reqwest::Client::new();
        let releases = client
            .get("https://api.github.com/repos/MicroCBer/BetterNCM/releases/latest")
            .header(
                "User-Agent",
                format!("BetterNCM Installer {};", data.installer_version),
            )
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        let releases: Value = serde_json::from_str(releases.as_str()).unwrap();

        event_sink.add_idle_callback(move |data: &mut AppData| {
            (*data).latest_version = Some(Version::from_version_string(
                releases["tag_name"].as_str().unwrap(),
            ));
            (*data).latest_download_url = Some(
                releases["assets"][0]["browser_download_url"]
                    .as_str()
                    .unwrap()
                    .to_string(),
            );
        });
    });

    get_local_version(&mut data);

    launcher.log_to_console().launch(data)
}

fn get_ncm_localdata_path() -> String{
    let appdata=env::var("APPDATA").unwrap();
    let ncmdata=Path::new(&appdata);
    ncmdata.parent().unwrap().join("Local").join("Netease").join("CloudMusic").to_str().unwrap().to_string()
}

fn set_noproxy_localdata(){
    fs::write(get_ncm_localdata_path()+"/localdata",include_bytes!("localdata/localdata_noproxy")).unwrap();
}

fn set_proxied_localdata(){
    fs::write(get_ncm_localdata_path()+"/localdata",include_bytes!("localdata/localdata_proxied")).unwrap();
}

fn get_local_version(data: &mut AppData) {
    println!("Attempt to get local version...");
    if let Ok(path) = get_ncm_install_path() {
        println!(" Netease cloud music install path:{}",path);
        let path = Path::new(&path);
        if let Ok(p) = std::fs::try_exists(path.join("cloudmusicn.exe").to_str().unwrap()) {
            if p {
                std::fs::remove_file(format!("{}/version.txt", config_path()));
                let _ = std::process::Command::new(path.join("cloudmusic.exe").to_str().unwrap())
                    .arg("--version")
                    .spawn()
                    .unwrap()
                    .wait()
                    .unwrap();
                if let Ok(version) = std::fs::read_to_string(format!("{}/version.txt", config_path())) {
                    data.local_version = Some(Version::from_version_string(&version));
                }else{
                    data.local_version=Some(Version{major:0,minor:1,patch:1});
                }
            } else {
                (*data).local_version = Some(Version::from_version_string("0.0.0"));
            }
        }
    } else {
        (*data).local_version = Some(Version::from_version_string("0.0.0"));
    }
    println!("Succeeded in getting local version!");
}

fn get_ncm_install_path() -> Result<String, std::io::Error> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let path: String = hklm
        .open_subkey("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\App Paths\\cloudmusic.exe")?
        .get_value("")?;
    let path = Path::new(&path);
    if let Some(path) = path.parent() {
        let path = path.to_str().unwrap().to_string();
        Ok(path)
    } else {
        Err(Error::new(ErrorKind::Other, "Could not find path"))
    }
}

fn ui_builder() -> impl Widget<AppData> {
    let title = Label::new(String::from("BetterNCM Installer"))
        .with_font(
            FontDescriptor::default()
                .with_size(30.)
                .with_weight(FontWeight::BOLD),
        )
        .padding(5.0)
        .center();

    let installer_version_label = Flex::row()
        .with_child(Label::new("Installer版本"))
        .with_child(
            Label::new(|data: &AppData, _env: &_| -> String {
                format!("{}", data.installer_version.to_string())
            })
            .with_font(
                FontDescriptor::default()
                    .with_size(20.)
                    .with_weight(FontWeight::SEMI_BOLD),
            ),
        );

    let latest_version_label = Flex::row().with_child(Label::new("最新版本")).with_child(
        Label::new(|data: &AppData, _env: &_| -> String {
            match data.latest_version {
                Some(version) => format!("{}", version.to_string()),
                None => String::from("获取中..."),
            }
        })
        .with_font(
            FontDescriptor::default()
                .with_size(20.)
                .with_weight(FontWeight::SEMI_BOLD),
        ),
    );

    let local_version_label = Flex::row().with_child(Label::new("已安装版本")).with_child(
        Label::new(|data: &AppData, _env: &_| -> String {
            match data.local_version {
                Some(version) => String::from(match version.to_string().as_str() {
                    "0.0.0" => "未安装",
                    o => o,
                }),
                None => String::from("获取中..."),
            }
        })
        .with_font(
            FontDescriptor::default()
                .with_size(20.)
                .with_weight(FontWeight::SEMI_BOLD),
        ),
    );

    let install_path_label = Flex::row()
        .with_child(Label::new("网易云安装路径："))
        .with_child(
            Label::new(|data: &AppData, _env: &_| -> String {
                match data.ncm_install_path.clone() {
                    Some(path) => format!("{}", path.to_string()),
                    None => "未安装".to_string(),
                }
            })
            .with_font(
                FontDescriptor::default()
                    .with_size(10.)
                    .with_weight(FontWeight::THIN),
            ),
        );

    let button_update = Button::new("更新")
        .disabled_if(|data: &AppData, _env: &_| {
            data.latest_version.is_none()
                || data.local_version.is_none()
                || data.local_version.unwrap() >= data.latest_version.unwrap()
                || data.local_version.unwrap().to_string() == "0.0.0"
        })
        .on_click(|_ctx, data, _env| {
            let event_sink = _ctx.get_external_handle();
            let event_sink_getvers = _ctx.get_external_handle();
            let url: String = data.latest_download_url.as_ref().unwrap().clone();
            tokio::spawn(async move {
                fs::remove_file("betterncm.exe");
                download_file(&url, &"betterncm.exe".to_string(), event_sink).await;
                Command::new("taskkill.exe")
                    .args(["/f", "/im", "cloudmusic.exe"])
                    .spawn()
                    .unwrap()
                    .wait()
                    .unwrap();
                
                fs::remove_file(format!(
                    "{}/cloudmusic.exe",
                    get_ncm_install_path().unwrap()
                ))
                .unwrap();
                set_proxied_localdata();
                fs::copy(
                    "betterncm.exe",
                    format!("{}/cloudmusic.exe", get_ncm_install_path().unwrap()),
                )
                .unwrap();
                event_sink_getvers.add_idle_callback(|data| {
                    get_local_version(data);
                });
                Command::new(format!(
                    "{}/cloudmusic.exe",
                    get_ncm_install_path().unwrap()
                ))
                .current_dir(get_ncm_install_path().unwrap())
                .spawn()
            });
        })
        .padding(5.0);

    let button_install = Button::new("安装")
        .disabled_if(|data: &AppData, _env: &_| {
            data.latest_version.is_none()
                || data.local_version.is_none()
                || data.local_version.unwrap().to_string() != "0.0.0"
        })
        .on_click(|ctx, data, _env| {
            let event_sink = ctx.get_external_handle();
            let event_sink_getvers = ctx.get_external_handle();
            let url: String = data.latest_download_url.as_ref().unwrap().clone();
            tokio::spawn(async move {
                tokio::fs::remove_file("betterncm.exe");
                download_file(&url, &"betterncm.exe".to_string(), event_sink).await;
                Command::new("taskkill.exe")
                    .args(["/f", "/im", "cloudmusic.exe"])
                    .spawn()
                    .unwrap()
                    .wait()
                    .unwrap();

                tokio::fs::rename(
                    format!("{}/cloudmusic.exe", get_ncm_install_path().unwrap()),
                    format!("{}/cloudmusicn.exe", get_ncm_install_path().unwrap()),
                ).await
                .unwrap();
                set_proxied_localdata();
                tokio::fs::copy(
                    "betterncm.exe",
                    format!("{}/cloudmusic.exe", get_ncm_install_path().unwrap()),
                ).await
                .unwrap();
                event_sink_getvers.add_idle_callback(|data| {
                    get_local_version(data);
                });
                Command::new(format!(
                    "{}/cloudmusic.exe",
                    get_ncm_install_path().unwrap()
                ))
                .current_dir(get_ncm_install_path().unwrap())
                .spawn()
            });
        })
        .padding(5.0);

    let button_uninstall = Button::new("卸载")
        .disabled_if(|data: &AppData, _env: &_| {
            data.local_version.is_none() || data.local_version.unwrap().to_string() == "0.0.0"
        })
        .on_click(|_ctx, data, _env| {
            fs::remove_dir_all(config_path());
            Command::new("taskkill.exe")
                .args(["/f", "/im", "cloudmusic.exe"])
                .spawn()
                .unwrap()
                .wait();
            Command::new("taskkill.exe")
                .args(["/f", "/im", "cloudmusicn.exe"])
                .spawn()
                .unwrap()
                .wait();
            fs::remove_file(format!(
                "{}/cloudmusic.exe",
                get_ncm_install_path().unwrap()
            ));
            fs::rename(
                format!("{}/cloudmusicn.exe", get_ncm_install_path().unwrap()),
                format!("{}/cloudmusic.exe", get_ncm_install_path().unwrap()),
            );
            set_noproxy_localdata();
            fs::remove_file(format!("{}/betterncm.exe", get_ncm_install_path().unwrap()));
            get_local_version(data);
            process::
            Command::new(format!(
                "{}/cloudmusic.exe",
                get_ncm_install_path().unwrap()
            ))
            .current_dir(get_ncm_install_path().unwrap())
            .spawn();
        })
        .padding(5.0);

    let progress_bar = ProgressBar::new()
        .lens(AppData::progress)
        .padding((20., 0.))
        .expand_width();

    Flex::column()
        .with_child(title)
        .with_child(installer_version_label)
        .with_child(latest_version_label)
        .with_child(local_version_label)
        .with_child(install_path_label)
        .with_child(
            Flex::row()
                .with_child(button_update)
                .with_child(button_install)
                .with_child(button_uninstall),
        )
        .with_child(progress_bar)
        .with_child(Label::new(|data: &AppData, _env: &_| -> String {
            data.tips_string.clone()
        }))
}

async fn download_file(url: &String, path: &String, event_sink: druid::ExtEventSink) {
    let tip_str = format!("正在下载: {}", path).to_string();
    event_sink.add_idle_callback(move |data: &mut AppData| {
        (*data).tips_string = tip_str;
    });
    use std::cmp::min;
    use std::fs::File;
    use std::io::Write;

    use futures_util::StreamExt;
    use reqwest::Client;

    let client = reqwest::Client::new();
    let mut res = client
        .get(url)
        .header(
            "User-Agent",
            format!("BetterNCM Installer {};", env!("CARGO_PKG_VERSION")),
        )
        .send()
        .await
        .unwrap();

    let total_size = res
        .content_length()
        .ok_or(format!("Failed to get content length from '{}'", &url))
        .unwrap();

    let mut file = File::create(path)
        .or(Err(format!("Failed to create file '{}'", path)))
        .unwrap();
    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item
            .or(Err(format!("Error while downloading file")))
            .unwrap();
        file.write_all(&chunk)
            .or(Err(format!("Error while writing to file")))
            .unwrap();
        let new = min(downloaded + (chunk.len() as u64), total_size);
        downloaded = new;
        event_sink.add_idle_callback(move |data: &mut AppData| {
            (*data).progress = (downloaded as f64) / (total_size as f64);
        });
        let tip_str = format!(
            "正在下载: {} ({}/100)",
            path,
            ((downloaded as f64) / (total_size as f64) * 100.).floor()
        )
        .to_string();
        event_sink.add_idle_callback(move |data: &mut AppData| {
            (*data).tips_string = tip_str;
        });
    }
    event_sink.add_idle_callback(move |data: &mut AppData| {
        (*data).tips_string = "".to_string();
        (*data).progress = 0.;
    });
}
