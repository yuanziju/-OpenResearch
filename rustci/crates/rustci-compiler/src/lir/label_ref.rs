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

//! 镜像 `jdk.graal.compiler.lir.LabelRef`：标签引用。
//!
//! 偏离记录：Java `final class LabelRef` → Rust struct。

/// 对应 `public final class LabelRef`。
///
/// 对 LIR 中基本块的标签引用。用于跳转、分支指令的目标。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LabelRef {
    /// 对应 `blockIndex`：目标基本块索引。
    block_index: usize,
}

impl LabelRef {
    /// 创建标签引用。
    pub fn new(block_index: usize) -> Self {
        Self { block_index }
    }

    /// 对应 `getBlockIndex()`：获取目标基本块索引。
    pub fn get_block_index(&self) -> usize {
        self.block_index
    }

    /// 设置目标基本块索引。
    pub fn set_block_index(&mut self, block_index: usize) {
        self.block_index = block_index;
    }
}