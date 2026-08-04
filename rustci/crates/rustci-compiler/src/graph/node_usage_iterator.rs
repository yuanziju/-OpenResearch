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
// Rust mirror of `jdk.graal.compiler.graph.NodeUsageIterator`. Faithful 1:1 port.

use crate::graph::node::UsageRecord;
use std::fmt::Debug;

/// Corresponds to `public final class NodeUsageIterator`.
///
/// Iterator over usages of a node. Each usage is a `UsageRecord` containing
/// the using node and the input index where this node is used.
///
/// Java final class → Rust struct+impl.
#[derive(Debug, Clone)]
pub struct NodeUsageIterator {
    /// Corresponds to the `usages` field.
    records: Vec<UsageRecord>,
    /// Current position.
    position: usize,
    /// The node id whose usages are being iterated.
    node_id: usize,
}

impl NodeUsageIterator {
    /// Corresponds to `NodeUsageIterator(Node node)`.
    pub fn new(records: Vec<UsageRecord>, node_id: usize) -> Self {
        NodeUsageIterator {
            records,
            position: 0,
            node_id,
        }
    }

    /// Corresponds to `boolean hasNext()`.
    pub fn has_next(&self) -> bool {
        self.position < self.records.len()
    }

    /// Corresponds to `Node next()`.
    pub fn next(&mut self) -> Option<UsageRecord> {
        if self.position < self.records.len() {
            let record = self.records[self.position];
            self.position += 1;
            Some(record)
        } else {
            None
        }
    }

    /// Corresponds to the node whose usages are being iterated.
    pub fn node_id(&self) -> usize {
        self.node_id
    }

    /// Returns the number of remaining usages.
    pub fn remaining(&self) -> usize {
        self.records.len() - self.position
    }
}

impl Iterator for NodeUsageIterator {
    type Item = UsageRecord;

    fn next(&mut self) -> Option<UsageRecord> {
        if self.position < self.records.len() {
            let record = self.records[self.position];
            self.position += 1;
            Some(record)
        } else {
            None
        }
    }
}