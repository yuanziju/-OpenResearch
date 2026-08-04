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

//! 镜像 `jdk.graal.compiler.lir.LIRIntrospection`：LIR 内省。
//!
//! 偏离记录：Java `interface LIRIntrospection` → Rust trait。
//! LIR 内省接口提供对 LIR 指令操作数的元数据访问。

use rustci_vm_ci::meta::value::Value;

use crate::lir::lir_instruction::LIRInstruction;
use crate::lir::value_procedure::{
    InstructionValueConsumer, OperandFlags, OperandMode,
};

/// 对应 `interface LIRIntrospection`。
///
/// 提供对 LIR 指令操作数的内省访问。
pub trait LIRIntrospection {
    /// 对应 `forEachValue(LIRInstruction, InstructionValueConsumer)`：
    /// 遍历指令的所有值操作数。
    fn for_each_value(&self, instruction: &dyn LIRInstruction, proc: &mut dyn InstructionValueConsumer);

    /// 对应 `forEachState(LIRInstruction, InstructionValueConsumer)`：
    /// 遍历指令状态中的所有值。
    fn for_each_state_value(&self, instruction: &dyn LIRInstruction, proc: &mut dyn InstructionValueConsumer);

    /// 对应 `getInputs(LIRInstruction)`：获取输入操作数。
    fn get_inputs(&self, instruction: &dyn LIRInstruction) -> Vec<Box<dyn Value>>;

    /// 对应 `getOutputs(LIRInstruction)`：获取输出操作数。
    fn get_outputs(&self, instruction: &dyn LIRInstruction) -> Vec<Box<dyn Value>>;

    /// 对应 `getTemps(LIRInstruction)`：获取临时操作数。
    fn get_temps(&self, instruction: &dyn LIRInstruction) -> Vec<Box<dyn Value>>;

    /// 对应 `getAliveValues(LIRInstruction)`：获取活跃操作数。
    fn get_alive_values(&self, instruction: &dyn LIRInstruction) -> Vec<Box<dyn Value>>;

    /// 对应 `getStateValues(LIRInstruction)`：获取状态中的值。
    fn get_state_values(&self, instruction: &dyn LIRInstruction) -> Vec<Box<dyn Value>>;

    /// 对应 `getOperandFlags(LIRInstruction, int)`：获取操作数标志。
    fn get_operand_flags(&self, instruction: &dyn LIRInstruction, index: usize) -> OperandFlags;

    /// 对应 `getOperandMode(LIRInstruction, int)`：获取操作数模式。
    fn get_operand_mode(&self, instruction: &dyn LIRInstruction, index: usize) -> OperandMode;

    /// 对应 `isInput(LIRInstruction, int)`：检查是否为输入操作数。
    fn is_input(&self, instruction: &dyn LIRInstruction, index: usize) -> bool;

    /// 对应 `isOutput(LIRInstruction, int)`：检查是否为输出操作数。
    fn is_output(&self, instruction: &dyn LIRInstruction, index: usize) -> bool;

    /// 对应 `isTemp(LIRInstruction, int)`：检查是否为临时操作数。
    fn is_temp(&self, instruction: &dyn LIRInstruction, index: usize) -> bool;

    /// 对应 `operandCount(LIRInstruction)`：获取操作数数量。
    fn operand_count(&self, instruction: &dyn LIRInstruction) -> usize;
}

/// 对应 `LIRIntrospection.LIRIntrospectionValues` 内部接口。
///
/// 提供对 LIR 指令值的元数据。
pub trait LIRIntrospectionValues {
    /// 获取操作数在各操作数数组中的索引。
    fn get_operand_index(&self, value: &dyn Value) -> Option<usize>;

    /// 获取操作数标志。
    fn get_operand_flags(&self, value: &dyn Value) -> OperandFlags;

    /// 获取操作数模式。
    fn get_operand_mode(&self, value: &dyn Value) -> OperandMode;
}