// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2012, 2019, Oracle and/or its affiliates. All rights reserved.
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

//! 镜像 `jdk.vm.ci.hotspot.HotSpotResolvedJavaType`：HotSpot 已解析 Java 类型抽象类。
//!
//! 偏离记录：
//! - Java `abstract class HotSpotResolvedJavaType extends HotSpotJavaType implements ResolvedJavaType` →
//!   Rust `pub struct HotSpotResolvedJavaType`，本期只声明数据结构，方法实现留待后端绑定。
//! - Rust 无抽象类，用 struct + 方法 + 子类型组合表达。
//! - 方法名 camelCase → snake_case。

/// 对应 `abstract class HotSpotResolvedJavaType`。
pub struct HotSpotResolvedJavaType {
    /// 对应构造函数参数 `String name`。
    pub name: String,
    /// 对应 `HotSpotResolvedObjectTypeImpl arrayOfType`。
    pub array_of_type: *mut std::ffi::c_void,
}

impl HotSpotResolvedJavaType {
    /// 对应 `HotSpotResolvedJavaType(String name)`。
    pub fn new(name: String) -> Self {
        Self {
            name,
            array_of_type: std::ptr::null_mut(),
        }
    }

    /// 对应 `getName()`。
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// 对应 `getArrayClass()`。
    pub fn get_array_class(&self) -> *mut std::ffi::c_void {
        self.array_of_type
    }

    /// 对应 `isBeingInitialized()`。
    pub fn is_being_initialized(&self) -> bool {
        false
    }
}
