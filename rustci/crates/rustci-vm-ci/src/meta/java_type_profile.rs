// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2011, 2019, Oracle and/or its affiliates. All rights reserved.
 * DO NOT ALTER OR REMOVE COPYRIGHT NOTICES IN THIS FILE HEADER.
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
 * Please contact Oracle, 500 Oracle Parkway, Redwood Shores, CA 94065 USA
 * or visit Oracle if you need additional information or have any
 * questions.
 */

//! 镜像 `jdk.vm.ci.meta.JavaTypeProfile`：某 BCI 处的类型 profile。
//!
//! 偏离记录：
//! - Java `final class JavaTypeProfile extends AbstractJavaProfile<ProfiledType, ResolvedJavaType>`
//!   → Rust `pub struct JavaTypeProfile` 以组合持有 `AbstractJavaProfile<ProfiledType,
//!   dyn ResolvedJavaType>` 与 `null_seen: TriState` 字段，方法委托对齐 Java 继承语义。
//! - Java `ProfiledType` 持 `ResolvedJavaType` 引用；Rust 侧持 `Box<dyn ResolvedJavaType>`
//!   （所有权转入；T7 改共享指针对齐引用语义）。
//! - `AbstractProfiledItem<U>` 中 `U` 对应 Java 泛型 `ResolvedJavaType`，Rust 侧为
//!   `dyn ResolvedJavaType`（unsized），`item()` 返回 `&dyn ResolvedJavaType`。
//! - Java `restrict(JavaTypeProfile)`/`restrict(ResolvedJavaType, boolean)` 返回
//!   `JavaTypeProfile`（引用类型，可为 `this`/`otherProfile`/新对象）。Rust 侧 `Box<dyn
//!   ResolvedJavaType>` 不可克隆（需 T7 共享指针），无法在"返回 this/other"路径构造等价
//!   owned 值，故返回 `Option<JavaTypeProfile>`：`None` 表示不变（对齐 Java `return this`），
//!   `Some(new)` 表示新 profile（对齐 Java `return new JavaTypeProfile(...)`）。Java
//!   `return otherProfile` 路径需重建 other（克隆其 pitems），T7 前该路径 `panic!` 标注。
//! - `createAdjustedProfile` 重建路径（非空结果 + 概率调整）需克隆 `ResolvedJavaType`，
//!   T7 前 `panic!` 标注（同 `Assumptions.clone_type_box` 偏离）。不变路径（`return this`）
//!   返回 `None`，空结果路径返回 `Some(空 profile)`，均无需克隆。
//! - Java `asSingleType()` 返回 nullable `ResolvedJavaType` → Rust `Option<&dyn ResolvedJavaType>`
//!   （借用引用，避免克隆；对齐 `get_component_type_resolved` 等既有 nullable 返回模式）。
//! - Java `equals`/`hashCode` 覆写：`super.equals`/`super.hashCode` + `nullSeen` → Rust
//!   `profile_eq`/`profile_hash` 委托 `inner` 后追加 `null_seen`。
//! - `ProfiledType.toString`/`JavaTypeProfile.toString` 中 `%s` 调 `item.toString()`；
//!   `dyn ResolvedJavaType` 无 `Display`，改用 `to_java_name()` 作可读近似（同
//!   `ExceptionHandler` 偏离）。

use std::fmt;
use std::hash::{Hash, Hasher};

use crate::meta::abstract_java_profile::{AbstractJavaProfile, AbstractProfiledItem};
use crate::meta::resolved_java_type::ResolvedJavaType;
use crate::meta::tri_state::TriState;

/// 对应 `public static class JavaTypeProfile.ProfiledType extends AbstractProfiledItem<ResolvedJavaType>`。
pub struct ProfiledType {
    r#type: Box<dyn ResolvedJavaType>,
    probability: f64,
}

impl ProfiledType {
    /// 对应 `ProfiledType(ResolvedJavaType type, double probability)`。
    pub fn new(r#type: Box<dyn ResolvedJavaType>, probability: f64) -> Self {
        // 对应 Java `assert type.isArray() || type.isConcrete()`。
        // `is_concrete` 在 `ModifiersProvider`（默认）与 `ResolvedJavaType`（覆写）中均有，
        // 显式消歧取 `ResolvedJavaType` 覆写版（对齐 Java 协变覆写语义，同
        // `ResolvedJavaType::is_java_lang_object` 中 `is_interface` 消歧）。
        debug_assert!(r#type.is_array() || ResolvedJavaType::is_concrete(r#type.as_ref()));
        Self {
            r#type,
            probability,
        }
    }

    /// 对应 `ResolvedJavaType getType()`：返回 `getItem()`。
    pub fn get_type(&self) -> &dyn ResolvedJavaType {
        self.r#type.as_ref()
    }
}

impl AbstractProfiledItem<dyn ResolvedJavaType> for ProfiledType {
    // `+ 'static`：trait 类型参数位 `dyn ResolvedJavaType` 默认 `'static`，impl 侧返回
    // 类型须显式标注对齐（`Box<dyn ResolvedJavaType>` 即 `+ 'static`）。
    fn item(&self) -> &(dyn ResolvedJavaType + 'static) {
        self.r#type.as_ref()
    }

    fn probability(&self) -> f64 {
        self.probability
    }
}

impl fmt::Display for ProfiledType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 对应 `toString()`：`String.format("%.6f#%s", probability, item)`。
        // `item.toString()` 以 `to_java_name()` 近似（偏离）。
        write!(f, "{:.6}#{}", self.probability, self.r#type.to_java_name())
    }
}

impl fmt::Debug for ProfiledType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.6}#{}", self.probability, self.r#type.to_java_name())
    }
}

/// 对应 `public final class JavaTypeProfile extends AbstractJavaProfile<ProfiledType, ResolvedJavaType>`。
pub struct JavaTypeProfile {
    inner: AbstractJavaProfile<ProfiledType, dyn ResolvedJavaType>,
    null_seen: TriState,
}

impl JavaTypeProfile {
    /// 对应 `JavaTypeProfile(TriState nullSeen, double notRecordedProbability, ProfiledType[] pitems)`。
    pub fn new(
        null_seen: TriState,
        not_recorded_probability: f64,
        pitems: Vec<ProfiledType>,
    ) -> Self {
        Self {
            inner: AbstractJavaProfile::new(not_recorded_probability, pitems),
            null_seen,
        }
    }

    /// 对应 `TriState getNullSeen()`。
    pub fn get_null_seen(&self) -> TriState {
        self.null_seen
    }

    /// 对应 `ProfiledType[] getTypes()`：委托 `getItems()`。
    pub fn get_types(&self) -> &[ProfiledType] {
        self.inner.get_items()
    }

    /// 对应 `AbstractJavaProfile.getNotRecordedProbability()`。
    pub fn get_not_recorded_probability(&self) -> f64 {
        self.inner.get_not_recorded_probability()
    }

    /// 对应 `AbstractJavaProfile.findEntry(ResolvedJavaType)`。
    pub fn find_entry(&self, type_: &(dyn ResolvedJavaType + 'static)) -> Option<&ProfiledType> {
        self.inner.find_entry(type_)
    }

    /// 对应 `AbstractJavaProfile.isIncluded(ResolvedJavaType)`。
    pub fn is_included(&self, type_: &(dyn ResolvedJavaType + 'static)) -> bool {
        self.inner.is_included(type_)
    }

    /// 对应 `boolean allTypesRecorded()`。
    pub fn all_types_recorded(&self) -> bool {
        self.get_not_recorded_probability() == 0.0
    }

    /// 对应 `ResolvedJavaType asSingleType()`：返回 `Option` 对齐 Java nullable。
    /// 返回借用引用（偏离：Java 返回引用类型；Rust 避免克隆）。
    pub fn as_single_type(&self) -> Option<&dyn ResolvedJavaType> {
        if self.all_types_recorded() && self.get_types().len() == 1 {
            Some(self.get_types()[0].get_type())
        } else {
            None
        }
    }

    /// 对应 `JavaTypeProfile restrict(JavaTypeProfile otherProfile)`。
    ///
    /// 偏离：返回 `Option<JavaTypeProfile>`（`None` = 不变/`return this`，`Some` = 新 profile）。
    /// Java `return otherProfile` 路径需克隆 other 的 pitems，T7 前 `panic!` 标注。
    pub fn restrict(&self, other: &JavaTypeProfile) -> Option<JavaTypeProfile> {
        if other.get_not_recorded_probability() > 0.0 {
            // 对应 `return this;`。
            return None;
        }
        if self.get_not_recorded_probability() > 0.0 {
            // 对应 `return otherProfile;`：需重建 other（克隆其 pitems），T7 共享指针前不可行。
            panic!("restrict: return otherProfile requires shared-pointer support (T7)");
        }
        // 过滤：仅保留 otherProfile.isIncluded(type) 的项。
        let pitems = self.inner.get_items();
        let result: Vec<&ProfiledType> = pitems
            .iter()
            .filter(|pt| other.is_included(pt.item()))
            .collect();
        let new_null_seen = if other.get_null_seen() == TriState::False {
            TriState::False
        } else {
            self.null_seen
        };
        let new_not_recorded = self.get_not_recorded_probability();
        self.create_adjusted_profile(&result, new_null_seen, new_not_recorded)
    }

    /// 对应 `JavaTypeProfile restrict(ResolvedJavaType declaredType, boolean nonNull)`。
    ///
    /// 偏离：返回 `Option<JavaTypeProfile>`（`None` = 不变/`return this`，`Some` = 新 profile）。
    pub fn restrict_with_declared(
        &self,
        declared_type: &dyn ResolvedJavaType,
        non_null: bool,
    ) -> Option<JavaTypeProfile> {
        let pitems = self.inner.get_items();
        let result: Vec<&ProfiledType> = pitems
            .iter()
            .filter(|pt| declared_type.is_assignable_from(pt.item()))
            .collect();
        let new_null_seen = if non_null {
            TriState::False
        } else {
            self.null_seen
        };
        let mut new_not_recorded = self.get_not_recorded_probability();
        // 对应 `if (getItems().length != 0) { newNotRecorded *= ((double) result.size() / (double) getItems().length); }`。
        if !pitems.is_empty() {
            new_not_recorded *= result.len() as f64 / pitems.len() as f64;
        }
        self.create_adjusted_profile(&result, new_null_seen, new_not_recorded)
    }

    /// 对应 `private JavaTypeProfile createAdjustedProfile(ArrayList<ProfiledType> result, TriState newNullSeen, double newNotRecorded)`。
    ///
    /// 偏离：返回 `Option<JavaTypeProfile>`（`None` = 不变/`return this`，`Some` = 新 profile）。
    /// 重建路径（非空结果 + 概率调整）需克隆 `ResolvedJavaType`，T7 前 `panic!` 标注。
    fn create_adjusted_profile(
        &self,
        result: &[&ProfiledType],
        new_null_seen: TriState,
        new_not_recorded: f64,
    ) -> Option<JavaTypeProfile> {
        let pitems = self.inner.get_items();
        let unchanged = result.len() == pitems.len()
            && new_not_recorded == self.get_not_recorded_probability()
            && new_null_seen == self.null_seen;
        if !unchanged {
            if result.is_empty() {
                // 对应 `return new JavaTypeProfile(newNullSeen, 1.0, EMPTY_ARRAY);`。
                return Some(JavaTypeProfile::new(new_null_seen, 1.0, Vec::new()));
            }
            // 重建路径：需为每个 result 项创建新 ProfiledType（克隆 type），T7 共享指针前不可行。
            panic!("createAdjustedProfile: rebuild requires shared-pointer support (T7)");
        }
        // 对应 `return this;`。
        None
    }

    /// 对应 `equals(Object other)`：`super.equals(other) && nullSeen.equals(other.nullSeen)`。
    pub fn profile_eq(&self, other: &JavaTypeProfile) -> bool {
        self.inner.profile_eq(&other.inner) && self.null_seen == other.null_seen
    }

    /// 对应 `hashCode()`：`nullSeen.hashCode() + super.hashCode()`。
    pub fn profile_hash<H: Hasher>(&self, state: &mut H) {
        self.null_seen.hash(state);
        self.inner.profile_hash(state);
    }
}

impl fmt::Display for JavaTypeProfile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 对应 `toString()`。
        f.write_str("JavaTypeProfile<nullSeen=")?;
        f.write_str(self.null_seen.name())?;
        f.write_str(", types=[")?;
        let types = self.get_types();
        for (j, ptype) in types.iter().enumerate() {
            if j != 0 {
                f.write_str(", ")?;
            }
            // 对应 `String.format("%.6f:%s", ptype.getProbability(), ptype.getType())`。
            write!(
                f,
                "{:.6}:{}",
                ptype.probability(),
                ptype.get_type().to_java_name()
            )?;
        }
        write!(
            f,
            "], notRecorded:{:.6}>",
            self.get_not_recorded_probability()
        )
    }
}

impl fmt::Debug for JavaTypeProfile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("JavaTypeProfile")
            .field("nullSeen", &self.null_seen)
            .field(
                "notRecordedProbability",
                &self.inner.get_not_recorded_probability(),
            )
            .field("pitems", &self.inner.get_items())
            .finish()
    }
}
