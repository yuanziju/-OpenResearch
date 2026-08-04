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

//! 镜像 `jdk.graal.compiler.lir.stackslotalloc`：栈槽分配。
//!
//! 偏离记录：Java 子包 `lir.stackslotalloc` 含栈槽分配器抽象和实现。
//! Rust 侧提供 trait 和基础 struct 定义。

use rustci_vm_ci::meta::value::Value;

use crate::lir::lir::LIR;

/// 对应 `lir.stackslotalloc.StackSlotAllocator`：栈槽分配器。
pub trait StackSlotAllocator {
    /// 分配栈槽给 LIR 中的变量。
    fn allocate(&mut self, lir: &mut LIR);

    /// 获取分配的栈槽信息。
    fn get_allocated_slots(&self) -> &[StackSlotInfo];
}

/// 对应 `lir.stackslotalloc.StackSlotInfo`：栈槽信息。
#[derive(Debug, Clone)]
pub struct StackSlotInfo {
    /// 栈槽偏移。
    pub offset: i32,
    /// 栈槽大小。
    pub size: i32,
    /// 是否在调用者帧中。
    pub in_caller_frame: bool,
    /// 关联的变量索引。
    pub variable_index: i32,
}

impl StackSlotInfo {
    /// 创建栈槽信息。
    pub fn new(offset: i32, size: i32, in_caller_frame: bool, variable_index: i32) -> Self {
        Self {
            offset,
            size,
            in_caller_frame,
            variable_index,
        }
    }
}

/// 对应 `lir.stackslotalloc.SimpleStackSlotAllocator`：简单栈槽分配器。
#[derive(Debug, Clone, Default)]
pub struct SimpleStackSlotAllocator {
    /// 分配的栈槽列表。
    slots: Vec<StackSlotInfo>,
    /// 当前偏移。
    current_offset: i32,
}

impl SimpleStackSlotAllocator {
    /// 创建简单栈槽分配器。
    pub fn new() -> Self {
        Self {
            slots: Vec::new(),
            current_offset: 0,
        }
    }
}

impl StackSlotAllocator for SimpleStackSlotAllocator {
    fn allocate(&mut self, lir: &mut LIR) {
        self.slots.clear();
        self.current_offset = 0;

        let num_vars = lir.num_variables();
        for i in 0..num_vars {
            let var = &lir.get_variables()[i];
            let size = var.get_value_kind().get_platform_kind().get_size_in_bytes();
            let slot = StackSlotInfo::new(
                self.current_offset,
                size,
                false,
                var.get_index(),
            );
            self.slots.push(slot);
            self.current_offset += size;
        }

        lir.set_frame_size(self.current_offset);
    }

    fn get_allocated_slots(&self) -> &[StackSlotInfo] {
        &self.slots
    }
}