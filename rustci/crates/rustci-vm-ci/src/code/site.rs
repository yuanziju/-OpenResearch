// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2009, 2025, Oracle and/or its affiliates. All rights reserved.
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
 * or visit oracle.com if you need additional information or have any
 * questions.
 */

//! 镜像 `jdk.vm.ci.code.site`：机器码位置相关信息的表示（站点、信息点、引用、数据补丁等）。
//!
//! 偏离记录：
//! - Java `abstract class Site`（`final int pcOffset` + `final hashCode` 抛异常 + `toString` =
//!   `identityHashCodeString` + abstract `equals`）→ Rust `pub struct Site { pub pc_offset: i32 }`。
//!   Java abstract 不可实例化，Rust 侧为具体 struct（组合载体）。`hashCode` 抛
//!   `UnsupportedOperationException` → 不实现 `Hash`（与 `DebugInfo`/`RegisterSaveLayout` 等约定一致）。
//! - Java 继承 `X extends Site` / `Call extends Infopoint extends Site` → Rust 组合（子类持 `site: Site`
//!   或 `infopoint: Infopoint` 字段）+ `pc_offset()` 访问器委托。Rust 无继承，`super.equals` 经
//!   子类直接比对 `infopoint`/`site` 字段复刻。
//! - `Mark.id`：Java `Object id`（可 null，`instanceof Integer` 时按十六进制格式化，否则 `id.toString()`）。
//!   Rust 侧 `id: Box<dyn std::fmt::Debug>`（`dyn Any` 非 `Debug`，无法满足 `CodeCacheProvider::
//!   getMarkName` 默认 `format!("{:?}", mark.id)`；选 `Debug` 以复刻默认实现）。`instanceof Integer`
//!   十六进制路径舍弃（`dyn Debug` 不可下转），用 `{:?}` 格式化；null 路径舍弃（`Box` 非空）。
//!   `equals` 的 `Objects.equals(this.id, that.id)`（`Object.equals` 即引用相等）→ `ptr::eq` 比对
//!   fat 指针的数据指针。
//! - `DataPatch.note`：Java `Object note` → `Option<Box<dyn std::fmt::Debug>>`（同 `Mark.id` 理由）。
//!   `toString` 的 `note.toString()` → `format!("{:?}", note)`；`equals` 的 `Objects.equals` → `ptr::eq`。
//! - `Reference`（abstract，abstract `hashCode`/`equals`）→ `trait Reference: Debug`，仅设
//!   `reference_equals` + `as_any`（`hashCode` 无消费者，舍弃，与 `Site` 等约定一致）。
//! - `ConstantReference.equals`：Java `Objects.equals(this.constant, that.constant)`，`VMConstant` 未覆写
//!   `equals`（本 crate 无具体实现）→ 引用相等，Rust 用 `ptr::eq`。`hashCode`（`constant.hashCode()`，
//!   即 `Object.hashCode`）舍弃。
//! - `Call.toString`/`ImplicitExceptionDispatch.toString` 的 `target.toString()`（`InvokeTarget` 标记
//!   接口，默认 `Object.toString`）→ `{:?}`（`InvokeTarget: Debug`，无 `Display`）。
//! - `Infopoint.compareTo`（`Comparable<Infopoint>`，按 `pcOffset` 再 `reason`）→ `compare_to` 方法
//!   返回 `Ordering`（不实现 `Ord`：`DebugInfo` 无 `Ord`/`Eq`，无法满足 `Ord` 的 `Eq` 超 trait）。
//! - `ReferenceMap.toString()`（Java）→ `{:?}`（`ReferenceMap: Debug` 无 `Display`，见 `code_util` 偏离）。
//! - `appendDebugInfo`（`Infopoint` protected static）→ 模块私有 `fn append_debug_info`，复用 `DebugInfo`
//!   访问器；`info.frame()` 返回 `Option<&BytecodePosition>`，经 `frame_data()` 取帧字段（见
//!   `bytecode_frame.rs` 偏离）。

use std::any::Any;
use std::fmt::{self, Write};

use crate::code::debug_info::DebugInfo;
use crate::meta::invoke_target::InvokeTarget;
use crate::meta::meta_util;
use crate::meta::vm_constant::VMConstant;

/// 对应 `abstract class Site`。
pub struct Site {
    /// 对应 `public final int pcOffset`。
    pub pc_offset: i32,
}

impl Site {
    /// 对应 `Site(int pos)`。
    pub fn new(pc_offset: i32) -> Self {
        Self { pc_offset }
    }
}

impl fmt::Display for Site {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 对应 `toString() { return identityHashCodeString(this); }`。
        f.write_str(&meta_util::identity_hash_code_string(Some("Site")))
    }
}

impl fmt::Debug for Site {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Site")
            .field("pc_offset", &self.pc_offset)
            .finish()
    }
}

/// 对应 `enum InfopointReason`。声明顺序对齐 Java ordinal（`Comparable` 按 ordinal 比较）。
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum InfopointReason {
    Safepoint,
    Call,
    ImplicitException,
    MethodStart,
    MethodEnd,
    BytecodePosition,
}

impl fmt::Display for InfopointReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 对应 `Enum.toString()`（默认为名称）。
        let name = match self {
            InfopointReason::Safepoint => "SAFEPOINT",
            InfopointReason::Call => "CALL",
            InfopointReason::ImplicitException => "IMPLICIT_EXCEPTION",
            InfopointReason::MethodStart => "METHOD_START",
            InfopointReason::MethodEnd => "METHOD_END",
            InfopointReason::BytecodePosition => "BYTECODE_POSITION",
        };
        f.write_str(name)
    }
}

/// 对应 `class Infopoint extends Site implements Comparable<Infopoint>`。
pub struct Infopoint {
    site: Site,
    /// 对应 `public final DebugInfo debugInfo`。
    pub debug_info: Option<DebugInfo>,
    /// 对应 `public final InfopointReason reason`。
    pub reason: InfopointReason,
}

impl Infopoint {
    /// 对应 `Infopoint(int pcOffset, DebugInfo debugInfo, InfopointReason reason)`。
    ///
    /// Java `assert reason != CALL || this instanceof Call`：直接构造 `Infopoint` 时 reason 不得为
    /// `Call`（仅 `Call` 子类可使用）。`Call::new` 绕过本构造器直接组装 `Infopoint` 字段。
    pub fn new(pc_offset: i32, debug_info: Option<DebugInfo>, reason: InfopointReason) -> Self {
        debug_assert!(
            reason != InfopointReason::Call,
            "InfopointReason.CALL requires Call type"
        );
        Self {
            site: Site::new(pc_offset),
            debug_info,
            reason,
        }
    }

    /// 对应 `pcOffset`（继承自 `Site`）。
    pub fn pc_offset(&self) -> i32 {
        self.site.pc_offset
    }

    /// 对应 `int compareTo(Infopoint o)`：按 `pcOffset` 再 `reason`。
    pub fn compare_to(&self, other: &Infopoint) -> std::cmp::Ordering {
        self.site
            .pc_offset
            .cmp(&other.site.pc_offset)
            .then(self.reason.cmp(&other.reason))
    }

    /// 对应 `boolean equals(Object)`：`getClass()==getClass()` && `pcOffset` && `debugInfo` && `reason`。
    pub fn equals(&self, other: &Infopoint) -> bool {
        if std::ptr::eq(self as *const _, other as *const _) {
            return true;
        }
        if self.site.pc_offset != other.site.pc_offset || self.reason != other.reason {
            return false;
        }
        match (&self.debug_info, &other.debug_info) {
            (None, None) => true,
            (Some(a), Some(b)) => a.equals(b),
            _ => false,
        }
    }
}

impl fmt::Display for Infopoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 对应 `toString()`：`pcOffset[<infopoint>]` + appendDebugInfo。
        write!(f, "{}[<infopoint>]", self.site.pc_offset)?;
        if let Some(info) = &self.debug_info {
            let mut buf = String::new();
            append_debug_info(&mut buf, info);
            f.write_str(&buf)?;
        }
        Ok(())
    }
}

impl fmt::Debug for Infopoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Infopoint")
            .field("pc_offset", &self.site.pc_offset)
            .field("reason", &self.reason)
            .field("has_debug_info", &self.debug_info.is_some())
            .finish()
    }
}

/// 对应 `final class Call extends Infopoint`。
pub struct Call {
    infopoint: Infopoint,
    /// 对应 `public final InvokeTarget target`。
    pub target: Box<dyn InvokeTarget>,
    /// 对应 `public final int size`。
    pub size: i32,
    /// 对应 `public final boolean direct`。
    pub direct: bool,
}

impl Call {
    /// 对应 `Call(InvokeTarget target, int pcOffset, int size, boolean direct, DebugInfo debugInfo)`。
    pub fn new(
        target: Box<dyn InvokeTarget>,
        pc_offset: i32,
        size: i32,
        direct: bool,
        debug_info: Option<DebugInfo>,
    ) -> Self {
        Self {
            infopoint: Infopoint {
                site: Site::new(pc_offset),
                debug_info,
                reason: InfopointReason::Call,
            },
            target,
            size,
            direct,
        }
    }

    /// 对应 `pcOffset`（继承自 `Site`）。
    pub fn pc_offset(&self) -> i32 {
        self.infopoint.site.pc_offset
    }

    /// 对应 `debugInfo`（继承自 `Infopoint`）。
    pub fn debug_info(&self) -> Option<&DebugInfo> {
        self.infopoint.debug_info.as_ref()
    }

    /// 对应 `reason`（继承自 `Infopoint`）。
    pub fn reason(&self) -> InfopointReason {
        self.infopoint.reason
    }

    /// 对应 `boolean equals(Object)`：`super.equals` && `size` && `direct` && `Objects.equals(target)`。
    pub fn equals(&self, other: &Call) -> bool {
        if std::ptr::eq(self as *const _, other as *const _) {
            return true;
        }
        if !self.infopoint.equals(&other.infopoint) {
            return false;
        }
        if self.size != other.size || self.direct != other.direct {
            return false;
        }
        // `InvokeTarget` 未覆写 `equals`（标记接口）→ 引用相等。
        std::ptr::eq(
            self.target.as_ref() as *const dyn InvokeTarget,
            other.target.as_ref() as *const dyn InvokeTarget,
        )
    }
}

impl fmt::Display for Call {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 对应 `toString()`：`pcOffset[target]` + appendDebugInfo。
        write!(f, "{}[{:?}]", self.infopoint.site.pc_offset, self.target)?;
        if let Some(info) = &self.infopoint.debug_info {
            let mut buf = String::new();
            append_debug_info(&mut buf, info);
            f.write_str(&buf)?;
        }
        Ok(())
    }
}

impl fmt::Debug for Call {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Call")
            .field("pc_offset", &self.infopoint.site.pc_offset)
            .field("size", &self.size)
            .field("direct", &self.direct)
            .field("target", &self.target)
            .finish()
    }
}

/// 对应 `final class ImplicitExceptionDispatch extends Infopoint`。
pub struct ImplicitExceptionDispatch {
    infopoint: Infopoint,
    /// 对应 `public final int dispatchOffset`。
    pub dispatch_offset: i32,
}

impl ImplicitExceptionDispatch {
    /// 对应 `ImplicitExceptionDispatch(int pcOffset, int dispatchOffset, DebugInfo debugInfo)`。
    pub fn new(pc_offset: i32, dispatch_offset: i32, debug_info: Option<DebugInfo>) -> Self {
        Self {
            infopoint: Infopoint {
                site: Site::new(pc_offset),
                debug_info,
                reason: InfopointReason::ImplicitException,
            },
            dispatch_offset,
        }
    }

    /// 对应 `pcOffset`（继承自 `Site`）。
    pub fn pc_offset(&self) -> i32 {
        self.infopoint.site.pc_offset
    }

    /// 对应 `boolean equals(Object)`：`super.equals` && `dispatchOffset`。
    pub fn equals(&self, other: &ImplicitExceptionDispatch) -> bool {
        if std::ptr::eq(self as *const _, other as *const _) {
            return true;
        }
        if !self.infopoint.equals(&other.infopoint) {
            return false;
        }
        self.dispatch_offset == other.dispatch_offset
    }
}

impl fmt::Display for ImplicitExceptionDispatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 对应 `toString()`：`pcOffset->dispatchOffset` + appendDebugInfo。
        write!(
            f,
            "{}->{}",
            self.infopoint.site.pc_offset, self.dispatch_offset
        )?;
        if let Some(info) = &self.infopoint.debug_info {
            let mut buf = String::new();
            append_debug_info(&mut buf, info);
            f.write_str(&buf)?;
        }
        Ok(())
    }
}

impl fmt::Debug for ImplicitExceptionDispatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ImplicitExceptionDispatch")
            .field("pc_offset", &self.infopoint.site.pc_offset)
            .field("dispatch_offset", &self.dispatch_offset)
            .finish()
    }
}

/// 对应 `abstract class Reference`。
///
/// 增设 `Display` 超 trait：Java `Reference` 继承 `Object`（含 `toString`），子类
/// `ConstantReference`/`DataSectionReference` 覆写 `toString`；Rust 侧两实现已提供 `Display`，
/// 增设超 trait 使 `dyn Reference` 可经 `{}` 格式化（对齐 `DataPatch.toString` 的
/// `reference.toString()` 调用）。
pub trait Reference: fmt::Debug + fmt::Display {
    /// 对应 `abstract boolean equals(Object)`。
    fn reference_equals(&self, other: &dyn Reference) -> bool;

    /// Rust 增设：支持 `DataPatch.equals` 等的下转判型。
    fn as_any(&self) -> &dyn Any;
}

/// 对应 `final class ConstantReference extends Reference`。
pub struct ConstantReference {
    constant: Box<dyn VMConstant>,
}

impl ConstantReference {
    /// 对应 `ConstantReference(VMConstant constant)`。
    pub fn new(constant: Box<dyn VMConstant>) -> Self {
        Self { constant }
    }

    /// 对应 `VMConstant getConstant()`。
    pub fn get_constant(&self) -> &dyn VMConstant {
        self.constant.as_ref()
    }
}

impl Reference for ConstantReference {
    fn reference_equals(&self, other: &dyn Reference) -> bool {
        match other.as_any().downcast_ref::<ConstantReference>() {
            None => false,
            Some(o) => std::ptr::eq(
                self.constant.as_ref() as *const dyn VMConstant,
                o.constant.as_ref() as *const dyn VMConstant,
            ),
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl fmt::Display for ConstantReference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 对应 `toString() { return constant.toString(); }`。
        write!(f, "{}", self.constant)
    }
}

impl fmt::Debug for ConstantReference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ConstantReference")
            .field("constant", &self.constant)
            .finish()
    }
}

/// 对应 `final class DataSectionReference extends Reference`。
pub struct DataSectionReference {
    initialized: bool,
    offset: i32,
}

impl DataSectionReference {
    /// 对应 `DataSectionReference()`：`offset = 0xDEADDEAD`（数据节布局固定前的哨兵值）。
    pub fn new() -> Self {
        Self {
            initialized: false,
            offset: 0xDEADDEAD_u32 as i32,
        }
    }

    /// 对应 `int getOffset()`（`assert initialized`）。
    pub fn get_offset(&self) -> i32 {
        assert!(self.initialized);
        self.offset
    }

    /// 对应 `void setOffset(int offset)`（`assert !initialized`，单次赋值）。
    pub fn set_offset(&mut self, offset: i32) {
        assert!(!self.initialized);
        self.initialized = true;
        self.offset = offset;
    }
}

impl Default for DataSectionReference {
    fn default() -> Self {
        Self::new()
    }
}

impl Reference for DataSectionReference {
    fn reference_equals(&self, other: &dyn Reference) -> bool {
        match other.as_any().downcast_ref::<DataSectionReference>() {
            None => false,
            Some(o) => self.offset == o.offset,
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl fmt::Display for DataSectionReference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.initialized {
            write!(f, "DataSection[0x{:x}]", self.offset)
        } else {
            f.write_str("DataSection[?]")
        }
    }
}

impl fmt::Debug for DataSectionReference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DataSectionReference")
            .field("initialized", &self.initialized)
            .field("offset", &self.offset)
            .finish()
    }
}

/// 对应 `final class DataPatch extends Site`。
pub struct DataPatch {
    site: Site,
    /// 对应 `public Reference reference`。
    pub reference: Box<dyn Reference>,
    /// 对应 `public Object note`。
    pub note: Option<Box<dyn fmt::Debug>>,
}

impl DataPatch {
    /// 对应 `DataPatch(int pcOffset, Reference reference)`（`note = null`）。
    pub fn new(pc_offset: i32, reference: Box<dyn Reference>) -> Self {
        Self {
            site: Site::new(pc_offset),
            reference,
            note: None,
        }
    }

    /// 对应 `DataPatch(int pcOffset, Reference reference, Object note)`。
    pub fn new_with_note(
        pc_offset: i32,
        reference: Box<dyn Reference>,
        note: Box<dyn fmt::Debug>,
    ) -> Self {
        Self {
            site: Site::new(pc_offset),
            reference,
            note: Some(note),
        }
    }

    /// 对应 `pcOffset`（继承自 `Site`）。
    pub fn pc_offset(&self) -> i32 {
        self.site.pc_offset
    }

    /// 对应 `boolean equals(Object)`：`pcOffset` && `Objects.equals(reference)` && `Objects.equals(note)`。
    pub fn equals(&self, other: &DataPatch) -> bool {
        if std::ptr::eq(self as *const _, other as *const _) {
            return true;
        }
        if self.site.pc_offset != other.site.pc_offset {
            return false;
        }
        if !self.reference.reference_equals(other.reference.as_ref()) {
            return false;
        }
        match (&self.note, &other.note) {
            (None, None) => true,
            (Some(a), Some(b)) => std::ptr::eq(
                a.as_ref() as *const dyn fmt::Debug,
                b.as_ref() as *const dyn fmt::Debug,
            ),
            _ => false,
        }
    }
}

impl fmt::Display for DataPatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 对应 `toString()`：`pcOffset[<data patch referring to REFERENCE>]`（含 note 时追加 `, note: NOTE`）。
        match &self.note {
            None => write!(
                f,
                "{}[<data patch referring to {}>]",
                self.site.pc_offset, self.reference
            ),
            Some(note) => write!(
                f,
                "{}[<data patch referring to {}>, note: {:?}]",
                self.site.pc_offset, self.reference, note
            ),
        }
    }
}

impl fmt::Debug for DataPatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DataPatch")
            .field("pc_offset", &self.site.pc_offset)
            .field("reference", &self.reference)
            .field("has_note", &self.note.is_some())
            .finish()
    }
}

/// 对应 `final class ExceptionHandler extends Site`。
pub struct ExceptionHandler {
    site: Site,
    /// 对应 `public final int handlerPos`。
    pub handler_pos: i32,
}

impl ExceptionHandler {
    /// 对应 `ExceptionHandler(int pcOffset, int handlerPos)`。
    pub fn new(pc_offset: i32, handler_pos: i32) -> Self {
        Self {
            site: Site::new(pc_offset),
            handler_pos,
        }
    }

    /// 对应 `pcOffset`（继承自 `Site`）。
    pub fn pc_offset(&self) -> i32 {
        self.site.pc_offset
    }

    /// 对应 `boolean equals(Object)`：`pcOffset` && `handlerPos`。
    pub fn equals(&self, other: &ExceptionHandler) -> bool {
        if std::ptr::eq(self as *const _, other as *const _) {
            return true;
        }
        self.site.pc_offset == other.site.pc_offset && self.handler_pos == other.handler_pos
    }
}

impl fmt::Display for ExceptionHandler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 对应 `toString()`：`pcOffset[<exception edge to handlerPos>]`。
        write!(
            f,
            "{}[<exception edge to {}>]",
            self.site.pc_offset, self.handler_pos
        )
    }
}

impl fmt::Debug for ExceptionHandler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ExceptionHandler")
            .field("pc_offset", &self.site.pc_offset)
            .field("handler_pos", &self.handler_pos)
            .finish()
    }
}

/// 对应 `final class Mark extends Site`。
pub struct Mark {
    site: Site,
    /// 对应 `public final Object id`。
    pub id: Box<dyn fmt::Debug>,
}

impl Mark {
    /// 对应 `Mark(int pcOffset, Object id)`。
    pub fn new(pc_offset: i32, id: Box<dyn fmt::Debug>) -> Self {
        Self {
            site: Site::new(pc_offset),
            id,
        }
    }

    /// 对应 `pcOffset`（继承自 `Site`）。
    pub fn pc_offset(&self) -> i32 {
        self.site.pc_offset
    }

    /// 对应 `boolean equals(Object)`：`pcOffset` && `Objects.equals(id)`（`Object.equals` 即引用相等）。
    pub fn equals(&self, other: &Mark) -> bool {
        if std::ptr::eq(self as *const _, other as *const _) {
            return true;
        }
        if self.site.pc_offset != other.site.pc_offset {
            return false;
        }
        std::ptr::eq(
            self.id.as_ref() as *const dyn fmt::Debug,
            other.id.as_ref() as *const dyn fmt::Debug,
        )
    }
}

impl fmt::Display for Mark {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 对应 `toString()`：`pcOffset[<mark>]`（id==null）或 `pcOffset[<mark with id STR>]`。
        // 偏离：`instanceof Integer` 十六进制路径舍弃（`dyn Debug` 不可下转），统一用 `{:?}`。
        write!(f, "{}[<mark with id {:?}>]", self.site.pc_offset, self.id)
    }
}

impl fmt::Debug for Mark {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Mark")
            .field("pc_offset", &self.site.pc_offset)
            .field("id", &self.id)
            .finish()
    }
}

/// 对应 `Infopoint.appendDebugInfo(StringBuilder, DebugInfo)`（protected static）。
fn append_debug_info(buf: &mut String, info: &DebugInfo) {
    if let Some(ref_map) = info.get_reference_map() {
        // Java `refMap.toString()`：`ReferenceMap` 抽象类，具体子类覆写；Rust `ReferenceMap: Debug`
        // 无 `Display`，用 `{:?}`（见 `code_util` 偏离）。
        let _ = write!(buf, "{:?}", ref_map);
        buf.push(']');
    }
    if let Some(callee_save_info) = info.get_callee_save_info() {
        buf.push_str(" callee-save-info[");
        let mut sep = "";
        for (reg, slot) in &callee_save_info.registers_to_slots(true) {
            let _ = write!(buf, "{}{}->{}", sep, reg, slot);
            sep = ", ";
        }
        buf.push(']');
    }
    let code_pos = info.get_bytecode_position();
    buf.push(' ');
    meta_util::append_location(buf, Some(code_pos.get_method()), code_pos.get_bci());
    if info.has_frame() {
        if let Some(frame_pos) = info.frame() {
            if let Some(f) = frame_pos.frame_data() {
                let _ = write!(buf, " #locals={} #expr={}", f.num_locals, f.num_stack);
                if f.num_locks > 0 {
                    let _ = write!(buf, " #locks={}", f.num_locks);
                }
            }
        }
    }
}
