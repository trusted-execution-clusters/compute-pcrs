// SPDX-FileCopyrightText: Beñat Gartzia Arruabarrena <bgartzia@redhat.com>
//
// SPDX-License-Identifier: MIT

use super::*;
use crate::tpmevents::{TPMEvent, TPMEventID};

#[test]
fn test_pcr_serialization() {
    let input = Pcr {
        id: 123,
        value: vec![0, 0, 0, 0, 0, 0, 0, 253],
        events: vec![TPMEvent {
            name: "foo".into(),
            pcr: 11,
            hash: vec![1, 0, 2, 3, 255],
            id: TPMEventID::Pcr11UnameContent,
        }],
    };
    let expected = String::from(
        "{\"id\":123,\"value\":\"00000000000000fd\",\"events\":[{\"name\":\"foo\",\"pcr\":11,\"hash\":\"01000203ff\",\"id\":\"Pcr11UnameContent\"}]}",
    );

    assert_eq!(serde_json::to_string(&input).unwrap(), expected);
}

#[test]
fn test_pcr_deserialization() {
    let expected = Pcr {
        id: 0,
        value: vec![0, 0, 0, 0, 0, 0, 0, 240],
        events: vec![TPMEvent {
            name: "foo".into(),
            pcr: 11,
            hash: vec![1, 0, 2, 3, 255],
            id: TPMEventID::Pcr11UnameContent,
        }],
    };

    let deserialized: Pcr = serde_json::from_str(
        "{\"id\":0,\"value\":\"00000000000000f0\",\"events\":[{\"name\":\"foo\",\"pcr\":11,\"hash\":\"01000203ff\",\"id\":\"Pcr11UnameContent\"}]}",
    ).unwrap();

    assert_eq!(deserialized, expected);
}

#[test]
fn test_pcr_compilation_from_tpmevents() {
    let input = vec![
        TPMEvent {
            name: "FOOBAR".into(),
            pcr: 4,
            hash: vec![
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0,
            ],
            id: TPMEventID::Pcr4EfiCall,
        },
        TPMEvent {
            name: "BARFOO".into(),
            pcr: 4,
            hash: vec![
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 1,
            ],
            id: TPMEventID::Pcr4Separator,
        },
    ];
    let expected = Pcr {
        id: 4,
        value: vec![
            65, 62, 10, 52, 9, 169, 42, 229, 47, 108, 155, 208, 62, 239, 192, 64, 254, 216, 40,
            213, 49, 150, 204, 191, 240, 146, 157, 233, 235, 71, 46, 91,
        ],
        events: input.clone(),
    };

    let res = Pcr::compile_from(&input);

    assert_eq!(res, expected);
}

#[test]
#[should_panic]
fn test_pcr_compilation_fails_for_heterogeneous_vecs() {
    let input = vec![
        TPMEvent {
            name: "FOOBAR".into(),
            pcr: 4,
            hash: vec![
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0,
            ],
            id: TPMEventID::Pcr4EfiCall,
        },
        TPMEvent {
            name: "BARFOO".into(),
            pcr: 7,
            hash: vec![
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 1,
            ],
            id: TPMEventID::Pcr7SecureBoot,
        },
    ];

    Pcr::compile_from(&input);
}

#[test]
fn test_pcr_compilation_from_heterogeneous_vec() {
    let input = [
        TPMEvent {
            name: "FOOBAR".into(),
            pcr: 4,
            hash: vec![
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0,
            ],
            id: TPMEventID::Pcr4EfiCall,
        },
        TPMEvent {
            name: "BARFOO".into(),
            pcr: 7,
            hash: vec![
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 1,
            ],
            id: TPMEventID::Pcr7SecureBoot,
        },
    ];

    let expected = vec![
        Pcr {
            id: 4,
            value: vec![
                78, 5, 240, 197, 137, 1, 49, 110, 26, 17, 206, 213, 73, 16, 170, 53, 124, 15, 18,
                16, 159, 35, 230, 209, 16, 42, 161, 172, 36, 158, 227, 74,
            ],
            events: vec![input[0].clone()],
        },
        Pcr {
            id: 7,
            value: vec![
                144, 244, 179, 149, 72, 223, 85, 173, 97, 135, 161, 210, 13, 115, 30, 206, 231,
                140, 84, 91, 148, 175, 209, 111, 66, 239, 117, 146, 217, 156, 211, 101,
            ],
            events: vec![input[1].clone()],
        },
    ];

    let res = compile_pcrs(&input);

    assert_eq!(res, expected);
}
