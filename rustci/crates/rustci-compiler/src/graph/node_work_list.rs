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
// Rust mirror of `jdk.graal.compiler.graph.NodeWorkList`. Faithful 1:1 port.

use crate::graph::node::NodeId;
use crate::graph::node_bit_map::NodeBitMap;
use std::collections::VecDeque;
use std::fmt::Debug;

/// Corresponds to `public class NodeWorkList implements Iterable<Node>`.
///
/// A work list for iterative graph algorithms. Nodes are added to the work
/// list and processed in FIFO or LIFO order. A `NodeBitMap` is used to
/// prevent duplicate entries.
///
/// Java class → Rust struct+impl.
#[derive(Debug, Clone)]
pub struct NodeWorkList {
    /// Corresponds to the `worklist` field (an `ArrayDeque`).
    queue: VecDeque<NodeId>,
    /// Corresponds to the `onWorklist` field.
    on_worklist: NodeBitMap,
    /// The graph id this worklist belongs to.
    graph_id: usize,
    /// Current iteration position.
    iteration_index: usize,
    /// Snapshot of the queue for iteration.
    iteration_snapshot: Vec<NodeId>,
}

impl NodeWorkList {
    /// Corresponds to `NodeWorkList(Graph graph)`.
    pub fn new(graph_id: usize, node_count: usize) -> Self {
        NodeWorkList {
            queue: VecDeque::new(),
            on_worklist: NodeBitMap::new(graph_id, node_count),
            graph_id,
            iteration_index: 0,
            iteration_snapshot: Vec::new(),
        }
    }

    /// Corresponds to `void add(Node node)`.
    /// Adds a node to the worklist if not already present.
    pub fn add(&mut self, node_id: NodeId) {
        if !self.on_worklist.is_marked(node_id) {
            self.on_worklist.mark(node_id);
            self.queue.push_back(node_id);
        }
    }

    /// Corresponds to `void addFirst(Node node)`.
    pub fn add_first(&mut self, node_id: NodeId) {
        if !self.on_worklist.is_marked(node_id) {
            self.on_worklist.mark(node_id);
            self.queue.push_front(node_id);
        }
    }

    /// Corresponds to `void addAll(Iterable<Node> nodes)`.
    /// Adds multiple nodes to the worklist at once.
    pub fn add_all(&mut self, node_ids: &[NodeId]) {
        for &node_id in node_ids {
            self.add(node_id);
        }
    }

    /// Corresponds to `boolean contains(Node node)`.
    pub fn contains(&self, node_id: NodeId) -> bool {
        self.on_worklist.is_marked(node_id)
    }

    /// Corresponds to `boolean isEmpty()`.
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    /// Corresponds to `boolean isNotEmpty()`.
    pub fn is_not_empty(&self) -> bool {
        !self.queue.is_empty()
    }

    /// Corresponds to `Node poll()`.
    pub fn poll(&mut self) -> Option<NodeId> {
        self.queue.pop_front()
    }

    /// Corresponds to `void clear()`.
    pub fn clear(&mut self) {
        self.queue.clear();
        self.on_worklist.clear_all();
        self.iteration_snapshot.clear();
        self.iteration_index = 0;
    }

    /// Returns the graph id.
    pub fn graph_id(&self) -> usize {
        self.graph_id
    }

    /// Returns an iterator over the worklist.
    pub fn iter(&self) -> impl Iterator<Item = &NodeId> {
        self.queue.iter()
    }
}