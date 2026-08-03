// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2015, 2019, Oracle and/or its affiliates. All rights reserved.
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

//! 镜像 `jdk.vm.ci.hotspot.HotSpotCompilationRequest`：HotSpot 编译请求。
//!
//! 偏离记录：
//! - Java `class HotSpotCompilationRequest extends CompilationRequest` →
//!   Rust 无继承，用组合方式：`HotSpotCompilationRequest` 持 `CompilationRequest` 作为
//!   基类字段。
//! - 方法名 camelCase → snake_case。

use crate::code::compilation_request::CompilationRequest;

/// 对应 `class HotSpotCompilationRequest extends CompilationRequest`。
pub struct HotSpotCompilationRequest {
    /// 基类字段（组合替代继承）。
    pub base: CompilationRequest,
    /// 对应 `private final long compileState`。
    pub compile_state: i64,
    /// 对应 `private final int id`。
    pub id: i32,
}

impl HotSpotCompilationRequest {
    /// 对应 `getJvmciEnv()`。
    pub fn get_jvmci_env(&self) -> i64 {
        self.compile_state
    }

    /// 对应 `getId()`。
    pub fn get_id(&self) -> i32 {
        self.id
    }
}
