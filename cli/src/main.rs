// SPDX-FileCopyrightText: Timothée Ravier <tim@siosm.fr>
// SPDX-FileCopyrightText: Beñat Gartzia Arruabarrena <bgartzia@redhat.com>
//
// SPDX-License-Identifier: MIT

use std::result::Result::Ok;

use anyhow::Result;
use clap::{Args, Parser, Subcommand};
use log::LevelFilter;
use serde::{Deserialize, Serialize};

use compute_pcrs_lib::*;

#[derive(Parser, Debug)]
#[command(
    version,
    about,
    long_about = "Pre-compute PCR values for Bootable Container systems"
)]
struct Cli {
    /// Log verbosity. Defaults to Warn, -v for Info, -vv for Debug, -vvv for Trace
    #[arg(short = 'v', long, action = clap::ArgAction::Count, global = true)]
    verbose: u8,

    #[command(subcommand)]
    command: Command,
}

#[derive(Args, Debug)]
#[group(required = true, multiple = false)]
struct SecureBootVarStores {
    #[arg(long, help = "Path to the directory storing EFIVar files")]
    efivars: Option<String>,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Compute all possible PCR values from the binaries available in the current environment. Meant to be run inside a Bootable Container.
    All {
        #[arg(
            long,
            short,
            default_value = "/",
            help = "Path to the target container image root filesystem"
        )]
        rootfs: String,
        #[command(flatten)]
        secureboot_variables: SecureBootVarStores,
        #[arg(
            long,
            short,
            default_value = "",
            help = "Path to the UKI binary. It will try to find it in ${rootfs}/boot/EFI/Linux/*.efi by default."
        )]
        uki: String,
        #[arg(
            long,
            help = "Path to a UKI addon relative to the root dir. It can be passed multiple times."
        )]
        uki_addon: Vec<String>,
        #[arg(
            long = "secureboot-disabled",
            default_value_t = false,
            help = "Compute PCRs as if secure boot was disabled in the system"
        )]
        no_secureboot: bool,
        #[arg(
            long = "mok-variables",
            required = true,
            help = "Path to directory storing MokListRT, MokListTrustedRT and MokListXRT"
        )]
        mok_variables: String,
    },
    /// Compute PCR 4
    ///
    /// It will try to find the UKI in the user-provided path. If empty,
    /// it will assume the default Bootable Container UKI path:
    /// ${rootfs}/boot/EFI/Linux/*.efi.
    /// If not found, it will then assume that it is the non UKI case.
    Pcr4 {
        #[arg(
            long,
            short,
            default_value = "/",
            help = "Path to the target container image root filesystem"
        )]
        rootfs: String,
        #[arg(
            long,
            short,
            default_value = "",
            help = "Path to the UKI binary. It will try to find it in ${rootfs}/boot/EFI/Linux/*.efi by default."
        )]
        uki: String,
        #[arg(
            long,
            help = "Path to a UKI addon relative to the root dir. It can be passed multiple times."
        )]
        uki_addon: Vec<String>,
        #[arg(
            long = "secureboot-disabled",
            default_value_t = false,
            help = "Compute PCRs as if secure boot was disabled in the system"
        )]
        no_secureboot: bool,
    },
    /// Compute PCR 7
    Pcr7 {
        #[arg(
            long,
            short,
            default_value = "/",
            help = "Path to the target container image root filesystem"
        )]
        rootfs: String,
        #[command(flatten)]
        secureboot_variables: SecureBootVarStores,
        #[arg(
            long = "secureboot-disabled",
            default_value_t = false,
            help = "Compute PCRs as if secure boot was disabled in the system"
        )]
        no_secureboot: bool,
    },
    /// Compute PCR 11
    Pcr11 {
        /// Path to a UKI
        uki: String,
    },
    /// Compute PCR 14
    Pcr14 {
        #[arg(
            long = "mok-variables",
            required = true,
            help = "Path to directory storing MokListRT, MokListTrustedRT and MokListXRT"
        )]
        mok_variables: String,
    },
}

#[derive(Serialize, Deserialize)]
struct Output {
    pcrs: Vec<Pcr>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let level = match cli.verbose {
        0 => LevelFilter::Warn,
        1 => LevelFilter::Info,
        2 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };

    env_logger::Builder::new()
        .filter(None, level)
        .format_timestamp(None)
        .init();

    match &cli.command {
        Command::All {
            rootfs,
            secureboot_variables,
            uki,
            uki_addon,
            no_secureboot,
            mok_variables,
        } => {
            let rfs = rootfs::RootFSTree::new(rootfs, uki, uki_addon.clone()).unwrap();
            let pcrs = vec![
                compute_pcr4(
                    rfs.vmlinuz(),
                    rfs.esp(),
                    rfs.uki(),
                    rfs.uki_addons(),
                    !no_secureboot,
                ),
                compute_pcr7(
                    secureboot_variables.efivars.as_deref(),
                    rfs.esp(),
                    !no_secureboot,
                ),
                /* compute_pcr11(), */
                compute_pcr14(mok_variables),
            ];
            println!(
                "{}",
                serde_json::to_string_pretty(&Output { pcrs }).unwrap()
            );
            Ok(())
        }
        Command::Pcr4 {
            rootfs,
            uki,
            uki_addon,
            no_secureboot,
        } => {
            log::debug!("{uki_addon:?}");
            let rfs = rootfs::RootFSTree::new(rootfs, uki, uki_addon.clone()).unwrap();
            let pcr = compute_pcr4(
                rfs.vmlinuz(),
                rfs.esp(),
                rfs.uki(),
                rfs.uki_addons(),
                !no_secureboot,
            );

            println!("{}", serde_json::to_string_pretty(&pcr).unwrap());
            Ok(())
        }
        Command::Pcr7 {
            rootfs,
            secureboot_variables,
            no_secureboot,
        } => {
            let rfs = rootfs::RootFSTree::new(rootfs, "", vec![]).unwrap();
            let pcr = compute_pcr7(
                secureboot_variables.efivars.as_deref(),
                rfs.esp(),
                !no_secureboot,
            );
            println!("{}", serde_json::to_string_pretty(&pcr).unwrap());
            Ok(())
        }
        Command::Pcr11 { uki } => {
            let pcr = compute_pcr11(uki);
            println!("{}", serde_json::to_string_pretty(&pcr).unwrap());
            Ok(())
        }
        Command::Pcr14 { mok_variables } => {
            let pcr = compute_pcr14(mok_variables);
            println!("{}", serde_json::to_string_pretty(&pcr).unwrap());
            Ok(())
        }
    }
}
