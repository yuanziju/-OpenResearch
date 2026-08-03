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

//! 镜像 `jdk.vm.ci.hotspot.HotSpotConstantReflectionProvider`：HotSpot 常量反射提供者。
//!
//! 偏离记录：
//! - Java `class HotSpotConstantReflectionProvider implements ConstantReflectionProvider` →
//!   Rust `pub trait HotSpotConstantReflectionProvider: ConstantReflectionProvider`。
//! - 本期只移植接口方法签名，具体实现留待后端绑定。
//! - 方法名 camelCase → snake_case。

use crate::meta::constant::Constant;
use crate::meta::constant_reflection::ConstantReflectionProvider;
use crate::meta::java_constant::JavaConstant;
use crate::meta::resolved_java_type::ResolvedJavaType;

/// 对应 `class HotSpotConstantReflectionProvider implements ConstantReflectionProvider`。
pub trait HotSpotConstantReflectionProvider: ConstantReflectionProvider {
    /// 对应 `JavaConstant forObject(Object)`。
    fn for_object(&self, value: *mut std::ffi::c_void) -> Box<dyn JavaConstant>;

    /// 对应 `JavaConstant forString(String)`。
    fn for_string(&self, value: &str) -> Box<dyn JavaConstant>;

    /// 对应 `Constant asObjectHub(ResolvedJavaType)`。
    fn as_object_hub(&self, type_: &dyn ResolvedJavaType) -> Box<dyn Constant>;
}
