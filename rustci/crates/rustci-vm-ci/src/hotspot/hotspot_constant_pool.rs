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

//! 镜像 `jdk.vm.ci.hotspot.HotSpotConstantPool`：HotSpot 常量池。
//!
//! 偏离记录：
//! - Java `final class HotSpotConstantPool implements ConstantPool, MetaspaceObject` →
//!   Rust `pub trait HotSpotConstantPool: ConstantPool`。
//! - 本期只移植接口方法签名，具体实现留待后端绑定。
//! - 方法名 camelCase → snake_case。

use std::ffi::c_void;

use crate::meta::constant_pool::ConstantPool;
use crate::meta::java_constant::JavaConstant;

/// 对应 `final class HotSpotConstantPool implements ConstantPool`。
pub trait HotSpotConstantPool: ConstantPool {
    /// 对应 `long getConstantPoolPointer()`。
    fn get_constant_pool_pointer(&self) -> i64;

    /// 对应 `HotSpotResolvedObjectType getHolder()`。
    fn get_holder(&self) -> *mut c_void;

    /// 对应 `JavaConstant getStaticFieldConstantValue(int)`。
    fn get_static_field_constant_value(&self, cpi: i32) -> Box<dyn JavaConstant>;

    /// 对应 `String getSourceFileName()`。
    fn get_source_file_name(&self) -> Option<String>;
}
