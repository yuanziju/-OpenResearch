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

//! 镜像 `jdk.graal.compiler.nodes.DeoptimizingNode`：可去优化的节点接口。
//!
//! 偏离记录：Java `DeoptimizingNode` 是接口（继承 `NodeWithState`），包含三个子接口
//! `DeoptBefore`、`DeoptAfter`、`DeoptDuring`。Rust 侧用 trait 表示。

use crate::nodes::frame_state::FrameState;

/// 对应 `interface DeoptimizingNode extends NodeWithState`。
///
/// 可能需要去优化信息的节点所实现的接口。
pub trait DeoptimizingNode {
    /// 对应 `canDeoptimize()`：判断本节点是否需要去优化信息。
    fn can_deoptimize(&self) -> bool;

    /// 对应 `validateDeoptFrameStates()`：判断是否必须验证 FrameState 的去优化有效性。
    fn validate_deopt_frame_states(&self) -> bool {
        true
    }
}

/// 对应 `interface DeoptBefore extends DeoptimizingNode`。
///
/// 需要在执行前去优化的节点接口。
pub trait DeoptBefore: DeoptimizingNode {
    /// 对应 `setStateBefore(FrameState)`：设置执行前的 FrameState。
    fn set_state_before(&mut self, state: FrameState);

    /// 对应 `stateBefore()`：获取执行前的 FrameState。
    fn state_before(&self) -> Option<&FrameState>;

    /// 对应 `canUseAsStateDuring()`：能否用作执行中的 FrameState。
    fn can_use_as_state_during(&self) -> bool {
        false
    }
}

/// 对应 `interface DeoptAfter extends DeoptimizingNode, StateSplit`。
///
/// 需要在执行后去优化的节点接口。
pub trait DeoptAfter: DeoptimizingNode {
    /// 获取执行后的 FrameState。
    fn state_after(&self) -> Option<&FrameState>;

    /// 设置执行后的 FrameState。
    fn set_state_after(&mut self, state: FrameState);
}

/// 对应 `interface DeoptDuring extends DeoptimizingNode, StateSplit`。
///
/// 需要在执行中去优化的节点接口（如 Invoke）。
pub trait DeoptDuring: DeoptimizingNode {
    /// 对应 `stateDuring()`：获取执行中的 FrameState。
    fn state_during(&self) -> Option<&FrameState>;

    /// 对应 `setStateDuring(FrameState)`：设置执行中的 FrameState。
    fn set_state_during(&mut self, state: FrameState);

    /// 对应 `computeStateDuring(FrameState)`：从执行后的 FrameState 计算执行中的 FrameState。
    fn compute_state_during(&mut self, state_after: &FrameState);
}
