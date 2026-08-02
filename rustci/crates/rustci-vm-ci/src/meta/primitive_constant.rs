// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2009, 2019, Oracle and/or its affiliates. All rights reserved.
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

//! 镜像 `jdk.vm.ci.meta.PrimitiveConstant`：基本类型常量值。
//!
//! 偏离记录：
//! - Java `public class PrimitiveConstant implements JavaConstant, SerializableConstant`
//!   → Rust `pub struct PrimitiveConstant` 实现 `JavaConstant + SerializableConstant`。
//! - Java 构造器 `protected PrimitiveConstant(JavaKind, long)` → Rust `pub fn new(JavaKind, i64)`。
//!   构造期 `assert kind.isPrimitive() || kind == JavaKind.Illegal` → Rust `debug_assert!`。
//! - Java `forTypeChar(char, long)` 静态方法 → Rust `for_type_char(char, i64)` 关联函数，
//!   调 `JavaConstant::for_integer_kind`。
//! - `asBoxedPrimitive()` 返回 `Object` → Rust `JavaObjectValue`（boxed primitive 变体）。
//! - `IllegalArgumentException`（Java 非受检）→ Rust `panic!`。

use std::fmt;

use crate::meta::byte_buffer::ByteBuffer;
use crate::meta::constant::Constant;
use crate::meta::java_constant::JavaConstant;
use crate::meta::java_kind::{JavaKind, JavaObjectValue};
use crate::meta::java_value::JavaValue;
use crate::meta::serializable_constant::SerializableConstant;

/// 对应 `public class PrimitiveConstant implements JavaConstant, SerializableConstant`。
#[derive(Debug, Clone, Copy)]
pub struct PrimitiveConstant {
    kind: JavaKind,
    /// 对应 Java `private final long primitive`：float/double 时为
    /// `Float.floatToRawIntBits`/`Double.doubleToRawLongBits` 结果。
    primitive: i64,
}

impl PrimitiveConstant {
    /// 对应 `protected PrimitiveConstant(JavaKind kind, long primitive)`。
    pub fn new(kind: JavaKind, primitive: i64) -> Self {
        debug_assert!(kind.is_primitive() || kind == JavaKind::Illegal);
        Self { kind, primitive }
    }

    /// 对应 `static PrimitiveConstant forTypeChar(char kind, long i)`。
    pub fn for_type_char(kind: char, i: i64) -> Self {
        crate::meta::java_constant::for_integer_kind(
            JavaKind::from_primitive_or_void_type_char(kind),
            i,
        )
    }

    /// 对应 `getRawValue()`。
    pub fn get_raw_value(&self) -> i64 {
        self.primitive
    }
}

impl JavaConstant for PrimitiveConstant {
    fn get_java_kind(&self) -> JavaKind {
        self.kind
    }

    fn is_null(&self) -> bool {
        false
    }

    fn as_boxed_primitive(&self) -> JavaObjectValue {
        match self.kind {
            JavaKind::Byte => JavaObjectValue::Byte(self.primitive as i8),
            JavaKind::Boolean => JavaObjectValue::Boolean(self.primitive != 0),
            JavaKind::Short => JavaObjectValue::Short(self.primitive as i16),
            JavaKind::Char => JavaObjectValue::Char(self.primitive as u16),
            JavaKind::Int => JavaObjectValue::Int(self.as_int()),
            JavaKind::Long => JavaObjectValue::Long(self.as_long()),
            JavaKind::Float => JavaObjectValue::Float(self.as_float()),
            JavaKind::Double => JavaObjectValue::Double(self.as_double()),
            _ => panic!("IllegalArgumentException: unexpected kind {:?}", self.kind),
        }
    }

    fn as_int(&self) -> i32 {
        debug_assert_eq!(
            self.kind.get_stack_kind(),
            JavaKind::Int,
            "{:?}",
            self.kind.get_stack_kind()
        );
        self.primitive as i32
    }

    fn as_boolean(&self) -> bool {
        // 对应 `asBoolean()`：仅 Boolean kind 合法，返回 `primitive != 0`。
        debug_assert_eq!(self.kind, JavaKind::Boolean);
        self.primitive != 0
    }

    fn as_long(&self) -> i64 {
        debug_assert!(self.kind.is_numeric_integer());
        self.primitive
    }

    fn as_float(&self) -> f32 {
        debug_assert_eq!(self.kind, JavaKind::Float);
        f32::from_bits(self.primitive as u32)
    }

    fn as_double(&self) -> f64 {
        debug_assert_eq!(self.kind, JavaKind::Double);
        f64::from_bits(self.primitive as u64)
    }

    fn constant_equals(&self, other: &dyn JavaConstant) -> bool {
        // 对应 `PrimitiveConstant.equals`：同型且 kind/primitive 相等。
        match Constant::as_any(other).downcast_ref::<PrimitiveConstant>() {
            None => false,
            Some(o) => self.kind == o.kind && self.primitive == o.primitive,
        }
    }
}

impl Constant for PrimitiveConstant {
    fn is_default_for_kind(&self) -> bool {
        self.primitive == 0
    }

    fn to_value_string(&self) -> String {
        // 对应 `JavaConstant.toValueString()` 默认实现（JavaConstant.java:136-142）：
        //   default String toValueString() {
        //       if (getJavaKind() == JavaKind.Illegal) return "illegal";
        //       return getJavaKind().format(asBoxedPrimitive());
        //   }
        // PrimitiveConstant 未覆写 toValueString()，继承默认实现。
        // 不得回调 java_constant::to_string(self)——后者单向调 to_value_string()，
        // 反向调用形成无限递归（B1 栈溢出）。
        if self.kind == JavaKind::Illegal {
            "illegal".to_string()
        } else {
            self.kind.format(&self.as_boxed_primitive())
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl JavaValue for PrimitiveConstant {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl SerializableConstant for PrimitiveConstant {
    fn get_serialized_size(&self) -> i32 {
        self.kind.get_byte_count()
    }

    fn serialize(&self, buffer: &mut ByteBuffer) {
        match self.kind {
            JavaKind::Byte | JavaKind::Boolean => {
                buffer.put(self.primitive as i8);
            }
            JavaKind::Short => {
                buffer.put_short(self.primitive as i16);
            }
            JavaKind::Char => {
                buffer.put_char(self.primitive as u16);
            }
            JavaKind::Int => {
                buffer.put_int(self.as_int());
            }
            JavaKind::Long => {
                buffer.put_long(self.as_long());
            }
            JavaKind::Float => {
                buffer.put_float(self.as_float());
            }
            JavaKind::Double => {
                buffer.put_double(self.as_double());
            }
            _ => panic!("IllegalArgumentException: unexpected kind {:?}", self.kind),
        }
    }
}

impl PartialEq for PrimitiveConstant {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind && self.primitive == other.primitive
    }
}

impl Eq for PrimitiveConstant {}

impl std::hash::Hash for PrimitiveConstant {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // 对应 `hashCode() { return (int)(primitive ^ (primitive >>> 32)) * (getJavaKind().ordinal() + 31); }`
        let ordinal = match self.kind {
            JavaKind::Boolean => 0,
            JavaKind::Byte => 1,
            JavaKind::Short => 2,
            JavaKind::Char => 3,
            JavaKind::Int => 4,
            JavaKind::Float => 5,
            JavaKind::Long => 6,
            JavaKind::Double => 7,
            JavaKind::Object => 8,
            JavaKind::Void => 9,
            JavaKind::Illegal => 10,
        };
        let mixed = ((self.primitive ^ (self.primitive >> 32)) as u32) as i64;
        let h = mixed.wrapping_mul((ordinal as i64) + 31);
        (h as u64).hash(state);
    }
}

impl fmt::Display for PrimitiveConstant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 对应 `PrimitiveConstant.toString()`。
        if self.kind == JavaKind::Illegal {
            f.write_str("illegal")
        } else {
            write!(
                f,
                "{}[{:?}|0x{:x}]",
                self.kind.get_java_name(),
                self.as_boxed_primitive(),
                self.primitive
            )
        }
    }
}
