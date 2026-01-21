// SPDX-FileCopyrightText: Beñat Gartzia Arruabarrena <bgartzia@redhat.com>
//
// SPDX-License-Identifier: MIT
use super::*;

#[test]
fn test_tpmevevent_next_first() {
    let event = TPMEventID::PcrRootNodeEvent;
    assert_eq!(event.next(), Some(TPMEventID::Pcr4EfiCall));
}

#[test]
fn test_tpmevevent_next_last() {
    let event = TPMEventID::Pcr14MokListTrusted;
    assert_eq!(event.next(), Some(TPMEventID::PcrLastNodeEvent));
}

#[test]
fn test_tpmevevent_lastly_last() {
    let event = TPMEventID::PcrLastNodeEvent;
    assert_eq!(event.next(), None);
}

#[test]
fn test_tpmevevent_next_some() {
    let event = TPMEventID::Pcr4Separator;
    assert_eq!(event.next(), Some(TPMEventID::Pcr4Shim));
}

#[test]
fn test_part_serialization() {
    let input = TPMEvent {
        name: "foo".into(),
        pcr: 11,
        hash: vec![1, 0, 2, 3, 255],
        id: TPMEventID::Pcr11UnameContent,
    };
    let expected = String::from(
        "{\"name\":\"foo\",\"pcr\":11,\"hash\":\"01000203ff\",\"id\":\"Pcr11UnameContent\"}",
    );

    assert_eq!(serde_json::to_string(&input).unwrap(), expected);
}

#[test]
fn test_tpmevent_deserialization() {
    let input =
        String::from("{\"name\":\"bar\",\"pcr\":7,\"hash\":\"0f0300\",\"id\":\"Pcr7Separator\"}");
    let expected = TPMEvent {
        name: "bar".into(),
        pcr: 7,
        hash: vec![15, 3, 0],
        id: TPMEventID::Pcr7Separator,
    };
    let deserialized: TPMEvent = serde_json::from_str(&input).unwrap();

    assert_eq!(deserialized, expected);
}
