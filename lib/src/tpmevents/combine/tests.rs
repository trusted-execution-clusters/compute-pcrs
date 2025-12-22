// SPDX-FileCopyrightText: Beñat Gartzia Arruabarrena <bgartzia@redhat.com>
//
// SPDX-License-Identifier: MIT
use super::*;
use crate::pcrs::{Pcr, compile_pcrs};
use crate::tpmevents::{TPMEvent, TPMEventID};

use hex::decode;
use log::Level;
use std::collections::{HashMap, HashSet};
use testing_logger;

#[test]
fn test_tpm_event_id_hashmap() {
    let foo = TPMEvent {
        name: "FOO".into(),
        pcr: 0x00,
        hash: vec![0, 0, 0],
        id: TPMEventID::PcrRootNodeEvent,
    };
    let bar = TPMEvent {
        name: "BAR".into(),
        pcr: 0xFF,
        hash: vec![4, 5, 6],
        id: TPMEventID::Pcr11Sbat,
    };
    let foobar = TPMEvent {
        name: "FOOBAR".into(),
        pcr: 0xe8,
        hash: vec![1, 2, 3, 4, 5],
        id: TPMEventID::Pcr11UnameContent,
    };
    let events = vec![foo.clone(), bar.clone(), foobar.clone()];

    let res = tpm_event_id_hashmap(&events);
    assert_eq!(
        res,
        HashMap::from([
            (TPMEventID::PcrRootNodeEvent, foo),
            (TPMEventID::Pcr11Sbat, bar),
            (TPMEventID::Pcr11UnameContent, foobar),
        ])
    );
}

#[test]
fn test_pcr4_kernel_update() {
    let this = vec![
        TPMEvent {
            name: "EV_EFI_ACTION".into(),
            pcr: 4,
            hash: decode("3d6772b4f84ed47595d72a2c4c5ffd15f5bb72c7507fe26f2aaee2c69d5633ba")
                .unwrap(),
            id: TPMEventID::Pcr4EfiCall,
        },
        TPMEvent {
            name: "EV_SEPARATOR".into(),
            pcr: 4,
            hash: decode("df3f619804a92fdb4057192dc43dd748ea778adc52bc498ce80524c014b81119")
                .unwrap(),
            id: TPMEventID::Pcr4Separator,
        },
        TPMEvent {
            name: "EV_EFI_BOOT_SERVICES_APPLICATION".into(),
            pcr: 4,
            hash: decode("94896c17d49fc8c8df0cc2836611586edab1615ce7cb58cf13fc5798de56b367")
                .unwrap(),
            id: TPMEventID::Pcr4Shim,
        },
        TPMEvent {
            name: "EV_EFI_BOOT_SERVICES_APPLICATION".into(),
            pcr: 4,
            hash: decode("bc6844fc7b59b4f0c7da70a307fc578465411d7a2c34b0f4dc2cc154c873b644")
                .unwrap(),
            id: TPMEventID::Pcr4Grub,
        },
        TPMEvent {
            name: "EV_EFI_BOOT_SERVICES_APPLICATION".into(),
            pcr: 4,
            hash: decode("2b1dc59bc61dbbc3db11a6f3b0708c948efd46cceb7f6c8ea2024b8d1b8c829a")
                .unwrap(),
            id: TPMEventID::Pcr4Vmlinuz,
        },
    ];
    let that = vec![
        TPMEvent {
            name: "EV_EFI_ACTION".into(),
            pcr: 4,
            hash: decode("3d6772b4f84ed47595d72a2c4c5ffd15f5bb72c7507fe26f2aaee2c69d5633ba")
                .unwrap(),
            id: TPMEventID::Pcr4EfiCall,
        },
        TPMEvent {
            name: "EV_SEPARATOR".into(),
            pcr: 4,
            hash: decode("df3f619804a92fdb4057192dc43dd748ea778adc52bc498ce80524c014b81119")
                .unwrap(),
            id: TPMEventID::Pcr4Separator,
        },
        TPMEvent {
            name: "EV_EFI_BOOT_SERVICES_APPLICATION".into(),
            pcr: 4,
            hash: decode("94896c17d49fc8c8df0cc2836611586edab1615ce7cb58cf13fc5798de56b367")
                .unwrap(),
            id: TPMEventID::Pcr4Shim,
        },
        TPMEvent {
            name: "EV_EFI_BOOT_SERVICES_APPLICATION".into(),
            pcr: 4,
            hash: decode("bc6844fc7b59b4f0c7da70a307fc578465411d7a2c34b0f4dc2cc154c873b644")
                .unwrap(),
            id: TPMEventID::Pcr4Grub,
        },
        TPMEvent {
            name: "EV_EFI_BOOT_SERVICES_APPLICATION".into(),
            pcr: 4,
            hash: decode("6115ef506eecf507d43279d2b5f11573c4011fab0f5bba6e22bb72dbf1d1ffd9")
                .unwrap(),
            id: TPMEventID::Pcr4Vmlinuz,
        },
    ];

    let res = combine(&this, &that);
    assert_eq!(
        res,
        vec![
            vec![Pcr::compile_from(&this)],
            vec![Pcr::compile_from(&that)]
        ]
    );
}

#[test]
fn test_pcr4_pcr7_bootloader_and_kernel_update() {
    let this = vec![
        TPMEvent {
            name: "EV_EFI_ACTION".into(),
            pcr: 4,
            hash: decode("3d6772b4f84ed47595d72a2c4c5ffd15f5bb72c7507fe26f2aaee2c69d5633ba")
                .unwrap(),
            id: TPMEventID::Pcr4EfiCall,
        },
        TPMEvent {
            name: "EV_SEPARATOR".into(),
            pcr: 4,
            hash: decode("df3f619804a92fdb4057192dc43dd748ea778adc52bc498ce80524c014b81119")
                .unwrap(),
            id: TPMEventID::Pcr4Separator,
        },
        TPMEvent {
            name: "EV_EFI_BOOT_SERVICES_APPLICATION".into(),
            pcr: 4,
            hash: decode("94896c17d49fc8c8df0cc2836611586edab1615ce7cb58cf13fc5798de56b367")
                .unwrap(),
            id: TPMEventID::Pcr4Shim,
        },
        TPMEvent {
            name: "EV_EFI_BOOT_SERVICES_APPLICATION".into(),
            pcr: 4,
            hash: decode("bc6844fc7b59b4f0c7da70a307fc578465411d7a2c34b0f4dc2cc154c873b644")
                .unwrap(),
            id: TPMEventID::Pcr4Grub,
        },
        TPMEvent {
            name: "EV_EFI_BOOT_SERVICES_APPLICATION".into(),
            pcr: 4,
            hash: decode("2b1dc59bc61dbbc3db11a6f3b0708c948efd46cceb7f6c8ea2024b8d1b8c829a")
                .unwrap(),
            id: TPMEventID::Pcr4Vmlinuz,
        },
        TPMEvent {
            name: "EV_EFI_VARIABLE_DRIVER_CONFIG".into(),
            pcr: 7,
            hash: decode("ccfc4bb32888a345bc8aeadaba552b627d99348c767681ab3141f5b01e40a40e")
                .unwrap(),
            id: TPMEventID::Pcr7SecureBoot,
        },
        TPMEvent {
            name: "EV_EFI_VARIABLE_DRIVER_CONFIG".into(),
            pcr: 7,
            hash: decode("adb6fc232943e39c374bf4782b6c697f43c39fca1f4b51dfceda21164e19a893")
                .unwrap(),
            id: TPMEventID::Pcr7Pk,
        },
        TPMEvent {
            name: "EV_EFI_VARIABLE_DRIVER_CONFIG".into(),
            pcr: 7,
            hash: decode("b5432fe20c624811cb0296391bfdf948ebd02f0705ab8229bea09774023f0ebf")
                .unwrap(),
            id: TPMEventID::Pcr7Kek,
        },
        TPMEvent {
            name: "EV_EFI_VARIABLE_DRIVER_CONFIG".into(),
            pcr: 7,
            hash: decode("4313e43de720194a0eabf4d6415d42b5a03a34fdc47bb1fc924cc4e665e6893d")
                .unwrap(),
            id: TPMEventID::Pcr7Db,
        },
        TPMEvent {
            name: "EV_EFI_VARIABLE_DRIVER_CONFIG".into(),
            pcr: 7,
            hash: decode("001004ba58a184f09be6c1f4ec75a246cc2eefa9637b48ee428b6aa9bce48c55")
                .unwrap(),
            id: TPMEventID::Pcr7Dbx,
        },
        TPMEvent {
            name: "EV_SEPARATOR".into(),
            pcr: 7,
            hash: decode("df3f619804a92fdb4057192dc43dd748ea778adc52bc498ce80524c014b81119")
                .unwrap(),
            id: TPMEventID::Pcr7Separator,
        },
        TPMEvent {
            name: "EV_EFI_VARIABLE_AUTHORITY".into(),
            pcr: 7,
            hash: decode("4d4a8e2c74133bbdc01a16eaf2dbb5d575afeb36f5d8dfcf609ae043909e2ee9")
                .unwrap(),
            id: TPMEventID::Pcr7ShimCert,
        },
        TPMEvent {
            name: "EV_EFI_VARIABLE_AUTHORITY".into(),
            pcr: 7,
            hash: decode("e8e9578f5951ef16b1c1aa18ef02944b8375ec45ed4b5d8cdb30428db4a31016")
                .unwrap(),
            id: TPMEventID::Pcr7SbatLevel,
        },
        TPMEvent {
            name: "EV_EFI_VARIABLE_AUTHORITY".into(),
            pcr: 7,
            hash: decode("ad5901fd581e6640c742c488083b9ac2c48255bd28a16c106c6f9df52702ee3f")
                .unwrap(),
            id: TPMEventID::Pcr7GrubVendorDbCert,
        },
    ];
    let that = vec![
        TPMEvent {
            name: "EV_EFI_ACTION".into(),
            pcr: 4,
            hash: decode("3d6772b4f84ed47595d72a2c4c5ffd15f5bb72c7507fe26f2aaee2c69d5633ba")
                .unwrap(),
            id: TPMEventID::Pcr4EfiCall,
        },
        TPMEvent {
            name: "EV_SEPARATOR".into(),
            pcr: 4,
            hash: decode("df3f619804a92fdb4057192dc43dd748ea778adc52bc498ce80524c014b81119")
                .unwrap(),
            id: TPMEventID::Pcr4Separator,
        },
        TPMEvent {
            name: "EV_EFI_BOOT_SERVICES_APPLICATION".into(),
            pcr: 4,
            hash: decode("5ad8a618488664b5a909f08262a28354181e39ce9fc8c8df0cc2836611586eda")
                .unwrap(),
            id: TPMEventID::Pcr4Shim,
        },
        TPMEvent {
            name: "EV_EFI_BOOT_SERVICES_APPLICATION".into(),
            pcr: 4,
            hash: decode("5f77e2690dab289917fe18116ed779941c32d316883d25f2e21ccd392926bf48")
                .unwrap(),
            id: TPMEventID::Pcr4Grub,
        },
        TPMEvent {
            name: "EV_EFI_BOOT_SERVICES_APPLICATION".into(),
            pcr: 4,
            hash: decode("6115ef506eecf507d43279d2b5f11573c4011fab0f5bba6e22bb72dbf1d1ffd9")
                .unwrap(),
            id: TPMEventID::Pcr4Vmlinuz,
        },
        TPMEvent {
            name: "EV_EFI_VARIABLE_DRIVER_CONFIG".into(),
            pcr: 7,
            hash: decode("ccfc4bb32888a345bc8aeadaba552b627d99348c767681ab3141f5b01e40a40e")
                .unwrap(),
            id: TPMEventID::Pcr7SecureBoot,
        },
        TPMEvent {
            name: "EV_EFI_VARIABLE_DRIVER_CONFIG".into(),
            pcr: 7,
            hash: decode("adb6fc232943e39c374bf4782b6c697f43c39fca1f4b51dfceda21164e19a893")
                .unwrap(),
            id: TPMEventID::Pcr7Pk,
        },
        TPMEvent {
            name: "EV_EFI_VARIABLE_DRIVER_CONFIG".into(),
            pcr: 7,
            hash: decode("b5432fe20c624811cb0296391bfdf948ebd02f0705ab8229bea09774023f0ebf")
                .unwrap(),
            id: TPMEventID::Pcr7Kek,
        },
        TPMEvent {
            name: "EV_EFI_VARIABLE_DRIVER_CONFIG".into(),
            pcr: 7,
            hash: decode("4313e43de720194a0eabf4d6415d42b5a03a34fdc47bb1fc924cc4e665e6893d")
                .unwrap(),
            id: TPMEventID::Pcr7Db,
        },
        TPMEvent {
            name: "EV_EFI_VARIABLE_DRIVER_CONFIG".into(),
            pcr: 7,
            hash: decode("001004ba58a184f09be6c1f4ec75a246cc2eefa9637b48ee428b6aa9bce48c55")
                .unwrap(),
            id: TPMEventID::Pcr7Dbx,
        },
        TPMEvent {
            name: "EV_SEPARATOR".into(),
            pcr: 7,
            hash: decode("df3f619804a92fdb4057192dc43dd748ea778adc52bc498ce80524c014b81119")
                .unwrap(),
            id: TPMEventID::Pcr7Separator,
        },
        TPMEvent {
            name: "EV_EFI_VARIABLE_AUTHORITY".into(),
            pcr: 7,
            hash: decode("7b052cea168123d110d99d617a4a0d2723562a10909578c4b739afe245cc3903")
                .unwrap(),
            id: TPMEventID::Pcr7ShimCert,
        },
        TPMEvent {
            name: "EV_EFI_VARIABLE_AUTHORITY".into(),
            pcr: 7,
            hash: decode("e8e9578f5951ef16b1c1aa18ef02944b8375ec45ed4b5d8cdb30428db4a31016")
                .unwrap(),
            id: TPMEventID::Pcr7SbatLevel,
        },
        TPMEvent {
            name: "EV_EFI_VARIABLE_AUTHORITY".into(),
            pcr: 7,
            hash: decode("c3ab8ff13720e8ad9047dd39466b3c8974e592c2fa383d4a3960714caef0c4f2")
                .unwrap(),
            id: TPMEventID::Pcr7GrubVendorDbCert,
        },
    ];

    let expected_this_kernel_that_bootloader = vec![
        TPMEvent {
            name: "EV_EFI_ACTION".into(),
            pcr: 4,
            hash: decode("3d6772b4f84ed47595d72a2c4c5ffd15f5bb72c7507fe26f2aaee2c69d5633ba")
                .unwrap(),
            id: TPMEventID::Pcr4EfiCall,
        },
        TPMEvent {
            name: "EV_SEPARATOR".into(),
            pcr: 4,
            hash: decode("df3f619804a92fdb4057192dc43dd748ea778adc52bc498ce80524c014b81119")
                .unwrap(),
            id: TPMEventID::Pcr4Separator,
        },
        TPMEvent {
            name: "EV_EFI_BOOT_SERVICES_APPLICATION".into(),
            pcr: 4,
            hash: decode("5ad8a618488664b5a909f08262a28354181e39ce9fc8c8df0cc2836611586eda")
                .unwrap(),
            id: TPMEventID::Pcr4Shim,
        },
        TPMEvent {
            name: "EV_EFI_BOOT_SERVICES_APPLICATION".into(),
            pcr: 4,
            hash: decode("5f77e2690dab289917fe18116ed779941c32d316883d25f2e21ccd392926bf48")
                .unwrap(),
            id: TPMEventID::Pcr4Grub,
        },
        TPMEvent {
            name: "EV_EFI_BOOT_SERVICES_APPLICATION".into(),
            pcr: 4,
            hash: decode("2b1dc59bc61dbbc3db11a6f3b0708c948efd46cceb7f6c8ea2024b8d1b8c829a")
                .unwrap(),
            id: TPMEventID::Pcr4Vmlinuz,
        },
        TPMEvent {
            name: "EV_EFI_VARIABLE_DRIVER_CONFIG".into(),
            pcr: 7,
            hash: decode("ccfc4bb32888a345bc8aeadaba552b627d99348c767681ab3141f5b01e40a40e")
                .unwrap(),
            id: TPMEventID::Pcr7SecureBoot,
        },
        TPMEvent {
            name: "EV_EFI_VARIABLE_DRIVER_CONFIG".into(),
            pcr: 7,
            hash: decode("adb6fc232943e39c374bf4782b6c697f43c39fca1f4b51dfceda21164e19a893")
                .unwrap(),
            id: TPMEventID::Pcr7Pk,
        },
        TPMEvent {
            name: "EV_EFI_VARIABLE_DRIVER_CONFIG".into(),
            pcr: 7,
            hash: decode("b5432fe20c624811cb0296391bfdf948ebd02f0705ab8229bea09774023f0ebf")
                .unwrap(),
            id: TPMEventID::Pcr7Kek,
        },
        TPMEvent {
            name: "EV_EFI_VARIABLE_DRIVER_CONFIG".into(),
            pcr: 7,
            hash: decode("4313e43de720194a0eabf4d6415d42b5a03a34fdc47bb1fc924cc4e665e6893d")
                .unwrap(),
            id: TPMEventID::Pcr7Db,
        },
        TPMEvent {
            name: "EV_EFI_VARIABLE_DRIVER_CONFIG".into(),
            pcr: 7,
            hash: decode("001004ba58a184f09be6c1f4ec75a246cc2eefa9637b48ee428b6aa9bce48c55")
                .unwrap(),
            id: TPMEventID::Pcr7Dbx,
        },
        TPMEvent {
            name: "EV_SEPARATOR".into(),
            pcr: 7,
            hash: decode("df3f619804a92fdb4057192dc43dd748ea778adc52bc498ce80524c014b81119")
                .unwrap(),
            id: TPMEventID::Pcr7Separator,
        },
        TPMEvent {
            name: "EV_EFI_VARIABLE_AUTHORITY".into(),
            pcr: 7,
            hash: decode("7b052cea168123d110d99d617a4a0d2723562a10909578c4b739afe245cc3903")
                .unwrap(),
            id: TPMEventID::Pcr7ShimCert,
        },
        TPMEvent {
            name: "EV_EFI_VARIABLE_AUTHORITY".into(),
            pcr: 7,
            hash: decode("e8e9578f5951ef16b1c1aa18ef02944b8375ec45ed4b5d8cdb30428db4a31016")
                .unwrap(),
            id: TPMEventID::Pcr7SbatLevel,
        },
        TPMEvent {
            name: "EV_EFI_VARIABLE_AUTHORITY".into(),
            pcr: 7,
            hash: decode("c3ab8ff13720e8ad9047dd39466b3c8974e592c2fa383d4a3960714caef0c4f2")
                .unwrap(),
            id: TPMEventID::Pcr7GrubVendorDbCert,
        },
    ];

    let expected_that_kernel_this_bootloader = vec![
        TPMEvent {
            name: "EV_EFI_ACTION".into(),
            pcr: 4,
            hash: decode("3d6772b4f84ed47595d72a2c4c5ffd15f5bb72c7507fe26f2aaee2c69d5633ba")
                .unwrap(),
            id: TPMEventID::Pcr4EfiCall,
        },
        TPMEvent {
            name: "EV_SEPARATOR".into(),
            pcr: 4,
            hash: decode("df3f619804a92fdb4057192dc43dd748ea778adc52bc498ce80524c014b81119")
                .unwrap(),
            id: TPMEventID::Pcr4Separator,
        },
        TPMEvent {
            name: "EV_EFI_BOOT_SERVICES_APPLICATION".into(),
            pcr: 4,
            hash: decode("94896c17d49fc8c8df0cc2836611586edab1615ce7cb58cf13fc5798de56b367")
                .unwrap(),
            id: TPMEventID::Pcr4Shim,
        },
        TPMEvent {
            name: "EV_EFI_BOOT_SERVICES_APPLICATION".into(),
            pcr: 4,
            hash: decode("bc6844fc7b59b4f0c7da70a307fc578465411d7a2c34b0f4dc2cc154c873b644")
                .unwrap(),
            id: TPMEventID::Pcr4Grub,
        },
        TPMEvent {
            name: "EV_EFI_BOOT_SERVICES_APPLICATION".into(),
            pcr: 4,
            hash: decode("6115ef506eecf507d43279d2b5f11573c4011fab0f5bba6e22bb72dbf1d1ffd9")
                .unwrap(),
            id: TPMEventID::Pcr4Vmlinuz,
        },
        TPMEvent {
            name: "EV_EFI_VARIABLE_DRIVER_CONFIG".into(),
            pcr: 7,
            hash: decode("ccfc4bb32888a345bc8aeadaba552b627d99348c767681ab3141f5b01e40a40e")
                .unwrap(),
            id: TPMEventID::Pcr7SecureBoot,
        },
        TPMEvent {
            name: "EV_EFI_VARIABLE_DRIVER_CONFIG".into(),
            pcr: 7,
            hash: decode("adb6fc232943e39c374bf4782b6c697f43c39fca1f4b51dfceda21164e19a893")
                .unwrap(),
            id: TPMEventID::Pcr7Pk,
        },
        TPMEvent {
            name: "EV_EFI_VARIABLE_DRIVER_CONFIG".into(),
            pcr: 7,
            hash: decode("b5432fe20c624811cb0296391bfdf948ebd02f0705ab8229bea09774023f0ebf")
                .unwrap(),
            id: TPMEventID::Pcr7Kek,
        },
        TPMEvent {
            name: "EV_EFI_VARIABLE_DRIVER_CONFIG".into(),
            pcr: 7,
            hash: decode("4313e43de720194a0eabf4d6415d42b5a03a34fdc47bb1fc924cc4e665e6893d")
                .unwrap(),
            id: TPMEventID::Pcr7Db,
        },
        TPMEvent {
            name: "EV_EFI_VARIABLE_DRIVER_CONFIG".into(),
            pcr: 7,
            hash: decode("001004ba58a184f09be6c1f4ec75a246cc2eefa9637b48ee428b6aa9bce48c55")
                .unwrap(),
            id: TPMEventID::Pcr7Dbx,
        },
        TPMEvent {
            name: "EV_SEPARATOR".into(),
            pcr: 7,
            hash: decode("df3f619804a92fdb4057192dc43dd748ea778adc52bc498ce80524c014b81119")
                .unwrap(),
            id: TPMEventID::Pcr7Separator,
        },
        TPMEvent {
            name: "EV_EFI_VARIABLE_AUTHORITY".into(),
            pcr: 7,
            hash: decode("4d4a8e2c74133bbdc01a16eaf2dbb5d575afeb36f5d8dfcf609ae043909e2ee9")
                .unwrap(),
            id: TPMEventID::Pcr7ShimCert,
        },
        TPMEvent {
            name: "EV_EFI_VARIABLE_AUTHORITY".into(),
            pcr: 7,
            hash: decode("e8e9578f5951ef16b1c1aa18ef02944b8375ec45ed4b5d8cdb30428db4a31016")
                .unwrap(),
            id: TPMEventID::Pcr7SbatLevel,
        },
        TPMEvent {
            name: "EV_EFI_VARIABLE_AUTHORITY".into(),
            pcr: 7,
            hash: decode("ad5901fd581e6640c742c488083b9ac2c48255bd28a16c106c6f9df52702ee3f")
                .unwrap(),
            id: TPMEventID::Pcr7GrubVendorDbCert,
        },
    ];

    let res = combine(&this, &that);
    let expected = vec![
        compile_pcrs(&this),
        compile_pcrs(&expected_that_kernel_this_bootloader),
        compile_pcrs(&expected_this_kernel_that_bootloader),
        compile_pcrs(&that),
    ];

    assert_eq!(res, expected);
}

#[test]
fn test_pcr4_pcr7_bootloader_and_kernel_update_same_certs() {
    let this = vec![
        TPMEvent {
            name: "EV_EFI_ACTION".into(),
            pcr: 4,
            hash: decode("3d6772b4f84ed47595d72a2c4c5ffd15f5bb72c7507fe26f2aaee2c69d5633ba")
                .unwrap(),
            id: TPMEventID::Pcr4EfiCall,
        },
        TPMEvent {
            name: "EV_SEPARATOR".into(),
            pcr: 4,
            hash: decode("df3f619804a92fdb4057192dc43dd748ea778adc52bc498ce80524c014b81119")
                .unwrap(),
            id: TPMEventID::Pcr4Separator,
        },
        TPMEvent {
            name: "EV_EFI_BOOT_SERVICES_APPLICATION".into(),
            pcr: 4,
            hash: decode("94896c17d49fc8c8df0cc2836611586edab1615ce7cb58cf13fc5798de56b367")
                .unwrap(),
            id: TPMEventID::Pcr4Shim,
        },
        TPMEvent {
            name: "EV_EFI_BOOT_SERVICES_APPLICATION".into(),
            pcr: 4,
            hash: decode("bc6844fc7b59b4f0c7da70a307fc578465411d7a2c34b0f4dc2cc154c873b644")
                .unwrap(),
            id: TPMEventID::Pcr4Grub,
        },
        TPMEvent {
            name: "EV_EFI_BOOT_SERVICES_APPLICATION".into(),
            pcr: 4,
            hash: decode("2b1dc59bc61dbbc3db11a6f3b0708c948efd46cceb7f6c8ea2024b8d1b8c829a")
                .unwrap(),
            id: TPMEventID::Pcr4Vmlinuz,
        },
        TPMEvent {
            name: "EV_EFI_VARIABLE_DRIVER_CONFIG".into(),
            pcr: 7,
            hash: decode("ccfc4bb32888a345bc8aeadaba552b627d99348c767681ab3141f5b01e40a40e")
                .unwrap(),
            id: TPMEventID::Pcr7SecureBoot,
        },
        TPMEvent {
            name: "EV_EFI_VARIABLE_DRIVER_CONFIG".into(),
            pcr: 7,
            hash: decode("adb6fc232943e39c374bf4782b6c697f43c39fca1f4b51dfceda21164e19a893")
                .unwrap(),
            id: TPMEventID::Pcr7Pk,
        },
        TPMEvent {
            name: "EV_EFI_VARIABLE_DRIVER_CONFIG".into(),
            pcr: 7,
            hash: decode("b5432fe20c624811cb0296391bfdf948ebd02f0705ab8229bea09774023f0ebf")
                .unwrap(),
            id: TPMEventID::Pcr7Kek,
        },
        TPMEvent {
            name: "EV_EFI_VARIABLE_DRIVER_CONFIG".into(),
            pcr: 7,
            hash: decode("4313e43de720194a0eabf4d6415d42b5a03a34fdc47bb1fc924cc4e665e6893d")
                .unwrap(),
            id: TPMEventID::Pcr7Db,
        },
        TPMEvent {
            name: "EV_EFI_VARIABLE_DRIVER_CONFIG".into(),
            pcr: 7,
            hash: decode("001004ba58a184f09be6c1f4ec75a246cc2eefa9637b48ee428b6aa9bce48c55")
                .unwrap(),
            id: TPMEventID::Pcr7Dbx,
        },
        TPMEvent {
            name: "EV_SEPARATOR".into(),
            pcr: 7,
            hash: decode("df3f619804a92fdb4057192dc43dd748ea778adc52bc498ce80524c014b81119")
                .unwrap(),
            id: TPMEventID::Pcr7Separator,
        },
        TPMEvent {
            name: "EV_EFI_VARIABLE_AUTHORITY".into(),
            pcr: 7,
            hash: decode("4d4a8e2c74133bbdc01a16eaf2dbb5d575afeb36f5d8dfcf609ae043909e2ee9")
                .unwrap(),
            id: TPMEventID::Pcr7ShimCert,
        },
        TPMEvent {
            name: "EV_EFI_VARIABLE_AUTHORITY".into(),
            pcr: 7,
            hash: decode("e8e9578f5951ef16b1c1aa18ef02944b8375ec45ed4b5d8cdb30428db4a31016")
                .unwrap(),
            id: TPMEventID::Pcr7SbatLevel,
        },
        TPMEvent {
            name: "EV_EFI_VARIABLE_AUTHORITY".into(),
            pcr: 7,
            hash: decode("ad5901fd581e6640c742c488083b9ac2c48255bd28a16c106c6f9df52702ee3f")
                .unwrap(),
            id: TPMEventID::Pcr7GrubVendorDbCert,
        },
    ];
    let that = vec![
        TPMEvent {
            name: "EV_EFI_ACTION".into(),
            pcr: 4,
            hash: decode("3d6772b4f84ed47595d72a2c4c5ffd15f5bb72c7507fe26f2aaee2c69d5633ba")
                .unwrap(),
            id: TPMEventID::Pcr4EfiCall,
        },
        TPMEvent {
            name: "EV_SEPARATOR".into(),
            pcr: 4,
            hash: decode("df3f619804a92fdb4057192dc43dd748ea778adc52bc498ce80524c014b81119")
                .unwrap(),
            id: TPMEventID::Pcr4Separator,
        },
        TPMEvent {
            name: "EV_EFI_BOOT_SERVICES_APPLICATION".into(),
            pcr: 4,
            hash: decode("5ad8a618488664b5a909f08262a28354181e39ce9fc8c8df0cc2836611586eda")
                .unwrap(),
            id: TPMEventID::Pcr4Shim,
        },
        TPMEvent {
            name: "EV_EFI_BOOT_SERVICES_APPLICATION".into(),
            pcr: 4,
            hash: decode("5f77e2690dab289917fe18116ed779941c32d316883d25f2e21ccd392926bf48")
                .unwrap(),
            id: TPMEventID::Pcr4Grub,
        },
        TPMEvent {
            name: "EV_EFI_BOOT_SERVICES_APPLICATION".into(),
            pcr: 4,
            hash: decode("6115ef506eecf507d43279d2b5f11573c4011fab0f5bba6e22bb72dbf1d1ffd9")
                .unwrap(),
            id: TPMEventID::Pcr4Vmlinuz,
        },
        TPMEvent {
            name: "EV_EFI_VARIABLE_DRIVER_CONFIG".into(),
            pcr: 7,
            hash: decode("ccfc4bb32888a345bc8aeadaba552b627d99348c767681ab3141f5b01e40a40e")
                .unwrap(),
            id: TPMEventID::Pcr7SecureBoot,
        },
        TPMEvent {
            name: "EV_EFI_VARIABLE_DRIVER_CONFIG".into(),
            pcr: 7,
            hash: decode("adb6fc232943e39c374bf4782b6c697f43c39fca1f4b51dfceda21164e19a893")
                .unwrap(),
            id: TPMEventID::Pcr7Pk,
        },
        TPMEvent {
            name: "EV_EFI_VARIABLE_DRIVER_CONFIG".into(),
            pcr: 7,
            hash: decode("b5432fe20c624811cb0296391bfdf948ebd02f0705ab8229bea09774023f0ebf")
                .unwrap(),
            id: TPMEventID::Pcr7Kek,
        },
        TPMEvent {
            name: "EV_EFI_VARIABLE_DRIVER_CONFIG".into(),
            pcr: 7,
            hash: decode("4313e43de720194a0eabf4d6415d42b5a03a34fdc47bb1fc924cc4e665e6893d")
                .unwrap(),
            id: TPMEventID::Pcr7Db,
        },
        TPMEvent {
            name: "EV_EFI_VARIABLE_DRIVER_CONFIG".into(),
            pcr: 7,
            hash: decode("001004ba58a184f09be6c1f4ec75a246cc2eefa9637b48ee428b6aa9bce48c55")
                .unwrap(),
            id: TPMEventID::Pcr7Dbx,
        },
        TPMEvent {
            name: "EV_SEPARATOR".into(),
            pcr: 7,
            hash: decode("df3f619804a92fdb4057192dc43dd748ea778adc52bc498ce80524c014b81119")
                .unwrap(),
            id: TPMEventID::Pcr7Separator,
        },
        TPMEvent {
            name: "EV_EFI_VARIABLE_AUTHORITY".into(),
            pcr: 7,
            hash: decode("4d4a8e2c74133bbdc01a16eaf2dbb5d575afeb36f5d8dfcf609ae043909e2ee9")
                .unwrap(),
            id: TPMEventID::Pcr7ShimCert,
        },
        TPMEvent {
            name: "EV_EFI_VARIABLE_AUTHORITY".into(),
            pcr: 7,
            hash: decode("e8e9578f5951ef16b1c1aa18ef02944b8375ec45ed4b5d8cdb30428db4a31016")
                .unwrap(),
            id: TPMEventID::Pcr7SbatLevel,
        },
        TPMEvent {
            name: "EV_EFI_VARIABLE_AUTHORITY".into(),
            pcr: 7,
            hash: decode("ad5901fd581e6640c742c488083b9ac2c48255bd28a16c106c6f9df52702ee3f")
                .unwrap(),
            id: TPMEventID::Pcr7GrubVendorDbCert,
        },
    ];

    let expected_this_kernel_that_bootloader = vec![
        TPMEvent {
            name: "EV_EFI_ACTION".into(),
            pcr: 4,
            hash: decode("3d6772b4f84ed47595d72a2c4c5ffd15f5bb72c7507fe26f2aaee2c69d5633ba")
                .unwrap(),
            id: TPMEventID::Pcr4EfiCall,
        },
        TPMEvent {
            name: "EV_SEPARATOR".into(),
            pcr: 4,
            hash: decode("df3f619804a92fdb4057192dc43dd748ea778adc52bc498ce80524c014b81119")
                .unwrap(),
            id: TPMEventID::Pcr4Separator,
        },
        TPMEvent {
            name: "EV_EFI_BOOT_SERVICES_APPLICATION".into(),
            pcr: 4,
            hash: decode("5ad8a618488664b5a909f08262a28354181e39ce9fc8c8df0cc2836611586eda")
                .unwrap(),
            id: TPMEventID::Pcr4Shim,
        },
        TPMEvent {
            name: "EV_EFI_BOOT_SERVICES_APPLICATION".into(),
            pcr: 4,
            hash: decode("5f77e2690dab289917fe18116ed779941c32d316883d25f2e21ccd392926bf48")
                .unwrap(),
            id: TPMEventID::Pcr4Grub,
        },
        TPMEvent {
            name: "EV_EFI_BOOT_SERVICES_APPLICATION".into(),
            pcr: 4,
            hash: decode("2b1dc59bc61dbbc3db11a6f3b0708c948efd46cceb7f6c8ea2024b8d1b8c829a")
                .unwrap(),
            id: TPMEventID::Pcr4Vmlinuz,
        },
        TPMEvent {
            name: "EV_EFI_VARIABLE_DRIVER_CONFIG".into(),
            pcr: 7,
            hash: decode("ccfc4bb32888a345bc8aeadaba552b627d99348c767681ab3141f5b01e40a40e")
                .unwrap(),
            id: TPMEventID::Pcr7SecureBoot,
        },
        TPMEvent {
            name: "EV_EFI_VARIABLE_DRIVER_CONFIG".into(),
            pcr: 7,
            hash: decode("adb6fc232943e39c374bf4782b6c697f43c39fca1f4b51dfceda21164e19a893")
                .unwrap(),
            id: TPMEventID::Pcr7Pk,
        },
        TPMEvent {
            name: "EV_EFI_VARIABLE_DRIVER_CONFIG".into(),
            pcr: 7,
            hash: decode("b5432fe20c624811cb0296391bfdf948ebd02f0705ab8229bea09774023f0ebf")
                .unwrap(),
            id: TPMEventID::Pcr7Kek,
        },
        TPMEvent {
            name: "EV_EFI_VARIABLE_DRIVER_CONFIG".into(),
            pcr: 7,
            hash: decode("4313e43de720194a0eabf4d6415d42b5a03a34fdc47bb1fc924cc4e665e6893d")
                .unwrap(),
            id: TPMEventID::Pcr7Db,
        },
        TPMEvent {
            name: "EV_EFI_VARIABLE_DRIVER_CONFIG".into(),
            pcr: 7,
            hash: decode("001004ba58a184f09be6c1f4ec75a246cc2eefa9637b48ee428b6aa9bce48c55")
                .unwrap(),
            id: TPMEventID::Pcr7Dbx,
        },
        TPMEvent {
            name: "EV_SEPARATOR".into(),
            pcr: 7,
            hash: decode("df3f619804a92fdb4057192dc43dd748ea778adc52bc498ce80524c014b81119")
                .unwrap(),
            id: TPMEventID::Pcr7Separator,
        },
        TPMEvent {
            name: "EV_EFI_VARIABLE_AUTHORITY".into(),
            pcr: 7,
            hash: decode("4d4a8e2c74133bbdc01a16eaf2dbb5d575afeb36f5d8dfcf609ae043909e2ee9")
                .unwrap(),
            id: TPMEventID::Pcr7ShimCert,
        },
        TPMEvent {
            name: "EV_EFI_VARIABLE_AUTHORITY".into(),
            pcr: 7,
            hash: decode("e8e9578f5951ef16b1c1aa18ef02944b8375ec45ed4b5d8cdb30428db4a31016")
                .unwrap(),
            id: TPMEventID::Pcr7SbatLevel,
        },
        TPMEvent {
            name: "EV_EFI_VARIABLE_AUTHORITY".into(),
            pcr: 7,
            hash: decode("ad5901fd581e6640c742c488083b9ac2c48255bd28a16c106c6f9df52702ee3f")
                .unwrap(),
            id: TPMEventID::Pcr7GrubVendorDbCert,
        },
    ];

    let expected_that_kernel_this_bootloader = vec![
        TPMEvent {
            name: "EV_EFI_ACTION".into(),
            pcr: 4,
            hash: decode("3d6772b4f84ed47595d72a2c4c5ffd15f5bb72c7507fe26f2aaee2c69d5633ba")
                .unwrap(),
            id: TPMEventID::Pcr4EfiCall,
        },
        TPMEvent {
            name: "EV_SEPARATOR".into(),
            pcr: 4,
            hash: decode("df3f619804a92fdb4057192dc43dd748ea778adc52bc498ce80524c014b81119")
                .unwrap(),
            id: TPMEventID::Pcr4Separator,
        },
        TPMEvent {
            name: "EV_EFI_BOOT_SERVICES_APPLICATION".into(),
            pcr: 4,
            hash: decode("94896c17d49fc8c8df0cc2836611586edab1615ce7cb58cf13fc5798de56b367")
                .unwrap(),
            id: TPMEventID::Pcr4Shim,
        },
        TPMEvent {
            name: "EV_EFI_BOOT_SERVICES_APPLICATION".into(),
            pcr: 4,
            hash: decode("bc6844fc7b59b4f0c7da70a307fc578465411d7a2c34b0f4dc2cc154c873b644")
                .unwrap(),
            id: TPMEventID::Pcr4Grub,
        },
        TPMEvent {
            name: "EV_EFI_BOOT_SERVICES_APPLICATION".into(),
            pcr: 4,
            hash: decode("6115ef506eecf507d43279d2b5f11573c4011fab0f5bba6e22bb72dbf1d1ffd9")
                .unwrap(),
            id: TPMEventID::Pcr4Vmlinuz,
        },
        TPMEvent {
            name: "EV_EFI_VARIABLE_DRIVER_CONFIG".into(),
            pcr: 7,
            hash: decode("ccfc4bb32888a345bc8aeadaba552b627d99348c767681ab3141f5b01e40a40e")
                .unwrap(),
            id: TPMEventID::Pcr7SecureBoot,
        },
        TPMEvent {
            name: "EV_EFI_VARIABLE_DRIVER_CONFIG".into(),
            pcr: 7,
            hash: decode("adb6fc232943e39c374bf4782b6c697f43c39fca1f4b51dfceda21164e19a893")
                .unwrap(),
            id: TPMEventID::Pcr7Pk,
        },
        TPMEvent {
            name: "EV_EFI_VARIABLE_DRIVER_CONFIG".into(),
            pcr: 7,
            hash: decode("b5432fe20c624811cb0296391bfdf948ebd02f0705ab8229bea09774023f0ebf")
                .unwrap(),
            id: TPMEventID::Pcr7Kek,
        },
        TPMEvent {
            name: "EV_EFI_VARIABLE_DRIVER_CONFIG".into(),
            pcr: 7,
            hash: decode("4313e43de720194a0eabf4d6415d42b5a03a34fdc47bb1fc924cc4e665e6893d")
                .unwrap(),
            id: TPMEventID::Pcr7Db,
        },
        TPMEvent {
            name: "EV_EFI_VARIABLE_DRIVER_CONFIG".into(),
            pcr: 7,
            hash: decode("001004ba58a184f09be6c1f4ec75a246cc2eefa9637b48ee428b6aa9bce48c55")
                .unwrap(),
            id: TPMEventID::Pcr7Dbx,
        },
        TPMEvent {
            name: "EV_SEPARATOR".into(),
            pcr: 7,
            hash: decode("df3f619804a92fdb4057192dc43dd748ea778adc52bc498ce80524c014b81119")
                .unwrap(),
            id: TPMEventID::Pcr7Separator,
        },
        TPMEvent {
            name: "EV_EFI_VARIABLE_AUTHORITY".into(),
            pcr: 7,
            hash: decode("4d4a8e2c74133bbdc01a16eaf2dbb5d575afeb36f5d8dfcf609ae043909e2ee9")
                .unwrap(),
            id: TPMEventID::Pcr7ShimCert,
        },
        TPMEvent {
            name: "EV_EFI_VARIABLE_AUTHORITY".into(),
            pcr: 7,
            hash: decode("e8e9578f5951ef16b1c1aa18ef02944b8375ec45ed4b5d8cdb30428db4a31016")
                .unwrap(),
            id: TPMEventID::Pcr7SbatLevel,
        },
        TPMEvent {
            name: "EV_EFI_VARIABLE_AUTHORITY".into(),
            pcr: 7,
            hash: decode("ad5901fd581e6640c742c488083b9ac2c48255bd28a16c106c6f9df52702ee3f")
                .unwrap(),
            id: TPMEventID::Pcr7GrubVendorDbCert,
        },
    ];

    let res = combine(&this, &that);
    let expected = vec![
        compile_pcrs(&this),
        compile_pcrs(&expected_that_kernel_this_bootloader),
        compile_pcrs(&expected_this_kernel_that_bootloader),
        compile_pcrs(&that),
    ];

    assert_eq!(res, expected);
}

#[test]
fn test_pcr4_pcr7_bootloader_secureboot_update() {
    let this = vec![
        TPMEvent {
            name: "EV_EFI_BOOT_SERVICES_APPLICATION".into(),
            pcr: 4,
            hash: decode("94896c17d49fc8c8df0cc2836611586edab1615ce7cb58cf13fc5798de56b367")
                .unwrap(),
            id: TPMEventID::Pcr4Shim,
        },
        TPMEvent {
            name: "EV_EFI_VARIABLE_DRIVER_CONFIG".into(),
            pcr: 7,
            hash: decode("ccfc4bb32888a345bc8aeadaba552b627d99348c767681ab3141f5b01e40a40e")
                .unwrap(),
            id: TPMEventID::Pcr7SecureBoot,
        },
        TPMEvent {
            name: "EV_EFI_VARIABLE_AUTHORITY".into(),
            pcr: 7,
            hash: decode("4d4a8e2c74133bbdc01a16eaf2dbb5d575afeb36f5d8dfcf609ae043909e2ee9")
                .unwrap(),
            id: TPMEventID::Pcr7ShimCert,
        },
    ];
    let that = vec![
        TPMEvent {
            name: "EV_EFI_BOOT_SERVICES_APPLICATION".into(),
            pcr: 4,
            hash: decode("6d1b10555d58f566c4f7fd9356ce7ffa9ecc614aa04498c5db7666a577106e08")
                .unwrap(),
            id: TPMEventID::Pcr4Shim,
        },
        TPMEvent {
            name: "EV_EFI_VARIABLE_DRIVER_CONFIG".into(),
            pcr: 7,
            hash: decode("d1893345c5196d4ad661ad0ef3d87f7de0ad0343134f1296ae853b7cb8067518")
                .unwrap(),
            id: TPMEventID::Pcr7SecureBoot,
        },
        TPMEvent {
            name: "EV_EFI_VARIABLE_AUTHORITY".into(),
            pcr: 7,
            hash: decode("d25488faf16c53c9ba858cdb31158c35c66c637977e565117ad5c79a18fecb86")
                .unwrap(),
            id: TPMEventID::Pcr7ShimCert,
        },
    ];
    testing_logger::setup();
    let res = combine(&this, &that);
    testing_logger::validate(|logs| {
        assert_eq!(logs.len(), 2);
        assert_eq!(
            logs[0].body,
            "Event group conflict hit combining Pcr7ShimCert"
        );
        assert_eq!(logs[0].level, Level::Warn);
        assert_eq!(
            logs[1].body,
            "Event group conflict hit combining Pcr7ShimCert"
        );
        assert_eq!(logs[1].level, Level::Warn);
    });

    assert_eq!(res.len(), 2);
}

#[test]
fn test_pcr7_enable_secureboot() {
    let this = vec![
        TPMEvent {
            name: "EV_EFI_ACTION".into(),
            pcr: 4,
            hash: decode("3d6772b4f84ed47595d72a2c4c5ffd15f5bb72c7507fe26f2aaee2c69d5633ba")
                .unwrap(),
            id: TPMEventID::Pcr4EfiCall,
        },
        TPMEvent {
            name: "EV_SEPARATOR".into(),
            pcr: 4,
            hash: decode("df3f619804a92fdb4057192dc43dd748ea778adc52bc498ce80524c014b81119")
                .unwrap(),
            id: TPMEventID::Pcr4Separator,
        },
        TPMEvent {
            name: "EV_EFI_BOOT_SERVICES_APPLICATION".into(),
            pcr: 4,
            hash: decode("94896c17d49fc8c8df0cc2836611586edab1615ce7cb58cf13fc5798de56b367")
                .unwrap(),
            id: TPMEventID::Pcr4Shim,
        },
        TPMEvent {
            name: "EV_EFI_BOOT_SERVICES_APPLICATION".into(),
            pcr: 4,
            hash: decode("bc6844fc7b59b4f0c7da70a307fc578465411d7a2c34b0f4dc2cc154c873b644")
                .unwrap(),
            id: TPMEventID::Pcr4Grub,
        },
        TPMEvent {
            name: "EV_EFI_BOOT_SERVICES_APPLICATION".into(),
            pcr: 4,
            hash: decode("2b1dc59bc61dbbc3db11a6f3b0708c948efd46cceb7f6c8ea2024b8d1b8c829a")
                .unwrap(),
            id: TPMEventID::Pcr4Vmlinuz,
        },
        TPMEvent {
            name: "EV_EFI_VARIABLE_DRIVER_CONFIG".into(),
            pcr: 7,
            hash: decode("115aa827dbccfb44d216ad9ecfda56bdea620b860a94bed5b7a27bba1c4d02d8")
                .unwrap(),
            id: TPMEventID::Pcr7SecureBoot,
        },
        TPMEvent {
            name: "EV_EFI_VARIABLE_DRIVER_CONFIG".into(),
            pcr: 7,
            hash: decode("dea7b80ab53a3daaa24d5cc46c64e1fa9ffd03739f90aadbd8c0867c4a5b4890")
                .unwrap(),
            id: TPMEventID::Pcr7Pk,
        },
        TPMEvent {
            name: "EV_EFI_VARIABLE_DRIVER_CONFIG".into(),
            pcr: 7,
            hash: decode("e670e121fcebd473b8bc41bb801301fc1d9afa33904f06f7149b74f12c47a68f")
                .unwrap(),
            id: TPMEventID::Pcr7Kek,
        },
        TPMEvent {
            name: "EV_EFI_VARIABLE_DRIVER_CONFIG".into(),
            pcr: 7,
            hash: decode("baf89a3ccace52750c5f0128351e0422a41597a1adfd50822aa363b9d124ea7c")
                .unwrap(),
            id: TPMEventID::Pcr7Db,
        },
        TPMEvent {
            name: "EV_EFI_VARIABLE_DRIVER_CONFIG".into(),
            pcr: 7,
            hash: decode("9f75b6823bff6af1024a4e2036719cdd548d3cbc2bf1de8e7ef4d0ed01f94bf9")
                .unwrap(),
            id: TPMEventID::Pcr7Dbx,
        },
        TPMEvent {
            name: "EV_SEPARATOR".into(),
            pcr: 7,
            hash: decode("df3f619804a92fdb4057192dc43dd748ea778adc52bc498ce80524c014b81119")
                .unwrap(),
            id: TPMEventID::Pcr7Separator,
        },
        TPMEvent {
            name: "EV_EFI_VARIABLE_AUTHORITY".into(),
            pcr: 7,
            hash: decode("922e939a5565798a5ef12fe09d8b49bf951a8e7f89a0cca7a51636693d41a34d")
                .unwrap(),
            id: TPMEventID::Pcr7SbatLevel,
        },
    ];
    let that = vec![
        TPMEvent {
            name: "EV_EFI_ACTION".into(),
            pcr: 4,
            hash: decode("3d6772b4f84ed47595d72a2c4c5ffd15f5bb72c7507fe26f2aaee2c69d5633ba")
                .unwrap(),
            id: TPMEventID::Pcr4EfiCall,
        },
        TPMEvent {
            name: "EV_SEPARATOR".into(),
            pcr: 4,
            hash: decode("df3f619804a92fdb4057192dc43dd748ea778adc52bc498ce80524c014b81119")
                .unwrap(),
            id: TPMEventID::Pcr4Separator,
        },
        TPMEvent {
            name: "EV_EFI_BOOT_SERVICES_APPLICATION".into(),
            pcr: 4,
            hash: decode("94896c17d49fc8c8df0cc2836611586edab1615ce7cb58cf13fc5798de56b367")
                .unwrap(),
            id: TPMEventID::Pcr4Shim,
        },
        TPMEvent {
            name: "EV_EFI_BOOT_SERVICES_APPLICATION".into(),
            pcr: 4,
            hash: decode("bc6844fc7b59b4f0c7da70a307fc578465411d7a2c34b0f4dc2cc154c873b644")
                .unwrap(),
            id: TPMEventID::Pcr4Grub,
        },
        TPMEvent {
            name: "EV_EFI_BOOT_SERVICES_APPLICATION".into(),
            pcr: 4,
            hash: decode("2b1dc59bc61dbbc3db11a6f3b0708c948efd46cceb7f6c8ea2024b8d1b8c829a")
                .unwrap(),
            id: TPMEventID::Pcr4Vmlinuz,
        },
        TPMEvent {
            name: "EV_EFI_VARIABLE_DRIVER_CONFIG".into(),
            pcr: 7,
            hash: decode("ccfc4bb32888a345bc8aeadaba552b627d99348c767681ab3141f5b01e40a40e")
                .unwrap(),
            id: TPMEventID::Pcr7SecureBoot,
        },
        TPMEvent {
            name: "EV_EFI_VARIABLE_DRIVER_CONFIG".into(),
            pcr: 7,
            hash: decode("adb6fc232943e39c374bf4782b6c697f43c39fca1f4b51dfceda21164e19a893")
                .unwrap(),
            id: TPMEventID::Pcr7Pk,
        },
        TPMEvent {
            name: "EV_EFI_VARIABLE_DRIVER_CONFIG".into(),
            pcr: 7,
            hash: decode("b5432fe20c624811cb0296391bfdf948ebd02f0705ab8229bea09774023f0ebf")
                .unwrap(),
            id: TPMEventID::Pcr7Kek,
        },
        TPMEvent {
            name: "EV_EFI_VARIABLE_DRIVER_CONFIG".into(),
            pcr: 7,
            hash: decode("4313e43de720194a0eabf4d6415d42b5a03a34fdc47bb1fc924cc4e665e6893d")
                .unwrap(),
            id: TPMEventID::Pcr7Db,
        },
        TPMEvent {
            name: "EV_EFI_VARIABLE_DRIVER_CONFIG".into(),
            pcr: 7,
            hash: decode("001004ba58a184f09be6c1f4ec75a246cc2eefa9637b48ee428b6aa9bce48c55")
                .unwrap(),
            id: TPMEventID::Pcr7Dbx,
        },
        TPMEvent {
            name: "EV_SEPARATOR".into(),
            pcr: 7,
            hash: decode("df3f619804a92fdb4057192dc43dd748ea778adc52bc498ce80524c014b81119")
                .unwrap(),
            id: TPMEventID::Pcr7Separator,
        },
        TPMEvent {
            name: "EV_EFI_VARIABLE_AUTHORITY".into(),
            pcr: 7,
            hash: decode("4d4a8e2c74133bbdc01a16eaf2dbb5d575afeb36f5d8dfcf609ae043909e2ee9")
                .unwrap(),
            id: TPMEventID::Pcr7ShimCert,
        },
        TPMEvent {
            name: "EV_EFI_VARIABLE_AUTHORITY".into(),
            pcr: 7,
            hash: decode("e8e9578f5951ef16b1c1aa18ef02944b8375ec45ed4b5d8cdb30428db4a31016")
                .unwrap(),
            id: TPMEventID::Pcr7SbatLevel,
        },
        TPMEvent {
            name: "EV_EFI_VARIABLE_AUTHORITY".into(),
            pcr: 7,
            hash: decode("ad5901fd581e6640c742c488083b9ac2c48255bd28a16c106c6f9df52702ee3f")
                .unwrap(),
            id: TPMEventID::Pcr7GrubVendorDbCert,
        },
    ];

    let res = combine(&this, &that);
    let expected = vec![compile_pcrs(&this), compile_pcrs(&that)];

    assert_eq!(res, expected);
}

#[test]
fn test_image_combinations() {
    let shim1 = TPMEvent {
        name: "shim1".into(),
        pcr: 4,
        hash: decode("f6f919856f814f30c2043b567c9434b73b658f2360175f18e49da81112216be0").unwrap(),
        id: TPMEventID::Pcr4Shim,
    };
    let shim2 = TPMEvent {
        name: "shim2".into(),
        pcr: 4,
        hash: decode("5921135eb8f625f3050a92d66551ef0a6682b8c393af8ef8379a1332f1f1872f").unwrap(),
        id: TPMEventID::Pcr4Shim,
    };
    let kernel1 = TPMEvent {
        name: "kernel1".into(),
        pcr: 4,
        hash: decode("2b1dc59bc61dbbc3db11a6f3b0708c948efd46cceb7f6c8ea2024b8d1b8c829a").unwrap(),
        id: TPMEventID::Pcr4Vmlinuz,
    };
    let kernel2 = TPMEvent {
        name: "kernel2".into(),
        pcr: 4,
        hash: decode("d4735e3a265e16eee03f59718b9b5d03019c07d8b6c51f90da3a666eec13ab35").unwrap(),
        id: TPMEventID::Pcr4Vmlinuz,
    };
    let kernel3 = TPMEvent {
        name: "kernel3".into(),
        pcr: 4,
        hash: decode("4e07408562bedb8b60ce05c1decfe3ad16b72230967de01f640b7e4729b49fce").unwrap(),
        id: TPMEventID::Pcr4Vmlinuz,
    };
    let kernel4 = TPMEvent {
        name: "kernel4".into(),
        pcr: 4,
        hash: decode("4b227777d4dd1fc61c6f884f48641d02b4d121d3fd328cb08b5531fcacdabf8a").unwrap(),
        id: TPMEventID::Pcr4Vmlinuz,
    };

    let images = vec![
        vec![shim1.clone(), kernel1.clone()],
        vec![shim1.clone(), kernel2.clone()],
        vec![shim2.clone(), kernel3.clone()],
        vec![shim2.clone(), kernel4.clone()],
    ];

    let res = combine_images(&images);
    let expected: Vec<Vec<Pcr>> = vec![
        compile_pcrs(&[shim1.clone(), kernel1.clone()]),
        compile_pcrs(&[shim1.clone(), kernel2.clone()]),
        compile_pcrs(&[shim1.clone(), kernel3.clone()]),
        compile_pcrs(&[shim1.clone(), kernel4.clone()]),
        compile_pcrs(&[shim2.clone(), kernel1.clone()]),
        compile_pcrs(&[shim2.clone(), kernel2.clone()]),
        compile_pcrs(&[shim2.clone(), kernel3.clone()]),
        compile_pcrs(&[shim2.clone(), kernel4.clone()]),
    ];

    assert_eq!(res.len(), expected.len());
    assert_eq!(
        HashSet::<_>::from_iter(res.iter().flat_map(|e| e.clone())),
        HashSet::<_>::from_iter(expected.iter().flat_map(|e| e.clone())),
    );
}

#[test]
fn test_combine_one_image() {
    let images = vec![vec![
        TPMEvent {
            name: "pcr4".into(),
            pcr: 4,
            hash: decode("f6f919856f814f30c2043b567c9434b73b658f2360175f18e49da81112216be0")
                .unwrap(),
            id: TPMEventID::Pcr4EfiCall,
        },
        TPMEvent {
            name: "pcr7".into(),
            pcr: 7,
            hash: decode("1111111111111111111111111111111111111111111111111111111111111111")
                .unwrap(),
            id: TPMEventID::Pcr7SecureBoot,
        },
    ]];

    let res = combine_images(&images);

    let image_pcrs: Vec<Vec<Pcr>> = images.iter().map(|e| compile_pcrs(e)).collect();
    assert_eq!(image_pcrs, res);
}
