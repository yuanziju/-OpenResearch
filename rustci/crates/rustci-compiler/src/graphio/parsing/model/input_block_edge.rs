/*
 * Copyright (c) 2013, 2026, Oracle and/or its affiliates. All rights reserved.
 * DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
 *
 * The Universal Permissive License (UPL), Version 1.0
 * ...
 */

// SPDX-License-Identifier: UPL-1.0 OR GPL-2.0-with-classpath-exception

/// Represents an edge between two blocks in a parsed graph.
/// Mirrors `jdk.graal.compiler.graphio.parsing.model.InputBlockEdge`.
#[derive(Debug, Clone)]
pub struct InputBlockEdge {
    from: i32,
    to: i32,
    from_index: i32,
    to_index: i32,
    label: String,
}

impl InputBlockEdge {
    pub fn new(from: i32, to: i32, label: String) -> Self {
        InputBlockEdge {
            from,
            to,
            from_index: from,
            to_index: to,
            label,
        }
    }

    /// Returns the source block id.
    pub fn get_from(&self) -> i32 {
        self.from
    }

    /// Returns the destination block id.
    pub fn get_to(&self) -> i32 {
        self.to
    }

    /// Returns the label of this edge.
    pub fn get_label(&self) -> &str {
        &self.label
    }

    /// Returns the source index.
    pub fn get_from_index(&self) -> i32 {
        self.from_index
    }

    /// Returns the destination index.
    pub fn get_to_index(&self) -> i32 {
        self.to_index
    }
}

impl PartialEq for InputBlockEdge {
    fn eq(&self, other: &Self) -> bool {
        self.from == other.from
            && self.to == other.to
            && self.from_index == other.from_index
            && self.to_index == other.to_index
    }
}

impl Eq for InputBlockEdge {}

impl std::hash::Hash for InputBlockEdge {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.from.hash(state);
        self.to.hash(state);
        self.from_index.hash(state);
        self.to_index.hash(state);
    }
}