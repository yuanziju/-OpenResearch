// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2009, 2015, Oracle and/or its affiliates. All rights reserved.
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

//! 镜像 `jdk.vm.ci.code.DebugInfo`：特定执行点的调试信息。
//!
//! 偏离记录：
//! - Java `final class DebugInfo`（持 `bytecodePosition`/`referenceMap`/`virtualObjectMapping`/
//!   `calleeSaveInfo`）→ Rust `pub struct DebugInfo`。`virtualObjectMapping` 的 nullable `VirtualObject[]`
//!   → `Vec<VirtualObject>`（null → 空 vec，对齐 `getVirtualObjectMapping` 返回非空切片的语义）。
//! - `bytecodePosition` 持 `BytecodePosition`（可能含 `FrameData`，对应 Java `instanceof BytecodeFrame`）。
//!   `frame()` 返回 `Option<&BytecodePosition>`（偏离：Java 返回 `BytecodeFrame`，Rust 侧 `BytecodeFrame`
//!   为 `BytecodePosition` 的 newtype 包装，无法从 `&BytecodePosition` 构造 `&BytecodeFrame`；
//!   调用方经 `frame_data()` 访问帧字段，同 `BytecodeFrame::caller()` 偏离约定）。
//! - `equals`：Java `Objects.equals` 对 `calleeSaveInfo`/`referenceMap` 用引用相等（两者均未覆写
//!   `equals`），Rust 侧 owned 字段经数据指针比对（对齐 Java 引用相等语义）。
//! - `hashCode` 抛 `UnsupportedOperationException` → 不实现 `Hash`。

use std::fmt;

use crate::code::bytecode_position::BytecodePosition;
use crate::code::code_util;
use crate::code::reference_map::ReferenceMap;
use crate::code::register_save_layout::RegisterSaveLayout;
use crate::code::virtual_object::VirtualObject;

/// 对应 `final class DebugInfo`。
pub struct DebugInfo {
    bytecode_position: BytecodePosition,
    reference_map: Option<Box<dyn ReferenceMap>>,
    virtual_object_mapping: Vec<VirtualObject>,
    callee_save_info: Option<RegisterSaveLayout>,
}

impl DebugInfo {
    /// 对应 `DebugInfo(BytecodePosition codePos, VirtualObject[] virtualObjectMapping)`。
    pub fn new(code_pos: BytecodePosition, virtual_object_mapping: Vec<VirtualObject>) -> Self {
        Self {
            bytecode_position: code_pos,
            reference_map: None,
            virtual_object_mapping,
            callee_save_info: None,
        }
    }

    /// 对应 `DebugInfo(BytecodePosition codePos)`：委托 `this(codePos, null)`。
    pub fn new_without_mapping(code_pos: BytecodePosition) -> Self {
        Self::new(code_pos, Vec::new())
    }

    /// 对应 `setReferenceMap(ReferenceMap)`。
    pub fn set_reference_map(&mut self, reference_map: Box<dyn ReferenceMap>) {
        self.reference_map = Some(reference_map);
    }

    /// 对应 `hasFrame()`（`getBytecodePosition() instanceof BytecodeFrame`）。
    pub fn has_frame(&self) -> bool {
        self.bytecode_position.has_frame()
    }

    /// 对应 `BytecodeFrame frame()`（偏离：返回 `Option<&BytecodePosition>`，见模块偏离记录）。
    pub fn frame(&self) -> Option<&BytecodePosition> {
        if self.has_frame() {
            Some(&self.bytecode_position)
        } else {
            None
        }
    }

    /// 对应 `getBytecodePosition()`。
    pub fn get_bytecode_position(&self) -> &BytecodePosition {
        &self.bytecode_position
    }

    /// 对应 `getReferenceMap()`（nullable → `Option`）。
    pub fn get_reference_map(&self) -> Option<&dyn ReferenceMap> {
        self.reference_map.as_deref()
    }

    /// 对应 `getVirtualObjectMapping()`。
    pub fn get_virtual_object_mapping(&self) -> &[VirtualObject] {
        &self.virtual_object_mapping
    }

    /// 对应 `setCalleeSaveInfo(RegisterSaveLayout)`。
    pub fn set_callee_save_info(&mut self, callee_save_info: RegisterSaveLayout) {
        self.callee_save_info = Some(callee_save_info);
    }

    /// 对应 `getCalleeSaveInfo()`（nullable → `Option`）。
    pub fn get_callee_save_info(&self) -> Option<&RegisterSaveLayout> {
        self.callee_save_info.as_ref()
    }

    /// 对应 `boolean equals(Object)`。
    pub fn equals(&self, other: &DebugInfo) -> bool {
        if std::ptr::eq(self, other) {
            return true;
        }
        if !self.bytecode_position.equals(&other.bytecode_position) {
            return false;
        }
        // calleeSaveInfo：Java 引用相等（无 equals 覆写），owned 字段经指针比对。
        match (&self.callee_save_info, &other.callee_save_info) {
            (None, None) => {}
            (Some(a), Some(b)) => {
                if !std::ptr::eq(
                    a as *const RegisterSaveLayout,
                    b as *const RegisterSaveLayout,
                ) {
                    return false;
                }
            }
            _ => return false,
        }
        // referenceMap：Java 引用相等（ReferenceMap 抽象类无 equals 覆写）。
        match (&self.reference_map, &other.reference_map) {
            (None, None) => true,
            (Some(a), Some(b)) => std::ptr::eq(
                a.as_ref() as *const dyn ReferenceMap,
                b.as_ref() as *const dyn ReferenceMap,
            ),
            _ => false,
        }
    }
}

impl fmt::Display for DebugInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 对应 `toString() { return CodeUtil.append(new StringBuilder(100), this, null).toString(); }`。
        let mut buf = String::with_capacity(100);
        code_util::append_debug_info(&mut buf, self, None);
        f.write_str(&buf)
    }
}

impl fmt::Debug for DebugInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DebugInfo")
            .field("has_frame", &self.has_frame())
            .field("virtual_object_count", &self.virtual_object_mapping.len())
            .field("has_reference_map", &self.reference_map.is_some())
            .field("has_callee_save_info", &self.callee_save_info.is_some())
            .finish()
    }
}
