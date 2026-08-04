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

//! 镜像 `jdk.graal.compiler.lir.LIR`：LIR 图主类。
//!
//! 偏离记录：Java `final class LIR` → Rust struct。
//! LIR 是低级中间表示图，包含基本块、指令和变量。

use std::fmt;

use crate::lir::label_ref::LabelRef;
use crate::lir::lir_instruction::LIRInstruction;
use crate::lir::variable::Variable;

/// 对应 `public final class LIR`。
///
/// LIR 图，包含基本块、指令列表和 SSA 变量。
#[derive(Debug)]
pub struct LIR {
    /// 对应 `blocks`：基本块列表。
    blocks: Vec<LIRBlock>,
    /// 对应 `variables`：SSA 变量列表。
    variables: Vec<Variable>,
    /// 对应 `nextVariableIndex`：下一个变量索引。
    next_variable_index: i32,
    /// 对应 `nextInstructionId`：下一条指令 ID。
    next_instruction_id: i32,
    /// 对应 `frameSize`：帧大小。
    frame_size: i32,
    /// 对应 `name`：方法名（调试用）。
    name: String,
}

/// 对应 `LIR.LIRBlock`：LIR 基本块。
#[derive(Debug)]
pub struct LIRBlock {
    /// 对应 `label`：基本块标签。
    pub label: LabelRef,
    /// 对应 `instructions`：指令列表。
    pub instructions: Vec<Box<dyn LIRInstruction>>,
    /// 对应 `successors`：后继基本块标签。
    pub successors: Vec<LabelRef>,
    /// 对应 `predecessors`：前驱基本块标签。
    pub predecessors: Vec<LabelRef>,
    /// 对应 `isExceptionEntry`：是否为异常入口。
    pub is_exception_entry: bool,
}

impl Clone for LIRBlock {
    fn clone(&self) -> Self {
        Self {
            label: self.label,
            instructions: self.instructions.iter().map(|i| i.clone_box()).collect(),
            successors: self.successors.clone(),
            predecessors: self.predecessors.clone(),
            is_exception_entry: self.is_exception_entry,
        }
    }
}

impl LIRBlock {
    /// 创建基本块。
    pub fn new(label: LabelRef) -> Self {
        Self {
            label,
            instructions: Vec::new(),
            successors: Vec::new(),
            predecessors: Vec::new(),
            is_exception_entry: false,
        }
    }

    /// 添加指令。
    pub fn add_instruction(&mut self, instruction: Box<dyn LIRInstruction>) {
        self.instructions.push(instruction);
    }

    /// 获取指令数量。
    pub fn instruction_count(&self) -> usize {
        self.instructions.len()
    }

    /// 获取第一条指令。
    pub fn first_instruction(&self) -> Option<&dyn LIRInstruction> {
        self.instructions.first().map(|i| i.as_ref())
    }

    /// 获取最后一条指令。
    pub fn last_instruction(&self) -> Option<&dyn LIRInstruction> {
        self.instructions.last().map(|i| i.as_ref())
    }
}

impl Clone for LIR {
    fn clone(&self) -> Self {
        Self {
            blocks: self.blocks.clone(),
            variables: self.variables.clone(),
            next_variable_index: self.next_variable_index,
            next_instruction_id: self.next_instruction_id,
            frame_size: self.frame_size,
            name: self.name.clone(),
        }
    }
}

impl LIR {
    /// 创建 LIR 图。
    pub fn new(name: &str) -> Self {
        Self {
            blocks: Vec::new(),
            variables: Vec::new(),
            next_variable_index: 0,
            next_instruction_id: 0,
            frame_size: 0,
            name: name.to_string(),
        }
    }

    /// 对应 `name()`：获取方法名。
    pub fn name(&self) -> &str {
        &self.name
    }

    /// 对应 `numBlocks()`：获取基本块数量。
    pub fn num_blocks(&self) -> usize {
        self.blocks.len()
    }

    /// 对应 `getBlocks()`：获取基本块列表。
    pub fn get_blocks(&self) -> &[LIRBlock] {
        &self.blocks
    }

    /// 获取可变基本块列表。
    pub fn get_blocks_mut(&mut self) -> &mut Vec<LIRBlock> {
        &mut self.blocks
    }

    /// 对应 `getBlock(int)`：获取指定索引的基本块。
    pub fn get_block(&self, index: usize) -> Option<&LIRBlock> {
        self.blocks.get(index)
    }

    /// 获取可变基本块。
    pub fn get_block_mut(&mut self, index: usize) -> Option<&mut LIRBlock> {
        self.blocks.get_mut(index)
    }

    /// 对应 `newBlock()`：创建新基本块。
    pub fn new_block(&mut self) -> LabelRef {
        let index = self.blocks.len();
        let label = LabelRef::new(index);
        self.blocks.push(LIRBlock::new(label));
        label
    }

    /// 对应 `numVariables()`：获取变量数量。
    pub fn num_variables(&self) -> usize {
        self.variables.len()
    }

    /// 对应 `getVariables()`：获取变量列表。
    pub fn get_variables(&self) -> &[Variable] {
        &self.variables
    }

    /// 对应 `newVariable(ValueKind<?>)`：创建新变量。
    pub fn new_variable(&mut self, kind: Box<dyn rustci_vm_ci::meta::value_kind::ValueKind>) -> Variable {
        let index = self.next_variable_index;
        self.next_variable_index += 1;
        let var = Variable::new(kind, index);
        self.variables.push(var.clone());
        var
    }

    /// 对应 `newVariable(ValueKind<?>, String)`：创建命名变量。
    pub fn new_variable_named(
        &mut self,
        kind: Box<dyn rustci_vm_ci::meta::value_kind::ValueKind>,
        name: &str,
    ) -> Variable {
        let index = self.next_variable_index;
        self.next_variable_index += 1;
        let var = Variable::new_named(kind, index, name);
        self.variables.push(var.clone());
        var
    }

    /// 对应 `nextInstructionId()`：获取下一条指令 ID。
    pub fn next_instruction_id(&mut self) -> i32 {
        let id = self.next_instruction_id;
        self.next_instruction_id += 1;
        id
    }

    /// 对应 `getFrameSize()`：获取帧大小。
    pub fn get_frame_size(&self) -> i32 {
        self.frame_size
    }

    /// 对应 `setFrameSize(int)`：设置帧大小。
    pub fn set_frame_size(&mut self, size: i32) {
        self.frame_size = size;
    }

    /// 追加指令到当前活跃基本块。
    pub fn append_instructions(&mut self, instructions: &mut Vec<Box<dyn LIRInstruction>>) {
        if let Some(block) = self.blocks.last_mut() {
            block.instructions.append(instructions);
        }
    }

    /// 添加后继边。
    pub fn add_successor(&mut self, from: usize, to: LabelRef) {
        if let Some(block) = self.blocks.get_mut(from) {
            block.successors.push(to);
        }
    }

    /// 添加前驱边。
    pub fn add_predecessor(&mut self, to: usize, from: LabelRef) {
        if let Some(block) = self.blocks.get_mut(to) {
            block.predecessors.push(from);
        }
    }
}

impl fmt::Display for LIR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "LIR for {}", self.name)?;
        for (i, block) in self.blocks.iter().enumerate() {
            writeln!(f, "Block {} (label={}):", i, block.label.get_block_index())?;
            for instr in &block.instructions {
                writeln!(f, "  {:?}", instr)?;
            }
        }
        Ok(())
    }
}