// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2011, 2025, Oracle and/or its affiliates. All rights reserved.
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

//! 镜像 `jdk.vm.ci.code.InstalledCode`：已安装机器码的句柄。
//!
//! 偏离记录：
//! - Java `class InstalledCode`（`protected` 可变字段 `address`/`entryPoint`/`version` +
//!   `protected final String name`）→ Rust `pub struct InstalledCode`。`protected` 字段 →
//!   `pub(crate)` 字段（子类/crate 内可写，对齐 Java `protected` 同包+子类可访问语义）+ `pub` getter。
//! - `name` nullable（Java 构造器 `Can be null`）→ `Option<String>`。
//! - `invalidate(boolean)`/`executeVarargs` 默认抛 `UnsupportedOperationException` →
//!   `invalidate_with` `panic!`（对齐 Java）；`execute_varargs` 返回
//!   `Result<Box<dyn Any>, InvalidInstalledCodeException>`，默认返回 `Err`（Java 抛
//!   `UnsupportedOperationException`，Rust 侧 `Result` 的 `Err` 类型固定为
//!   `InvalidInstalledCodeException`，且未安装代码 `isValid() == false`，执行即视为调用失效代码，
//!   故返回 `Err`）。
//! - `getCode` 默认 `null` → `Option<Vec<u8>>`，默认 `None`。
//! - Rust 无继承，`invalidate()`/`invalidate(boolean)`/`executeVarargs()`/`getCode()` 的覆写
//!   由子类以独立类型 + 组合实现，基类方法为本期默认实现。

use std::any::Any;
use std::fmt;

use crate::code::invalid_installed_code_exception::InvalidInstalledCodeException;

/// 对应 `public static final int MAX_NAME_LENGTH = 2048`。
pub const MAX_NAME_LENGTH: usize = 2048;

/// 对应 `class InstalledCode`。
pub struct InstalledCode {
    /// 对应 `protected long address`。
    pub(crate) address: i64,
    /// 对应 `protected long entryPoint`。
    pub(crate) entry_point: i64,
    /// 对应 `protected long version`。
    pub(crate) version: i64,
    name: Option<String>,
}

impl InstalledCode {
    /// 对应 `InstalledCode(String name)`：`name` 长度超 `MAX_NAME_LENGTH` 抛 `IllegalArgumentException`。
    pub fn new(name: Option<String>) -> Self {
        if let Some(ref n) = name {
            if n.len() > MAX_NAME_LENGTH {
                panic!(
                    "IllegalArgumentException: name length ({}) is greater than {} (name[0:{}] = {})",
                    n.len(),
                    MAX_NAME_LENGTH,
                    MAX_NAME_LENGTH,
                    &n[..MAX_NAME_LENGTH]
                );
            }
        }
        Self {
            address: 0,
            entry_point: 0,
            version: 0,
            name,
        }
    }

    /// 对应 `getAddress()`。
    pub fn get_address(&self) -> i64 {
        self.address
    }

    /// 对应 `getEntryPoint()`。
    pub fn get_entry_point(&self) -> i64 {
        self.entry_point
    }

    /// 对应 `final long getVersion()`。
    pub fn get_version(&self) -> i64 {
        self.version
    }

    /// 对应 `getName()`。
    pub fn get_name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// 对应 `getStart()`（默认 `0`）。
    pub fn get_start(&self) -> i64 {
        0
    }

    /// 对应 `isValid()`（`entryPoint != 0`）。
    pub fn is_valid(&self) -> bool {
        self.entry_point != 0
    }

    /// 对应 `isAlive()`（`address != 0`）。
    pub fn is_alive(&self) -> bool {
        self.address != 0
    }

    /// 对应 `byte[] getCode()`（默认 `null` → `None`）。
    pub fn get_code(&self) -> Option<Vec<u8>> {
        None
    }

    /// 对应 `invalidate()`：委托 `invalidate(true)`。
    pub fn invalidate(&mut self) {
        self.invalidate_with(true);
    }

    /// 对应 `invalidate(boolean deoptimize)`（默认抛 `UnsupportedOperationException`）。
    pub fn invalidate_with(&mut self, _deoptimize: bool) {
        panic!("UnsupportedOperationException: invalidate");
    }

    /// 对应 `Object executeVarargs(Object... args) throws InvalidInstalledCodeException`
    /// （默认抛 `UnsupportedOperationException`；Rust 侧返回 `Err`，见模块偏离记录）。
    pub fn execute_varargs(
        &self,
        _args: &[Box<dyn Any>],
    ) -> Result<Box<dyn Any>, InvalidInstalledCodeException> {
        Err(InvalidInstalledCodeException::new())
    }
}

impl fmt::Debug for InstalledCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("InstalledCode")
            .field("address", &self.address)
            .field("entry_point", &self.entry_point)
            .field("version", &self.version)
            .field("name", &self.name)
            .finish()
    }
}
