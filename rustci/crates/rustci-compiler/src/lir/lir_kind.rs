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

//! 镜像 `jdk.graal.compiler.lir.LIRKind`：LIR 值类型，扩展了平台种类并带有引用信息。
//!
//! 偏离记录：Java `LIRKind extends ValueKind<LIRKind>` → Rust struct 实现 `ValueKind` trait。
//! 自递归泛型在 Rust 无等价物，改为非泛型 struct。
//! `PlatformKind` trait 对象不支持 Clone，改用名称字符串标识平台种类。

use std::any::Any;
use std::fmt;

use rustci_vm_ci::meta::platform_kind::PlatformKind;
use rustci_vm_ci::meta::value_kind::ValueKind;

/// 对应 `public final class LIRKind extends ValueKind<LIRKind>`。
///
/// LIR 值类型，除了平台种类外，还携带引用类型信息（是否为对象引用、是否被压缩等）。
pub struct LIRKind {
    /// 对应 `getPlatformKind()`：底层平台种类。
    platform_kind: Box<dyn PlatformKind>,
    /// 对应 `referenceMask`：引用掩码，bit N 表示槽 N 是否为对象引用。
    reference_mask: u64,
    /// 对应 `compressedReferenceMask`：压缩引用掩码。
    compressed_reference_mask: u64,
    /// 对应 `derivedReferenceBase`：派生引用基址。
    derived_reference_base: DerivedReferenceBase,
    /// 对应 `numberOfFloats`：浮点槽数量。
    number_of_floats: u8,
}

impl Clone for LIRKind {
    fn clone(&self) -> Self {
        Self {
            platform_kind: self.platform_kind.clone_box(),
            reference_mask: self.reference_mask,
            compressed_reference_mask: self.compressed_reference_mask,
            derived_reference_base: self.derived_reference_base,
            number_of_floats: self.number_of_floats,
        }
    }
}

/// 对应 `LIRKind.DerivedReferenceBase` 内部枚举。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DerivedReferenceBase {
    /// 无派生引用基址。
    None,
    /// 派生引用基址来自指定槽。
    Slot(u8),
}

impl LIRKind {
    /// 对应 `LIRKind.value(PlatformKind)`：创建单槽值类型。
    pub fn value(platform_kind: Box<dyn PlatformKind>) -> Self {
        Self {
            platform_kind,
            reference_mask: 0,
            compressed_reference_mask: 0,
            derived_reference_base: DerivedReferenceBase::None,
            number_of_floats: 0,
        }
    }

    /// 对应 `LIRKind.reference(PlatformKind)`：创建单槽引用类型。
    pub fn reference(platform_kind: Box<dyn PlatformKind>) -> Self {
        Self {
            platform_kind,
            reference_mask: 1,
            compressed_reference_mask: 0,
            derived_reference_base: DerivedReferenceBase::None,
            number_of_floats: 0,
        }
    }

    /// 对应 `LIRKind.compressedReference(PlatformKind)`：创建单槽压缩引用类型。
    pub fn compressed_reference(platform_kind: Box<dyn PlatformKind>) -> Self {
        Self {
            platform_kind,
            reference_mask: 1,
            compressed_reference_mask: 1,
            derived_reference_base: DerivedReferenceBase::None,
            number_of_floats: 0,
        }
    }

    /// 对应 `LIRKind.derivedReference(PlatformKind, int)`：创建派生引用类型。
    pub fn derived_reference(platform_kind: Box<dyn PlatformKind>, base: u8) -> Self {
        Self {
            platform_kind,
            reference_mask: 1,
            compressed_reference_mask: 0,
            derived_reference_base: DerivedReferenceBase::Slot(base),
            number_of_floats: 0,
        }
    }

    /// 对应 `LIRKind.value(PlatformKind, int, int)`：创建多槽值类型。
    pub fn value_multi(
        platform_kind: Box<dyn PlatformKind>,
        reference_mask: u64,
        compressed_reference_mask: u64,
    ) -> Self {
        Self {
            platform_kind,
            reference_mask,
            compressed_reference_mask,
            derived_reference_base: DerivedReferenceBase::None,
            number_of_floats: 0,
        }
    }

    /// 对应 `isValue(int)`：检查指定槽是否为值（非引用）类型。
    pub fn is_value(&self, idx: u8) -> bool {
        !self.is_reference(idx)
    }

    /// 对应 `isReference(int)`：检查指定槽是否为引用类型。
    pub fn is_reference(&self, idx: u8) -> bool {
        let mask = 1u64 << idx;
        (self.reference_mask & mask) != 0
    }

    /// 对应 `isCompressedReference(int)`：检查指定槽是否为压缩引用类型。
    pub fn is_compressed_reference(&self, idx: u8) -> bool {
        let mask = 1u64 << idx;
        (self.compressed_reference_mask & mask) != 0
    }

    /// 对应 `getReferenceMask()`：获取引用掩码。
    pub fn get_reference_mask(&self) -> u64 {
        self.reference_mask
    }

    /// 对应 `getCompressedReferenceMask()`：获取压缩引用掩码。
    pub fn get_compressed_reference_mask(&self) -> u64 {
        self.compressed_reference_mask
    }

    /// 对应 `getDerivedReferenceBase()`：获取派生引用基址。
    pub fn get_derived_reference_base(&self) -> DerivedReferenceBase {
        self.derived_reference_base
    }

    /// 对应 `isDerivedReference()`：是否为派生引用。
    pub fn is_derived_reference(&self) -> bool {
        matches!(self.derived_reference_base, DerivedReferenceBase::Slot(_))
    }

    /// 对应 `getNumberOfFloats()`：获取浮点槽数量。
    pub fn get_number_of_floats(&self) -> u8 {
        self.number_of_floats
    }

    /// 设置浮点槽数量。
    pub fn set_number_of_floats(&mut self, n: u8) {
        self.number_of_floats = n;
    }

    /// 对应 `combine(LIRKind)`：合并两个 LIRKind。
    pub fn combine(&self, other: &LIRKind) -> LIRKind {
        LIRKind {
            platform_kind: self.platform_kind.clone_box(),
            reference_mask: self.reference_mask | other.reference_mask,
            compressed_reference_mask: self.compressed_reference_mask | other.compressed_reference_mask,
            derived_reference_base: if self.derived_reference_base == other.derived_reference_base {
                self.derived_reference_base
            } else {
                DerivedReferenceBase::None
            },
            number_of_floats: self.number_of_floats.max(other.number_of_floats),
        }
    }
}

impl fmt::Debug for LIRKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LIRKind")
            .field("platform_kind", &self.platform_kind.name())
            .field("reference_mask", &format_args!("{:b}", self.reference_mask))
            .field("compressed_reference_mask", &format_args!("{:b}", self.compressed_reference_mask))
            .field("derived_reference_base", &self.derived_reference_base)
            .finish()
    }
}

impl fmt::Display for LIRKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.platform_kind.name())
    }
}

impl ValueKind for LIRKind {
    fn get_platform_kind(&self) -> &dyn PlatformKind {
        self.platform_kind.as_ref()
    }

    fn change_type(&self, new_platform_kind: &dyn PlatformKind) -> Box<dyn ValueKind> {
        let mut new_kind = self.clone();
        new_kind.platform_kind = new_platform_kind.clone_box();
        Box::new(new_kind)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn clone_box(&self) -> Box<dyn ValueKind> {
        Box::new(self.clone())
    }

    fn kind_equals(&self, other: &dyn ValueKind) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<LIRKind>() {
            self.reference_mask == other.reference_mask
                && self.compressed_reference_mask == other.compressed_reference_mask
                && self.derived_reference_base == other.derived_reference_base
                && self.platform_kind.name() == other.platform_kind.name()
        } else {
            false
        }
    }

    fn kind_hash(&self) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut h: u64 = self.reference_mask;
        h = h.wrapping_mul(31).wrapping_add(self.compressed_reference_mask);
        let mut s = std::collections::hash_map::DefaultHasher::new();
        self.platform_kind.name().hash(&mut s);
        h = h.wrapping_mul(31).wrapping_add(s.finish());
        h
    }
}