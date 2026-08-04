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
// Rust mirror of `jdk.graal.compiler.graph.NodeBitMap`. Faithful 1:1 port.

use crate::graph::node::NodeId;
use std::fmt::Debug;

/// Corresponds to `public final class NodeBitMap extends NodeIdIterator`.
///
/// A bitmap for marking nodes in a graph. Nodes are identified by their
/// id (which is the index in the graph's node array). Supports set, clear,
/// test, and iteration over marked nodes.
///
/// Java final class → Rust struct+impl.
#[derive(Debug, Clone)]
pub struct NodeBitMap {
    /// Bit storage: each u64 holds 64 bits.
    bits: Vec<u64>,
    /// Number of bits (nodes) in the bitmap.
    size: usize,
    /// The graph id this bitmap belongs to.
    graph_id: usize,
    /// Counter for the number of marked bits.
    marked_count: usize,
}

impl NodeBitMap {
    /// Word size in bits.
    const WORD_BITS: usize = 64;

    /// Corresponds to `NodeBitMap(Graph graph)`.
    pub fn new(graph_id: usize, node_count: usize) -> Self {
        let words = (node_count + Self::WORD_BITS - 1) / Self::WORD_BITS;
        NodeBitMap {
            bits: vec![0u64; words],
            size: node_count,
            graph_id,
            marked_count: 0,
        }
    }

    /// Corresponds to `boolean isMarked(Node node)`.
    pub fn is_marked(&self, node_id: NodeId) -> bool {
        let word = node_id / Self::WORD_BITS;
        let bit = node_id % Self::WORD_BITS;
        if word < self.bits.len() {
            (self.bits[word] & (1u64 << bit)) != 0
        } else {
            false
        }
    }

    /// Corresponds to `boolean isNewMarked(Node node)`.
    /// Like `isMarked`, but also marks the node. Returns true if
    /// the node was already marked.
    pub fn is_new_marked(&mut self, node_id: NodeId) -> bool {
        let word = self.word_index(node_id);
        let bit = node_id % Self::WORD_BITS;
        let mask = 1u64 << bit;
        if word < self.bits.len() {
            if (self.bits[word] & mask) != 0 {
                true
            } else {
                self.bits[word] |= mask;
                self.marked_count += 1;
                false
            }
        } else {
            self.grow(node_id);
            self.bits[word] |= mask;
            self.marked_count += 1;
            false
        }
    }

    /// Corresponds to `void mark(Node node)`.
    pub fn mark(&mut self, node_id: NodeId) {
        let word = self.word_index(node_id);
        let bit = node_id % Self::WORD_BITS;
        let mask = 1u64 << bit;
        if word < self.bits.len() {
            if (self.bits[word] & mask) == 0 {
                self.bits[word] |= mask;
                self.marked_count += 1;
            }
        } else {
            self.grow(node_id);
            self.bits[word] |= mask;
            self.marked_count += 1;
        }
    }

    /// Corresponds to `void clear(Node node)`.
    pub fn clear(&mut self, node_id: NodeId) {
        let word = self.word_index(node_id);
        let bit = node_id % Self::WORD_BITS;
        let mask = 1u64 << bit;
        if word < self.bits.len() {
            if (self.bits[word] & mask) != 0 {
                self.bits[word] &= !mask;
                self.marked_count -= 1;
            }
        }
    }

    /// Corresponds to `void clearAll()`.
    pub fn clear_all(&mut self) {
        for word in &mut self.bits {
            *word = 0;
        }
        self.marked_count = 0;
    }

    /// Corresponds to `void grow()`.
    pub fn grow(&mut self, node_id: NodeId) {
        let new_words = (node_id + Self::WORD_BITS) / Self::WORD_BITS;
        if new_words > self.bits.len() {
            self.bits.resize(new_words, 0);
            self.size = node_id + 1;
        }
    }

    /// Corresponds to `int count()`.
    pub fn count(&self) -> usize {
        self.marked_count
    }

    /// Corresponds to `boolean isNotEmpty()`.
    pub fn is_not_empty(&self) -> bool {
        self.marked_count > 0
    }

    /// Corresponds to `boolean isEmpty()`.
    pub fn is_empty(&self) -> bool {
        self.marked_count == 0
    }

    /// Corresponds to `void intersect(NodeBitMap other)`.
    pub fn intersect(&mut self, other: &NodeBitMap) {
        let len = self.bits.len().min(other.bits.len());
        self.marked_count = 0;
        for i in 0..len {
            self.bits[i] &= other.bits[i];
            self.marked_count += self.bits[i].count_ones() as usize;
        }
    }

    /// Corresponds to `void setAll(NodeBitMap other)`.
    pub fn set_all(&mut self, other: &NodeBitMap) {
        let len = other.bits.len();
        if self.bits.len() < len {
            self.bits.resize(len, 0);
        }
        self.marked_count = 0;
        for i in 0..len {
            self.bits[i] = other.bits[i];
            self.marked_count += self.bits[i].count_ones() as usize;
        }
    }

    /// Corresponds to `void union(NodeBitMap other)`.
    pub fn union(&mut self, other: &NodeBitMap) {
        let len = other.bits.len();
        if self.bits.len() < len {
            self.bits.resize(len, 0);
        }
        self.marked_count = 0;
        for i in 0..len {
            self.bits[i] |= other.bits[i];
            self.marked_count += self.bits[i].count_ones() as usize;
        }
    }

    /// Corresponds to `void invert()`.
    pub fn invert(&mut self) {
        for word in &mut self.bits {
            *word = !*word;
        }
        self.marked_count = self.bits.len() * Self::WORD_BITS - self.marked_count;
    }

    /// Corresponds to `NodeBitMap copy()`.
    /// Creates a deep copy of this bitmap.
    pub fn copy(&self) -> Self {
        NodeBitMap {
            bits: self.bits.clone(),
            size: self.size,
            graph_id: self.graph_id,
            marked_count: self.marked_count,
        }
    }

    /// Corresponds to `void and(NodeBitMap other)`.
    /// Performs bitwise AND with another bitmap and stores the result.
    pub fn and(&mut self, other: &NodeBitMap) {
        self.intersect(other)
    }

    /// Returns the graph id.
    pub fn graph_id(&self) -> usize {
        self.graph_id
    }

    /// Returns an iterator over marked node ids.
    pub fn iter_marked(&self) -> NodeBitMapIter {
        NodeBitMapIter {
            bitmap: self,
            word_idx: 0,
            bit_idx: 0,
        }
    }

    fn word_index(&self, node_id: NodeId) -> usize {
        node_id / Self::WORD_BITS
    }
}

/// Iterator over marked nodes in a `NodeBitMap`.
#[derive(Debug, Clone)]
pub struct NodeBitMapIter<'a> {
    bitmap: &'a NodeBitMap,
    word_idx: usize,
    bit_idx: usize,
}

impl<'a> Iterator for NodeBitMapIter<'a> {
    type Item = NodeId;

    fn next(&mut self) -> Option<NodeId> {
        while self.word_idx < self.bitmap.bits.len() {
            let word = self.bitmap.bits[self.word_idx];
            while self.bit_idx < NodeBitMap::WORD_BITS {
                let bit = self.bit_idx;
                self.bit_idx += 1;
                if (word & (1u64 << bit)) != 0 {
                    return Some(self.word_idx * NodeBitMap::WORD_BITS + bit);
                }
            }
            self.word_idx += 1;
            self.bit_idx = 0;
        }
        None
    }
}