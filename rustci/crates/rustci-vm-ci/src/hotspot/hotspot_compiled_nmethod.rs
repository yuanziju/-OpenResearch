// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2011, 2024, Oracle and/or its affiliates. All rights reserved.
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

//! 镜像 `jdk.vm.ci.hotspot.HotSpotCompiledNmethod`：HotSpot nmethod 编译产物。
//!
//! 偏离记录：
//! - Java `final class HotSpotCompiledNmethod extends HotSpotCompiledCode` →
//!   Rust 无继承，用组合方式：`HotSpotCompiledNmethod` 持 `HotSpotCompiledCode` 作为
//!   基类字段，同时实现 `CompiledCode` trait。
//! - 方法名 camelCase → snake_case。

use crate::code::compiled_code::CompiledCode;

/// 对应 `final class HotSpotCompiledNmethod extends HotSpotCompiledCode`。
pub struct HotSpotCompiledNmethod {
    /// 基类字段（组合替代继承）。
    pub base: super::hotspot_compiled_code::HotSpotCompiledCode,
    /// 对应 `protected final HotSpotResolvedJavaMethod method`。
    pub method: *mut std::ffi::c_void,
    /// 对应 `protected final int entryBCI`。
    pub entry_bci: i32,
    /// 对应 `protected final int id`。
    pub id: i32,
    /// 对应 `protected final long compileState`。
    pub compile_state: i64,
    /// 对应 `protected final boolean hasUnsafeAccess`。
    pub has_unsafe_access: bool,
}

impl CompiledCode for HotSpotCompiledNmethod {}

impl HotSpotCompiledNmethod {
    /// 对应 `getInstallationFailureMessage()`。
    pub fn get_installation_failure_message(&self) -> Option<&str> {
        None
    }

    /// 对应 `hasScopedAccess()`。
    pub fn has_scoped_access(&self) -> bool {
        false
    }
}
