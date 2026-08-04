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

//! 镜像 `jdk.graal.compiler.lir.LIRInsertionBuffer`：LIR 插入缓冲区。
//!
//! 偏离记录：Java `final class LIRInsertionBuffer` → Rust struct。
//! 用于在 LIR 中批量插入指令，支持延迟插入和索引偏移。

use crate::lir::lir_instruction::LIRInstruction;

/// 对应 `public final class LIRInsertionBuffer`。
///
/// 用于在 LIR 基本块中批量插入指令的缓冲区。
/// 支持延迟插入：先收集要插入的指令，再一次性插入。
#[derive(Debug)]
pub struct LIRInsertionBuffer {
    /// 对应 `insertionIndexes`：插入位置索引数组。
    insertion_indexes: Vec<i32>,
    /// 对应 `insertions`：要插入的指令列表。
    insertions: Vec<Vec<Box<dyn LIRInstruction>>>,
    /// 对应 `insertionCount`：插入计数。
    insertion_count: usize,
    /// 对应 `initialized`：是否已初始化。
    initialized: bool,
}

impl LIRInsertionBuffer {
    /// 创建空的插入缓冲区。
    pub fn new() -> Self {
        Self {
            insertion_indexes: Vec::new(),
            insertions: Vec::new(),
            insertion_count: 0,
            initialized: false,
        }
    }

    /// 对应 `init(int)`：初始化为指定数量的插入位置。
    pub fn init(&mut self, count: usize) {
        self.insertion_indexes = vec![-1; count];
        self.insertions = (0..count).map(|_| Vec::new()).collect();
        self.insertion_count = 0;
        self.initialized = true;
    }

    /// 对应 `append(int, LIRInstruction)`：在指定索引处追加指令。
    pub fn append(&mut self, index: usize, instruction: Box<dyn LIRInstruction>) {
        assert!(self.initialized, "LIRInsertionBuffer not initialized");
        if index >= self.insertions.len() {
            self.insertion_indexes.resize(index + 1, -1);
            self.insertions.resize_with(index + 1, Vec::new);
        }
        if self.insertion_indexes[index] < 0 {
            self.insertion_indexes[index] = self.insertion_count as i32;
            self.insertion_count += 1;
        }
        self.insertions[index].push(instruction);
    }

    /// 对应 `append(int, List<LIRInstruction>)`：在指定索引处批量追加指令。
    pub fn append_all(&mut self, index: usize, instructions: Vec<Box<dyn LIRInstruction>>) {
        for instr in instructions {
            self.append(index, instr);
        }
    }

    /// 对应 `finish()`：完成插入，返回排序后的指令列表。
    pub fn finish(&self) -> Vec<&Box<dyn LIRInstruction>> {
        let mut result = Vec::new();
        let mut sorted: Vec<(i32, usize)> = self
            .insertion_indexes
            .iter()
            .enumerate()
            .filter(|(_, &idx)| idx >= 0)
            .map(|(i, &idx)| (idx, i))
            .collect();
        sorted.sort_by_key(|(idx, _)| *idx);

        for (_, i) in sorted {
            for instr in &self.insertions[i] {
                result.push(instr);
            }
        }
        result
    }

    /// 对应 `finish(LIR)`：完成插入，将指令写入 LIR 图。
    pub fn finish_into(&self, lir: &mut super::lir::LIR) {
        let mut sorted: Vec<(i32, usize)> = self
            .insertion_indexes
            .iter()
            .enumerate()
            .filter(|(_, &idx)| idx >= 0)
            .map(|(i, &idx)| (idx, i))
            .collect();
        sorted.sort_by_key(|(idx, _)| *idx);

        for (_, i) in sorted {
            let mut cloned: Vec<Box<dyn LIRInstruction>> = self.insertions[i]
                .iter()
                .map(|instr| instr.clone_box())
                .collect();
            lir.append_instructions(&mut cloned);
        }
    }

    /// 检查是否已初始化。
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    /// 获取插入计数。
    pub fn get_insertion_count(&self) -> usize {
        self.insertion_count
    }
}

impl Default for LIRInsertionBuffer {
    fn default() -> Self {
        Self::new()
    }
}