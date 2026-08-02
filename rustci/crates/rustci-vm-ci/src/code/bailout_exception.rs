// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2009, 2011, Oracle and/or its affiliates. All rights reserved.
 * DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
 *
 * This code is free software; you can redistribute it and/or modify it
 * under the terms of the GNU General Public License version 2 only, as
 * published by the Free Software Foundation.
 *
 * This code is distributed in the hope that it will be useful, but WITHOUT
 * ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
 * FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License
 * version 2 for more details (a copy of the GNU General Public License version
 * 2 along with this work; if not, write to the Free Software Foundation,
 * Inc., 51 Franklin St, Fifth Floor, Boston, MA 02110-1301 USA.
 *
 * Please contact Oracle, 500 Oracle Parkway, Redwood Shores, CA 94065 USA
 * or visit www.oracle.com if you need additional information or have any
 * questions.
 */

//! 镜像 `jdk.vm.ci.code.BailoutException`：编译器拒绝编译时抛出。
//!
//! 偏离记录：Java `BailoutException extends RuntimeException` → Rust `pub struct BailoutException`
//! 实现 `std::error::Error` + `Display`。`String.format(Locale.ENGLISH, ...)` → Rust `format!`。
//! `serialVersionUID` 字段无 Rust 等价物，省略。

use std::error::Error;
use std::fmt;

/// 对应 `public class BailoutException extends RuntimeException`。
#[derive(Debug)]
pub struct BailoutException {
    message: String,
    cause: Option<Box<dyn Error + Send + Sync>>,
    permanent: bool,
}

impl BailoutException {
    /// 对应 `BailoutException(String format, Object... args)`：`permanent = true`。
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            cause: None,
            permanent: true,
        }
    }

    /// 对应 `BailoutException(Throwable cause, String format, Object... args)`：`permanent = true`。
    pub fn with_cause(cause: Box<dyn Error + Send + Sync>, message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            cause: Some(cause),
            permanent: true,
        }
    }

    /// 对应 `BailoutException(boolean permanent, String format, Object... args)`。
    pub fn with_permanent(permanent: bool, message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            cause: None,
            permanent,
        }
    }

    /// 对应 `isPermanent()`。
    pub fn is_permanent(&self) -> bool {
        self.permanent
    }
}

impl fmt::Display for BailoutException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl Error for BailoutException {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.cause.as_deref().map(|e| e as &(dyn Error + 'static))
    }
}
