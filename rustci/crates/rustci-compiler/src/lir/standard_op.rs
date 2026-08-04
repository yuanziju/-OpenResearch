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

//! 镜像 `jdk.graal.compiler.lir.StandardOp`：标准操作接口。
//!
//! 偏离记录：Java `interface StandardOp` 含多个内部接口（`LabelOp`, `BlockEndOp`,
//! `JumpOp`, `MoveOp`, `NullCheck`, `SaveRegistersOp`, `RestoreRegistersOp` 等）
//! → Rust 统一定义为此文件多个 trait。

use rustci_vm_ci::meta::value::Value;

use crate::lir::label_ref::LabelRef;

/// 对应 `StandardOp.LabelOp`：标签操作。
///
/// 标记基本块入口的指令。
pub trait LabelOp {
    /// 对应 `getLabel()`：获取标签。
    fn get_label(&self) -> LabelRef;
}

/// 对应 `StandardOp.BlockEndOp`：基本块结束操作。
///
/// 标记基本块结束的指令。
pub trait BlockEndOp {
    /// 对应 `setOutgoingValues(BlockValue[])`：设置传出值（phi 值）。
    fn set_outgoing_values(&mut self, values: &[Box<dyn Value>]);

    /// 对应 `getOutgoingValues()`：获取传出值。
    fn get_outgoing_values(&self) -> &[Box<dyn Value>];

    /// 对应 `getSuccessorCount()`：获取后继基本块数量。
    fn get_successor_count(&self) -> usize;

    /// 对应 `blockSuccessor(int)`：获取第 i 个后继基本块标签。
    fn block_successor(&self, i: usize) -> LabelRef;
}

/// 对应 `StandardOp.JumpOp`：跳转操作。
///
/// 无条件跳转指令。
pub trait JumpOp: BlockEndOp {
    /// 对应 `getJumpDestination()`：获取跳转目标标签。
    fn get_jump_destination(&self) -> LabelRef;
}

/// 对应 `StandardOp.BranchOp`：分支操作。
///
/// 条件分支指令。
pub trait BranchOp: BlockEndOp {
    /// 对应 `getTrueDestination()`：获取真分支目标。
    fn get_true_destination(&self) -> LabelRef;

    /// 对应 `getFalseDestination()`：获取假分支目标。
    fn get_false_destination(&self) -> LabelRef;
}

/// 对应 `StandardOp.MoveOp`：移动操作。
///
/// 值移动指令（寄存器到寄存器，栈到寄存器等）。
pub trait MoveOp {
    /// 对应 `getInput()`：获取输入值。
    fn get_input(&self) -> &dyn Value;

    /// 对应 `getResult()`：获取结果值。
    fn get_result(&self) -> &dyn Value;
}

/// 对应 `StandardOp.NullCheck`：空值检查。
///
/// 隐式空值检查指令。
pub trait NullCheck {
    /// 对应 `getCheckedValue()`：获取被检查的值。
    fn get_checked_value(&self) -> &dyn Value;

    /// 对应 `getState()`：获取关联的帧状态。
    fn get_null_check_state(&self) -> &crate::lir::lir_frame_state::LIRFrameState;
}

/// 对应 `StandardOp.SaveRegistersOp`：保存寄存器操作。
pub trait SaveRegistersOp {
    /// 获取保存的寄存器信息。
    fn get_saved_registers(&self) -> &[Box<dyn Value>];
}

/// 对应 `StandardOp.RestoreRegistersOp`：恢复寄存器操作。
pub trait RestoreRegistersOp {
    /// 获取恢复的寄存器信息。
    fn get_restored_registers(&self) -> &[Box<dyn Value>];
}

/// 对应 `StandardOp.LoadConstantOp`：加载常量操作。
pub trait LoadConstantOp {
    /// 对应 `getConstant()`：获取常量值。
    fn get_constant(&self) -> &dyn Value;
}

/// 对应 `StandardOp.AllocOp`：分配操作。
pub trait AllocOp {
    /// 获取分配的大小。
    fn get_size(&self) -> usize;
}