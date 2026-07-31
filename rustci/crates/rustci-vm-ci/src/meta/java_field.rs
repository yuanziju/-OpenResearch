// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2009, 2016, Oracle and/or its affiliates. All rights reserved.
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

//! 镜像 `jdk.vm.ci.meta.JavaField`：Java 字段（已解析或未解析）的抽象。
//!
//! 偏离记录：
//! - `getType()`/`getDeclaringClass()` 返回引用 → Rust `&dyn JavaType`。
//! - `format(String)` 的 `'f'` 分支用 `this instanceof ResolvedJavaField` 判断已解析性；
//!   Rust 侧增设 `as_resolved_java_field()` 默认返回 `None`（`ResolvedJavaField` 实现覆写为 `Some(self)`）。
//! - `IllegalFormatException`/`UnknownFormatConversionException`（Java 非受检）→ Rust `panic!`。

use crate::meta::java_kind::JavaKind;
use crate::meta::java_type::JavaType;
use crate::meta::resolved_java_field::ResolvedJavaField;

/// 对应 `interface JavaField`。
pub trait JavaField {
    /// 对应 `getName()`。
    fn get_name(&self) -> &str;

    /// 对应 `getType()`。
    fn get_type(&self) -> &dyn JavaType;

    /// 对应 `getJavaKind()`（默认方法）。
    fn get_java_kind(&self) -> JavaKind {
        self.get_type().get_java_kind()
    }

    /// 对应 `getDeclaringClass()`。
    fn get_declaring_class(&self) -> &dyn JavaType;

    /// Rust 增设：对应 Java `format` 中 `this instanceof ResolvedJavaField` 的下转。
    fn as_resolved_java_field(&self) -> Option<&dyn ResolvedJavaField> {
        None
    }

    /// 对应 `format(String)`。
    fn format(&self, format: &str) -> String {
        let mut sb = String::new();
        let mut chars = format.chars().peekable();
        while let Some(ch) = chars.next() {
            if ch == '%' {
                let specifier = chars.next().unwrap_or_else(|| {
                    panic!(
                        "An unquoted '%' character cannot terminate a field format specification"
                    );
                });
                match specifier {
                    'T' | 't' => {
                        sb.push_str(&self.get_type().to_java_name_qualified(specifier == 'T'));
                    }
                    'H' | 'h' => {
                        sb.push_str(
                            &self
                                .get_declaring_class()
                                .to_java_name_qualified(specifier == 'H'),
                        );
                    }
                    'n' => {
                        sb.push_str(self.get_name());
                    }
                    'f' => match self.as_resolved_java_field() {
                        Some(rf) => {
                            if rf.is_static() {
                                sb.push_str("static");
                            } else {
                                sb.push_str("instance");
                            }
                        }
                        None => {
                            sb.push_str("unresolved");
                        }
                    },
                    '%' => {
                        sb.push('%');
                    }
                    other => {
                        panic!("Unknown format conversion: {}", other);
                    }
                }
            } else {
                sb.push(ch);
            }
        }
        sb
    }
}
