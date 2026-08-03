// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2011, 2019, Oracle and/or its affiliates. All rights reserved.
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
 * or visit www.oracle.com if you need additional information or have any
 * questions.
 */

//! 镜像 `jdk.vm.ci.hotspot.HotSpotInstalledCode`：HotSpot 已安装代码（CodeBlob）。
//!
//! 偏离记录：
//! - Java `abstract class HotSpotInstalledCode extends InstalledCode` →
//!   Rust 无继承，`HotSpotInstalledCode` 为独立 struct，通过组合持有 `InstalledCode`。
//! - Java 的 `protected` 字段（`size`/`codeStart`/`codeSize`）→ Rust `pub(crate)` 字段。
//! - 方法名 camelCase → snake_case。

use crate::code::installed_code::InstalledCode;

/// 对应 `abstract class HotSpotInstalledCode extends InstalledCode`。
pub struct HotSpotInstalledCode {
    /// 基类字段（组合替代继承）。
    pub base: InstalledCode,
    /// 对应 `private int size`（CodeBlob::size()）。
    pub(crate) size: i32,
    /// 对应 `private long codeStart`（CodeBlob::code_begin()）。
    pub(crate) code_start: i64,
    /// 对应 `private int codeSize`（CodeBlob::code_size()）。
    pub(crate) code_size: i32,
}

impl HotSpotInstalledCode {
    /// 对应 `HotSpotInstalledCode(String name)`。
    pub fn new(name: Option<String>) -> Self {
        Self {
            base: InstalledCode::new(name),
            size: 0,
            code_start: 0,
            code_size: 0,
        }
    }

    /// 对应 `getSize()`。
    pub fn get_size(&self) -> i32 {
        self.size
    }

    /// 对应 `getStart()`（覆写）。
    pub fn get_start(&self) -> i64 {
        self.code_start
    }

    /// 对应 `getCodeSize()`。
    pub fn get_code_size(&self) -> i64 {
        self.code_size as i64
    }
}
