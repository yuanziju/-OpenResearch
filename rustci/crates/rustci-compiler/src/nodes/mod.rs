// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2024, 2026, Oracle and/or its affiliates. All rights reserved.
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

//! Rust mirror of `jdk.graal.compiler.nodes` core subset (~30 classes).
//!
//! Faithful 1:1 port of the Graal IR node hierarchy: root-level abstract nodes,
//! basic control-flow nodes, value nodes, frame state, structured graph, graph
//! encoding/decoding, and optimization logging.

pub mod abstract_begin_node;
pub mod abstract_end_node;
pub mod abstract_merge_node;
pub mod begin_node;
pub mod constant_node;
pub mod control_sink_node;
pub mod control_split_node;
pub mod deoptimizing_node;
pub mod end_node;
pub mod fixed_node;
pub mod fixed_with_next_node;
pub mod frame_state;
pub mod graph_decoder;
pub mod graph_encoder;
pub mod graph_state;
pub mod guard_node;
pub mod guarding_node;
pub mod if_node;
pub mod inlining_log;
pub mod loop_begin_node;
pub mod loop_end_node;
pub mod loop_exit_node;
pub mod merge_node;
pub mod optimization_log;
pub mod optimization_log_impl;
pub mod parameter_node;
pub mod pi_node;
pub mod return_node;
pub mod simplifying_graph_decoder;
pub mod start_node;
pub mod structured_graph;
pub mod value_node;

pub use abstract_begin_node::AbstractBeginNode;
pub use abstract_end_node::AbstractEndNode;
pub use abstract_merge_node::AbstractMergeNode;
pub use begin_node::BeginNode;
pub use constant_node::ConstantNode;
pub use control_sink_node::ControlSinkNode;
pub use control_split_node::ControlSplitNode;
pub use deoptimizing_node::DeoptimizingNode;
pub use end_node::EndNode;
pub use fixed_node::FixedNode;
pub use fixed_with_next_node::FixedWithNextNode;
pub use frame_state::FrameState;
pub use graph_decoder::GraphDecoder;
pub use graph_encoder::GraphEncoder;
pub use graph_state::GraphState;
pub use guard_node::GuardNode;
pub use guarding_node::GuardingNode;
pub use if_node::IfNode;
pub use inlining_log::InliningLog;
pub use loop_begin_node::LoopBeginNode;
pub use loop_end_node::LoopEndNode;
pub use loop_exit_node::LoopExitNode;
pub use merge_node::MergeNode;
pub use optimization_log::OptimizationLog;
pub use optimization_log_impl::OptimizationLogImpl;
pub use parameter_node::ParameterNode;
pub use pi_node::PiNode;
pub use return_node::ReturnNode;
pub use simplifying_graph_decoder::SimplifyingGraphDecoder;
pub use start_node::StartNode;
pub use structured_graph::StructuredGraph;
pub use value_node::ValueNode;
