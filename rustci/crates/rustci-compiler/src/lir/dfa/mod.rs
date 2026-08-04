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

//! 镜像 `jdk.graal.compiler.lir.dfa`：数据流分析。
//!
//! 偏离记录：Java 子包 `lir.dfa` 含寄存器压力等 DFA 相关工具。
//! Rust 侧提供 trait 和基础 struct 定义。

use rustci_vm_ci::meta::value::Value;

use crate::lir::lir::LIR;

/// 对应 `lir.dfa.RegPressure`：寄存器压力。
#[derive(Debug, Clone, Default)]
pub struct RegPressure {
    /// 需要的整数寄存器数量。
    pub int_reg_count: usize,
    /// 需要的浮点寄存器数量。
    pub fp_reg_count: usize,
}

impl RegPressure {
    /// 创建寄存器压力。
    pub fn new(int_reg_count: usize, fp_reg_count: usize) -> Self {
        Self {
            int_reg_count,
            fp_reg_count,
        }
    }

    /// 合并两个寄存器压力。
    pub fn combine(&self, other: &RegPressure) -> RegPressure {
        RegPressure {
            int_reg_count: self.int_reg_count.max(other.int_reg_count),
            fp_reg_count: self.fp_reg_count.max(other.fp_reg_count),
        }
    }
}

/// 对应 `lir.dfa.LocationMarker`：位置标记。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocationMarker {
    /// 寄存器中的值。
    Register,
    /// 栈上的值。
    Stack,
    /// 未分配位置。
    Unallocated,
}

/// 对应 `lir.dfa.RegPressureTracker`：寄存器压力跟踪器。
pub trait RegPressureTracker {
    /// 计算指定位置处的寄存器压力。
    fn compute_pressure(&self, lir: &LIR, block_index: usize, instr_index: usize) -> RegPressure;

    /// 计算整个基本块的寄存器压力。
    fn compute_block_pressure(&self, lir: &LIR, block_index: usize) -> RegPressure;

    /// 获取指定位置处的活跃变量。
    fn get_live_variables(&self, lir: &LIR, block_index: usize, instr_index: usize) -> Vec<&dyn Value>;
}