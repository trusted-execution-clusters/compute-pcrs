// SPDX-FileCopyrightText: Beñat Gartzia Arruabarrena <bgartzia@redhat.com>
//
// SPDX-License-Identifier: MIT
const MAX_EXPECTED_CHILDREN: usize = 2;

#[cfg(test)]
mod tests;

#[derive(Clone)]
pub struct Node<T> {
    event: T,
    children: Vec<Node<T>>,
    root: bool,
}

impl<T: Clone> Node<T> {
    pub fn new(event: T) -> Node<T> {
        Node {
            event,
            children: Vec::with_capacity(MAX_EXPECTED_CHILDREN),
            root: true,
        }
    }

    pub fn add_child(&mut self, mut child: Node<T>) {
        child.root = false;
        self.children.push(child);
    }

    pub fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    pub fn branches(&self) -> Vec<Vec<T>> {
        if self.is_leaf() {
            return vec![vec![self.event.clone()]];
        }

        let mut ret = vec![];
        for child in &self.children {
            for child_branch in &child.branches() {
                let mut branch = vec![self.event.clone()];
                branch.append(&mut child_branch.clone());
                ret.push(branch);
            }
        }

        ret
    }
}

pub type ResultNode<T, E> = Node<Result<T, E>>;

impl<T, E> ResultNode<T, E>
where
    T: Clone,
    E: Clone,
{
    pub fn new_ok(data: T) -> ResultNode<T, E> {
        Node::new(Ok(data))
    }

    pub fn new_err(err: E) -> ResultNode<T, E> {
        Node::new(Err(err))
    }

    /// Returns those branches that do not contain errors
    pub fn valid_branches(&self) -> Vec<Vec<T>> {
        self.branches()
            .iter()
            .filter_map(|v| v.iter().map(|e| e.clone().ok()).collect())
            .collect()
    }
}
