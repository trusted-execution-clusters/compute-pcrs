// SPDX-FileCopyrightText: Timothée Ravier <tim@siosm.fr>
// SPDX-FileCopyrightText: Beñat Gartzia Arruabarrena <bgartzia@redhat.com>
//
// SPDX-License-Identifier: MIT

use crate::tpmevents::TPMEvent;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sha2::{Digest, Sha256};

const PCR_INIT_VALUE: [u8; 32] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

#[cfg(test)]
mod tests;

#[serde_as]
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[cfg_attr(test, derive(Debug))]
pub struct Pcr {
    pub id: u64,
    #[serde_as(as = "serde_with::hex::Hex")]
    pub value: Vec<u8>,
    pub events: Vec<TPMEvent>,
}

impl Pcr {
    /// Only supports compiling PCRs from vectors of events that belong
    /// to the same PCR
    /// Note that events must be ordered per PCR and the order TPM would
    /// expect them to be logged
    pub fn compile_from(events: &Vec<TPMEvent>) -> Pcr {
        let mut result = PCR_INIT_VALUE.to_vec();
        let compiled_pcr: u8 = events[0].pcr;

        for event in events {
            if event.pcr != compiled_pcr {
                // FIXME: better error handling
                panic!(
                    "unexpected pcr#{} while compiling pcr#{}",
                    event.pcr, compiled_pcr
                );
            }
            let mut hasher = Sha256::new();
            hasher.update(result);
            hasher.update(event.hash.clone());
            result = hasher.finalize().to_vec();
        }

        Pcr {
            id: events[0].pcr.into(),
            value: result,
            events: events.clone(),
        }
    }
}

/// Supports compiling vectors of PCRs from vectors of events that belong
/// to different PCRs
/// Note that events must be ordered per PCR and the order TPM would
/// expect them to be logged
pub fn compile_pcrs(events: &[TPMEvent]) -> Vec<Pcr> {
    let pcrs: Vec<u8> = events.iter().map(|e| e.pcr).unique().collect();

    pcrs.iter()
        .map(|n| Pcr::compile_from(&events.iter().filter(|e| e.pcr == *n).cloned().collect()))
        .collect()
}
