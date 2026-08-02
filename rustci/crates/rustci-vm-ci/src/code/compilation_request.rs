// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2015, Oracle and/or its affiliates. All rights reserved.
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

//! 镜像 `jdk.vm.ci.code.CompilationRequest`：编译方法请求。
//!
//! 偏离记录：Java `class CompilationRequest`（持 `method`/`entryBCI`）→ Rust
//! `pub struct CompilationRequest`。`method` 为 `Box<dyn ResolvedJavaMethod>`（Java 持引用，
//! Rust 持所有权）。单参构造器委托 `entryBCI = -1`。

use std::fmt;

use crate::meta::resolved_java_method::ResolvedJavaMethod;

/// 对应 `class CompilationRequest`。
pub struct CompilationRequest {
    method: Box<dyn ResolvedJavaMethod>,
    entry_bci: i32,
}

impl CompilationRequest {
    /// 对应 `CompilationRequest(ResolvedJavaMethod method)`：`entryBCI = -1`。
    pub fn new(method: Box<dyn ResolvedJavaMethod>) -> Self {
        Self::with_entry_bci(method, -1)
    }

    /// 对应 `CompilationRequest(ResolvedJavaMethod method, int entryBCI)`。
    pub fn with_entry_bci(method: Box<dyn ResolvedJavaMethod>, entry_bci: i32) -> Self {
        Self { method, entry_bci }
    }

    /// 对应 `getMethod()`。
    pub fn get_method(&self) -> &dyn ResolvedJavaMethod {
        self.method.as_ref()
    }

    /// 对应 `getEntryBCI()`。
    pub fn get_entry_bci(&self) -> i32 {
        self.entry_bci
    }
}

impl fmt::Display for CompilationRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 对应 `toString() { return method.format("%H.%n(%p)@" + entryBCI); }`
        write!(f, "{}@{}", self.method.format("%H.%n(%p)"), self.entry_bci)
    }
}
