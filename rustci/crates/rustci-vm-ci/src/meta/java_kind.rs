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

//! 镜像 `jdk.vm.ci.meta.JavaKind`。
//!
//! 偏离记录：Java `JavaKind` 是带每变体构造参数的 enum（typeChar/basicType/javaName/
//! slotCount/isStackInt/primitiveJavaClass/boxedJavaClass）。Rust 侧用无字段 enum +
//! 私有 `KindData`（经 `data()` 取回）镜像每变体常量数据。`Class<?>` 字段映射为
//! `Option<JavaClass>`（primitive 为 `Some`，Object/Illegal 为 `None`），保留
//! `isPrimitive`/`fromJavaClass` 的区分行为。`format(Object)` 的 `Object` 参数映射为
//! `JavaObjectValue` 判别联合，覆盖原 Java `format` 通过 `instanceof` 分派的全部值域
//! （boxed primitive / String / JavaType / Enum / FormatWithToString / Class / array /
//! 其它），逐分支对齐 Java 行为。

use crate::meta::java_reflect::JavaClass;
use crate::meta::java_type::JavaType;

/// 镜像 `jdk.vm.ci.meta.JavaKind`：CRI 中类型的基本种类（primitive、Object、Void、Illegal）。
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum JavaKind {
    /// primitive boolean，栈上以 int 表示。
    Boolean,
    /// primitive byte，栈上以 int 表示。
    Byte,
    /// primitive short，栈上以 int 表示。
    Short,
    /// primitive char，栈上以 int 表示。
    Char,
    /// primitive int。
    Int,
    /// primitive float。
    Float,
    /// primitive long。
    Long,
    /// primitive double。
    Double,
    /// Object 种类（数组同此）。
    Object,
    /// void 种类。
    Void,
    /// 非类型。
    Illegal,
}

/// 私有：对应 Java `JavaKind` 各变体的构造常量。
struct KindData {
    type_char: char,
    basic_type: i32,
    java_name: &'static str,
    slot_count: i32,
    is_stack_int: bool,
    primitive_java_class: Option<JavaClass>,
    boxed_java_class: Option<JavaClass>,
}

const BOOLEAN_TYPE: JavaClass = JavaClass::new("boolean");
const BYTE_TYPE: JavaClass = JavaClass::new("byte");
const SHORT_TYPE: JavaClass = JavaClass::new("short");
const CHAR_TYPE: JavaClass = JavaClass::new("char");
const INT_TYPE: JavaClass = JavaClass::new("int");
const FLOAT_TYPE: JavaClass = JavaClass::new("float");
const LONG_TYPE: JavaClass = JavaClass::new("long");
const DOUBLE_TYPE: JavaClass = JavaClass::new("double");
const VOID_TYPE: JavaClass = JavaClass::new("void");

const BOOLEAN_BOXED: JavaClass = JavaClass::new("java.lang.Boolean");
const BYTE_BOXED: JavaClass = JavaClass::new("java.lang.Byte");
const SHORT_BOXED: JavaClass = JavaClass::new("java.lang.Short");
const CHAR_BOXED: JavaClass = JavaClass::new("java.lang.Character");
const INT_BOXED: JavaClass = JavaClass::new("java.lang.Integer");
const FLOAT_BOXED: JavaClass = JavaClass::new("java.lang.Float");
const LONG_BOXED: JavaClass = JavaClass::new("java.lang.Long");
const DOUBLE_BOXED: JavaClass = JavaClass::new("java.lang.Double");
const VOID_BOXED: JavaClass = JavaClass::new("java.lang.Void");

impl JavaKind {
    const fn data(self) -> KindData {
        match self {
            JavaKind::Boolean => KindData {
                type_char: 'Z',
                basic_type: 4,
                java_name: "boolean",
                slot_count: 1,
                is_stack_int: true,
                primitive_java_class: Some(BOOLEAN_TYPE),
                boxed_java_class: Some(BOOLEAN_BOXED),
            },
            JavaKind::Byte => KindData {
                type_char: 'B',
                basic_type: 8,
                java_name: "byte",
                slot_count: 1,
                is_stack_int: true,
                primitive_java_class: Some(BYTE_TYPE),
                boxed_java_class: Some(BYTE_BOXED),
            },
            JavaKind::Short => KindData {
                type_char: 'S',
                basic_type: 9,
                java_name: "short",
                slot_count: 1,
                is_stack_int: true,
                primitive_java_class: Some(SHORT_TYPE),
                boxed_java_class: Some(SHORT_BOXED),
            },
            JavaKind::Char => KindData {
                type_char: 'C',
                basic_type: 5,
                java_name: "char",
                slot_count: 1,
                is_stack_int: true,
                primitive_java_class: Some(CHAR_TYPE),
                boxed_java_class: Some(CHAR_BOXED),
            },
            JavaKind::Int => KindData {
                type_char: 'I',
                basic_type: 10,
                java_name: "int",
                slot_count: 1,
                is_stack_int: true,
                primitive_java_class: Some(INT_TYPE),
                boxed_java_class: Some(INT_BOXED),
            },
            JavaKind::Float => KindData {
                type_char: 'F',
                basic_type: 6,
                java_name: "float",
                slot_count: 1,
                is_stack_int: false,
                primitive_java_class: Some(FLOAT_TYPE),
                boxed_java_class: Some(FLOAT_BOXED),
            },
            JavaKind::Long => KindData {
                type_char: 'J',
                basic_type: 11,
                java_name: "long",
                slot_count: 2,
                is_stack_int: false,
                primitive_java_class: Some(LONG_TYPE),
                boxed_java_class: Some(LONG_BOXED),
            },
            JavaKind::Double => KindData {
                type_char: 'D',
                basic_type: 7,
                java_name: "double",
                slot_count: 2,
                is_stack_int: false,
                primitive_java_class: Some(DOUBLE_TYPE),
                boxed_java_class: Some(DOUBLE_BOXED),
            },
            JavaKind::Object => KindData {
                type_char: 'A',
                basic_type: 12,
                java_name: "Object",
                slot_count: 1,
                is_stack_int: false,
                primitive_java_class: None,
                boxed_java_class: None,
            },
            JavaKind::Void => KindData {
                type_char: 'V',
                basic_type: 14,
                java_name: "void",
                slot_count: 0,
                is_stack_int: false,
                primitive_java_class: Some(VOID_TYPE),
                boxed_java_class: Some(VOID_BOXED),
            },
            JavaKind::Illegal => KindData {
                type_char: '-',
                basic_type: 99,
                java_name: "illegal",
                slot_count: 0,
                is_stack_int: false,
                primitive_java_class: None,
                boxed_java_class: None,
            },
        }
    }

    pub fn get_slot_count(self) -> i32 {
        self.data().slot_count
    }

    pub fn needs_two_slots(self) -> bool {
        self.data().slot_count == 2
    }

    pub fn get_type_char(self) -> char {
        self.data().type_char
    }

    pub fn get_basic_type(self) -> i32 {
        self.data().basic_type
    }

    pub fn get_java_name(self) -> &'static str {
        self.data().java_name
    }

    pub fn is_primitive(self) -> bool {
        self.data().primitive_java_class.is_some()
    }

    pub fn get_stack_kind(self) -> JavaKind {
        if self.data().is_stack_int {
            JavaKind::Int
        } else {
            self
        }
    }

    pub fn is_numeric_integer(self) -> bool {
        self.data().is_stack_int || self == JavaKind::Long
    }

    pub fn is_unsigned(self) -> bool {
        self == JavaKind::Boolean || self == JavaKind::Char
    }

    pub fn is_numeric_float(self) -> bool {
        self == JavaKind::Float || self == JavaKind::Double
    }

    pub fn is_object(self) -> bool {
        self == JavaKind::Object
    }

    /// 对应 `JavaKind.fromTypeString(String)`。
    pub fn from_type_string(type_string: &str) -> JavaKind {
        debug_assert!(!type_string.is_empty());
        let first = type_string.chars().next().unwrap();
        if first == '[' || first == 'L' {
            return JavaKind::Object;
        }
        JavaKind::from_primitive_or_void_type_char(first)
    }

    /// 对应 `JavaKind.fromWordSize(int)`。
    pub fn from_word_size(word_size_in_bytes: i32) -> JavaKind {
        if word_size_in_bytes == 8 {
            JavaKind::Long
        } else {
            debug_assert!(word_size_in_bytes == 4, "Unsupported word size!");
            JavaKind::Int
        }
    }

    /// 对应 `JavaKind.fromPrimitiveOrVoidTypeChar(char)`。
    pub fn from_primitive_or_void_type_char(ch: char) -> JavaKind {
        match ch {
            'Z' => JavaKind::Boolean,
            'C' => JavaKind::Char,
            'F' => JavaKind::Float,
            'D' => JavaKind::Double,
            'B' => JavaKind::Byte,
            'S' => JavaKind::Short,
            'I' => JavaKind::Int,
            'J' => JavaKind::Long,
            'V' => JavaKind::Void,
            _ => panic!("unknown primitive or void type character: {}", ch),
        }
    }

    /// 对应 `JavaKind.fromJavaClass(Class<?>)`。
    pub fn from_java_class(klass: &JavaClass) -> JavaKind {
        if klass == &BOOLEAN_TYPE {
            JavaKind::Boolean
        } else if klass == &BYTE_TYPE {
            JavaKind::Byte
        } else if klass == &SHORT_TYPE {
            JavaKind::Short
        } else if klass == &CHAR_TYPE {
            JavaKind::Char
        } else if klass == &INT_TYPE {
            JavaKind::Int
        } else if klass == &LONG_TYPE {
            JavaKind::Long
        } else if klass == &FLOAT_TYPE {
            JavaKind::Float
        } else if klass == &DOUBLE_TYPE {
            JavaKind::Double
        } else if klass == &VOID_TYPE {
            JavaKind::Void
        } else {
            JavaKind::Object
        }
    }

    /// 对应 `JavaKind.toJavaClass()`。
    pub fn to_java_class(self) -> Option<JavaClass> {
        self.data().primitive_java_class.clone()
    }

    /// 对应 `JavaKind.toBoxedJavaClass()`。
    pub fn to_boxed_java_class(self) -> Option<JavaClass> {
        self.data().boxed_java_class.clone()
    }

    /// 对应 `JavaKind.toString()`。
    pub fn as_str(self) -> &'static str {
        self.data().java_name
    }

    /// 对应 `JavaKind.getMinValue()`。
    pub fn get_min_value(self) -> i64 {
        match self {
            JavaKind::Boolean => 0,
            JavaKind::Byte => i8::MIN as i64,
            JavaKind::Char => u16::MIN as i64,
            JavaKind::Short => i16::MIN as i64,
            JavaKind::Int => i32::MIN as i64,
            JavaKind::Long => i64::MIN,
            JavaKind::Float => f32::to_bits(f32::MIN) as i64,
            JavaKind::Double => f64::to_bits(f64::MIN) as i64,
            _ => panic!("illegal call to minValue on {:?}", self),
        }
    }

    /// 对应 `JavaKind.getMaxValue()`。
    pub fn get_max_value(self) -> i64 {
        match self {
            JavaKind::Boolean => 1,
            JavaKind::Byte => i8::MAX as i64,
            JavaKind::Char => u16::MAX as i64,
            JavaKind::Short => i16::MAX as i64,
            JavaKind::Int => i32::MAX as i64,
            JavaKind::Long => i64::MAX,
            JavaKind::Float => f32::to_bits(f32::MAX) as i64,
            JavaKind::Double => f64::to_bits(f64::MAX) as i64,
            _ => panic!("illegal call to maxValue on {:?}", self),
        }
    }

    /// 对应 `JavaKind.getByteCount()`。
    pub fn get_byte_count(self) -> i32 {
        if self == JavaKind::Boolean {
            1
        } else {
            self.get_bit_count() >> 3
        }
    }

    /// 对应 `JavaKind.getBitCount()`。
    pub fn get_bit_count(self) -> i32 {
        match self {
            JavaKind::Boolean => 1,
            JavaKind::Byte => 8,
            JavaKind::Char | JavaKind::Short => 16,
            JavaKind::Float | JavaKind::Int => 32,
            JavaKind::Double | JavaKind::Long => 64,
            _ => panic!("illegal call to getBitCount() on {:?}", self),
        }
    }

    /// 对应 `JavaKind.format(Object)`。
    pub fn format(self, value: &JavaObjectValue) -> String {
        if self.is_primitive() {
            debug_assert!(is_to_string_safe_value(value));
            return primitive_to_string(value);
        }
        match value {
            JavaObjectValue::Null => "null".to_string(),
            JavaObjectValue::String(s) => {
                if s.len() > 50 {
                    format!("String:\"{}...\"", &s[..30])
                } else {
                    format!("String:\"{}\"", s)
                }
            }
            JavaObjectValue::JavaType(t) => format!("JavaType:{}", t.to_java_name()),
            JavaObjectValue::Enum {
                simple_class_name,
                name,
            } => format!("{}:{}", simple_class_name, name),
            JavaObjectValue::FormatWithToString {
                simple_class_name,
                value,
            } => format!("{}:{}", simple_class_name, value),
            JavaObjectValue::Class(c) => format!("Class:{}", c.name()),
            other => {
                if is_to_string_safe_value(other) {
                    primitive_to_string(other)
                } else if let JavaObjectValue::Array {
                    component_simple_name,
                    items,
                } = other
                {
                    format_array(component_simple_name, items)
                } else {
                    let (simple, hash) = other_simple_name_and_hash(other);
                    format!("{}@{}", simple, hash)
                }
            }
        }
    }
}

impl std::fmt::Display for JavaKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.data().java_name)
    }
}

/// 镜像 `JavaKind.FormatWithToString`：标记可用 `toString` 安全格式化的类型。
pub trait FormatWithToString: std::any::Any {}

fn is_to_string_safe_value(value: &JavaObjectValue) -> bool {
    matches!(
        value,
        JavaObjectValue::Boolean(_)
            | JavaObjectValue::Byte(_)
            | JavaObjectValue::Char(_)
            | JavaObjectValue::Short(_)
            | JavaObjectValue::Int(_)
            | JavaObjectValue::Float(_)
            | JavaObjectValue::Long(_)
            | JavaObjectValue::Double(_)
    )
}

fn primitive_to_string(value: &JavaObjectValue) -> String {
    match value {
        JavaObjectValue::Boolean(b) => b.to_string(),
        JavaObjectValue::Byte(b) => b.to_string(),
        JavaObjectValue::Char(c) => c.to_string(),
        JavaObjectValue::Short(s) => s.to_string(),
        JavaObjectValue::Int(i) => i.to_string(),
        JavaObjectValue::Float(fv) => fv.to_string(),
        JavaObjectValue::Long(l) => l.to_string(),
        JavaObjectValue::Double(d) => d.to_string(),
        _ => panic!("not a primitive boxed value"),
    }
}

fn other_simple_name_and_hash(value: &JavaObjectValue) -> (String, u64) {
    match value {
        JavaObjectValue::Other {
            simple_class_name,
            identity_hash,
        } => (simple_class_name.clone(), *identity_hash),
        _ => ("Object".to_string(), 0),
    }
}

const MAX_FORMAT_ARRAY_LENGTH: usize = 5;

fn format_array(component_simple_name: &str, items: &[JavaObjectValue]) -> String {
    let array_length = items.len();
    let mut buf = String::new();
    buf.push_str(component_simple_name);
    buf.push('[');
    buf.push_str(&array_length.to_string());
    buf.push_str("]{");
    let length = MAX_FORMAT_ARRAY_LENGTH.min(array_length);
    for (i, item) in items.iter().take(length).enumerate() {
        if i != 0 {
            buf.push_str(", ");
        }
        buf.push_str(&JavaKind::Object.format(item));
    }
    if array_length != length {
        buf.push_str(", ...");
    }
    buf.push('}');
    buf
}

/// 镜像 Java `format(Object)` 路径上 `Object` 值域的判别联合。对应 `asBoxedPrimitive()`
/// 返回的 boxed primitive，以及 `format` 通过 `instanceof` 分派的全部对象种类。
///
/// 偏离记录：Java 侧该值域可由 `Object` 携带并在 `format` 中 `instanceof` 分派；Rust 侧
/// 用判别联合覆盖全部分支。`JavaType` 变体持 `Box<dyn JavaType>`，故不派生 `Clone`
/// （`dyn JavaType` 不可 `Clone`）；`JavaKind::format` 仅以引用读取，无需克隆。
#[derive(Debug)]
pub enum JavaObjectValue {
    Null,
    Boolean(bool),
    Byte(i8),
    Char(u16),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    String(String),
    JavaType(Box<dyn JavaType>),
    Enum {
        simple_class_name: String,
        name: String,
    },
    FormatWithToString {
        simple_class_name: String,
        value: String,
    },
    Class(JavaClass),
    Array {
        component_simple_name: String,
        items: Vec<JavaObjectValue>,
    },
    Other {
        simple_class_name: String,
        identity_hash: u64,
    },
}

impl JavaObjectValue {
    pub fn null() -> Self {
        JavaObjectValue::Null
    }
}
