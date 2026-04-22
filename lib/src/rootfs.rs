// SPDX-FileCopyrightText: Timothée Ravier <tim@siosm.fr>
// SPDX-FileCopyrightText: Beñat Gartzia Arruabarrena <bgartzia@redhat.com>
//
// SPDX-License-Identifier: MIT

use std::fs;
use std::io;
use std::path;

use glob::glob;

const RELATIVE_KERNELS_PATH: &str = "usr/lib/modules/";
const RELATIVE_ESP_OLD: &str = "usr/lib/bootupd/updates/";
// From fcos-44 on shim/grub are stored in different directories
// see https://fedoraproject.org/wiki/Changes/BootLoaderUpdatesPhase1
const RELATIVE_ESP_NEW: &str = "usr/lib/efi";
const RELATIVE_UKI_DISCOVERY_PATTERN: &str = "boot/EFI/Linux/*.efi";

pub struct RootFSTree {
    esp_path: String,
    kernels_path: String,
    uki_path: Option<String>,
    uki_addons: Vec<String>,
}

fn esp_path_absolute(rootfs_path: &path::Path) -> io::Result<path::PathBuf> {
    let temptative = rootfs_path.join(RELATIVE_ESP_NEW);
    match fs::exists(&temptative)? {
        true => Ok(temptative),
        false => Ok(rootfs_path.join(RELATIVE_ESP_OLD)),
    }
}

fn uki_discover(rootfs_path: &path::Path) -> io::Result<Option<String>> {
    let absolute_uki_discovery_pattern = rootfs_path.join(RELATIVE_UKI_DISCOVERY_PATTERN);
    let mut discovered = glob(absolute_uki_discovery_pattern.to_str().unwrap())
        .expect("invalid uki discovery glob pattern")
        .filter_map(Result::ok);

    let temptative = match discovered.next() {
        Some(p) => p,
        None => return Ok(None),
    };

    if discovered.next().is_some() {
        return Err(io::Error::other(
            "found more than 1 .efi during automatic UKI discovery",
        ));
    }

    if fs::exists(&temptative)? {
        return Ok(Some(temptative.to_str().unwrap().into()));
    }
    Ok(None)
}

fn uki_path_absolute(rootfs_path: &path::Path, uki: &str) -> io::Result<Option<String>> {
    let relative_uki_path = if uki.is_empty() {
        return uki_discover(rootfs_path);
    } else {
        uki
    };

    let temptative = rootfs_path.join(relative_uki_path);
    if fs::exists(&temptative)? {
        return Ok(Some(temptative.to_str().unwrap().into()));
    }
    Ok(None)
}

fn uki_addons_absolute(
    rootfs_path: &path::Path,
    uki_addons: Vec<String>,
) -> io::Result<Vec<String>> {
    let mut absolute_addon_paths = vec![];
    for addon in uki_addons.iter() {
        let temptative = rootfs_path.join(addon);
        if !fs::exists(&temptative)? {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Couldn't find uki addon {temptative:?}"),
            ));
        }
        absolute_addon_paths.push(temptative.to_str().unwrap().into())
    }

    Ok(absolute_addon_paths)
}

impl RootFSTree {
    pub fn new(rootfs_path: &str, uki: &str, uki_addons: Vec<String>) -> io::Result<RootFSTree> {
        let rootfs_path = path::absolute(rootfs_path)?;
        let kernels_path = rootfs_path.join(RELATIVE_KERNELS_PATH);
        let esp_path = esp_path_absolute(&rootfs_path)?;

        let uki_path = uki_path_absolute(&rootfs_path, uki)?;
        if uki_path.is_none() && !uki_addons.is_empty() {
            return Err(io::Error::other(
                "uki addons were provided but a valid uki is not found",
            ));
        }
        let uki_addons_paths = uki_addons_absolute(&rootfs_path, uki_addons)?;

        Ok(RootFSTree {
            esp_path: esp_path.to_str().unwrap().into(),
            kernels_path: kernels_path.to_str().unwrap().into(),
            uki_path,
            uki_addons: uki_addons_paths,
        })
    }

    pub fn esp(&self) -> &str {
        self.esp_path.as_str()
    }

    pub fn vmlinuz(&self) -> &str {
        self.kernels_path.as_str()
    }

    pub fn uki(&self) -> Option<&String> {
        self.uki_path.as_ref()
    }

    pub fn uki_addons(&self) -> &Vec<String> {
        self.uki_addons.as_ref()
    }
}
