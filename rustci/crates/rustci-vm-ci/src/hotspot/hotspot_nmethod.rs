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

//! 镜像 `jdk.vm.ci.hotspot.HotSpotNmethod`：HotSpot nmethod 已安装代码。
//!
//! 偏离记录：
//! - Java `class HotSpotNmethod extends HotSpotInstalledCode` →
//!   Rust 无继承，`HotSpotNmethod` 为独立 struct，通过组合持有 `HotSpotInstalledCode`。
//! - 方法名 camelCase → snake_case。

use std::any::Any;

use crate::code::invalid_installed_code_exception::InvalidInstalledCodeException;

/// 对应 `class HotSpotNmethod extends HotSpotInstalledCode`。
pub struct HotSpotNmethod {
    /// 基类字段（组合替代继承）。
    pub base: super::hotspot_installed_code::HotSpotInstalledCode,
    /// 对应 `private final HotSpotResolvedJavaMethodImpl method`。
    method: *mut std::ffi::c_void,
    /// 对应 `private final boolean isDefault`。
    is_default: bool,
    /// 对应 `private final long compileIdSnapshot`。
    compile_id_snapshot: i64,
}

impl HotSpotNmethod {
    /// 对应 `HotSpotNmethod(HotSpotResolvedJavaMethodImpl, String, boolean, long)`。
    pub fn new(
        method: *mut std::ffi::c_void,
        name: Option<String>,
        is_default: bool,
        compile_id: i64,
    ) -> Self {
        Self {
            base: super::hotspot_installed_code::HotSpotInstalledCode::new(name),
            method,
            is_default,
            compile_id_snapshot: if is_default { compile_id } else { 0 },
        }
    }

    /// 对应 `isDefault()`。
    pub fn is_default(&self) -> bool {
        self.is_default
    }

    /// 对应 `getMethod()`。
    pub fn get_method(&self) -> *mut std::ffi::c_void {
        self.method
    }

    /// 对应 `inOopsTable()`。
    pub fn in_oops_table(&self) -> bool {
        self.compile_id_snapshot != 0
    }

    /// 对应 `executeVarargs(Object...)`。
    pub fn execute_varargs(
        &self,
        _args: &[Box<dyn Any>],
    ) -> Result<Box<dyn Any>, InvalidInstalledCodeException> {
        Err(InvalidInstalledCodeException::new())
    }
}
