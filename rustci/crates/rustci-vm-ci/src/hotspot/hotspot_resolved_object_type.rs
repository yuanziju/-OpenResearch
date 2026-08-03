// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2011, 2015, Oracle and/or its affiliates. All rights reserved.
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

//! 镜像 `jdk.vm.ci.hotspot.HotSpotResolvedObjectType`：HotSpot 已解析对象类型接口。
//!
//! 偏离记录：
//! - Java `interface HotSpotResolvedObjectType extends ResolvedJavaType` →
//!   Rust `pub trait HotSpotResolvedObjectType: ResolvedJavaType`。
//! - 本期只移植接口方法签名，具体实现留待后端绑定。
//! - 方法名 camelCase → snake_case。

use std::ffi::c_void;

use crate::meta::constant::Constant;
use crate::meta::java_kind::JavaKind;
use crate::meta::resolved_java_type::ResolvedJavaType;

/// 对应 `interface HotSpotResolvedObjectType extends ResolvedJavaType`。
pub trait HotSpotResolvedObjectType: ResolvedJavaType {
    /// 对应 `HotSpotResolvedObjectType getSupertype()`。
    fn get_supertype(&self) -> Option<Box<dyn HotSpotResolvedObjectType>>;

    /// 对应 `ConstantPool getConstantPool()`。
    fn get_constant_pool(&self) -> *mut c_void;

    /// 对应 `int instanceSize()`。
    fn instance_size(&self) -> i32;

    /// 对应 `int getVtableLength()`。
    fn get_vtable_length(&self) -> i32;

    /// 对应 `boolean isDefinitelyResolvedWithRespectTo(ResolvedJavaType)`。
    fn is_definitely_resolved_with_respect_to(
        &self,
        accessing_class: &dyn ResolvedJavaType,
    ) -> bool;

    /// 对应 `Constant klass()`。
    fn klass(&self) -> Box<dyn Constant>;

    /// 对应 `boolean isPrimaryType()`。
    fn is_primary_type(&self) -> bool;

    /// 对应 `int superCheckOffset()`。
    fn super_check_offset(&self) -> i32;

    /// 对应 `long prototypeMarkWord()`。
    fn prototype_mark_word(&self) -> i64;

    /// 对应 `int layoutHelper()`。
    fn layout_helper(&self) -> i32;

    /// 对应 `default boolean isPrimitive()`。
    fn is_primitive(&self) -> bool {
        false
    }

    /// 对应 `default JavaKind getJavaKind()`。
    fn get_java_kind(&self) -> JavaKind {
        JavaKind::Object
    }
}
