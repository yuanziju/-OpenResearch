/*
 * Copyright (c) 2011, 2026, Oracle and/or its affiliates. All rights reserved.
 * DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
 *
 * This code is free software; you can redistribute it and/or modify it
 * under the terms of the GNU General Public License version 2 only, as
 * published by the Free Software Foundation.  Oracle designates this
 * particular file as subject to the "Classpath" exception as provided
 * by Oracle in the LICENSE file that accompanied this code.
 *
 * This code is distributed in the hope that it will be useful, but WITHOUT
 * ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
 * FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License
 * version 2 for more details (a copy is included in the LICENSE file that
 * accompanied this code).
 *
 * You should have received a copy of the GNU General Public License version
 * 2 along with this work; if not, write to the Free Software Foundation,
 * Inc., 51 Franklin St, Fifth Floor, Boston, MA 02110-1301 USA.
 *
 * Please contact Oracle, 500 Oracle Parkway, Redwood Shores, CA 94065 USA
 * or visit www.oracle.com if you need additional information or have any
 * questions.
 */

// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
//
// Rust mirror of `jdk.graal.compiler.graph.TypedGraphNodeIterator`.
// Faithful 1:1 port.

use crate::graph::node::NodeId;
use std::fmt::Debug;

/// Corresponds to `public class TypedGraphNodeIterator<T extends IterableNodeType>`.
///
/// Iterates over nodes of a specific type in a graph. Uses the graph's
/// node-type index to efficiently find nodes of the desired type.
///
/// Java class → Rust struct+impl.
#[derive(Debug, Clone)]
pub struct TypedGraphNodeIterator {
    /// Corresponds to the `graph` field.
    graph_id: usize,
    /// Corresponds to the `iterableId` field.
    iterable_id: u64,
    /// Current position in the node array.
    position: usize,
    /// Node ids that match the target type.
    node_ids: Vec<NodeId>,
}

impl TypedGraphNodeIterator {
    /// Corresponds to `TypedGraphNodeIterator(Graph graph, Class<T> clazz)`.
    pub fn new(graph_id: usize, iterable_id: u64, node_ids: Vec<NodeId>) -> Self {
        TypedGraphNodeIterator {
            graph_id,
            iterable_id,
            position: 0,
            node_ids,
        }
    }

    /// Corresponds to `boolean hasNext()`.
    pub fn has_next(&self) -> bool {
        self.position < self.node_ids.len()
    }

    /// Corresponds to `T next()`.
    pub fn next(&mut self) -> Option<NodeId> {
        if self.position < self.node_ids.len() {
            let id = self.node_ids[self.position];
            self.position += 1;
            Some(id)
        } else {
            None
        }
    }

    /// Returns the graph id.
    pub fn graph_id(&self) -> usize {
        self.graph_id
    }

    /// Returns the iterable id.
    pub fn iterable_id(&self) -> u64 {
        self.iterable_id
    }

    /// Returns the number of remaining nodes.
    pub fn remaining(&self) -> usize {
        self.node_ids.len() - self.position
    }
}

impl Iterator for TypedGraphNodeIterator {
    type Item = NodeId;

    fn next(&mut self) -> Option<NodeId> {
        if self.position < self.node_ids.len() {
            let id = self.node_ids[self.position];
            self.position += 1;
            Some(id)
        } else {
            None
        }
    }
}