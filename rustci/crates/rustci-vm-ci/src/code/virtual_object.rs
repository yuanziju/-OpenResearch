// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2010, 2019, Oracle and/or its affiliates. All rights reserved.
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

//! 镜像 `jdk.vm.ci.code.VirtualObject`：逃逸分析移除分配的对象的表示。
//!
//! 偏离记录：
//! - Java `final class VirtualObject implements JavaValue`（持可变 `values`/`slotKinds`/
//!   `isAutoBox`）→ Rust `pub struct VirtualObject` 持 `type_`/`values`/`slot_kinds`/`id`/
//!   `is_auto_box`。`setValues` 取所有权替换。
//! - `verifyLayout(LayoutVerifier)` 的 `JVMCIError` → Rust `panic!`。
//! - `toString` 的递归 `appendValue` 用 `IdentityHashMap` 防环 → Rust 用 `HashSet<*const ()>`
//!   按数据指针防环。
//! - `equals` 按类型+`values.length`+逐元素 identity（Java `same()` 即 `o1 == o2`）→ Rust
//!   `as_any` 数据指针比对。
//! - 嵌套 `interface LayoutVerifier` → 同文件 `pub trait LayoutVerifier`。

use std::any::Any;
use std::collections::HashSet;
use std::fmt;

use crate::code::code_util;
use crate::meta::java_kind::JavaKind;
use crate::meta::java_value::JavaValue;
use crate::meta::resolved_java_field::ResolvedJavaField;
use crate::meta::resolved_java_type::ResolvedJavaType;

/// 对应 `VirtualObject.LayoutVerifier`。
pub trait LayoutVerifier {
    /// 对应 `getOffset(ResolvedJavaField)`。
    fn get_offset(&self, field: &dyn ResolvedJavaField) -> i32;

    /// 对应 `getStorageKind(ResolvedJavaField)`（默认 `field.getType().getJavaKind()`）。
    fn get_storage_kind(&self, field: &dyn ResolvedJavaField) -> JavaKind {
        field.get_type().get_java_kind()
    }
}

/// 对应 `final class VirtualObject implements JavaValue`。
pub struct VirtualObject {
    type_: Box<dyn ResolvedJavaType>,
    values: Vec<Box<dyn JavaValue>>,
    slot_kinds: Vec<JavaKind>,
    id: i32,
    is_auto_box: bool,
}

impl VirtualObject {
    /// 对应 `static VirtualObject get(ResolvedJavaType, int)`。
    pub fn get(type_: Box<dyn ResolvedJavaType>, id: i32) -> Self {
        Self::new_inner(type_, id, false)
    }

    /// 对应 `static VirtualObject get(ResolvedJavaType, int, boolean)`。
    pub fn get_with_auto_box(type_: Box<dyn ResolvedJavaType>, id: i32, is_auto_box: bool) -> Self {
        Self::new_inner(type_, id, is_auto_box)
    }

    fn new_inner(type_: Box<dyn ResolvedJavaType>, id: i32, is_auto_box: bool) -> Self {
        Self {
            type_,
            values: Vec::new(),
            slot_kinds: Vec::new(),
            id,
            is_auto_box,
        }
    }

    /// 对应 `verifyLayout(LayoutVerifier)`。
    pub fn verify_layout(&self, verifier: &dyn LayoutVerifier) {
        if !self.type_.is_array() {
            let fields = self.type_.get_instance_fields(true);
            let mut field_index = 0usize;
            let mut i = 0usize;
            while i < self.values.len() {
                let slot_kind = self.slot_kinds[i];
                if field_index >= fields.len() {
                    panic!(
                        "JVMCIError: Not enough fields for the values provided for {}",
                        self
                    );
                }
                let field = &fields[field_index];
                let field_kind = verifier.get_storage_kind(field.as_ref());
                if slot_kind.get_slot_count() == 2 && field_kind == JavaKind::Int {
                    let offset = verifier.get_offset(field.as_ref());
                    if offset % 8 != 0 {
                        panic!(
                            "JVMCIError: Double word value stored across two ints must be aligned {}",
                            self
                        );
                    }
                    if field_index + 1 >= fields.len() {
                        panic!(
                            "JVMCIError: Missing second field for double word value stored in two ints {}",
                            self
                        );
                    }
                    let field2 = &fields[field_index + 1];
                    if field2.get_type().get_java_kind() != JavaKind::Int {
                        panic!(
                            "JVMCIError: Second field for double word value stored in two ints must be int but got {:?} in {}",
                            field2.get_type().get_java_kind(),
                            self
                        );
                    }
                    let offset2 = verifier.get_offset(field2.as_ref());
                    if offset + 4 != offset2 {
                        panic!(
                            "JVMCIError: Double word value stored across two ints must be sequential {}",
                            self
                        );
                    }
                    field_index += 1;
                } else if field_kind.get_stack_kind() != slot_kind.get_stack_kind() {
                    panic!(
                        "JVMCIError: Expected value of kind {:?} but got {:?} for field {} in {}",
                        field_kind,
                        slot_kind,
                        field.get_name(),
                        self
                    );
                }
                i += 1;
                field_index += 1;
            }
            if field_index < fields.len() {
                panic!(
                    "JVMCIError: Not enough values provided for fields in {}",
                    self
                );
            }
        } else if self
            .type_
            .get_component_type()
            .map_or(JavaKind::Object, |c| c.get_java_kind())
            == JavaKind::Byte
        {
            let mut i = 0usize;
            while i < self.values.len() {
                let slotkind = self.slot_kinds[i];
                if slotkind != JavaKind::Byte {
                    if !slotkind.is_primitive() {
                        panic!(
                            "JVMCIError: Storing a non-primitive in a byte array: {:?} {}",
                            slotkind, self
                        );
                    }
                    let mut byte_count = 1;
                    i += 1;
                    while i < self.values.len() && self.slot_kinds[i] == JavaKind::Illegal {
                        byte_count += 1;
                        i += 1;
                    }
                    if !code_util::is_power_of2_i32(byte_count)
                        || (slotkind.get_stack_kind() != JavaKind::Int
                            && byte_count != slotkind.get_byte_count())
                        || byte_count > JavaKind::Long.get_byte_count()
                    {
                        panic!(
                            "JVMCIError: Invalid number of illegals to reconstruct a byte array: {} in {}",
                            byte_count, self
                        );
                    }
                    continue;
                }
                i += 1;
            }
        }
    }

    /// 对应 `getType()`。
    pub fn get_type(&self) -> &dyn ResolvedJavaType {
        self.type_.as_ref()
    }

    /// 对应 `getValues()`。
    pub fn get_values(&self) -> &[Box<dyn JavaValue>] {
        &self.values
    }

    /// 对应 `getSlotKind(int)`。
    pub fn get_slot_kind(&self, index: usize) -> JavaKind {
        self.slot_kinds[index]
    }

    /// 对应 `getId()`。
    pub fn get_id(&self) -> i32 {
        self.id
    }

    /// 对应 `isAutoBox()`。
    pub fn is_auto_box(&self) -> bool {
        self.is_auto_box
    }

    /// 对应 `setValues(JavaValue[], JavaKind[])`。
    pub fn set_values(&mut self, values: Vec<Box<dyn JavaValue>>, slot_kinds: Vec<JavaKind>) {
        debug_assert_eq!(values.len(), slot_kinds.len());
        self.values = values;
        self.slot_kinds = slot_kinds;
    }
}

impl fmt::Debug for VirtualObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("VirtualObject")
            .field("id", &self.id)
            .field("is_auto_box", &self.is_auto_box)
            .field("values_len", &self.values.len())
            .finish()
    }
}

impl fmt::Display for VirtualObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut visited: HashSet<*const ()> = HashSet::new();
        append_value(
            f,
            self as *const VirtualObject as *const (),
            self,
            &mut visited,
        )
    }
}

impl JavaValue for VirtualObject {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// 对应 `VirtualObject.appendValue(StringBuilder, JavaValue, Set<VirtualObject>)`。
fn append_value(
    f: &mut fmt::Formatter<'_>,
    self_ptr: *const (),
    vo: &VirtualObject,
    visited: &mut HashSet<*const ()>,
) -> fmt::Result {
    write!(
        f,
        "vobject:{}:{}",
        vo.type_.to_java_name_qualified(false),
        vo.id
    )?;
    if visited.insert(self_ptr) {
        f.write_str("{")?;
        if vo.values.is_empty() {
            f.write_str("<uninitialized>")?;
        } else if vo.type_.is_array() {
            for (i, v) in vo.values.iter().enumerate() {
                if i != 0 {
                    f.write_str(",")?;
                }
                write!(f, "{}=", i)?;
                append_value_any(f, v.as_ref(), visited)?;
            }
        } else {
            let fields = vo.type_.get_instance_fields(true);
            let mut field_index = 0usize;
            for (i, v) in vo.values.iter().enumerate() {
                if i != 0 {
                    f.write_str(",")?;
                }
                if field_index >= fields.len() {
                    f.write_str("<missing field>")?;
                } else {
                    let field = &fields[field_index];
                    write!(f, "{}", field.get_name())?;
                    if vo.slot_kinds[i].get_slot_count() == 2
                        && field.get_type().get_java_kind().get_slot_count() == 1
                    {
                        if field_index + 1 >= fields.len() {
                            f.write_str("/<missing field>")?;
                        } else {
                            field_index += 1;
                            let field2 = &fields[field_index];
                            write!(f, "/{}", field2.get_name())?;
                        }
                    }
                }
                f.write_str("=")?;
                append_value_any(f, v.as_ref(), visited)?;
                field_index += 1;
            }
            while field_index < fields.len() {
                write!(f, ",{}=<missing value>", fields[field_index].get_name())?;
                field_index += 1;
            }
        }
        f.write_str("}")?;
    }
    Ok(())
}

/// 对应 `appendValue` 中非 `VirtualObject` 分支：`buf.append(value)`。
fn append_value_any(
    f: &mut fmt::Formatter<'_>,
    value: &dyn JavaValue,
    visited: &mut HashSet<*const ()>,
) -> fmt::Result {
    if let Some(vo) = value.as_any().downcast_ref::<VirtualObject>() {
        append_value(f, vo as *const VirtualObject as *const (), vo, visited)
    } else {
        write!(f, "{:?}", value)
    }
}
