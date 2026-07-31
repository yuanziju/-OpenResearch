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

//! 镜像 `jdk.vm.ci.meta.LocalVariableTable`：方法的 `Local` 集合。
//!
//! 偏离记录：Java `Local[]` 字段 → Rust `Vec<Local>`，构造期转移所有权（对齐 Java
//! `EI_EXPOSE_REP2` 文档"caller transfers ownership"）。`getLocals()`/`getLocalsAt()`
//! 在 Java 返回 `Local[]`（克隆或新建），Rust 侧返回 `Vec<&Local>` 视图（避免克隆大对象，
//! 调用方可按需 `clone`）。`getLocal` 返回 `Option<&Local>`（对齐 Java 返回 `null`）。
//! `IllegalStateException`（Java 非受检，locals overlap）→ Rust `panic!`。

use crate::meta::local::Local;

/// 对应 `public class LocalVariableTable`。
pub struct LocalVariableTable {
    locals: Vec<Local>,
}

impl LocalVariableTable {
    /// 对应 `LocalVariableTable(Local[] locals)`。
    pub fn new(locals: Vec<Local>) -> Self {
        Self { locals }
    }

    /// 对应 `getLocal(int slot, int bci)`：返回 `Option` 对齐 Java nullable 语义。
    pub fn get_local(&self, slot: i32, bci: i32) -> Option<&Local> {
        let mut result: Option<&Local> = None;
        for local in &self.locals {
            if local.get_slot() == slot
                && local.get_start_bci() <= bci
                && local.get_end_bci() >= bci
            {
                if result.is_some() {
                    panic!("IllegalStateException: Locals overlap!");
                }
                result = Some(local);
            }
        }
        result
    }

    /// 对应 `getLocals()`：返回视图（Java 返回 `locals.clone()`，Rust 侧返回引用切片）。
    pub fn get_locals(&self) -> &[Local] {
        &self.locals
    }

    /// 对应 `getLocalsAt(int bci)`：返回该 BCI 处存活的全部 local 视图。
    pub fn get_locals_at(&self, bci: i32) -> Vec<&Local> {
        let mut result: Vec<&Local> = Vec::new();
        for l in &self.locals {
            if l.get_start_bci() <= bci && bci <= l.get_end_bci() {
                result.push(l);
            }
        }
        result
    }
}
