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

//! 镜像 `jdk.graal.compiler.lir.gen`：LIR 生成。
//!
//! 偏离记录：Java 子包 `lir.gen` 含 LIR 生成器抽象和节点匹配逻辑。
//! Rust 侧提供 trait 定义。

use rustci_vm_ci::meta::value::Value;

use crate::lir::lir::LIR;
use crate::lir::lir_instruction::LIRInstruction;

/// 对应 `lir.gen.LIRGeneratorTool`：LIR 生成器工具接口。
///
/// LIR 生成过程中使用的核心接口，提供变量创建、指令发射等功能。
pub trait LIRGeneratorTool {
    /// 创建新变量。
    fn new_variable(
        &mut self,
        kind: Box<dyn rustci_vm_ci::meta::value_kind::ValueKind>,
    ) -> crate::lir::variable::Variable;

    /// 发射指令到 LIR 图。
    fn emit(&mut self, instruction: Box<dyn LIRInstruction>);

    /// 获取 LIR 图。
    fn get_lir(&self) -> &LIR;

    /// 获取可变 LIR 图。
    fn get_lir_mut(&mut self) -> &mut LIR;

    /// 创建标签。
    fn new_label(&mut self) -> crate::lir::label_ref::LabelRef;

    /// 获取当前基本块。
    fn current_block_index(&self) -> usize;
}

/// 对应 `lir.gen.LIRGenerationResult`：LIR 生成结果。
#[derive(Debug, Clone)]
pub struct LIRGenerationResult {
    /// 生成的 LIR 图。
    pub lir: LIR,
    /// 生成的指令总数。
    pub instruction_count: u64,
    /// 帧大小。
    pub frame_size: i32,
}

impl LIRGenerationResult {
    /// 创建 LIR 生成结果。
    pub fn new(lir: LIR) -> Self {
        Self {
            lir,
            instruction_count: 0,
            frame_size: 0,
        }
    }
}

/// 对应 `lir.gen.ArithmeticLIRGeneratorTool`：算术操作生成器。
pub trait ArithmeticLIRGeneratorTool {
    /// 生成加法操作。
    fn emit_add(&mut self, a: &dyn Value, b: &dyn Value) -> Box<dyn Value>;

    /// 生成减法操作。
    fn emit_sub(&mut self, a: &dyn Value, b: &dyn Value) -> Box<dyn Value>;

    /// 生成乘法操作。
    fn emit_mul(&mut self, a: &dyn Value, b: &dyn Value) -> Box<dyn Value>;

    /// 生成除法操作。
    fn emit_div(&mut self, a: &dyn Value, b: &dyn Value) -> Box<dyn Value>;

    /// 生成取余操作。
    fn emit_rem(&mut self, a: &dyn Value, b: &dyn Value) -> Box<dyn Value>;

    /// 生成取反操作。
    fn emit_neg(&mut self, a: &dyn Value) -> Box<dyn Value>;
}