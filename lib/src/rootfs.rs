// SPDX-FileCopyrightText: Timothée Ravier <tim@siosm.fr>
// SPDX-FileCopyrightText: Beñat Gartzia Arruabarrena <bgartzia@redhat.com>
//
// SPDX-License-Identifier: MIT

use std::fs;
use std::io;
use std::path;

const RELATIVE_KERNELS_PATH: &str = "usr/lib/modules/";
const RELATIVE_ESP_OLD: &str = "usr/lib/bootupd/updates/";
// From fcos-44 on shim/grub are stored in different directories
// see https://fedoraproject.org/wiki/Changes/BootLoaderUpdatesPhase1
const RELATIVE_ESP_NEW: &str = "usr/lib/efi";

pub struct RootFSTree {
    esp_path: String,
    kernels_path: String,
}

fn esp_path_absolute(rootfs_path: &path::Path) -> io::Result<path::PathBuf> {
    let temptative = rootfs_path.join(RELATIVE_ESP_NEW);
    match fs::exists(&temptative)? {
        true => Ok(temptative),
        false => Ok(rootfs_path.join(RELATIVE_ESP_OLD)),
    }
}

impl RootFSTree {
    pub fn new(rootfs_path: &str) -> io::Result<RootFSTree> {
        let rootfs_path = path::absolute(rootfs_path)?;
        let kernels_path = rootfs_path.join(RELATIVE_KERNELS_PATH);
        let esp_path = esp_path_absolute(&rootfs_path)?;
        Ok(RootFSTree {
            esp_path: esp_path.to_str().unwrap().into(),
            kernels_path: kernels_path.to_str().unwrap().into(),
        })
    }

    pub fn esp(&self) -> &str {
        self.esp_path.as_str()
    }

    pub fn vmlinuz(&self) -> &str {
        self.kernels_path.as_str()
    }
}
