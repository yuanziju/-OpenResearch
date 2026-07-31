// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2011, Oracle and/or its affiliates. All rights reserved.
 * DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
 *
 * This code is free software; you can redistribute it and/or modify it
 * under the terms of the GNU General Public License version 2 only, as
 * published by the Free Software Foundation.
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
 * or visit www.oracle if you need additional information or have any
 * questions.
 */

//! 镜像 `jdk.vm.ci.meta.LineNumberTable`：BCI → 源行号映射表。
//!
//! 偏离记录：Java `int[]` 字段 → Rust `Vec<i32>`，构造期转移所有权（对齐 Java
//! `EI_EXPOSE_REP2` 文档"caller transfers ownership"）。`getLineNumbers()`/`getBcis()`
//! 在 Java 返回 `.clone()`，Rust 侧返回 `Vec<i32>` 克隆。

/// 对应 `public class LineNumberTable`。
pub struct LineNumberTable {
    line_numbers: Vec<i32>,
    bcis: Vec<i32>,
}

impl LineNumberTable {
    /// 对应 `LineNumberTable(int[] lineNumbers, int[] bcis)`。
    pub fn new(line_numbers: Vec<i32>, bcis: Vec<i32>) -> Self {
        debug_assert_eq!(bcis.len(), line_numbers.len());
        Self { line_numbers, bcis }
    }

    /// 对应 `getLineNumber(int atBci)`。
    pub fn get_line_number(&self, at_bci: i32) -> i32 {
        let n = self.bcis.len();
        for i in 0..n.saturating_sub(1) {
            if self.bcis[i] <= at_bci && at_bci < self.bcis[i + 1] {
                return self.line_numbers[i];
            }
        }
        self.line_numbers[n - 1]
    }

    /// 对应 `getLineNumbers()`：返回克隆。
    pub fn get_line_numbers(&self) -> Vec<i32> {
        self.line_numbers.clone()
    }

    /// 对应 `getBcis()`：返回克隆。
    pub fn get_bcis(&self) -> Vec<i32> {
        self.bcis.clone()
    }
}
