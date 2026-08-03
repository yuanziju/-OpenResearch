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

//! 镜像 `jdk.vm.ci.hotspot.HotSpotResolvedObjectTypeImpl`：HotSpot 已解析对象类型实现。
//!
//! 偏离记录：
//! - Java `final class HotSpotResolvedObjectTypeImpl extends HotSpotResolvedJavaType implements HotSpotResolvedObjectType, MetaspaceObject` →
//!   Rust `pub struct HotSpotResolvedObjectTypeImpl`，本期只声明数据结构，方法实现留待后端绑定。
//! - 方法名 camelCase → snake_case。

/// 对应 `final class HotSpotResolvedObjectTypeImpl`。
pub struct HotSpotResolvedObjectTypeImpl {
    /// 基类字段（组合替代继承）。
    pub base: super::hotspot_resolved_java_type::HotSpotResolvedJavaType,
    /// 对应 `private final long klassPointer`（Klass*）。
    pub klass_pointer: i64,
    /// 对应 `private final JavaConstant mirror`。
    pub mirror: *mut std::ffi::c_void,
}

impl HotSpotResolvedObjectTypeImpl {
    /// 对应 `HotSpotResolvedObjectTypeImpl(long, String)`。
    pub fn new(klass: i64, name: String) -> Self {
        assert!(klass != 0);
        Self {
            base: super::hotspot_resolved_java_type::HotSpotResolvedJavaType::new(name),
            klass_pointer: klass,
            mirror: std::ptr::null_mut(),
        }
    }

    /// 对应 `getKlassPointer()`。
    pub fn get_klass_pointer(&self) -> i64 {
        self.klass_pointer
    }

    /// 对应 `getMetaspacePointer()`。
    pub fn get_metaspace_pointer(&self) -> i64 {
        self.klass_pointer
    }

    /// 对应 `getModifiers()`。
    pub fn get_modifiers(&self) -> i32 {
        0
    }

    /// 对应 `getAccessFlags()`。
    pub fn get_access_flags(&self) -> i32 {
        0
    }

    /// 对应 `getMiscFlags()`。
    pub fn get_misc_flags(&self) -> i32 {
        0
    }
}
