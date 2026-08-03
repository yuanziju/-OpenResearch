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

//! 镜像 `jdk.graal.compiler.nodes.FrameState`：帧状态节点。
//!
//! 偏离记录：Java `FrameState` 继承 `VirtualState` 并实现 `IterableNodeType`，
//! 包含局部变量、操作数栈、锁对象等完整帧状态信息。Rust 侧为具体 struct。

use rustci_vm_ci::meta::java_kind::JavaKind;

use crate::nodes::value_node::ValueNode;

/// 对应 `FrameState.StackState` 枚举：表达式栈状态。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StackState {
    /// 当前字节码的参数在栈顶。
    BeforePop,
    /// 参数已弹出，但结果尚未压入。
    AfterPop,
    /// 所有参数已弹出，栈顶为待重新抛出的异常。
    Rethrow,
}

impl StackState {
    /// 对应 `StackState.of(boolean, boolean)`。
    pub fn of(during_call: bool, rethrow_exception: bool) -> Self {
        if rethrow_exception {
            StackState::Rethrow
        } else if during_call {
            StackState::AfterPop
        } else {
            StackState::BeforePop
        }
    }
}

/// 对应 `final class FrameState extends VirtualState implements IterableNodeType`。
///
/// 封装特定抽象解释点的帧状态（局部变量、操作数栈和锁对象）。
/// 可用作调试或去优化信息。
#[derive(Debug)]
pub struct FrameState {
    /// 对应 `bci`：此帧状态对应的字节码索引。
    pub bci: i32,
    /// 对应 `localsSize`：局部变量数量。
    pub locals_size: u16,
    /// 对应 `stackSize`：表达式栈条目数。
    pub stack_size: u16,
    /// 对应 `locksSize`：锁对象数量。
    pub locks_size: u16,
    /// 对应 `stackState`：表达式栈状态。
    pub stack_state: StackState,
    /// 对应 `validForDeoptimization`：是否表示有效的去优化状态。
    pub valid_for_deoptimization: bool,
    /// 对应 `outerFrameState`：外层帧状态（内联时）。
    pub outer_frame_state: Option<Box<FrameState>>,
    /// 对应 `values`：帧状态值（局部变量 + 栈 + 锁）。
    pub values: Vec<Option<Box<dyn ValueNode>>>,
    /// 对应 `virtualObjectMappings`：虚拟对象映射。
    pub virtual_object_mappings: Vec<Box<dyn ValueNode>>,
}

impl FrameState {
    /// 创建一个新的 FrameState。
    pub fn new(
        bci: i32,
        locals_size: u16,
        stack_size: u16,
        locks_size: u16,
        stack_state: StackState,
        valid_for_deoptimization: bool,
    ) -> Self {
        let total_size = locals_size as usize + stack_size as usize + locks_size as usize;
        FrameState {
            bci,
            locals_size,
            stack_size,
            locks_size,
            stack_state,
            valid_for_deoptimization,
            outer_frame_state: None,
            values: {
                let mut v = Vec::with_capacity(total_size);
                v.resize_with(total_size, || None);
                v
            },
            virtual_object_mappings: Vec::new(),
        }
    }

    /// 对应 `localAt(int)`：获取指定索引的局部变量。
    pub fn local_at(&self, index: usize) -> Option<&dyn ValueNode> {
        if index < self.locals_size as usize {
            self.values[index].as_ref().map(|v| v.as_ref())
        } else {
            None
        }
    }

    /// 对应 `stackAt(int)`：获取指定索引的栈值。
    pub fn stack_at(&self, index: usize) -> Option<&dyn ValueNode> {
        let offset = self.locals_size as usize;
        if index < self.stack_size as usize {
            self.values[offset + index].as_ref().map(|v| v.as_ref())
        } else {
            None
        }
    }

    /// 对应 `lockAt(int)`：获取指定索引的锁。
    pub fn lock_at(&self, index: usize) -> Option<&dyn ValueNode> {
        let offset = self.locals_size as usize + self.stack_size as usize;
        if index < self.locks_size as usize {
            self.values[offset + index].as_ref().map(|v| v.as_ref())
        } else {
            None
        }
    }

    /// 对应 `setLocalAt(int, ValueNode)`：设置局部变量。
    pub fn set_local_at(&mut self, index: usize, value: Option<Box<dyn ValueNode>>) {
        if index < self.locals_size as usize {
            self.values[index] = value;
        }
    }

    /// 对应 `setStackAt(int, ValueNode)`：设置栈值。
    pub fn set_stack_at(&mut self, index: usize, value: Option<Box<dyn ValueNode>>) {
        let offset = self.locals_size as usize;
        if index < self.stack_size as usize {
            self.values[offset + index] = value;
        }
    }

    /// 对应 `setLockAt(int, ValueNode)`：设置锁。
    pub fn set_lock_at(&mut self, index: usize, value: Option<Box<dyn ValueNode>>) {
        let offset = self.locals_size as usize + self.stack_size as usize;
        if index < self.locks_size as usize {
            self.values[offset + index] = value;
        }
    }

    /// 对应 `isValidForDeoptimization()`：是否对去优化有效。
    pub fn is_valid_for_deoptimization(&self) -> bool {
        self.valid_for_deoptimization
    }

    /// 对应 `setValidForDeoptimization(boolean)`：设置去优化有效性。
    pub fn set_valid_for_deoptimization(&mut self, valid: bool) {
        self.valid_for_deoptimization = valid;
    }

    /// 对应 `outerFrameState()`：获取外层帧状态。
    pub fn outer_frame_state(&self) -> Option<&FrameState> {
        self.outer_frame_state.as_ref().map(|f| f.as_ref())
    }

    /// 对应 `setOuterFrameState(FrameState)`：设置外层帧状态。
    pub fn set_outer_frame_state(&mut self, outer: FrameState) {
        self.outer_frame_state = Some(Box::new(outer));
    }

    /// 创建一个占位帧状态。
    pub fn placeholder(bci: i32) -> Self {
        FrameState::new(bci, 0, 0, 0, StackState::BeforePop, true)
    }

    /// 对应 `create(FrameState, int, int, int, boolean)`：创建帧状态的工厂方法。
    pub fn create(
        outer: Option<&FrameState>,
        bci: i32,
        locals_size: u16,
        stack_size: u16,
        locks_size: u16,
        rethrow_exception: bool,
    ) -> Self {
        let stack_state = StackState::of(false, rethrow_exception);
        let mut fs = FrameState::new(bci, locals_size, stack_size, locks_size, stack_state, true);
        if let Some(_outer_fs) = outer {
            // In the full implementation, this clones the outer FrameState.
            // For now, we skip cloning since Box<dyn ValueNode> is not Clone.
            fs.outer_frame_state = None;
        }
        fs
    }

    /// 对应 `create(int, int, int, int, boolean)`：另一个工厂方法（无外层帧状态）。
    pub fn create_simple(
        bci: i32,
        locals_size: u16,
        stack_size: u16,
        locks_size: u16,
        rethrow_exception: bool,
    ) -> Self {
        FrameState::create(None, bci, locals_size, stack_size, locks_size, rethrow_exception)
    }

    /// 对应 `hasExactlyOneUsage()`：是否有恰好一个用法。
    pub fn has_exactly_one_usage(&self) -> bool {
        // In the full implementation, this checks the usage count in the graph.
        // FrameState nodes typically have one usage from a state split node.
        true
    }

    /// 对应 `hasNoUsages()`：是否无用法。
    pub fn has_no_usages(&self) -> bool {
        // In the full implementation, this checks the usage count in the graph.
        false
    }

    /// 对应 `topFrameSize()`：顶层帧大小（局部变量 + 栈 + 锁）。
    pub fn top_frame_size(&self) -> u16 {
        self.locals_size + self.stack_size + self.locks_size
    }
}

impl ValueNode for FrameState {
    fn get_stack_kind(&self) -> JavaKind {
        JavaKind::Void
    }
}
