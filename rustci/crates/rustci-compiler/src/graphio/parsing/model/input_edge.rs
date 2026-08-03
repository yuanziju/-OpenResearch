/*
 * Copyright (c) 2013, 2026, Oracle and/or its affiliates. All rights reserved.
 * DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
 *
 * The Universal Permissive License (UPL), Version 1.0
 * ...
 */

// SPDX-License-Identifier: UPL-1.0 OR GPL-2.0-with-classpath-exception

/// Represents an edge between two nodes in a parsed graph.
/// Mirrors `jdk.graal.compiler.graphio.parsing.model.InputEdge`.
#[derive(Debug, Clone)]
pub struct InputEdge {
    pub(crate) from: i32,
    pub(crate) to: i32,
    pub(crate) index: i32,
    pub(crate) from_index: i32,
    pub(crate) to_index: i32,
    pub(crate) label: String,
    pub(crate) edge_type: String,
}

impl InputEdge {
    pub fn new(
        from: i32,
        to: i32,
        from_index: i32,
        to_index: i32,
        label: String,
        edge_type: String,
    ) -> Self {
        InputEdge {
            from,
            to,
            index: from_index,
            from_index,
            to_index,
            label,
            edge_type,
        }
    }

    /// Returns the source node id.
    pub fn get_from(&self) -> i32 {
        self.from
    }

    /// Returns the destination node id.
    pub fn get_to(&self) -> i32 {
        self.to
    }

    /// Returns the index of this edge.
    pub fn get_index(&self) -> i32 {
        self.index
    }

    /// Returns the source index.
    pub fn get_from_index(&self) -> i32 {
        self.from_index
    }

    /// Returns the destination index.
    pub fn get_to_index(&self) -> i32 {
        self.to_index
    }

    /// Returns the label of this edge.
    pub fn get_label(&self) -> &str {
        &self.label
    }

    /// Returns the type of this edge.
    pub fn get_type(&self) -> &str {
        &self.edge_type
    }
}

impl PartialEq for InputEdge {
    fn eq(&self, other: &Self) -> bool {
        self.from == other.from
            && self.to == other.to
            && self.from_index == other.from_index
            && self.to_index == other.to_index
    }
}

impl Eq for InputEdge {}

impl std::hash::Hash for InputEdge {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.from.hash(state);
        self.to.hash(state);
        self.from_index.hash(state);
        self.to_index.hash(state);
    }
}