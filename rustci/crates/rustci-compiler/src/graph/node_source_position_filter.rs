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
// Rust mirror of `jdk.graal.compiler.graph.NodeSourcePositionFilter`.
// Faithful 1:1 port.

use crate::graph::node_source_position::NodeSourcePosition;
use std::fmt::Debug;

/// Corresponds to `public class NodeSourcePositionFilter`.
///
/// A filter for matching source positions. Supports filtering by method
/// name, bytecode index range, and call depth.
///
/// Java class → Rust struct+impl.
#[derive(Debug, Clone)]
pub struct NodeSourcePositionFilter {
    /// Corresponds to the `methodPattern` field.
    method_pattern: Option<String>,
    /// Corresponds to the `minBci` field.
    min_bci: i32,
    /// Corresponds to the `maxBci` field.
    max_bci: i32,
    /// Corresponds to the `maxDepth` field.
    max_depth: Option<usize>,
}

impl NodeSourcePositionFilter {
    /// Corresponds to `NodeSourcePositionFilter()`.
    pub fn new() -> Self {
        NodeSourcePositionFilter {
            method_pattern: None,
            min_bci: i32::MIN,
            max_bci: i32::MAX,
            max_depth: None,
        }
    }

    /// Corresponds to `void setMethodPattern(String pattern)`.
    pub fn set_method_pattern(&mut self, pattern: Option<String>) {
        self.method_pattern = pattern;
    }

    /// Corresponds to `void setBciRange(int min, int max)`.
    pub fn set_bci_range(&mut self, min: i32, max: i32) {
        self.min_bci = min;
        self.max_bci = max;
    }

    /// Corresponds to `void setMaxDepth(int depth)`.
    pub fn set_max_depth(&mut self, depth: Option<usize>) {
        self.max_depth = depth;
    }

    /// Corresponds to `boolean matches(NodeSourcePosition pos)`.
    pub fn matches(&self, pos: &NodeSourcePosition) -> bool {
        if let Some(ref pattern) = self.method_pattern {
            if let Some(method) = pos.get_method() {
                if !method.contains(pattern.as_str()) {
                    return false;
                }
            } else {
                return false;
            }
        }
        if pos.bci < self.min_bci || pos.bci > self.max_bci {
            return false;
        }
        if let Some(max_depth) = self.max_depth {
            if pos.get_call_depth() > max_depth {
                return false;
            }
        }
        true
    }
}

impl Default for NodeSourcePositionFilter {
    fn default() -> Self {
        Self::new()
    }
}