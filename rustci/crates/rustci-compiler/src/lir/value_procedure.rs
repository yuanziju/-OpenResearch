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

//! 镜像 `jdk.graal.compiler.lir.ValueConsumer` / `ValueProcedure` /
//! `InstructionValueProcedure` / `InstructionStateProcedure` / `InstructionValueConsumer`。
//!
//! 偏离记录：Java 各为独立接口 → Rust 统一定义在此文件，避免循环引用。

use rustci_vm_ci::meta::value::Value;

use crate::lir::lir_frame_state::LIRFrameState;
use crate::lir::lir_instruction::LIRInstruction;

/// 对应 `jdk.graal.compiler.lir.ValueConsumer`。
///
/// 消费 LIR 值的函数式接口。
pub trait ValueConsumer {
    /// 对应 `consumeValue(Value)`：消费一个值。
    fn consume_value(&mut self, value: &dyn Value);
}

/// 对应 `jdk.graal.compiler.lir.ValueProcedure`。
///
/// 过程式处理 LIR 值的函数式接口。
pub trait ValueProcedure {
    /// 对应 `doValue(Value, OperandMode, EnumSet<OperandFlag>)`：处理一个值。
    /// 返回修改后的值。如果返回 None，表示移除该值。
    fn do_value(&mut self, value: &dyn Value, mode: OperandMode, flags: OperandFlags) -> Option<Box<dyn Value>>;
}

/// 对应 `jdk.graal.compiler.lir.InstructionValueProcedure`。
///
/// 处理指令中每个值的过程式接口。
pub trait InstructionValueProcedure {
    /// 对应 `doValue(LIRInstruction, Value, OperandMode, EnumSet<OperandFlag>)`：
    /// 处理指令中的一个值。
    fn do_value(
        &mut self,
        instruction: &dyn LIRInstruction,
        value: &dyn Value,
        mode: OperandMode,
        flags: OperandFlags,
    ) -> Option<Box<dyn Value>>;
}

/// 对应 `jdk.graal.compiler.lir.InstructionStateProcedure`。
///
/// 处理指令中状态的过程式接口。
pub trait InstructionStateProcedure {
    /// 对应 `doState(LIRInstruction, LIRFrameState)`：处理指令的帧状态。
    fn do_state(&mut self, instruction: &dyn LIRInstruction, state: &LIRFrameState);
}

/// 对应 `jdk.graal.compiler.lir.InstructionValueConsumer`。
///
/// 消费指令中值的函数式接口。
pub trait InstructionValueConsumer {
    /// 对应 `consumeValue(LIRInstruction, Value, OperandMode, EnumSet<OperandFlag>)`：
    /// 消费指令中的一个值。
    fn consume_value(
        &mut self,
        instruction: &dyn LIRInstruction,
        value: &dyn Value,
        mode: OperandMode,
        flags: OperandFlags,
    );
}

/// 对应 `jdk.graal.compiler.lir.OperandMode`（Java 内部枚举）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OperandMode {
    /// 输入操作数。
    Input,
    /// 输出操作数。
    Output,
    /// 临时操作数。
    Temp,
}

/// 对应 `jdk.graal.compiler.lir.OperandFlag`（Java 内部枚举）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OperandFlag {
    /// 寄存器操作数。
    Register,
    /// 栈操作数。
    Stack,
    /// 复合操作数。
    Composite,
    /// 非法操作数。
    Illegal,
    /// 未初始化操作数。
    Uninitialized,
}

/// 对应 Java `EnumSet<OperandFlag>` 的位掩码表示。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct OperandFlags(u8);

impl OperandFlags {
    pub const REGISTER: u8 = 1 << 0;
    pub const STACK: u8 = 1 << 1;
    pub const COMPOSITE: u8 = 1 << 2;
    pub const ILLEGAL: u8 = 1 << 3;
    pub const UNINITIALIZED: u8 = 1 << 4;

    /// 创建空标志集。
    pub fn new() -> Self {
        Self(0)
    }

    /// 添加标志。
    pub fn with(mut self, flag: OperandFlag) -> Self {
        self.0 |= Self::flag_to_bit(flag);
        self
    }

    /// 检查是否包含某个标志。
    pub fn contains(&self, flag: OperandFlag) -> bool {
        (self.0 & Self::flag_to_bit(flag)) != 0
    }

    fn flag_to_bit(flag: OperandFlag) -> u8 {
        match flag {
            OperandFlag::Register => Self::REGISTER,
            OperandFlag::Stack => Self::STACK,
            OperandFlag::Composite => Self::COMPOSITE,
            OperandFlag::Illegal => Self::ILLEGAL,
            OperandFlag::Uninitialized => Self::UNINITIALIZED,
        }
    }
}