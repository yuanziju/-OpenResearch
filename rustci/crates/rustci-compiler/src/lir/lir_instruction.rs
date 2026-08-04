// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2019, 2024, Oracle and/or its affiliates. All rights reserved.
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

//! 镜像 `jdk.graal.compiler.lir.LIRInstruction`：LIR 指令基类。
//!
//! 偏离记录：Java `abstract class LIRInstruction` → Rust trait。
//! LIR 指令是 LIR 图中的基本操作单元，每条指令有操作码、输入/输出/临时操作数。

use std::any::Any;
use std::fmt::Debug;

use crate::lir::lir_frame_state::LIRFrameState;
use crate::lir::value_procedure::{
    InstructionStateProcedure, InstructionValueConsumer, InstructionValueProcedure,
};

/// 对应 `abstract class LIRInstruction`。
///
/// LIR 指令抽象基类。每条指令有唯一 ID、操作码，以及可选帧状态。
pub trait LIRInstruction: Debug {
    /// 对应 `id()`：获取指令 ID。
    fn id(&self) -> i32;

    /// 对应 `opcode()`：获取操作码。
    fn opcode(&self) -> &str;

    /// 对应 `name()`：获取指令名称（用于调试打印）。
    fn name(&self) -> &str;

    /// 对应 `hasState()`：是否有帧状态。
    fn has_state(&self) -> bool;

    /// 对应 `forEachState(InstructionStateProcedure)`：遍历帧状态。
    fn for_each_state(&self, proc: &mut dyn InstructionStateProcedure);

    /// 对应 `forEachInput(InstructionValueConsumer)`：遍历所有输入操作数。
    fn for_each_input(&self, proc: &mut dyn InstructionValueConsumer);

    /// 对应 `forEachAlive(InstructionValueConsumer)`：遍历所有活跃操作数。
    fn for_each_alive(&self, proc: &mut dyn InstructionValueConsumer);

    /// 对应 `forEachStatePos(InstructionValueConsumer)`：遍历所有状态相关位置。
    fn for_each_state_pos(&self, proc: &mut dyn InstructionValueConsumer);

    /// 对应 `forEachTemp(InstructionValueConsumer)`：遍历所有临时操作数。
    fn for_each_temp(&self, proc: &mut dyn InstructionValueConsumer);

    /// 对应 `forEachOutput(InstructionValueConsumer)`：遍历所有输出操作数。
    fn for_each_output(&self, proc: &mut dyn InstructionValueConsumer);

    /// 对应 `forEachValue(InstructionValueProcedure)`：遍历所有值并可按过程修改。
    fn for_each_value(&self, proc: &mut dyn InstructionValueProcedure);

    /// 对应 `hasOperands()`：是否有操作数。
    fn has_operands(&self) -> bool;

    /// 对应 `asAny()`：支持下转型。
    fn as_any(&self) -> &dyn Any;

    /// 对应 `asAnyMut()`：支持可变下转型。
    fn as_any_mut(&mut self) -> &mut dyn Any;

    /// 对应 `cloneBox()`：深拷贝指令。
    fn clone_box(&self) -> Box<dyn LIRInstruction>;
}

/// 对应 `abstract class LIRInstruction` 的默认实现辅助。
///
/// 提供默认的遍历方法实现，具体指令类型只需覆写相关方法。
pub struct LIRInstructionBase {
    /// 指令 ID。
    id: i32,
    /// 操作码。
    opcode: String,
    /// 指令名称。
    name: String,
    /// 帧状态。
    state: Option<LIRFrameState>,
}

impl LIRInstructionBase {
    /// 创建指令基类。
    pub fn new(id: i32, opcode: &str, name: &str) -> Self {
        Self {
            id,
            opcode: opcode.to_string(),
            name: name.to_string(),
            state: None,
        }
    }

    /// 获取指令 ID。
    pub fn get_id(&self) -> i32 {
        self.id
    }

    /// 获取操作码。
    pub fn get_opcode(&self) -> &str {
        &self.opcode
    }

    /// 获取指令名称。
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// 获取帧状态引用。
    pub fn get_state(&self) -> Option<&LIRFrameState> {
        self.state.as_ref()
    }

    /// 获取帧状态可变引用。
    pub fn get_state_mut(&mut self) -> Option<&mut LIRFrameState> {
        self.state.as_mut()
    }

    /// 设置帧状态。
    pub fn set_state(&mut self, state: LIRFrameState) {
        self.state = Some(state);
    }

    /// 清除帧状态。
    pub fn clear_state(&mut self) {
        self.state = None;
    }

    /// 检查是否有帧状态。
    pub fn has_state(&self) -> bool {
        self.state.is_some()
    }

    /// 遍历帧状态。
    pub fn for_each_state_base(&self, proc: &mut dyn InstructionStateProcedure) {
        if let Some(ref state) = self.state {
            // 通过 self 指针传递，调用者需提供具体指令引用
            proc.do_state(&DummyInstruction, state);
        }
    }
}

impl fmt::Debug for LIRInstructionBase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}(id={})", self.name, self.id)
    }
}

use std::fmt;

/// 用于 `for_each_state_base` 的占位指令（遍历时不需要具体指令类型）。
struct DummyInstruction;

impl Debug for DummyInstruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DummyInstruction")
    }
}

impl LIRInstruction for DummyInstruction {
    fn id(&self) -> i32 { 0 }
    fn opcode(&self) -> &str { "DUMMY" }
    fn name(&self) -> &str { "DUMMY" }
    fn has_state(&self) -> bool { false }
    fn for_each_state(&self, _proc: &mut dyn InstructionStateProcedure) {}
    fn for_each_input(&self, _proc: &mut dyn InstructionValueConsumer) {}
    fn for_each_alive(&self, _proc: &mut dyn InstructionValueConsumer) {}
    fn for_each_state_pos(&self, _proc: &mut dyn InstructionValueConsumer) {}
    fn for_each_temp(&self, _proc: &mut dyn InstructionValueConsumer) {}
    fn for_each_output(&self, _proc: &mut dyn InstructionValueConsumer) {}
    fn for_each_value(&self, _proc: &mut dyn InstructionValueProcedure) {}
    fn has_operands(&self) -> bool { false }
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn clone_box(&self) -> Box<dyn LIRInstruction> { Box::new(DummyInstruction) }
}