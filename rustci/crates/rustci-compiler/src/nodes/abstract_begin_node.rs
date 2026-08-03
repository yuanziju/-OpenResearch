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

//! Mirror `jdk.graal.compiler.nodes.AbstractBeginNode`: abstract base class for begin nodes.
//!
//! Deviation: Java `AbstractBeginNode` extends `FixedWithNextNode` and implements `LIRLowerable`,
//! `GuardingNode`, `AnchoringNode`, `IterableNodeType`. Rust side is a trait.

use crate::nodes::fixed_node::FixedNode;
use crate::nodes::fixed_with_next_node::FixedWithNextNode;
use crate::nodes::guarding_node::GuardingNode;

/// Mirrors `abstract class AbstractBeginNode extends FixedWithNextNode
/// implements LIRLowerable, GuardingNode, AnchoringNode, IterableNodeType`.
///
/// Abstract base class for all begin nodes. Used as guard anchor and anchoring node.
pub trait AbstractBeginNode: FixedWithNextNode + GuardingNode {
    /// Mirrors `prevBegin(FixedNode)`: find the nearest AbstractBeginNode preceding the given node.
    fn prev_begin(from: &dyn FixedNode) -> Option<&dyn AbstractBeginNode>
    where
        Self: Sized;

    /// Mirrors `isUsedAsGuardInput()`: whether this node is used as a guard input.
    fn is_used_as_guard_input(&self) -> bool;

    /// Mirrors `hasSpeculationFence()`: whether this node has a speculation fence.
    fn has_speculation_fence(&self) -> bool;

    /// Mirrors `prepareDelete()`: prepare to delete this node, evacuating anchored values.
    fn prepare_delete(&mut self) {}

    /// Mirrors `prepareDelete(FixedNode)`: prepare delete with evacuation starting point.
    fn prepare_delete_from(&mut self, _evacuate_from: &dyn FixedNode) {}

    /// Mirrors `verifyNode()`: verify predecessor is not null or is graph.start() or AbstractMergeNode.
    fn verify_node(&self) -> bool {
        true
    }

    /// Mirrors `setHasSpeculationFence()`: set speculation fence flag.
    fn set_has_speculation_fence(&mut self) {}

    /// Mirrors `mustNotMoveAttachedGuards()`: whether the optimizer may move attached guards.
    fn must_not_move_attached_guards(&self) -> bool {
        false
    }
}