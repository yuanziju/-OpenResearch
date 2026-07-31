// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2012, 2023, Oracle and/or its affiliates. All rights reserved.
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

//! 镜像 `jdk.vm.ci.meta.MetaUtil`：`jdk.vm.ci.meta` 及其客户端使用的工具集。
//!
//! 偏离记录：
//! - `getSimpleName(Class<?>, boolean)` 在 Java 侧通过 `Class` 反射取 enclosing 类链；
//!   Rust 侧 `JavaClass` 仅携带类名，无法遍历 enclosing 类，`with_enclosing_class`
//!   行为降级为返回 simple name（无 enclosing 前缀），匿名/局部类按 Java 逻辑解析名。
//! - `appendProfile(StringBuilder, AbstractJavaProfile<?,?>, ...)` 的 nullable profile
//!   → `Option<&AbstractJavaProfile<T,U>>`；`StringBuilder` → `&mut String`。

use std::fmt::{Display, Write};

use crate::meta::abstract_java_profile::{AbstractJavaProfile, AbstractProfiledItem};
use crate::meta::java_kind::JavaKind;
use crate::meta::java_reflect::JavaClass;
use crate::meta::resolved_java_method::ResolvedJavaMethod;

pub const PACKAGE_SEPARATOR_INTERNAL: char = '/';
pub const HIDDEN_SEPARATOR_INTERNAL: char = '.';
pub const PACKAGE_SEPARATOR_JAVA: char = HIDDEN_SEPARATOR_INTERNAL;
pub const HIDDEN_SEPARATOR_JAVA: char = PACKAGE_SEPARATOR_INTERNAL;

/// 镜像 `java.lang.StackTraceElement`（JVMCI meta 路径用到的字段子集）。
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct StackTraceElement {
    declaring_class: String,
    method_name: String,
    file_name: Option<String>,
    line_number: i32,
}

impl StackTraceElement {
    pub fn new(
        declaring_class: impl Into<String>,
        method_name: impl Into<String>,
        file_name: Option<String>,
        line_number: i32,
    ) -> Self {
        Self {
            declaring_class: declaring_class.into(),
            method_name: method_name.into(),
            file_name,
            line_number,
        }
    }

    pub fn get_file_name(&self) -> Option<&str> {
        self.file_name.as_deref()
    }

    pub fn get_line_number(&self) -> i32 {
        self.line_number
    }

    pub fn get_class_name(&self) -> &str {
        &self.declaring_class
    }

    pub fn get_method_name(&self) -> &str {
        &self.method_name
    }
}

impl Display for StackTraceElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // 对齐 java.lang.StackTraceElement.toString 行为。
        write!(f, "{}.{}", self.declaring_class, self.method_name)?;
        match (&self.file_name, self.line_number) {
            (Some(file), ln) if ln >= 0 => write!(f, "({}:{})", file, ln),
            (Some(file), _) => write!(f, "({})", file),
            (None, ln) if ln >= 0 => write!(f, "(Unknown Source:{})", ln),
            (None, _) => write!(f, "(Unknown Source)"),
        }
    }
}

/// 对应 `MetaUtil.getSimpleName(Class<?>, boolean)`。
pub fn get_simple_name(klass: &JavaClass, _with_enclosing_class: bool) -> String {
    let simple = safe_simple_name(klass);
    if !simple.is_empty() {
        return simple;
    }
    // anonymous / local class
    let name = klass.name();
    match name.find('$') {
        None => name.to_string(),
        Some(idx) => {
            let before = &name[..idx];
            match before.rfind('.') {
                None => name.to_string(),
                Some(d) => name[d + 1..].to_string(),
            }
        }
    }
}

fn safe_simple_name(klass: &JavaClass) -> String {
    let name = klass.name();
    match name.rfind('.') {
        None => name.to_string(),
        Some(d) => name[d + 1..].to_string(),
    }
}

/// 对应 `MetaUtil.internalNameToJava(String, boolean, boolean)`。
pub fn internal_name_to_java(
    name: &str,
    qualified: bool,
    class_for_name_compatible: bool,
) -> String {
    let first = name.chars().next().unwrap();
    match first {
        'L' => {
            let type_ = &name[1..name.len() - 1];
            let mut result = replace_package_and_hidden_separators(
                type_,
                PACKAGE_SEPARATOR_INTERNAL,
                HIDDEN_SEPARATOR_INTERNAL,
            );
            if !qualified {
                if let Some(last_dot) = result.rfind(HIDDEN_SEPARATOR_INTERNAL) {
                    result = result[last_dot + 1..].to_string();
                }
            }
            result
        }
        '[' => {
            if class_for_name_compatible {
                replace_package_and_hidden_separators(
                    name,
                    PACKAGE_SEPARATOR_INTERNAL,
                    HIDDEN_SEPARATOR_INTERNAL,
                )
            } else {
                format!("{}[]", internal_name_to_java(&name[1..], qualified, false))
            }
        }
        _ => {
            if name.chars().count() != 1 {
                panic!("Illegal internal name: {}", name);
            }
            JavaKind::from_primitive_or_void_type_char(first)
                .get_java_name()
                .to_string()
        }
    }
}

/// 对应 `MetaUtil.replacePackageAndHiddenSeparators`。
fn replace_package_and_hidden_separators(
    name: &str,
    package_separator: char,
    hidden_separator: char,
) -> String {
    let index = name.find(hidden_separator);
    let mut buf = String::with_capacity(name.len());
    match index {
        None => {
            buf.push_str(&name.replace(package_separator, &hidden_separator.to_string()));
        }
        Some(idx) => {
            buf.push_str(&name[..idx].replace(package_separator, &hidden_separator.to_string()));
            buf.push(package_separator);
            buf.push_str(&name[idx + hidden_separator.len_utf8()..]);
        }
    }
    buf
}

/// 对应 `MetaUtil.toLocation(ResolvedJavaMethod, int)`。
pub fn to_location(method: Option<&dyn ResolvedJavaMethod>, bci: i32) -> String {
    let mut buf = String::new();
    append_location(&mut buf, method, bci);
    buf
}

/// 对应 `MetaUtil.appendLocation(StringBuilder, ResolvedJavaMethod, int)`。
pub fn append_location(buf: &mut String, method: Option<&dyn ResolvedJavaMethod>, bci: i32) {
    match method {
        Some(m) => {
            let ste = m.as_stack_trace_element(bci);
            if ste.get_file_name().is_some() && ste.get_line_number() > 0 {
                let _ = write!(buf, "{}", ste);
            } else {
                let _ = write!(buf, "{}", m.format("%H.%n(%p)"));
            }
        }
        None => {
            buf.push_str("Null method");
        }
    }
    let _ = write!(buf, " [bci: {}]", bci);
}

/// 对应 `MetaUtil.appendProfile(StringBuilder, AbstractJavaProfile<?,?>, int, String, String)`。
pub fn append_profile<T: AbstractProfiledItem<U>, U: Display + ?Sized>(
    buf: &mut String,
    profile: Option<&AbstractJavaProfile<T, U>>,
    bci: i32,
    type_: &str,
    sep: &str,
) {
    if let Some(profile) = profile {
        let pitems = profile.get_items();
        if !pitems.is_empty() {
            let _ = write!(buf, "{}@{}:", type_, bci);
            for pitem in pitems {
                let _ = write!(buf, " {:.6} ({}){}", pitem.probability(), pitem.item(), sep);
            }
            if profile.get_not_recorded_probability() != 0.0 {
                let _ = write!(
                    buf,
                    " {:.6} <other {}>{}",
                    profile.get_not_recorded_probability(),
                    type_,
                    sep
                );
            } else {
                let _ = write!(buf, " <no other {}>{}", type_, sep);
            }
        }
    }
}

/// 对应 `MetaUtil.toInternalName(String)`。
pub fn to_internal_name(class_name: &str) -> String {
    if class_name.starts_with('[') {
        return replace_package_and_hidden_separators(
            class_name,
            PACKAGE_SEPARATOR_JAVA,
            HIDDEN_SEPARATOR_JAVA,
        );
    }
    let mut result = String::new();
    let mut base = class_name;
    while base.ends_with("[]") {
        result.push('[');
        base = &base[..base.len() - 2];
    }
    match base {
        "boolean" => result.push('Z'),
        "byte" => result.push('B'),
        "short" => result.push('S'),
        "char" => result.push('C'),
        "int" => result.push('I'),
        "float" => result.push('F'),
        "long" => result.push('J'),
        "double" => result.push('D'),
        "void" => result.push('V'),
        _ => {
            result.push('L');
            result.push_str(&replace_package_and_hidden_separators(
                base,
                PACKAGE_SEPARATOR_JAVA,
                HIDDEN_SEPARATOR_JAVA,
            ));
            result.push(';');
        }
    }
    result
}

/// 对应 `MetaUtil.identityHashCodeString(Object)`。
pub fn identity_hash_code_string(obj_name: Option<&str>) -> String {
    match obj_name {
        None => "null".to_string(),
        Some(name) => format!("{}@{}", name, name.as_ptr() as usize),
    }
}
