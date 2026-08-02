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

//! 镜像 `jdk.vm.ci.code.TargetDescription`：编译目标机器描述。
//!
//! 偏离记录：
//! - Java `class TargetDescription`（`public final` 字段）→ Rust `pub struct TargetDescription`
//!   持 `pub` 字段（对齐 Java `public final` 可达性，同 `Location`/`Register` 约定）。
//! - `linuxOs`/`macOs`：Java 经 `OperatingSystem.isLinux()`/`isMacOS()` 运行时判定 → Rust
//!   `cfg!(target_os = ...)` 编译期判定（对齐目标平台语义）。
//! - 构造器 `assert arch.getPlatformKind(wordJavaKind).equals(arch.getWordKind())` →
//!   `debug_assert!` 比较 kind 的 `get_key` 指针（对齐 Java `PlatformKind.equals` 默认引用相等）。
//! - `hashCode` 抛 `UnsupportedOperationException` → 不实现 `Hash`。`toString` 委托
//!   `MetaUtil.identityHashCodeString`（Java 用 `getClass().getName() + "@" + identityHashCode`）。
//! - `equals`（Java `final`）按全部字段逐一比较。

use crate::code::architecture::Architecture;
use crate::meta::java_kind::JavaKind;
use crate::meta::meta_util;
use crate::meta::platform_kind::Key;

/// 对应 `class TargetDescription`。
pub struct TargetDescription {
    /// 对应 `public final boolean linuxOs`。
    pub linux_os: bool,
    /// 对应 `public final boolean macOs`。
    pub mac_os: bool,
    /// 对应 `public final Architecture arch`。
    pub arch: Architecture,
    /// 对应 `public final boolean isMP`。
    pub is_mp: bool,
    /// 对应 `public final boolean inlineObjects`。
    pub inline_objects: bool,
    /// 对应 `public final int wordSize`。
    pub word_size: i32,
    /// 对应 `public final JavaKind wordJavaKind`。
    pub word_java_kind: JavaKind,
    /// 对应 `public final int stackAlignment`。
    pub stack_alignment: i32,
    /// 对应 `public final int implicitNullCheckLimit`。
    pub implicit_null_check_limit: i32,
}

impl TargetDescription {
    /// 对应 `TargetDescription(Architecture arch, boolean isMP, int stackAlignment,
    /// int implicitNullCheckLimit, boolean inlineObjects)`。
    pub fn new(
        arch: Architecture,
        is_mp: bool,
        stack_alignment: i32,
        implicit_null_check_limit: i32,
        inline_objects: bool,
    ) -> Self {
        let word_size = arch.get_word_size();
        let word_java_kind = JavaKind::from_word_size(word_size);
        // 对应 `assert arch.getPlatformKind(wordJavaKind).equals(arch.getWordKind())`。
        if let Some(pk) = arch.get_platform_kind(word_java_kind) {
            let a = pk.get_key();
            let b = arch.get_word_kind().get_key();
            debug_assert!(std::ptr::eq(a as *const dyn Key, b as *const dyn Key));
        }
        Self {
            linux_os: cfg!(target_os = "linux"),
            mac_os: cfg!(target_os = "macos"),
            arch,
            is_mp,
            inline_objects,
            word_size,
            word_java_kind,
            stack_alignment,
            implicit_null_check_limit,
        }
    }

    /// 对应 `final boolean equals(Object)`。
    pub fn equals(&self, other: &TargetDescription) -> bool {
        if std::ptr::eq(self, other) {
            return true;
        }
        self.implicit_null_check_limit == other.implicit_null_check_limit
            && self.inline_objects == other.inline_objects
            && self.is_mp == other.is_mp
            && self.stack_alignment == other.stack_alignment
            && self.word_java_kind == other.word_java_kind
            && self.word_size == other.word_size
            && self.arch.equals(&other.arch)
    }
}

impl std::fmt::Display for TargetDescription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // 对应 `toString() { return identityHashCodeString(this); }`。
        f.write_str(&meta_util::identity_hash_code_string(Some(
            "TargetDescription",
        )))
    }
}

impl std::fmt::Debug for TargetDescription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TargetDescription")
            .field("word_size", &self.word_size)
            .field("word_java_kind", &self.word_java_kind)
            .field("stack_alignment", &self.stack_alignment)
            .field("implicit_null_check_limit", &self.implicit_null_check_limit)
            .field("is_mp", &self.is_mp)
            .field("inline_objects", &self.inline_objects)
            .finish_non_exhaustive()
    }
}
