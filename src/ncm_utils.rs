extern crate winreg;
use std::path::Path;
use std::path::PathBuf;

use anyhow::*;
use pelite::pe32::Pe;
use pelite::pe32::PeFile;
use pelite::FileMap;
use semver::{BuildMetadata, Prerelease, Version};
use winreg::enums::*;
use winreg::RegKey;

pub fn get_ncm_install_path() -> Result<PathBuf> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let path: String = hklm
        .open_subkey("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\App Paths\\cloudmusic.exe")?
        .get_value("")?;
    let path = Path::new(&path);
    if let Some(path) = path.parent() {
        let path = path.to_str().unwrap().to_string();
        Ok(Path::new(&path).to_path_buf())
    } else {
        bail!("Could not find path")
    }
}

pub fn is_vc_redist_14_x86_installed() -> bool {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    hklm.open_subkey("SOFTWARE\\WOW6432Node\\Microsoft\\VisualStudio\\14.0\\VC\\Runtimes\\X86")
        .is_ok()
}

pub fn get_ncm_version() -> Result<Version> {
    let map = FileMap::open(&get_ncm_install_path()?.join("cloudmusic.exe"))?;
    let file = PeFile::from_bytes(&map)?;
    let version = file.resources()?.version_info()?;
    println!("{:#?}", version.file_info());
    Ok(version
        .file_info()
        .fixed
        .map(|f| Version {
            major: f.dwFileVersion.Major as u64,
            minor: f.dwFileVersion.Minor as u64,
            patch: f.dwFileVersion.Patch as u64,
            build: BuildMetadata::EMPTY,
            pre: Prerelease::EMPTY,
        })
        .context("Empty file version")?)
}
