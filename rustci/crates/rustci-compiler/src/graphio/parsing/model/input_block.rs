/*
 * Copyright (c) 2013, 2026, Oracle and/or its affiliates. All rights reserved.
 * DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
 *
 * The Universal Permissive License (UPL), Version 1.0
 * ...
 */

// SPDX-License-Identifier: UPL-1.0 OR GPL-2.0-with-classpath-exception

/// Represents a block in a parsed graph.
/// Mirrors `jdk.graal.compiler.graphio.parsing.model.InputBlock`.
#[derive(Debug, Clone)]
pub struct InputBlock {
    name: String,
    nodes: Vec<i32>,
    successors: Vec<i32>,
}

impl InputBlock {
    pub fn new(name: String) -> Self {
        InputBlock {
            name,
            nodes: Vec::new(),
            successors: Vec::new(),
        }
    }

    /// Returns the name of this block.
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Returns the node ids in this block.
    pub fn get_nodes(&self) -> &[i32] {
        &self.nodes
    }

    /// Adds a node to this block.
    pub fn add_node(&mut self, node_id: i32) {
        if !self.nodes.contains(&node_id) {
            self.nodes.push(node_id);
        }
    }

    /// Returns the successor block ids.
    pub fn get_successors(&self) -> &[i32] {
        &self.successors
    }

    /// Adds a successor block.
    pub fn add_successor(&mut self, block_id: i32) {
        if !self.successors.contains(&block_id) {
            self.successors.push(block_id);
        }
    }
}

impl PartialEq for InputBlock {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for InputBlock {}

impl std::hash::Hash for InputBlock {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}