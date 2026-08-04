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
// Rust mirror of `jdk.graal.compiler.graph.NodeInputList`. Faithful 1:1 port.

use crate::graph::node::NodeId;
use crate::graph::node_list::NodeList;
use std::fmt::Debug;

/// Corresponds to `public final class NodeInputList<T extends Node> extends NodeList<T>`.
///
/// A list of input nodes. Unlike `NodeSuccessorList`, input list entries
/// participate in usage tracking: when a node is added to an input list,
/// the source node's usage count is incremented.
///
/// Java final class → Rust struct+impl.
#[derive(Debug, Clone)]
pub struct NodeInputList {
    /// Corresponds to the `nodes` field.
    node_ids: Vec<usize>,
    /// The owning node id.
    owner_id: usize,
}

impl NodeInputList {
    /// Corresponds to `NodeInputList(Node node, int size)`.
    pub fn new(owner_id: usize, size: usize) -> Self {
        NodeInputList {
            node_ids: vec![0; size],
            owner_id,
        }
    }

    /// Corresponds to `NodeInputList(Node node, T[] nodes)`.
    pub fn from_slice(owner_id: usize, node_ids: &[usize]) -> Self {
        NodeInputList {
            node_ids: node_ids.to_vec(),
            owner_id,
        }
    }

    /// Corresponds to `NodeInputList(Node node, List<T> nodes)`.
    pub fn from_vec(owner_id: usize, node_ids: Vec<usize>) -> Self {
        NodeInputList {
            node_ids,
            owner_id,
        }
    }

    /// Corresponds to `int size()`.
    pub fn size(&self) -> usize {
        self.node_ids.len()
    }

    /// Returns the owning node id.
    pub fn owner_id(&self) -> usize {
        self.owner_id
    }

    /// Returns a slice of node ids.
    pub fn node_ids(&self) -> &[usize] {
        &self.node_ids
    }

    /// Corresponds to `T get(int index)`.
    pub fn get_id(&self, index: usize) -> Option<usize> {
        if index < self.node_ids.len() {
            let id = self.node_ids[index];
            if id == 0 {
                None
            } else {
                Some(id)
            }
        } else {
            None
        }
    }

    /// Corresponds to `void set(int index, T node)`.
    pub fn set_id(&mut self, index: usize, node_id: usize) {
        if index < self.node_ids.len() {
            self.node_ids[index] = node_id;
        }
    }
}

impl NodeList for NodeInputList {
    fn count(&self) -> usize {
        self.node_ids.len()
    }

    fn get(&self, index: usize) -> Option<NodeId> {
        self.node_ids.get(index).copied()
    }

    fn contains(&self, node: NodeId) -> bool {
        self.node_ids.contains(&node)
    }

    fn index_of(&self, node: NodeId) -> Option<usize> {
        self.node_ids.iter().position(|&n| n == node)
    }

    fn is_list(&self) -> bool {
        false
    }
}