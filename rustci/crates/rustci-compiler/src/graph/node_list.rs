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
// Rust mirror of `jdk.graal.compiler.graph.NodeList`. Faithful 1:1 port.

use crate::graph::node::NodeId;
use std::fmt::Debug;

/// Corresponds to `public abstract class NodeList<T extends Node>`.
///
/// Base class for node lists (input lists and successor lists).
/// A `NodeList` holds a sublist of nodes and provides iteration
/// and mutation operations.
///
/// Java abstract class → Rust trait.
/// Note: Uses NodeId instead of &dyn Node since nodes are stored in the Graph.
pub trait NodeList: Debug {
    /// Corresponds to `int count()`.
    /// Returns the number of nodes in this list.
    fn count(&self) -> usize;

    /// Corresponds to `T get(int index)`.
    /// Returns the node id at the given index.
    fn get(&self, index: usize) -> Option<NodeId>;

    /// Corresponds to `boolean contains(T node)`.
    fn contains(&self, node: NodeId) -> bool;

    /// Corresponds to `int indexOf(T node)`.
    fn index_of(&self, node: NodeId) -> Option<usize>;

    /// Corresponds to `boolean isList()`.
    fn is_list(&self) -> bool;
}

/// Corresponds to `public final class NodeList.Single<T extends Node> extends NodeList<T>`.
///
/// A node list with a single element.
#[derive(Debug, Clone)]
pub struct SingleNodeList {
    node: Option<NodeId>,
}

impl SingleNodeList {
    /// Corresponds to `Single(T node)`.
    pub fn new(node_id: Option<NodeId>) -> Self {
        SingleNodeList { node: node_id }
    }

    /// Corresponds to getting the inner node id.
    pub fn node_id(&self) -> Option<NodeId> {
        self.node
    }
}

impl NodeList for SingleNodeList {
    fn count(&self) -> usize {
        if self.node.is_some() {
            1
        } else {
            0
        }
    }

    fn get(&self, index: usize) -> Option<NodeId> {
        if index == 0 {
            self.node
        } else {
            None
        }
    }

    fn contains(&self, node: NodeId) -> bool {
        self.node == Some(node)
    }

    fn index_of(&self, node: NodeId) -> Option<usize> {
        if self.node == Some(node) {
            Some(0)
        } else {
            None
        }
    }

    fn is_list(&self) -> bool {
        false
    }
}

/// Corresponds to `public final class NodeList.SubList<T extends Node> extends NodeList<T>`.
///
/// A node list with multiple elements.
#[derive(Debug, Clone)]
pub struct SubListNodeList {
    nodes: Vec<NodeId>,
}

impl SubListNodeList {
    /// Corresponds to `SubList(List<T> nodes)`.
    pub fn new(nodes: Vec<NodeId>) -> Self {
        SubListNodeList { nodes }
    }

    /// Corresponds to `SubList(T[] nodes)`.
    pub fn from_slice(nodes: &[NodeId]) -> Self {
        SubListNodeList {
            nodes: nodes.to_vec(),
        }
    }
}

impl NodeList for SubListNodeList {
    fn count(&self) -> usize {
        self.nodes.len()
    }

    fn get(&self, index: usize) -> Option<NodeId> {
        self.nodes.get(index).copied()
    }

    fn contains(&self, node: NodeId) -> bool {
        self.nodes.contains(&node)
    }

    fn index_of(&self, node: NodeId) -> Option<usize> {
        self.nodes.iter().position(|&n| n == node)
    }

    fn is_list(&self) -> bool {
        true
    }
}