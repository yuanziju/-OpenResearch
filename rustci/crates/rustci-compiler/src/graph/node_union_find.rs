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
// Rust mirror of `jdk.graal.compiler.graph.NodeUnionFind`. Faithful 1:1 port.

use crate::graph::node::NodeId;
use std::fmt::Debug;

/// Corresponds to `public class NodeUnionFind`.
///
/// Union-Find data structure for nodes. Used to partition nodes into
/// equivalence classes. Supports `union` and `find` operations.
///
/// Java class → Rust struct+impl.
#[derive(Debug, Clone)]
pub struct NodeUnionFind {
    /// Corresponds to the `parent` array.
    parent: Vec<usize>,
    /// Corresponds to the `rank` array (union by rank).
    rank: Vec<u32>,
    /// The graph id.
    graph_id: usize,
}

impl NodeUnionFind {
    /// Corresponds to `NodeUnionFind(Graph graph)`.
    pub fn new(graph_id: usize, node_count: usize) -> Self {
        let mut parent = Vec::with_capacity(node_count);
        for i in 0..node_count {
            parent.push(i);
        }
        NodeUnionFind {
            parent,
            rank: vec![0u32; node_count],
            graph_id,
        }
    }

    /// Corresponds to `void union(Node a, Node b)`.
    pub fn union(&mut self, a: NodeId, b: NodeId) {
        let root_a = self.find(a);
        let root_b = self.find(b);
        if root_a != root_b {
            if self.rank[root_a] < self.rank[root_b] {
                self.parent[root_a] = root_b;
            } else if self.rank[root_a] > self.rank[root_b] {
                self.parent[root_b] = root_a;
            } else {
                self.parent[root_b] = root_a;
                self.rank[root_a] += 1;
            }
        }
    }

    /// Corresponds to `Node find(Node node)`.
    pub fn find(&mut self, node_id: NodeId) -> NodeId {
        if node_id >= self.parent.len() {
            return node_id;
        }
        let mut current = node_id;
        while self.parent[current] != current {
            self.parent[current] = self.parent[self.parent[current]];
            current = self.parent[current];
        }
        current
    }

    /// Corresponds to `boolean isSame(Node a, Node b)`.
    pub fn is_same(&mut self, a: NodeId, b: NodeId) -> bool {
        self.find(a) == self.find(b)
    }

    /// Returns the graph id.
    pub fn graph_id(&self) -> usize {
        self.graph_id
    }
}