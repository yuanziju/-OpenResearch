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
 * version 2 for more details (a copy of the GNU General Public License version
 * 2 along with this work; if not, write to the Free Software Foundation,
 * Inc., 51 Franklin St, Fifth Floor, Boston, MA 02110-1301 USA.
 *
 * Please contact Oracle, 500 Oracle Parkway, Redwood Shores, CA 94065 USA
 * or visit www.oracle.com if you need additional information or have any
 * questions.
 */

//! 镜像 `jdk.vm.ci.code.BytecodePosition`：内联方法链与字节码位置的表示。
//!
//! 偏离记录：
//! - Java `class BytecodePosition`（持 `caller`/`method`/`bci`）→ Rust `pub struct BytecodePosition`。
//!   `caller` 类型为 `Option<Box<BytecodePosition>>`（Java nullable `BytecodePosition caller`）。
//!   `method` 为 `Box<dyn ResolvedJavaMethod>`（Java 持引用，Rust 持所有权）。
//! - 构造器断言 `method != null` → Rust `debug_assert!`；`codeSize != 0 && bci >= codeSize` 抛
//!   `IllegalArgumentException` → Rust `panic!`。
//! - `equals` 深比较 `bci`/`method`/`caller`；`method` 为 trait 对象，用 `as_any` 数据指针
//!   比对（对齐 Java `Objects.equals` 对无覆写 `equals` 类型的引用相等语义）。
//! - `addCaller(link)` 递归插入：Java 不消费 `this`，Rust 侧取 `self: Box<Self>` 所有权以
//!   复用 `method` 字段（`ResolvedJavaMethod` 无 `Clone`，无法复制 trait 对象）。
//! - `toString` 委托 `CodeUtil.append`。
//! - `FrameData`：`BytecodeFrame extends BytecodePosition` 在 Rust 无继承等价物，帧特有字段
//!   以 `Option<FrameData>` 嵌入 `BytecodePosition`，`BytecodeFrame` 为 newtype 包装（见
//!   `bytecode_frame.rs`）。`DebugInfo` 经 `has_frame`/`frame_data` 判别。

use std::any::Any;
use std::fmt;

use crate::code::code_util;
use crate::meta::java_kind::JavaKind;
use crate::meta::java_value::JavaValue;
use crate::meta::resolved_java_method::ResolvedJavaMethod;

/// `BytecodeFrame` 特有字段（对应 Java `BytecodeFrame extends BytecodePosition` 新增字段）。
pub struct FrameData {
    /// 对应 `public final JavaValue[] values`。
    pub values: Vec<Box<dyn JavaValue>>,
    /// 对应 `private final JavaKind[] slotKinds`。
    pub slot_kinds: Vec<JavaKind>,
    /// 对应 `public final int numLocals`。
    pub num_locals: i32,
    /// 对应 `public final int numStack`。
    pub num_stack: i32,
    /// 对应 `public final int numLocks`。
    pub num_locks: i32,
    /// 对应 `public final boolean rethrowException`。
    pub rethrow_exception: bool,
    /// 对应 `public final boolean duringCall`。
    pub during_call: bool,
}

/// 对应 `class BytecodePosition`。
pub struct BytecodePosition {
    caller: Option<Box<BytecodePosition>>,
    method: Box<dyn ResolvedJavaMethod>,
    bci: i32,
    frame: Option<FrameData>,
}

impl BytecodePosition {
    /// 对应 `BytecodePosition(BytecodePosition caller, ResolvedJavaMethod method, int bci)`。
    pub fn new(
        caller: Option<Box<BytecodePosition>>,
        method: Box<dyn ResolvedJavaMethod>,
        bci: i32,
    ) -> Self {
        let code_size = method.get_code_size();
        if code_size != 0 && bci >= code_size {
            panic!(
                "IllegalArgumentException: bci {} is out of range for {} {} bytes",
                bci,
                method.format("%H.%n(%p)"),
                code_size
            );
        }
        Self {
            caller,
            method,
            bci,
            frame: None,
        }
    }

    /// Rust 增设：以帧数据构造（供 `BytecodeFrame::new` 使用）。
    pub(crate) fn new_with_frame(
        caller: Option<Box<BytecodePosition>>,
        method: Box<dyn ResolvedJavaMethod>,
        bci: i32,
        frame: FrameData,
    ) -> Self {
        let code_size = method.get_code_size();
        if code_size != 0 && bci >= code_size {
            panic!(
                "IllegalArgumentException: bci {} is out of range for {} {} bytes",
                bci,
                method.format("%H.%n(%p)"),
                code_size
            );
        }
        Self {
            caller,
            method,
            bci,
            frame: Some(frame),
        }
    }

    /// 对应 `getBCI()`。
    pub fn get_bci(&self) -> i32 {
        self.bci
    }

    /// 对应 `getMethod()`。
    pub fn get_method(&self) -> &dyn ResolvedJavaMethod {
        self.method.as_ref()
    }

    /// 对应 `getCaller()`。
    pub fn get_caller(&self) -> Option<&BytecodePosition> {
        self.caller.as_deref()
    }

    /// 对应 `addCaller(BytecodePosition link)`：递归插入。取 `self` 所有权以复用 `method`。
    #[allow(clippy::boxed_local)]
    pub fn add_caller(self: Box<Self>, link: Box<BytecodePosition>) -> Box<BytecodePosition> {
        match self.caller {
            None => Box::new(BytecodePosition::new(Some(link), self.method, self.bci)),
            Some(c) => Box::new(BytecodePosition::new(
                Some(c.add_caller(link)),
                self.method,
                self.bci,
            )),
        }
    }

    /// Rust 增设：是否含帧数据（对应 Java `instanceof BytecodeFrame` 判别）。
    pub fn has_frame(&self) -> bool {
        self.frame.is_some()
    }

    /// Rust 增设：取帧数据引用（供 `DebugInfo.frame` 等使用）。
    pub fn frame_data(&self) -> Option<&FrameData> {
        self.frame.as_ref()
    }

    /// Rust 增设：值相等（对齐 Java `equals`）。`method`/`caller` 为 trait 对象，
    /// 按 `as_any` 数据指针比对（对齐 Java `Objects.equals` 引用相等语义）。
    pub fn equals(&self, other: &BytecodePosition) -> bool {
        if std::ptr::eq(self as *const _, other as *const _) {
            return true;
        }
        if self.bci != other.bci {
            return false;
        }
        if !std::ptr::eq(
            self.method.as_any() as *const dyn Any,
            other.method.as_any() as *const dyn Any,
        ) {
            return false;
        }
        match (&self.caller, &other.caller) {
            (None, None) => true,
            (Some(a), Some(b)) => a.equals(b),
            _ => false,
        }
    }
}

impl fmt::Debug for BytecodePosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BytecodePosition")
            .field("bci", &self.bci)
            .finish_non_exhaustive()
    }
}

impl fmt::Display for BytecodePosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buf = String::new();
        code_util::append_position(&mut buf, self);
        f.write_str(&buf)
    }
}
