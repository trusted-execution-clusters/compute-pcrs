// SPDX-FileCopyrightText: Timothée Ravier <tim@siosm.fr>
// SPDX-FileCopyrightText: Beñat Gartzia Arruabarrena <bgartzia@redhat.com>
//
// SPDX-License-Identifier: MIT

use crate::pefile;
use glob::glob;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Esp {
    shim: PathBuf,
    grub: PathBuf,
}

fn find_efi_bin(search_path: &Path, bin_name: &str) -> io::Result<PathBuf> {
    let glob_path = search_path.join(Path::new("**/EFI/*/").join(bin_name));
    let glob_pattern = glob_path.to_str().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "Invalid efi bin search pattern",
        )
    })?;

    let search_results = match glob(glob_pattern) {
        Ok(results) => results,
        Err(_) => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid efi bin search pattern",
            ));
        }
    };
    if let Some(path) = search_results.filter_map(Result::ok).next() {
        // Assume there's just one of them; return the first one
        return Ok(path);
    }

    Err(io::Error::new(
        io::ErrorKind::NotFound,
        format!("{bin_name} not found"),
    ))
}

impl Esp {
    pub fn new(path: &str) -> io::Result<Esp> {
        let path_pb = PathBuf::from(path);
        if !fs::metadata(path)?.is_dir() {
            return Err(io::Error::new(io::ErrorKind::NotADirectory, path));
        }

        Ok(Esp {
            grub: find_efi_bin(&path_pb, "grubx64.efi")?,
            shim: find_efi_bin(&path_pb, "shimx64.efi")?,
        })
    }

    /// Tries loading the shim binary
    pub fn shim(&self) -> pefile::PeFile {
        pefile::PeFile::load_from_file(&self.shim.to_string_lossy(), false)
            .expect("Can't open shim binary")
    }

    /// Tries loading the grub binary
    pub fn grub(&self) -> pefile::PeFile {
        pefile::PeFile::load_from_file(&self.grub.to_string_lossy(), false)
            .expect("Can't open grub binary")
    }
}
