// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2009, 2021, Oracle and/or its affiliates. All rights reserved.
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

//! 镜像 `jdk.vm.ci.meta.JavaConstant`：Java 常量值（基本类型/null/对象）的表示。
//!
//! 偏离记录：
//! - Java `JavaConstant extends Constant, JavaValue` → Rust `trait JavaConstant: Constant + JavaValue`。
//! - `asBoxedPrimitive()` 返回 `Object` → Rust `JavaObjectValue`（判别联合覆盖 boxed primitive 值域）。
//! - Java 静态字段（`NULL_POINTER`/`INT_0` 等）→ Rust 关联函数（`null_pointer()`/`int_0()` 等），
//!   返回具体类型 `NullConstant`/`PrimitiveConstant`（均 `Copy`）。Java 静态字段持有单例引用，
//!   Rust 侧每次构造等值新值，行为一致（值类型，无 identity 差异）。
//! - Java `isNull(Constant c)` 静态方法用 `instanceof` → Rust 用 `Constant::as_any` 下转 `JavaConstant`。
//! - 增设 `constant_equals(&dyn JavaConstant) -> bool`：Java 侧 `CallSiteTargetValue.equals` 调
//!   `callSite.equals(other.callSite)`（`JavaConstant` 子类覆写 `Object.equals` 为值相等），
//!   Rust trait 对象无多态 `equals`，故增设此方法由各实现按类型+值比对（`NullConstant`/`PrimitiveConstant`）。
//! - `IllegalArgumentException`（Java 非受检）→ Rust `panic!`。
//! - Java `JavaConstant` 重新声明 `isDefaultForKind()`/`toValueString()`（覆写等价，`toValueString`
//!   带默认实现）。Rust 不支持 subtrait 覆写 supertrait 默认方法签名，且双 trait 同名方法致调用
//!   歧义；故 `JavaConstant` 不重新声明此二方法，统一由 `Constant` 提供（具体类型在 `Constant`
//!   impl 中提供等价实现）。

use crate::meta::constant::Constant;
use crate::meta::java_kind::{JavaKind, JavaObjectValue};
use crate::meta::java_value::JavaValue;
use crate::meta::null_constant::NullConstant;
use crate::meta::primitive_constant::PrimitiveConstant;

/// 对应 `interface JavaConstant extends Constant, JavaValue`。
pub trait JavaConstant: Constant + JavaValue {
    /// 对应 `getJavaKind()`。
    fn get_java_kind(&self) -> JavaKind;

    /// 对应 `isNull()`。
    fn is_null(&self) -> bool;

    /// 对应 `isNonNull()`（默认方法）。
    fn is_non_null(&self) -> bool {
        !self.is_null()
    }

    /// 对应 `asBoxedPrimitive()`：返回 boxed primitive 值。
    fn as_boxed_primitive(&self) -> JavaObjectValue;

    /// 对应 `asInt()`。
    fn as_int(&self) -> i32;

    /// 对应 `asBoolean()`。
    fn as_boolean(&self) -> bool;

    /// 对应 `asLong()`。
    fn as_long(&self) -> i64;

    /// 对应 `asFloat()`。
    fn as_float(&self) -> f32;

    /// 对应 `asDouble()`。
    fn as_double(&self) -> f64;

    /// Rust 增设：对应 Java `CallSiteTargetValue.equals` 中 `callSite.equals(other.callSite)`。
    /// 各实现按类型+值比对（对齐 `NullConstant.equals`/`PrimitiveConstant.equals`）。
    fn constant_equals(&self, other: &dyn JavaConstant) -> bool;
}

/// 对应 `JavaConstant.isNull(Constant c)` 静态方法。
///
/// 偏离记录：Java 用 `c instanceof JavaConstant` 判断后调 `isNull()`。Rust 侧 `Constant`
/// trait 对象无法直接 `instanceof` trait，故逐已知实现类型（`NullConstant`/`PrimitiveConstant`）
/// 下转判断；T7 绑定更多 `JavaConstant` 实现时此处需同步扩展。
pub fn is_null(c: &dyn Constant) -> bool {
    if let Some(nc) = c.as_any().downcast_ref::<NullConstant>() {
        return nc.is_null();
    }
    if let Some(pc) = c.as_any().downcast_ref::<PrimitiveConstant>() {
        return pc.is_null();
    }
    false
}

/// 对应 `JavaConstant.toString(JavaConstant)` 静态方法。
pub fn to_string(constant: &dyn JavaConstant) -> String {
    if constant.get_java_kind() == JavaKind::Illegal {
        "illegal".to_string()
    } else {
        format!(
            "{}[{}]",
            constant.get_java_kind().get_java_name(),
            constant.to_value_string()
        )
    }
}

// --- 静态常量（Java 静态字段 → Rust 关联函数，返回具体 `Copy` 类型） ---

/// 对应 `JavaConstant.NULL_POINTER`。
pub fn null_pointer() -> NullConstant {
    NullConstant
}

/// 对应 `JavaConstant.INT_MINUS_1`。
pub fn int_minus_1() -> PrimitiveConstant {
    PrimitiveConstant::new(JavaKind::Int, -1)
}

/// 对应 `JavaConstant.INT_0`。
pub fn int_0() -> PrimitiveConstant {
    PrimitiveConstant::new(JavaKind::Int, 0)
}

/// 对应 `JavaConstant.INT_1`。
pub fn int_1() -> PrimitiveConstant {
    PrimitiveConstant::new(JavaKind::Int, 1)
}

/// 对应 `JavaConstant.INT_2`。
pub fn int_2() -> PrimitiveConstant {
    PrimitiveConstant::new(JavaKind::Int, 2)
}

/// 对应 `JavaConstant.LONG_0`。
pub fn long_0() -> PrimitiveConstant {
    PrimitiveConstant::new(JavaKind::Long, 0)
}

/// 对应 `JavaConstant.LONG_1`。
pub fn long_1() -> PrimitiveConstant {
    PrimitiveConstant::new(JavaKind::Long, 1)
}

/// 对应 `JavaConstant.FLOAT_0`。
pub fn float_0() -> PrimitiveConstant {
    PrimitiveConstant::new(JavaKind::Float, f32::to_bits(0.0) as i64)
}

/// 对应 `JavaConstant.FLOAT_1`。
pub fn float_1() -> PrimitiveConstant {
    PrimitiveConstant::new(JavaKind::Float, f32::to_bits(1.0) as i64)
}

/// 对应 `JavaConstant.DOUBLE_0`。
pub fn double_0() -> PrimitiveConstant {
    PrimitiveConstant::new(JavaKind::Double, f64::to_bits(0.0) as i64)
}

/// 对应 `JavaConstant.DOUBLE_1`。
pub fn double_1() -> PrimitiveConstant {
    PrimitiveConstant::new(JavaKind::Double, f64::to_bits(1.0) as i64)
}

/// 对应 `JavaConstant.TRUE`。
pub fn true_constant() -> PrimitiveConstant {
    PrimitiveConstant::new(JavaKind::Boolean, 1)
}

/// 对应 `JavaConstant.FALSE`。
pub fn false_constant() -> PrimitiveConstant {
    PrimitiveConstant::new(JavaKind::Boolean, 0)
}

/// 对应 `JavaConstant.ILLEGAL`。
pub fn illegal() -> PrimitiveConstant {
    PrimitiveConstant::new(JavaKind::Illegal, 0)
}

// --- 静态工厂方法 ---

/// 对应 `JavaConstant.forDouble(double)`。
pub fn for_double(d: f64) -> PrimitiveConstant {
    if d == 0.0 && !d.is_nan() {
        return double_0();
    }
    if d == 1.0 {
        return double_1();
    }
    PrimitiveConstant::new(JavaKind::Double, f64::to_bits(d) as i64)
}

/// 对应 `JavaConstant.forFloat(float)`。
pub fn for_float(f: f32) -> PrimitiveConstant {
    if f == 0.0 && !f.is_nan() {
        return float_0();
    }
    if f == 1.0 {
        return float_1();
    }
    PrimitiveConstant::new(JavaKind::Float, f32::to_bits(f) as i64)
}

/// 对应 `JavaConstant.forLong(long)`。
pub fn for_long(i: i64) -> PrimitiveConstant {
    if i == 0 {
        long_0()
    } else if i == 1 {
        long_1()
    } else {
        PrimitiveConstant::new(JavaKind::Long, i)
    }
}

/// 对应 `JavaConstant.forInt(int)`。
pub fn for_int(i: i32) -> PrimitiveConstant {
    match i {
        -1 => int_minus_1(),
        0 => int_0(),
        1 => int_1(),
        2 => int_2(),
        _ => PrimitiveConstant::new(JavaKind::Int, i as i64),
    }
}

/// 对应 `JavaConstant.forByte(byte)`。
pub fn for_byte(i: i8) -> PrimitiveConstant {
    PrimitiveConstant::new(JavaKind::Byte, i as i64)
}

/// 对应 `JavaConstant.forBoolean(boolean)`。
pub fn for_boolean(i: bool) -> PrimitiveConstant {
    if i {
        true_constant()
    } else {
        false_constant()
    }
}

/// 对应 `JavaConstant.forChar(char)`。
pub fn for_char(i: u16) -> PrimitiveConstant {
    PrimitiveConstant::new(JavaKind::Char, i as i64)
}

/// 对应 `JavaConstant.forShort(short)`。
pub fn for_short(i: i16) -> PrimitiveConstant {
    PrimitiveConstant::new(JavaKind::Short, i as i64)
}

/// 对应 `JavaConstant.forIntegerKind(JavaKind, long)`。
pub fn for_integer_kind(kind: JavaKind, i: i64) -> PrimitiveConstant {
    match kind {
        JavaKind::Boolean => for_boolean(i != 0),
        JavaKind::Byte => for_byte(i as i8),
        JavaKind::Short => for_short(i as i16),
        JavaKind::Char => for_char(i as u16),
        JavaKind::Int => for_int(i as i32),
        JavaKind::Long => for_long(i),
        _ => panic!("not an integer kind: {:?}", kind),
    }
}

/// 对应 `JavaConstant.forPrimitiveInt(int bits, long i)`。
pub fn for_primitive_int(bits: i32, i: i64) -> PrimitiveConstant {
    assert!(bits <= 64);
    match bits {
        1 => for_boolean(i != 0),
        8 => for_byte(i as i8),
        16 => for_short(i as i16),
        32 => for_int(i as i32),
        64 => for_long(i),
        _ => panic!("unsupported integer width: {}", bits),
    }
}

/// 对应 `JavaConstant.forPrimitive(char typeChar, long rawValue)`。
pub fn for_primitive_by_char(type_char: char, raw_value: i64) -> PrimitiveConstant {
    for_primitive(
        JavaKind::from_primitive_or_void_type_char(type_char),
        raw_value,
    )
}

/// 对应 `JavaConstant.forPrimitive(JavaKind, long rawValue)`。
pub fn for_primitive(kind: JavaKind, raw_value: i64) -> PrimitiveConstant {
    match kind {
        JavaKind::Boolean => for_boolean(raw_value != 0),
        JavaKind::Byte => for_byte(raw_value as i8),
        JavaKind::Char => for_char(raw_value as u16),
        JavaKind::Short => for_short(raw_value as i16),
        JavaKind::Int => for_int(raw_value as i32),
        JavaKind::Long => for_long(raw_value),
        JavaKind::Float => for_float(f32::from_bits(raw_value as u32)),
        JavaKind::Double => for_double(f64::from_bits(raw_value as u64)),
        _ => panic!("Unsupported kind: {:?}", kind),
    }
}

/// 对应 `JavaConstant.forBoxedPrimitive(Object value)`。Rust 侧以 `JavaObjectValue` 传入。
pub fn for_boxed_primitive(value: &JavaObjectValue) -> Option<PrimitiveConstant> {
    match value {
        JavaObjectValue::Boolean(b) => Some(for_boolean(*b)),
        JavaObjectValue::Byte(b) => Some(for_byte(*b)),
        JavaObjectValue::Char(c) => Some(for_char(*c)),
        JavaObjectValue::Short(s) => Some(for_short(*s)),
        JavaObjectValue::Int(i) => Some(for_int(*i)),
        JavaObjectValue::Long(l) => Some(for_long(*l)),
        JavaObjectValue::Float(f) => Some(for_float(*f)),
        JavaObjectValue::Double(d) => Some(for_double(*d)),
        _ => None,
    }
}

/// 对应 `JavaConstant.forIllegal()`。
pub fn for_illegal() -> PrimitiveConstant {
    illegal()
}

/// 对应 `JavaConstant.defaultForKind(JavaKind)`。返回 `Box<dyn JavaConstant>`。
pub fn default_for_kind(kind: JavaKind) -> Box<dyn JavaConstant> {
    match kind {
        JavaKind::Boolean => Box::new(false_constant()),
        JavaKind::Byte => Box::new(for_byte(0)),
        JavaKind::Char => Box::new(for_char(0)),
        JavaKind::Short => Box::new(for_short(0)),
        JavaKind::Int => Box::new(int_0()),
        JavaKind::Double => Box::new(double_0()),
        JavaKind::Float => Box::new(float_0()),
        JavaKind::Long => Box::new(long_0()),
        JavaKind::Object => Box::new(NullConstant),
        _ => panic!("Unsupported kind: {:?}", kind),
    }
}
