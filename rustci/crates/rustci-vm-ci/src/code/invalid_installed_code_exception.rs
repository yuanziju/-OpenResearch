// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2013, 2019, Oracle and/or its affiliates. All rights reserved.
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

//! 镜像 `jdk.vm.ci.code.InvalidInstalledCodeException`：调用已失效机器码时抛出。
//!
//! 偏离记录：Java `final class InvalidInstalledCodeException extends Exception`
//! → Rust `pub struct InvalidInstalledCodeException` 实现 `std::error::Error` + `Display`。
//! `serialVersionUID` 字段无 Rust 等价物，省略。

use std::error::Error;
use std::fmt;

/// 对应 `final class InvalidInstalledCodeException extends Exception`。
#[derive(Debug)]
pub struct InvalidInstalledCodeException {
    message: Option<String>,
}

impl InvalidInstalledCodeException {
    /// 对应 `InvalidInstalledCodeException()`。
    pub fn new() -> Self {
        Self { message: None }
    }

    /// 对应 `InvalidInstalledCodeException(String message)`。
    pub fn with_message(message: impl Into<String>) -> Self {
        Self {
            message: Some(message.into()),
        }
    }
}

impl Default for InvalidInstalledCodeException {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for InvalidInstalledCodeException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.message {
            Some(m) => f.write_str(m),
            None => Ok(()),
        }
    }
}

impl Error for InvalidInstalledCodeException {}
