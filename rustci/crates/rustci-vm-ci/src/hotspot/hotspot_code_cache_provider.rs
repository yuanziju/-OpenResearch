// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2013, 2019, Oracle and/or its affiliates. All rights reserved.
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

//! 镜像 `jdk.vm.ci.hotspot.HotSpotCodeCacheProvider`：HotSpot 代码缓存提供者。
//!
//! 偏离记录：
//! - Java `class HotSpotCodeCacheProvider implements CodeCacheProvider` → Rust
//!   `pub trait HotSpotCodeCacheProvider: CodeCacheProvider`。
//! - 本期只移植接口方法签名，具体实现（与 HotSpotJVMCIRuntime 的耦合）留待后端绑定。
//! - 方法名 camelCase → snake_case。

use std::ffi::c_void;

use crate::code::code_cache_provider::CodeCacheProvider;
use crate::code::installed_code::InstalledCode;

/// 对应 `class HotSpotCodeCacheProvider implements CodeCacheProvider`。
pub trait HotSpotCodeCacheProvider: CodeCacheProvider {
    /// 对应 `String disassemble(InstalledCode)`。
    fn disassemble(&self, code: &InstalledCode) -> Option<String>;

    /// 对应 `int interpreterFrameSize(BytecodeFrame)`。
    fn interpreter_frame_size(&self, pos: *mut c_void) -> i32;

    /// 对应 `void resetCompilationStatistics()`。
    fn reset_compilation_statistics(&self);
}
