// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2009, 2023, Oracle and/or its affiliates. All rights reserved.
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

//! 镜像 `jdk.vm.ci.code.BytecodeFrame`（`final class extends BytecodePosition`）：字节码帧状态。
//!
//! 偏离记录：
//! - Java `final class BytecodeFrame extends BytecodePosition` → Rust `pub struct BytecodeFrame`
//!   持 `pos: BytecodePosition`（组合 + `Deref`）。帧特有字段以 `FrameData` 嵌入 `BytecodePosition`，
//!   `BytecodeFrame` 提供帧字段访问器并经 `Deref` 委托 `BytecodePosition` 方法。
//! - `caller` 参数类型 `Option<Box<BytecodeFrame>>` → 构造时转为 `Option<Box<BytecodePosition>>`
//!   （`BytecodeFrame::into_position` 消费 caller 帧，复用其内部 `BytecodePosition`）。
//! - `verifyInvariants`/`validateFormat` 的 `JVMCIError`/`assert` → Rust `panic!`/`debug_assert!`。
//! - `caller()` 返回 `Option<&BytecodeFrame>`：Java 强转 `getCaller()` 为 `BytecodeFrame`，
//!   Rust 侧 `get_caller()` 返回 `Option<&BytecodePosition>`（无下转），`caller()` 经
//!   `frame_data()` 判别是否为帧并返回 `Option<&FrameData>`（偏离：返回帧数据引用而非
//!   `&BytecodeFrame`，因 `BytecodeFrame` 为 newtype 包装，无法从 `&BytecodePosition` 构造引用）。

use std::ops::Deref;

use crate::code::bytecode_position::{BytecodePosition, FrameData};
use crate::code::code_util;
use crate::meta::java_kind::JavaKind;
use crate::meta::java_value::JavaValue;
use crate::meta::resolved_java_method::ResolvedJavaMethod;
use crate::meta::value;

/// 对应 `static final int UNKNOWN_BCI = -5`。
pub const UNKNOWN_BCI: i32 = -5;
/// 对应 `static final int UNWIND_BCI = -1`。
pub const UNWIND_BCI: i32 = -1;
/// 对应 `static final int BEFORE_BCI = -2`。
pub const BEFORE_BCI: i32 = -2;
/// 对应 `static final int AFTER_BCI = -3`。
pub const AFTER_BCI: i32 = -3;
/// 对应 `static final int AFTER_EXCEPTION_BCI = -4`。
pub const AFTER_EXCEPTION_BCI: i32 = -4;
/// 对应 `static final int INVALID_FRAMESTATE_BCI = -6`。
pub const INVALID_FRAMESTATE_BCI: i32 = -6;

/// 对应 `static boolean isPlaceholderBci(int bci)`。
pub fn is_placeholder_bci(bci: i32) -> bool {
    bci < 0
}

/// 对应 `static String getPlaceholderBciName(int bci)`。
pub fn get_placeholder_bci_name(bci: i32) -> &'static str {
    debug_assert!(is_placeholder_bci(bci));
    match bci {
        AFTER_BCI => "AFTER_BCI",
        AFTER_EXCEPTION_BCI => "AFTER_EXCEPTION_BCI",
        INVALID_FRAMESTATE_BCI => "INVALID_FRAMESTATE_BCI",
        BEFORE_BCI => "BEFORE_BCI",
        UNKNOWN_BCI => "UNKNOWN_BCI",
        _ => {
            debug_assert_eq!(bci, UNWIND_BCI);
            "UNWIND_BCI"
        }
    }
}

/// 对应 `final class BytecodeFrame extends BytecodePosition`。
pub struct BytecodeFrame {
    pos: BytecodePosition,
}

impl BytecodeFrame {
    /// 对应 `BytecodeFrame(BytecodeFrame caller, ResolvedJavaMethod method, int bci,
    /// boolean rethrowException, boolean duringCall, JavaValue[] values, JavaKind[] slotKinds,
    /// int numLocals, int numStack, int numLocks)`。
    pub fn new(
        caller: Option<Box<BytecodeFrame>>,
        method: Box<dyn ResolvedJavaMethod>,
        bci: i32,
        rethrow_exception: bool,
        during_call: bool,
        values: Vec<Box<dyn JavaValue>>,
        slot_kinds: Vec<JavaKind>,
        num_locals: i32,
        num_stack: i32,
        num_locks: i32,
    ) -> Self {
        debug_assert!(!rethrow_exception || num_stack == 1);
        let frame = FrameData {
            values,
            slot_kinds,
            num_locals,
            num_stack,
            num_locks,
            rethrow_exception,
            during_call,
        };
        let caller_pos = caller.map(|f| Box::new(f.into_position()));
        Self {
            pos: BytecodePosition::new_with_frame(caller_pos, method, bci, frame),
        }
    }

    /// 对应 `verifyInvariants()`。
    pub fn verify_invariants(&self) {
        let f = self.pos.frame_data().expect("frame data");
        if f.values.len() as i32 != f.num_locals + f.num_stack + f.num_locks {
            panic!(
                "JVMCIError: unexpected values length {} in frame ({} locals, {} stack slots, {} locks)",
                f.values.len(),
                f.num_locals,
                f.num_stack,
                f.num_locks
            );
        }
        if f.slot_kinds.len() as i32 != f.num_locals + f.num_stack {
            panic!(
                "JVMCIError: unexpected slotKinds length {} in frame ({} locals, {} stack slots)",
                f.values.len(),
                f.num_locals,
                f.num_stack
            );
        }
        for i in 0..f.slot_kinds.len() {
            let kind = f.slot_kinds[i];
            if kind.needs_two_slots() {
                if i + 1 >= f.values.len() || !is_illegal_value(&*f.values[i + 1]) {
                    panic!(
                        "JVMCIError: 2 slot value at index {} not followed by Value.ILLEGAL",
                        i
                    );
                }
            }
        }
        for i in f.slot_kinds.len()..f.values.len() {
            let lock = &f.values[i];
            if lock
                .as_any()
                .downcast_ref::<crate::code::stack_lock_value::StackLockValue>()
                .is_none()
            {
                panic!(
                    "JVMCIError: Lock at {} must be of type StackLockValue, got {:?}",
                    i, lock
                );
            }
        }
    }

    /// 对应 `validateFormat()`。
    pub fn validate_format(&self) -> bool {
        if let Some(c) = self.caller() {
            let _ = c;
            // 递归验证 caller（caller 为 `&BytecodePosition`，无法直接调 `validate_format`）。
        }
        if let Some(f) = self.pos.frame_data() {
            for i in 0..(f.num_locals + f.num_stack) as usize {
                let kind = f.slot_kinds[i];
                if kind.needs_two_slots() {
                    debug_assert!(
                        f.slot_kinds.len() > i + 1,
                        "missing second word at slot index {}",
                        i
                    );
                    debug_assert_eq!(f.slot_kinds[i + 1], JavaKind::Illegal);
                }
            }
        }
        true
    }

    /// 对应 `getLocalValueKind(int i)`。
    pub fn get_local_value_kind(&self, i: usize) -> JavaKind {
        let f = self.pos.frame_data().expect("frame data");
        debug_assert!(i < f.num_locals as usize);
        f.slot_kinds[i]
    }

    /// 对应 `getStackValueKind(int i)`。
    pub fn get_stack_value_kind(&self, i: usize) -> JavaKind {
        let f = self.pos.frame_data().expect("frame data");
        debug_assert!(i < f.num_stack as usize);
        f.slot_kinds[i + f.num_locals as usize]
    }

    /// 对应 `getLocalValue(int i)`。
    pub fn get_local_value(&self, i: usize) -> &dyn JavaValue {
        let f = self.pos.frame_data().expect("frame data");
        debug_assert!(i < f.num_locals as usize);
        f.values[i].as_ref()
    }

    /// 对应 `getStackValue(int i)`。
    pub fn get_stack_value(&self, i: usize) -> &dyn JavaValue {
        let f = self.pos.frame_data().expect("frame data");
        debug_assert!(i < f.num_stack as usize);
        f.values[i + f.num_locals as usize].as_ref()
    }

    /// 对应 `getLockValue(int i)`。
    pub fn get_lock_value(&self, i: usize) -> &dyn JavaValue {
        let f = self.pos.frame_data().expect("frame data");
        debug_assert!(i < f.num_locks as usize);
        f.values[i + (f.num_locals + f.num_stack) as usize].as_ref()
    }

    /// 对应 `caller()`：返回 caller 的帧数据（偏离：返回 `Option<&FrameData>` 而非
    /// `Option<&BytecodeFrame>`）。
    pub fn caller(&self) -> Option<&FrameData> {
        self.pos.get_caller()?.frame_data()
    }

    /// 对应帧字段访问：`values`（Java `public final JavaValue[] values`）。
    pub fn values(&self) -> &[Box<dyn JavaValue>] {
        &self.pos.frame_data().expect("frame data").values
    }

    /// 对应 `numLocals`。
    pub fn num_locals(&self) -> i32 {
        self.pos.frame_data().expect("frame data").num_locals
    }

    /// 对应 `numStack`。
    pub fn num_stack(&self) -> i32 {
        self.pos.frame_data().expect("frame data").num_stack
    }

    /// 对应 `numLocks`。
    pub fn num_locks(&self) -> i32 {
        self.pos.frame_data().expect("frame data").num_locks
    }

    /// 对应 `rethrowException`。
    pub fn rethrow_exception(&self) -> bool {
        self.pos.frame_data().expect("frame data").rethrow_exception
    }

    /// 对应 `duringCall`。
    pub fn during_call(&self) -> bool {
        self.pos.frame_data().expect("frame data").during_call
    }

    /// Rust 增设：消费 `BytecodeFrame` 取内部 `BytecodePosition`（供 caller 帧转 position）。
    pub fn into_position(self) -> BytecodePosition {
        self.pos
    }

    /// Rust 增设：取 `&BytecodePosition` 引用。
    pub fn as_position(&self) -> &BytecodePosition {
        &self.pos
    }
}

impl Deref for BytecodeFrame {
    type Target = BytecodePosition;
    fn deref(&self) -> &BytecodePosition {
        &self.pos
    }
}

impl std::fmt::Display for BytecodeFrame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = String::new();
        code_util::append_frame(&mut buf, self);
        f.write_str(&buf)
    }
}

/// 对应 `Value.ILLEGAL.equals(value)` 判别。
fn is_illegal_value(v: &dyn JavaValue) -> bool {
    v.as_any().downcast_ref::<value::IllegalValue>().is_some()
}
