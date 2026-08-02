// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2009, 2015, Oracle and/or its affiliates. All rights reserved.
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
 * or visit www.oracle if you need additional information or have any
 * questions.
 */

//! 镜像 `jdk.vm.ci.meta.RawConstant`：以 `JavaKind.Int` 持有原始 long 值的 `PrimitiveConstant` 子类。
//!
//! 偏离记录：Java `public class RawConstant extends PrimitiveConstant` → Rust `pub struct RawConstant`
//! 持 `PrimitiveConstant` 字段并以 `Deref`/转发方法暴露 `PrimitiveConstant` 行为。Rust struct 无
//! 继承，故组合 + 转发；`RawConstant` 自身作为 `JavaConstant`/`Constant`/`JavaValue`/
//! `SerializableConstant` 的实现委托给内部 `PrimitiveConstant`。

use std::fmt;

use crate::meta::byte_buffer::ByteBuffer;
use crate::meta::constant::Constant;
use crate::meta::java_constant::JavaConstant;
use crate::meta::java_kind::{JavaKind, JavaObjectValue};
use crate::meta::java_value::JavaValue;
use crate::meta::primitive_constant::PrimitiveConstant;
use crate::meta::serializable_constant::SerializableConstant;

/// 对应 `public class RawConstant extends PrimitiveConstant`。
#[derive(Debug, Clone, Copy)]
pub struct RawConstant {
    inner: PrimitiveConstant,
}

impl RawConstant {
    /// 对应 `public RawConstant(long rawValue) { super(JavaKind.Int, rawValue); }`。
    pub fn new(raw_value: i64) -> Self {
        Self {
            inner: PrimitiveConstant::new(JavaKind::Int, raw_value),
        }
    }

    /// 对应 `PrimitiveConstant.getRawValue()`。
    pub fn get_raw_value(&self) -> i64 {
        self.inner.get_raw_value()
    }
}

impl JavaConstant for RawConstant {
    fn get_java_kind(&self) -> JavaKind {
        self.inner.get_java_kind()
    }

    fn is_null(&self) -> bool {
        self.inner.is_null()
    }

    fn as_boxed_primitive(&self) -> JavaObjectValue {
        self.inner.as_boxed_primitive()
    }

    fn as_int(&self) -> i32 {
        self.inner.as_int()
    }

    fn as_boolean(&self) -> bool {
        self.inner.as_boolean()
    }

    fn as_long(&self) -> i64 {
        self.inner.as_long()
    }

    fn as_float(&self) -> f32 {
        self.inner.as_float()
    }

    fn as_double(&self) -> f64 {
        self.inner.as_double()
    }

    fn constant_equals(&self, other: &dyn JavaConstant) -> bool {
        // 对齐 `PrimitiveConstant.equals`（RawConstant 经 super 走 PrimitiveConstant.equals）。
        match Constant::as_any(other).downcast_ref::<PrimitiveConstant>() {
            None => false,
            Some(o) => self.inner == *o,
        }
    }
}

impl Constant for RawConstant {
    fn is_default_for_kind(&self) -> bool {
        Constant::is_default_for_kind(&self.inner)
    }

    fn to_value_string(&self) -> String {
        Constant::to_value_string(&self.inner)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl JavaValue for RawConstant {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl SerializableConstant for RawConstant {
    fn get_serialized_size(&self) -> i32 {
        self.inner.get_serialized_size()
    }

    fn serialize(&self, buffer: &mut ByteBuffer) {
        self.inner.serialize(buffer);
    }
}

impl From<RawConstant> for PrimitiveConstant {
    fn from(value: RawConstant) -> Self {
        value.inner
    }
}

impl fmt::Display for RawConstant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.inner, f)
    }
}
