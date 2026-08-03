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

//! 镜像 `jdk.vm.ci.hotspot.HotSpotCompilationRequestResult`：HotSpot 编译请求结果。
//!
//! 偏离记录：
//! - Java `final class HotSpotCompilationRequestResult implements CompilationRequestResult` →
//!   Rust `pub struct HotSpotCompilationRequestResult`。
//! - 方法名 camelCase → snake_case。

use std::any::Any;

use crate::code::compilation_request_result::CompilationRequestResult;

/// 对应 `final class HotSpotCompilationRequestResult implements CompilationRequestResult`。
pub struct HotSpotCompilationRequestResult {
    /// 对应 `private final String failureMessage`。
    failure_message: Option<String>,
    /// 对应 `private final boolean retry`。
    retry: bool,
    /// 对应 `private final int inlinedBytecodes`。
    inlined_bytecodes: i32,
}

impl CompilationRequestResult for HotSpotCompilationRequestResult {
    /// 对应 `getFailure()`。
    fn get_failure(&self) -> Option<&dyn Any> {
        self.failure_message.as_ref().map(|s| s as &dyn Any)
    }
}

impl HotSpotCompilationRequestResult {
    /// 对应 `static success(int)`。
    pub fn success(inlined_bytecodes: i32) -> Self {
        Self {
            failure_message: None,
            retry: true,
            inlined_bytecodes,
        }
    }

    /// 对应 `static failure(String, boolean)`。
    pub fn failure(failure_message: String, retry: bool) -> Self {
        Self {
            failure_message: Some(failure_message),
            retry,
            inlined_bytecodes: 0,
        }
    }

    /// 对应 `getFailureMessage()`。
    pub fn get_failure_message(&self) -> Option<&str> {
        self.failure_message.as_deref()
    }

    /// 对应 `getRetry()`。
    pub fn get_retry(&self) -> bool {
        self.retry
    }

    /// 对应 `getInlinedBytecodes()`。
    pub fn get_inlined_bytecodes(&self) -> i32 {
        self.inlined_bytecodes
    }
}
