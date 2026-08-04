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

//! 镜像 `jdk.graal.compiler.lir.LIRVerifier`：LIR 验证器。
//!
//! 偏离记录：Java `final class LIRVerifier` → Rust struct。
//! LIR 验证器检查 LIR 图的一致性。

use crate::lir::lir::LIR;
use crate::lir::lir_instruction::LIRInstruction;
use crate::lir::value_procedure::InstructionValueConsumer;

/// 对应 `public final class LIRVerifier`。
///
/// LIR 验证器，对 LIR 图进行一致性检查。
#[derive(Debug, Clone, Default)]
pub struct LIRVerifier {
    /// 验证时发现的错误列表。
    errors: Vec<String>,
    /// 是否启用验证。
    enabled: bool,
}

impl LIRVerifier {
    /// 创建新的验证器。
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
            enabled: true,
        }
    }

    /// 对应 `verify(LIR)`：验证 LIR 图。
    pub fn verify(&mut self, lir: &LIR) -> bool {
        self.errors.clear();
        if !self.enabled {
            return true;
        }

        self.verify_blocks(lir);
        self.verify_instructions(lir);
        self.verify_variables(lir);

        self.errors.is_empty()
    }

    /// 验证基本块结构。
    fn verify_blocks(&mut self, lir: &LIR) {
        let block_count = lir.num_blocks();
        if block_count == 0 {
            self.errors.push("LIR has no blocks".to_string());
            return;
        }

        for (i, block) in lir.get_blocks().iter().enumerate() {
            if block.label.get_block_index() != i {
                self.errors.push(format!(
                    "Block {} has label referencing block {}",
                    i,
                    block.label.get_block_index()
                ));
            }
        }
    }

    /// 验证指令。
    fn verify_instructions(&mut self, lir: &LIR) {
        for (block_idx, block) in lir.get_blocks().iter().enumerate() {
            for (instr_idx, instr) in block.instructions.iter().enumerate() {
                // 验证每条指令有非空操作码
                if instr.opcode().is_empty() {
                    self.errors.push(format!(
                        "Block {}, instruction {} has empty opcode",
                        block_idx, instr_idx
                    ));
                }

                // 验证指令 ID 非负
                if instr.id() < 0 {
                    self.errors.push(format!(
                        "Block {}, instruction {} has negative id",
                        block_idx, instr_idx
                    ));
                }
            }
        }
    }

    /// 验证变量。
    fn verify_variables(&mut self, lir: &LIR) {
        let num_variables = lir.num_variables();
        // 变量索引应在 [0, num_variables) 范围内
        for block in lir.get_blocks() {
            for instr in &block.instructions {
                // 遍历指令的所有值，检查变量索引
                let mut collector = VariableIndexCollector::new(num_variables, &mut self.errors);
                instr.for_each_input(&mut collector);
                instr.for_each_output(&mut collector);
                instr.for_each_temp(&mut collector);
            }
        }
    }

    /// 获取错误列表。
    pub fn get_errors(&self) -> &[String] {
        &self.errors
    }

    /// 启用/禁用验证。
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// 检查是否启用。
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

/// 用于收集变量索引的消费者。
struct VariableIndexCollector<'a> {
    num_variables: usize,
    errors: &'a mut Vec<String>,
}

impl<'a> VariableIndexCollector<'a> {
    fn new(num_variables: usize, errors: &'a mut Vec<String>) -> Self {
        Self {
            num_variables,
            errors,
        }
    }
}

impl<'a> InstructionValueConsumer for VariableIndexCollector<'a> {
    fn consume_value(
        &mut self,
        _instruction: &dyn LIRInstruction,
        value: &dyn rustci_vm_ci::meta::value::Value,
        _mode: crate::lir::value_procedure::OperandMode,
        _flags: crate::lir::value_procedure::OperandFlags,
    ) {
        if let Some(var) = value.as_any().downcast_ref::<crate::lir::variable::Variable>() {
            let idx = var.get_index() as usize;
            if idx >= self.num_variables {
                self.errors.push(format!(
                    "Variable index {} out of range (max {})",
                    idx, self.num_variables
                ));
            }
        }
    }
}