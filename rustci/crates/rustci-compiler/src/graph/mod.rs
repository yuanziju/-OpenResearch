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
// Rust mirror of `jdk.graal.compiler.graph` — module declarations for the
// 19 root-level classes/interfaces. Faithful 1:1 port of the Graal IR graph
// infrastructure: Node, Graph, NodeClass, edge lists, bitmaps, iterators,
// and traversal utilities.

pub mod graal_graph_error;
pub mod graph;
pub mod iterable_node_type;
pub mod node;
pub mod node_bit_map;
pub mod node_class;
pub mod node_flood;
pub mod node_input_list;
pub mod node_list;
pub mod node_source_position;
pub mod node_source_position_filter;
pub mod node_successor_list;
pub mod node_union_find;
pub mod node_usage_iterator;
pub mod node_usage_with_count_iterator;
pub mod node_work_list;
pub mod position;
pub mod typed_graph_node_iterator;

pub use graal_graph_error::GraalGraphError;
pub use graph::{Graph, GraphId};
pub use iterable_node_type::IterableNodeType;
pub use node::{ModCounts, Node, NodeId, UsageRecord, Verbosity};
pub use node_bit_map::{NodeBitMap, NodeBitMapIter};
pub use node_class::{Edges, InputInfo, NodeClass};
pub use node_flood::NodeFlood;
pub use node_input_list::NodeInputList;
pub use node_list::{NodeList, SingleNodeList, SubListNodeList};
pub use node_source_position::NodeSourcePosition;
pub use node_source_position_filter::NodeSourcePositionFilter;
pub use node_successor_list::NodeSuccessorList;
pub use node_union_find::NodeUnionFind;
pub use node_usage_iterator::NodeUsageIterator;
pub use node_usage_with_count_iterator::NodeUsageWithCountIterator;
pub use node_work_list::NodeWorkList;
pub use position::{InputPosition, Position, SuccessorPosition};
pub use typed_graph_node_iterator::TypedGraphNodeIterator;