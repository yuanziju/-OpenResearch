// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2012, 2015, Oracle and/or its affiliates. All rights reserved.
 * DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
 *
 * This code is free software; you can redistribute it and/or modify it
 * under the terms of the GNU General Public License version 2 only, as
 * published by the Free Software Foundation.
 *
 * This code is distributed in the hope that it will be useful, but WITHOUT
 * ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
 * FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License
 * version 2 for more details (a copy has been included in the LICENSE file that
 * accompanied this code).
 *
 * You should have received a copy of the GNU General Public License version
 * 2 along with this work; if not, write to the Free Software Foundation,
 * Inc., 51 Franklin St, Fifth Floor, Boston, MA 02110-1301 USA.
 *
 * Please contact Oracle, 500 Oracle Packaging, Redwood Shores, CA 94065 USA
 * or visit www.oracle.com if you need additional information or have any
 * questions.
 */

//! 镜像 `jdk.vm.ci.code.ValueUtil`：`Value` 及其子类的工具方法（全静态）。
//!
//! 偏离记录：Java `instanceof` 下转 → Rust `Any::downcast_ref`（经 `Value::as_any`/`JavaValue::as_any`）。
//! - `isIllegal(Value)`/`isIllegalJavaValue(JavaValue)`：Java `Value.ILLEGAL.equals(value)` →
//!   `IllegalValue.equals` 按 `instanceof IllegalValue` 判等，等价于下转 `IllegalValue`。
//! - `isConstantJavaValue(JavaValue)`：Java `instanceof JavaConstant`（trait）→ Rust 无法下转 trait，
//!   逐已知 `JavaConstant` 具体实现（`PrimitiveConstant`/`NullConstant`/`RawConstant`）下转判断，
//!   与 `JavaConstant::is_null` 静态方法一致；T7 绑定更多实现时需同步扩展。
//! - `isAllocatableValue(Value)`：Java `instanceof AllocatableValue`（trait）→ Rust 逐已知
//!   `AllocatableValue` 具体实现（`RegisterValue`/`StackSlot`/`IllegalValue`）下转判断。
//! - `asConstantJavaValue`/`asAllocatableValue` 返回 `&dyn` trait 对象（Java 返回 trait 类型）。
//! - `asRegister(Value, PlatformKind)` 的 `value.getPlatformKind() != kind`（Java 引用相等）→
//!   Rust `std::ptr::eq` 比对 fat 指针（对齐 `PlatformKind` 默认引用相等语义）。

use crate::code::register::Register;
use crate::code::register_value::RegisterValue;
use crate::code::stack_slot::StackSlot;
use crate::code::virtual_object::VirtualObject;
use crate::meta::allocatable_value::AllocatableValue;
use crate::meta::java_constant::JavaConstant;
use crate::meta::java_value::JavaValue;
use crate::meta::null_constant::NullConstant;
use crate::meta::platform_kind::PlatformKind;
use crate::meta::primitive_constant::PrimitiveConstant;
use crate::meta::raw_constant::RawConstant;
use crate::meta::value::{IllegalValue, Value};

/// 对应 `isIllegal(Value)`。
pub fn is_illegal(value: &dyn Value) -> bool {
    value.as_any().is::<IllegalValue>()
}

/// 对应 `isIllegalJavaValue(JavaValue)`。
pub fn is_illegal_java_value(value: &dyn JavaValue) -> bool {
    value.as_any().is::<IllegalValue>()
}

/// 对应 `isLegal(Value)`。
pub fn is_legal(value: &dyn Value) -> bool {
    !is_illegal(value)
}

/// 对应 `isVirtualObject(JavaValue)`。
pub fn is_virtual_object(value: &dyn JavaValue) -> bool {
    value.as_any().is::<VirtualObject>()
}

/// 对应 `asVirtualObject(JavaValue)`。
pub fn as_virtual_object(value: &dyn JavaValue) -> &VirtualObject {
    value
        .as_any()
        .downcast_ref::<VirtualObject>()
        .expect("ClassCastException: not a VirtualObject")
}

/// 对应 `isConstantJavaValue(JavaValue)`。
pub fn is_constant_java_value(value: &dyn JavaValue) -> bool {
    value.as_any().is::<PrimitiveConstant>()
        || value.as_any().is::<NullConstant>()
        || value.as_any().is::<RawConstant>()
}

/// 对应 `asConstantJavaValue(JavaValue)`。
pub fn as_constant_java_value(value: &dyn JavaValue) -> &dyn JavaConstant {
    if let Some(pc) = value.as_any().downcast_ref::<PrimitiveConstant>() {
        return pc;
    }
    if let Some(nc) = value.as_any().downcast_ref::<NullConstant>() {
        return nc;
    }
    if let Some(rc) = value.as_any().downcast_ref::<RawConstant>() {
        return rc;
    }
    panic!("ClassCastException: not a JavaConstant")
}

/// 对应 `isAllocatableValue(Value)`。
pub fn is_allocatable_value(value: &dyn Value) -> bool {
    value.as_any().is::<RegisterValue>()
        || value.as_any().is::<StackSlot>()
        || value.as_any().is::<IllegalValue>()
}

/// 对应 `asAllocatableValue(Value)`。
pub fn as_allocatable_value(value: &dyn Value) -> &dyn AllocatableValue {
    if let Some(rv) = value.as_any().downcast_ref::<RegisterValue>() {
        return rv;
    }
    if let Some(ss) = value.as_any().downcast_ref::<StackSlot>() {
        return ss;
    }
    if let Some(iv) = value.as_any().downcast_ref::<IllegalValue>() {
        return iv;
    }
    panic!("ClassCastException: not an AllocatableValue")
}

/// 对应 `isStackSlot(Value)`。
pub fn is_stack_slot(value: &dyn Value) -> bool {
    value.as_any().is::<StackSlot>()
}

/// 对应 `asStackSlot(Value)`。
pub fn as_stack_slot(value: &dyn Value) -> &StackSlot {
    value
        .as_any()
        .downcast_ref::<StackSlot>()
        .expect("ClassCastException: not a StackSlot")
}

/// 对应 `isRegister(Value)`。
pub fn is_register(value: &dyn Value) -> bool {
    value.as_any().is::<RegisterValue>()
}

/// 对应 `asRegisterValue(Value)`。
pub fn as_register_value(value: &dyn Value) -> &RegisterValue {
    value
        .as_any()
        .downcast_ref::<RegisterValue>()
        .expect("ClassCastException: not a RegisterValue")
}

/// 对应 `asRegister(Value)`。
pub fn as_register(value: &dyn Value) -> Register {
    as_register_value(value).get_register()
}

/// 对应 `asRegister(Value, PlatformKind)`：kind 不匹配抛 `InternalError`。
pub fn as_register_with_kind(value: &dyn Value, kind: &dyn PlatformKind) -> Register {
    let value_kind = value.get_platform_kind();
    if !std::ptr::eq(
        value_kind as *const dyn PlatformKind,
        kind as *const dyn PlatformKind,
    ) {
        // `PlatformKind` 无 `Debug` 超 trait，用 `name()` 字符串化（对齐 Java `toString` 默认行为）。
        panic!(
            "InternalError: needed: {} got: {}",
            kind.name(),
            value_kind.name()
        );
    }
    as_register(value)
}
