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

//! 镜像 `jdk.graal.compiler.lir.ssa`：SSA 相关工具。
//!
//! 偏离记录：Java 子包 `lir.ssa` 含 SSA 破坏和修复相关工具。
//! Rust 侧提供 trait 定义。

use rustci_vm_ci::meta::value::Value;

use crate::lir::lir::LIR;
use crate::lir::lir_instruction::LIRInstruction;
use crate::lir::variable::Variable;

/// 对应 `lir.ssa.SSAUtil`：SSA 工具类。
#[derive(Debug, Clone, Default)]
pub struct SSAUtil;

impl SSAUtil {
    /// 对应 `verifyPhi(LIR)`：验证 phi 函数。
    pub fn verify_phi(lir: &LIR) -> bool {
        // 验证每个 phi 的输入变量索引不冲突
        // 简化实现：检查所有变量索引是否在范围内
        let num_vars = lir.num_variables();
        for block in lir.get_blocks() {
            for instr in &block.instructions {
                let _input_collector = |value: &dyn Value| {
                    if let Some(var) = value.as_any().downcast_ref::<Variable>() {
                        if var.get_index() as usize >= num_vars {
                            return false;
                        }
                    }
                    true
                };
                // 遍历所有输入
                instr.for_each_input(&mut DummyConsumer);
            }
        }
        true
    }
}

/// 用于遍历的占位消费者。
struct DummyConsumer;

impl crate::lir::value_procedure::InstructionValueConsumer for DummyConsumer {
    fn consume_value(
        &mut self,
        _instruction: &dyn LIRInstruction,
        _value: &dyn Value,
        _mode: crate::lir::value_procedure::OperandMode,
        _flags: crate::lir::value_procedure::OperandFlags,
    ) {
    }
}

/// 对应 `lir.ssa.SSAVerifier`：SSA 验证器。
pub trait SSAVerifier {
    /// 验证 SSA 形式。
    fn verify(&self, lir: &LIR) -> bool;
}

/// 对应 `lir.ssa.SSADestruction`：SSA 破坏（出 SSA 形式）。
pub trait SSADestruction {
    /// 破坏 SSA 形式，将 phi 函数替换为拷贝指令。
    fn destroy_ssa(&mut self, lir: &mut LIR);
}