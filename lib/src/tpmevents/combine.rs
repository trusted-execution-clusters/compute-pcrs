// SPDX-FileCopyrightText: Beñat Gartzia Arruabarrena <bgartzia@redhat.com>
//
// SPDX-License-Identifier: MIT
/*
 * We're receiving two event vectors that we don't know which PCR they
 * belong to.
 *
 * We need to combine events from vec "A" and "B" based on event groups.
 *
 * Let's say that vec A and B contain event ID "e1". e1 belongs to
 * groups g1 and g2.
 *   If the value of e1 doesn't change from A to B, it doesn't matter.
 *   If the value of e1 is different in A and B, then combinations must
 *   respect groups.
 *   That is, if e1 from A is chosen, all events "ei" that are from groups
 *   g1 or g2 must be chosen from A.
 *   Same applies for B.
 *   And all combinations must be calculated.
 *
 * Note that this kind of looks like an event tree at this point.
 * Each existing branch will be a possible solution to the problem.
 *
 * - PROBLEM:
 *   Imagine that event "Ej" is the product of artifacts a1 and a2.
 *   The event tracking a1 belongs to g1 and a2 to g2.
 *   Ej belongs to g1 and g2.
 *   Choose g1 from event vec A and g2 from event vec B.
 *   How can you compute Ej? There's a conflict.
 *     If g1 and g2 are from A, then Ej is from A.
 *     If g1 and g2 are from B, then Ej is from B.
 *     If g1 is from A and g2 from B, Ej needs to be recomputed.
 *   PROBLEM
 *   Analysis:
 *     Only PCR7 contains multigroup events.
 *     They are combinations of sb variables, bootloader, and mokvars.
 *     It would require upgrading the bootloader while updating secureboot
 *     variables to hit this issue.
 *     Could it be possible, in that case, that a weird mix happens?
 *
 *  Solution:
 *    - Right now, empty EventCombinationError instances are pushed into
 *      the tree when conflicts are hit.
 *    - Currently those are simply ignored when final PCR combinations
 *      are computed as we don't expect those to be hit under the
 *      use-cases accepted.
 *    - In the future, if this is a case that could happen, we should:
 *      - Add some fields to EventCombinationError containing: the state
 *        · The index of images that lead to the conflict.
 *        · The state of groups when the conflict was hit.
 *        · The event_id for which the conflict was hit.
 *      - Add recovery functions to EventCombinationError. We could
 *        model this process in two steps:
 *        · Let the operator know which information is needed (e.g.
 *          bootloader from case A, sb variables from case B), and make
 *          the operator mount the proper information into a computation
 *          container.
 *        · Use the information mounted to compute the TPMEvent based on
 *          the provided information. That would provide a valid
 *          TPMEvent from the conflict information contained in
 *          EventCombinationError.
 *      - Note that this would imply implementing the calculation of
 *        each possible TPMEvent sepparately, and making all those
 *        functions follow a similar interface.
 *
*/
use std::collections::HashMap;

use super::*;
use crate::pcrs::{Pcr, compile_pcrs};

use itertools::Itertools;
use log::warn;

#[cfg(test)]
mod tests;

#[derive(Clone, Debug)]
pub struct EventCombinationError {}

pub type EventNode = tree::ResultNode<TPMEvent, EventCombinationError>;

pub fn combine_images(images: &[Vec<TPMEvent>]) -> Vec<Vec<Pcr>> {
    if images.len() == 1 {
        return vec![compile_pcrs(&images[0])];
    }

    images
        .iter()
        .combinations(2)
        .flat_map(|p| combine(p[0], p[1]))
        .unique()
        .collect()
}

pub fn combine(this: &[TPMEvent], that: &[TPMEvent]) -> Vec<Vec<Pcr>> {
    let map_this = tpm_event_id_hashmap(this);
    let map_that = tpm_event_id_hashmap(that);

    let event = TPMEventID::PcrRootNodeEvent.next().unwrap();
    match event_subtree(&event, &map_this, &map_that, 0, 0) {
        Some(st) => st
            .iter()
            .flat_map(|t| t.valid_branches())
            .map(|e| compile_pcrs(&e))
            .collect(),
        None => vec![],
    }
}

fn event_subtree(
    event_id: &TPMEventID,
    map_this: &HashMap<TPMEventID, TPMEvent>,
    map_that: &HashMap<TPMEventID, TPMEvent>,
    group_this: u32,
    group_that: u32,
) -> Option<Vec<EventNode>> {
    // Groups can't overlap
    assert_eq!(group_this & group_that, 0);
    let opt_this = map_this.get(event_id);
    let opt_that = map_that.get(event_id);
    // Divergences contains tuples with events, and this/that masked groups
    let mut divs: Vec<(&TPMEvent, u32, u32)> = vec![];
    let mut nodes: Vec<EventNode> = vec![];
    let mut event_required = false;
    let event_groups = event_id.groups();

    if let Some(event_this) = opt_this
        && let Some(event_that) = opt_that
    {
        event_required = true;
        if event_this == event_that {
            divs.push((event_this, group_this, group_that));
        } else {
            if (event_groups & group_that) == 0 {
                divs.push((event_this, group_this | event_groups, group_that));
            }
            if (event_groups & group_this) == 0 {
                divs.push((event_that, group_this, group_that | event_groups));
            }
        }
    } else if let Some(event_this) = opt_this {
        // Assume the event is not required if it's only on one side
        if (event_groups & group_that) == 0 {
            divs.push((event_this, group_this | event_groups, group_that));
        }
    } else if let Some(event_that) = opt_that {
        // Assume the event is not required if it's only on one side
        if (event_groups & group_this) == 0 {
            divs.push((event_that, group_this, group_that | event_groups));
        }
    }

    if divs.is_empty() {
        // Event is required but wasn't pushed to divergences...
        // Means we met an event id/tree branching group conflict
        if event_required {
            warn!("Event group conflict hit combining {event_id:?}");

            let mut node = EventNode::new_err(EventCombinationError {});

            if let Some(children) = event_subtree(
                &event_id.next()?,
                map_this,
                map_that,
                group_this,
                group_that,
            ) {
                for c in children {
                    node.add_child(c);
                }
            }

            nodes.push(node);
        } else {
            return event_subtree(
                &event_id.next()?,
                map_this,
                map_that,
                group_this,
                group_that,
            );
        }
    }

    for (event, g_this, g_that) in divs {
        let mut node = EventNode::new_ok(event.clone());
        if let Some(children) = event_subtree(&event_id.next()?, map_this, map_that, g_this, g_that)
        {
            for c in children {
                node.add_child(c);
            }
        }
        nodes.push(node);
    }

    Some(nodes)
}

fn tpm_event_id_hashmap(events: &[TPMEvent]) -> HashMap<TPMEventID, TPMEvent> {
    events.iter().map(|e| (e.id.clone(), e.clone())).collect()
}
