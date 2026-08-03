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

//! 镜像 `jdk.vm.ci.hotspot.HotSpotCompiledCode`：HotSpot 编译产物。
//!
//! 偏离记录：
//! - Java `class HotSpotCompiledCode implements CompiledCode` → Rust
//!   `pub struct HotSpotCompiledCode`。字段用 `*mut c_void` 不透明指针表示
//!   （Java 数组/对象在 native 层为 JNI 引用，Rust 侧不解释）。
//! - Rust 无继承，`CompiledCode` trait 实现见 `impl CompiledCode for HotSpotCompiledCode`。
//! - 方法名 camelCase → snake_case。

use crate::code::compiled_code::CompiledCode;

/// 对应 `class HotSpotCompiledCode implements CompiledCode`。
pub struct HotSpotCompiledCode {
    /// 对应 `protected final String name`。
    pub name: String,
    /// 对应 `protected final byte[] targetCode`。
    pub target_code: *mut std::ffi::c_void,
    /// 对应 `protected final int targetCodeSize`。
    pub target_code_size: i32,
    /// 对应 `protected final Site[] sites`。
    pub sites: *mut std::ffi::c_void,
    /// 对应 `protected final Assumption[] assumptions`。
    pub assumptions: *mut std::ffi::c_void,
    /// 对应 `protected final ResolvedJavaMethod[] methods`。
    pub methods: *mut std::ffi::c_void,
    /// 对应 `protected final Comment[] comments`。
    pub comments: *mut std::ffi::c_void,
    /// 对应 `protected final byte[] dataSection`。
    pub data_section: *mut std::ffi::c_void,
    /// 对应 `protected final int dataSectionAlignment`。
    pub data_section_alignment: i32,
    /// 对应 `protected final DataPatch[] dataSectionPatches`。
    pub data_section_patches: *mut std::ffi::c_void,
    /// 对应 `protected final boolean isImmutablePIC`。
    pub is_immutable_pic: bool,
    /// 对应 `protected final int totalFrameSize`。
    pub total_frame_size: i32,
    /// 对应 `protected final StackSlot deoptRescueSlot`。
    pub deopt_rescue_slot: *mut std::ffi::c_void,
}

impl CompiledCode for HotSpotCompiledCode {}

impl HotSpotCompiledCode {
    /// 对应 `getName()`。
    pub fn get_name(&self) -> &str {
        &self.name
    }
}
