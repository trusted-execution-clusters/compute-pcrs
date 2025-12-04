// SPDX-FileCopyrightText: Beñat Gartzia Arruabarrena <bgartzia@redhat.com>
//
// SPDX-License-Identifier: MIT
use super::*;

#[derive(Clone, Debug, PartialEq)]
struct MockData {
    pub a: u32,
}

impl PartialEq<u32> for MockData {
    fn eq(&self, other: &u32) -> bool {
        self.a == *other
    }
}

#[derive(Clone, Debug)]
struct MockError {}

static PARENT_DATA: MockData = MockData { a: 0 };
static CHILD0_DATA: MockData = MockData { a: 10 };
static CHILD1_DATA: MockData = MockData { a: 11 };
static CHILD10_DATA: MockData = MockData { a: 110 };
static CHILD00_DATA: MockData = MockData { a: 100 };
static CHILD01_DATA: MockData = MockData { a: 101 };
static CHILD000_DATA: MockData = MockData { a: 1000 };

fn build_example() -> Node<MockData> {
    // Define data/nodes
    let mut root = Node::<MockData>::new(PARENT_DATA.clone());
    let mut child0 = Node::<MockData>::new(CHILD0_DATA.clone());
    let mut child1 = Node::<MockData>::new(CHILD1_DATA.clone());
    let child10 = Node::<MockData>::new(CHILD10_DATA.clone());
    let mut child00 = Node::<MockData>::new(CHILD00_DATA.clone());
    let child01 = Node::<MockData>::new(CHILD01_DATA.clone());
    let child000 = Node::<MockData>::new(CHILD000_DATA.clone());
    // Build the tree
    child00.add_child(child000);
    child0.add_child(child00);
    child0.add_child(child01);
    child1.add_child(child10);
    root.add_child(child0);
    root.add_child(child1);
    root
}

fn build_example_result_node() -> ResultNode<MockData, MockError> {
    // Define data/nodes
    let mut root = ResultNode::<MockData, MockError>::new_ok(PARENT_DATA.clone());
    let mut child0 = ResultNode::<MockData, MockError>::new_ok(CHILD0_DATA.clone());
    let mut child1 = ResultNode::<MockData, MockError>::new_ok(CHILD1_DATA.clone());
    let child10 = ResultNode::<MockData, MockError>::new_ok(CHILD10_DATA.clone());
    let mut child00 = ResultNode::<MockData, MockError>::new_err(MockError {});
    let child01 = ResultNode::<MockData, MockError>::new_ok(CHILD01_DATA.clone());
    let child000 = ResultNode::<MockData, MockError>::new_ok(CHILD000_DATA.clone());
    // Build the tree
    child00.add_child(child000);
    child0.add_child(child00);
    child0.add_child(child01);
    child1.add_child(child10);
    root.add_child(child0);
    root.add_child(child1);
    root
}

#[test]
fn test_create() {
    let mock = MockData { a: 123 };
    let node = Node::<MockData>::new(mock.clone());
    assert_eq!(node.children.len(), 0);
    assert_eq!(node.event, mock);
}

#[test]
fn test_add_child() {
    let root_data = MockData { a: 0 };
    let mut root = Node::<MockData>::new(root_data.clone());
    let child_data = MockData { a: 11111 };
    let child = Node::<MockData>::new(child_data.clone());
    root.add_child(child);
    assert_eq!(root.children.len(), 1);
    assert_eq!(root.event, root_data);
    assert_eq!(root.children[0].event, child_data);
}

#[test]
fn test_add_children() {
    let root = build_example();
    // Check the tree
    assert_eq!(root.children.len(), 2);
    assert_eq!(root.event, PARENT_DATA);
    assert_eq!(root.children[0].event, CHILD0_DATA);
    assert_eq!(root.children[1].event, CHILD1_DATA);
    assert_eq!(root.children[0].children.len(), 2);
    assert_eq!(root.children[1].children.len(), 1);
    assert_eq!(root.children[0].children[0].event, CHILD00_DATA);
    assert_eq!(root.children[0].children[0].children.len(), 1);
    assert_eq!(root.children[0].children[1].event, CHILD01_DATA);
    assert_eq!(root.children[0].children[1].children.len(), 0);
    assert_eq!(
        root.children[0].children[0].children[0].event,
        CHILD000_DATA
    );
    assert_eq!(root.children[0].children[0].children[0].children.len(), 0);
}

#[test]
fn test_leafs() {
    let root = build_example();
    assert!(!root.is_leaf());
    assert!(!root.children[0].is_leaf());
    assert!(!root.children[1].is_leaf());
    assert!(!root.children[0].children[0].is_leaf());
    assert!(root.children[0].children[1].is_leaf());
    assert!(root.children[0].children[0].children[0].is_leaf());
}

#[test]
fn test_branches_tree() {
    let root = build_example();
    let branches = root.branches();
    assert_eq!(
        branches,
        vec![vec![0, 10, 100, 1000], vec![0, 10, 101], vec![0, 11, 110],]
    );
}

#[test]
fn test_branches_node() {
    let mock = MockData { a: 123 };
    let node = Node::<MockData>::new(mock.clone());
    assert_eq!(node.branches(), vec![vec![123]]);
}

#[test]
fn test_valid_branches() {
    let tree = build_example_result_node();
    let branches = tree.valid_branches();
    assert_eq!(branches, vec![vec![0, 10, 101], vec![0, 11, 110],]);
}
