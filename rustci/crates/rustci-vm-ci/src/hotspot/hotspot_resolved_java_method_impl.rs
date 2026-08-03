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

//! 镜像 `jdk.vm.ci.hotspot.HotSpotResolvedJavaMethodImpl`：HotSpot 已解析方法实现。
//!
//! 偏离记录：
//! - Java `final class HotSpotResolvedJavaMethodImpl extends HotSpotMethod implements HotSpotResolvedJavaMethod, MetaspaceHandleObject` →
//!   Rust `pub struct HotSpotResolvedJavaMethodImpl`，本期只声明数据结构，方法实现留待后端绑定。
//! - 方法名 camelCase → snake_case。

/// 对应 `final class HotSpotResolvedJavaMethodImpl`。
pub struct HotSpotResolvedJavaMethodImpl {
    /// 对应 `private final long methodHandle`（JNI 层的 Method* 句柄）。
    pub method_handle: i64,
    /// 对应 `private final HotSpotResolvedObjectTypeImpl holder`。
    pub holder: *mut std::ffi::c_void,
    /// 对应 `private final HotSpotConstantPool constantPool`。
    pub constant_pool: *mut std::ffi::c_void,
    /// 对应 `final HotSpotSignature signature`。
    pub signature: *mut std::ffi::c_void,
}

impl HotSpotResolvedJavaMethodImpl {
    /// 对应 `getMethodPointer()`。
    pub fn get_method_pointer(&self) -> i64 {
        self.method_handle
    }

    /// 对应 `getName()`。
    pub fn get_name(&self) -> String {
        String::new()
    }

    /// 对应 `getDeclaringClass()`。
    pub fn get_declaring_class(&self) -> *mut std::ffi::c_void {
        self.holder
    }

    /// 对应 `getConstantPool()`。
    pub fn get_constant_pool(&self) -> *mut std::ffi::c_void {
        self.constant_pool
    }
}
