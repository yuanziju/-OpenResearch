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
// Rust mirror of `jdk.graal.compiler.graph.NodeSourcePosition`.
// Faithful 1:1 port.

use std::fmt::Debug;

/// Corresponds to `public class NodeSourcePosition extends BytecodePosition`.
///
/// Represents the source position of a node in the compilation graph.
/// Contains the method, bytecode index, caller source position, and
/// whether the position has been substituted.
///
/// Java class → Rust struct+impl.
#[derive(Debug, Clone)]
pub struct NodeSourcePosition {
    /// Corresponds to the `method` field.
    pub method: Option<String>,
    /// Corresponds to the `bci` field (bytecode index).
    pub bci: i32,
    /// Corresponds to the `caller` field.
    pub caller: Option<Box<NodeSourcePosition>>,
    /// Corresponds to the `substituted` field.
    pub substituted: bool,
}

impl NodeSourcePosition {
    /// Corresponds to `NodeSourcePosition(NodeSourcePosition caller, ResolvedJavaMethod method, int bci)`.
    pub fn new(caller: Option<NodeSourcePosition>, method: Option<String>, bci: i32) -> Self {
        NodeSourcePosition {
            method,
            bci,
            caller: caller.map(Box::new),
            substituted: false,
        }
    }

    /// Corresponds to `ResolvedJavaMethod getMethod()`.
    pub fn get_method(&self) -> Option<&str> {
        self.method.as_deref()
    }

    /// Corresponds to `int getBCI()`.
    pub fn get_bci(&self) -> i32 {
        self.bci
    }

    /// Corresponds to `NodeSourcePosition getCaller()`.
    pub fn get_caller(&self) -> Option<&NodeSourcePosition> {
        self.caller.as_deref()
    }

    /// Corresponds to `boolean isSubstituted()`.
    pub fn is_substituted(&self) -> bool {
        self.substituted
    }

    /// Corresponds to `void setSubstituted(boolean substituted)`.
    pub fn set_substituted(&mut self, substituted: bool) {
        self.substituted = substituted;
    }

    /// Corresponds to `int getCallDepth()`.
    pub fn get_call_depth(&self) -> usize {
        let mut depth = 0;
        let mut current = self;
        while let Some(ref caller) = current.caller {
            depth += 1;
            current = caller;
        }
        depth
    }

    /// Corresponds to `boolean equals(Object obj)`.
    pub fn source_equals(&self, other: &NodeSourcePosition) -> bool {
        self.method == other.method
            && self.bci == other.bci
            && self.substituted == other.substituted
            && match (&self.caller, &other.caller) {
                (Some(a), Some(b)) => a.source_equals(b),
                (None, None) => true,
                _ => false,
            }
    }
}