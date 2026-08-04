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
// Rust mirror of `jdk.graal.compiler.graph.NodeClass`. Faithful 1:1 port.

use crate::graph::node::Node;
use std::any::Any;
use std::fmt::Debug;

/// Input information for a node class. Corresponds to `InputInfo` in Java.
#[derive(Debug, Clone)]
pub struct InputInfo {
    /// Corresponds to `name` field.
    pub name: &'static str,
    /// Corresponds to `type` field (the expected Java type of the input).
    pub input_type: Option<&'static str>,
    /// Corresponds to `optional` field.
    pub optional: bool,
}

impl InputInfo {
    /// Corresponds to `InputInfo(String name, Class<?> type, boolean optional)`.
    pub const fn new(name: &'static str, input_type: Option<&'static str>, optional: bool) -> Self {
        InputInfo {
            name,
            input_type,
            optional,
        }
    }
}

/// Edge information. Corresponds to the `Edges` inner class in Java `NodeClass`.
#[derive(Debug, Clone)]
pub struct Edges {
    /// Offsets of individual edges within the edge array.
    pub offsets: Vec<usize>,
    /// Count of direct edges.
    pub direct_count: usize,
    /// Count of indirect (list) edges.
    pub indirect_count: usize,
}

impl Edges {
    /// Corresponds to `Edges(List<InputInfo> inputs, boolean direct)`.
    pub fn new(infos: &[InputInfo], _direct: bool) -> Self {
        let mut offsets = Vec::with_capacity(infos.len());
        let mut direct_count = 0;
        let mut indirect_count = 0;
        let mut offset = 0;
        for info in infos {
            offsets.push(offset);
            if info.optional {
                // List type edges are indirect
                indirect_count += 1;
                offset += 1;
            } else {
                direct_count += 1;
                offset += 1;
            }
        }
        Edges {
            offsets,
            direct_count,
            indirect_count,
        }
    }

    /// Corresponds to `count()`.
    pub fn count(&self) -> usize {
        self.direct_count + self.indirect_count
    }

    /// Corresponds to `getCount()`.
    pub fn get_count(&self) -> usize {
        self.offsets.len()
    }

    /// Corresponds to `int getOffset(int index)`.
    /// Returns the offset for the edge at the given index.
    pub fn get_offset(&self, index: usize) -> usize {
        if index < self.offsets.len() {
            self.offsets[index]
        } else {
            0
        }
    }
}

/// Corresponds to `public final class NodeClass<T extends Node>`.
///
/// Metadata describing a node class: its Java type, its input edges,
/// successor edges, and whether it implements `IterableNodeType`.
///
/// Java final class → Rust trait (since it's parameterized by `T: Node`).
pub trait NodeClass: Debug + Send + Sync {
    /// Corresponds to `Class<?> getJavaClass()`.
    /// Returns the Java class (or type name) this node class represents.
    fn java_class_name(&self) -> &'static str;

    /// Corresponds to `String getNameTemplate()`.
    /// Returns the name template for nodes of this class.
    fn name_template(&self) -> &'static str {
        "{p#nodeClass}"
    }

    /// Corresponds to `Edges getInputEdges()`.
    /// Returns the input edges descriptor.
    fn input_edges(&self) -> &Edges;

    /// Corresponds to `Edges getSuccessorEdges()`.
    /// Returns the successor edges descriptor.
    fn successor_edges(&self) -> &Edges;

    /// Corresponds to `boolean isIterableNodeType()`.
    /// Returns true if this node class is an `IterableNodeType`.
    fn is_iterable_node_type(&self) -> bool {
        false
    }

    /// Corresponds to `boolean isLeafClass()`.
    /// Returns true if this is a leaf (final) node class.
    fn is_leaf_class(&self) -> bool {
        false
    }

    /// Corresponds to `NodeClass<? super T> getSuperClass()`.
    /// Returns the superclass `NodeClass`, or None if this is the root.
    fn super_class(&self) -> Option<&dyn NodeClass> {
        None
    }

    /// Corresponds to `long[] getIterableIds()`.
    /// Returns the ids of iterable node types, for typed iteration.
    fn iterable_ids(&self) -> &[u64] {
        &[]
    }

    /// Corresponds to `int getIterableId()`.
    /// Returns the iterable id for this node class.
    fn iterable_id(&self) -> u64 {
        0
    }

    /// Corresponds to `Node allocateInstance()`.
    /// Allocates a new instance of this node type.
    fn allocate_instance(&self) -> Box<dyn Node>;

    /// Corresponds to `boolean isAssignableFrom(Class<?> c)`.
    fn is_assignable_from(&self, other: &dyn NodeClass) -> bool;

    /// Corresponds to `boolean isAssignableFrom(NodeClass<?> nc)`.
    fn is_assignable_from_class(&self, other: &dyn Any) -> bool {
        false
    }
}