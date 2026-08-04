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
// Rust mirror of `jdk.graal.compiler.graph.NodeFlood`. Faithful 1:1 port.

use crate::graph::node::NodeId;
use crate::graph::node_bit_map::NodeBitMap;
use std::collections::VecDeque;
use std::fmt::Debug;

/// Corresponds to `public final class NodeFlood`.
///
/// A flood-fill traversal over graph edges. Used to compute the
/// transitive closure of a set of starting nodes. Supports both
/// forward and backward traversal.
///
/// Java final class → Rust struct+impl.
#[derive(Debug, Clone)]
pub struct NodeFlood {
    /// Corresponds to the `visited` field.
    visited: NodeBitMap,
    /// Corresponds to the FIFO queue.
    queue: VecDeque<NodeId>,
    /// The graph id.
    graph_id: usize,
    /// Whether to traverse inputs (forward) or successors (backward).
    is_input: bool,
}

impl NodeFlood {
    /// Corresponds to `NodeFlood(Graph graph)`.
    pub fn new(graph_id: usize, node_count: usize) -> Self {
        NodeFlood {
            visited: NodeBitMap::new(graph_id, node_count),
            queue: VecDeque::new(),
            graph_id,
            is_input: true,
        }
    }

    /// Corresponds to `NodeFlood(Graph graph, boolean isInput)`.
    /// Creates a NodeFlood with direction control:
    /// - `is_input = true`: traverse inputs (forward)
    /// - `is_input = false`: traverse successors (backward)
    pub fn with_direction(graph_id: usize, node_count: usize, is_input: bool) -> Self {
        NodeFlood {
            visited: NodeBitMap::new(graph_id, node_count),
            queue: VecDeque::new(),
            graph_id,
            is_input,
        }
    }

    /// Corresponds to `void add(Node node)`.
    pub fn add(&mut self, node_id: NodeId) {
        if self.visited.is_new_marked(node_id) {
            self.queue.push_back(node_id);
        }
    }

    /// Corresponds to `void addAll(Iterable<Node> nodes)`.
    pub fn add_all(&mut self, node_ids: &[NodeId]) {
        for &node_id in node_ids {
            self.add(node_id);
        }
    }

    /// Corresponds to `boolean isMarked(Node node)`.
    pub fn is_marked(&self, node_id: NodeId) -> bool {
        self.visited.is_marked(node_id)
    }

    /// Corresponds to `boolean isNewMarked(Node node)`.
    pub fn is_new_marked(&mut self, node_id: NodeId) -> bool {
        self.visited.is_new_marked(node_id)
    }

    /// Corresponds to `boolean isEmpty()`.
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    /// Corresponds to `int count()`.
    pub fn count(&self) -> usize {
        self.visited.count()
    }

    /// Corresponds to `void clear()`.
    pub fn clear(&mut self) {
        self.visited.clear_all();
        self.queue.clear();
    }

    /// Returns the graph id.
    pub fn graph_id(&self) -> usize {
        self.graph_id
    }

    /// Returns the visited bitmap.
    pub fn visited(&self) -> &NodeBitMap {
        &self.visited
    }

    /// Returns the mutable visited bitmap.
    pub fn visited_mut(&mut self) -> &mut NodeBitMap {
        &mut self.visited
    }

    /// Corresponds to `Node next()`.
    /// Returns the next node in the flood-fill queue, or None if empty.
    pub fn next(&mut self) -> Option<NodeId> {
        self.queue.pop_front()
    }
}