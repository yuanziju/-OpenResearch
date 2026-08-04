// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2019, 2024, Oracle and/or its affiliates. All rights reserved.
 * DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
 *
 * This code is free software; you can redistribute it and/or modify it
 * under the terms of the GNU General Public License version 2 only, as
 * published by the Free Software Foundation.  Oracle designates this
 * particular file as subject to the "Classpath" exception as provided
 * by Oracle in the LICENSE file that accompanied this code.
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

//! 镜像 `jdk.graal.compiler.lir.LIRFrameState`：LIR 帧状态。
//!
//! 偏离记录：Java `final class LIRFrameState` → Rust struct。
//! LIR 帧状态关联去优化信息、调试信息和一个帧状态。

use std::fmt;

/// 对应 `LIRFrameState.StateProcedure` 内部接口。
pub trait StateProcedure {
    /// 对应 `doState(LIRFrameState)`：处理帧状态。
    fn do_state(&mut self, state: &LIRFrameState);
}

/// 对应 `LIRFrameState.StateConsumer` 内部接口。
pub trait StateConsumer {
    /// 对应 `consumeState(LIRFrameState)`：消费帧状态。
    fn consume_state(&mut self, state: &LIRFrameState);
}

/// 对应 `public final class LIRFrameState`。
///
/// LIR 指令的帧状态，包含去优化信息和调试信息。
#[derive(Debug)]
pub struct LIRFrameState {
    /// 对应 `topFrame`：顶层帧的字节码位置。
    pub top_frame: Option<BytecodePosition>,
    /// 对应 `debugInfo`：调试信息（寄存器/栈槽→值映射）。
    pub debug_info: Option<Box<dyn DebugInfo>>,
    /// 对应 `duringCall`：是否在方法调用期间。
    pub during_call: bool,
    /// 对应 `rethrowException`：是否重新抛出异常。
    pub rethrow_exception: bool,
}

impl Clone for LIRFrameState {
    fn clone(&self) -> Self {
        Self {
            top_frame: self.top_frame.clone(),
            debug_info: self.debug_info.as_ref().map(|d| d.clone_box()),
            during_call: self.during_call,
            rethrow_exception: self.rethrow_exception,
        }
    }
}

/// 对应 `jdk.vm.ci.code.BytecodePosition`：字节码位置。
#[derive(Debug, Clone)]
pub struct BytecodePosition {
    /// 方法标识。
    pub method: String,
    /// 字节码索引。
    pub bci: i32,
    /// 调用者位置（内联链）。
    pub caller: Option<Box<BytecodePosition>>,
}

impl BytecodePosition {
    /// 创建字节码位置。
    pub fn new(method: &str, bci: i32) -> Self {
        Self {
            method: method.to_string(),
            bci,
            caller: None,
        }
    }

    /// 创建带有调用者的字节码位置。
    pub fn new_with_caller(method: &str, bci: i32, caller: BytecodePosition) -> Self {
        Self {
            method: method.to_string(),
            bci,
            caller: Some(Box::new(caller)),
        }
    }
}

/// 对应 `jdk.vm.ci.code.DebugInfo`：调试信息。
pub trait DebugInfo: fmt::Debug {
    /// 获取调试信息中注册的虚拟机对象。
    fn get_virtual_objects(&self) -> &[Box<dyn VirtualObject>];
    /// 获取引用映射。
    fn get_reference_map(&self) -> Option<&dyn ReferenceMap>;
    /// 复制调试信息。
    fn clone_box(&self) -> Box<dyn DebugInfo>;
}

/// 对应 `jdk.vm.ci.code.VirtualObject`：虚拟机对象。
pub trait VirtualObject: fmt::Debug {
    /// 获取对象类型。
    fn get_type(&self) -> &str;
    /// 获取对象 ID。
    fn get_id(&self) -> i32;
    /// 获取入口值。
    fn get_values(&self) -> &[Box<dyn VirtualObject>];
    /// 复制虚拟机对象。
    fn clone_box(&self) -> Box<dyn VirtualObject>;
}

/// 对应 `jdk.vm.ci.code.ReferenceMap`：引用映射。
pub trait ReferenceMap: fmt::Debug {
    /// 检查指定寄存器/偏移是否持有引用。
    fn is_reference(&self, slot: usize) -> bool;
    /// 复制引用映射。
    fn clone_box(&self) -> Box<dyn ReferenceMap>;
}

impl LIRFrameState {
    /// 创建帧状态。
    pub fn new(
        top_frame: Option<BytecodePosition>,
        debug_info: Option<Box<dyn DebugInfo>>,
        during_call: bool,
        rethrow_exception: bool,
    ) -> Self {
        Self {
            top_frame,
            debug_info,
            during_call,
            rethrow_exception,
        }
    }

    /// 对应 `hasDebugInfo()`：是否有调试信息。
    pub fn has_debug_info(&self) -> bool {
        self.debug_info.is_some()
    }

    /// 对应 `clearDebugInfo()`：清除调试信息。
    pub fn clear_debug_info(&mut self) {
        self.debug_info = None;
    }
}