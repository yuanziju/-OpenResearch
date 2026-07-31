// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2013, Oracle and/or its affiliates. All rights reserved.
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

//! 镜像 `jdk.vm.ci.meta.JavaMethodProfile`：某 BCI 处的方法 profile。
//!
//! 偏离记录：
//! - Java `final class JavaMethodProfile extends AbstractJavaProfile<ProfiledMethod, ResolvedJavaMethod>`
//!   → Rust `pub struct JavaMethodProfile` 以组合持有 `AbstractJavaProfile<ProfiledMethod,
//!   dyn ResolvedJavaMethod>`，方法委托对齐 Java 继承语义。
//! - Java `ProfiledMethod` 持 `ResolvedJavaMethod` 引用；Rust 侧持 `Box<dyn ResolvedJavaMethod>`
//!   （所有权转入；T7 改共享指针对齐引用语义）。
//! - `AbstractProfiledItem<U>` 中 `U` 对应 Java 泛型 `ResolvedJavaMethod`，Rust 侧为
//!   `dyn ResolvedJavaMethod`（unsized），`item()` 返回 `&dyn ResolvedJavaMethod`。

use std::fmt;
use std::hash::Hasher;

use crate::meta::abstract_java_profile::{AbstractJavaProfile, AbstractProfiledItem};
use crate::meta::resolved_java_method::ResolvedJavaMethod;

/// 对应 `public static class JavaMethodProfile.ProfiledMethod extends AbstractProfiledItem<ResolvedJavaMethod>`。
pub struct ProfiledMethod {
    method: Box<dyn ResolvedJavaMethod>,
    probability: f64,
}

impl ProfiledMethod {
    /// 对应 `ProfiledMethod(ResolvedJavaMethod method, double probability)`。
    pub fn new(method: Box<dyn ResolvedJavaMethod>, probability: f64) -> Self {
        Self {
            method,
            probability,
        }
    }

    /// 对应 `ResolvedJavaMethod getMethod()`：返回 `getItem()`。
    pub fn get_method(&self) -> &dyn ResolvedJavaMethod {
        self.method.as_ref()
    }
}

impl AbstractProfiledItem<dyn ResolvedJavaMethod> for ProfiledMethod {
    // `+ 'static`：trait 类型参数位 `dyn ResolvedJavaMethod` 默认 `'static`，impl 侧返回
    // 类型须显式标注对齐（`Box<dyn ResolvedJavaMethod>` 即 `+ 'static`）。
    fn item(&self) -> &(dyn ResolvedJavaMethod + 'static) {
        self.method.as_ref()
    }

    fn probability(&self) -> f64 {
        self.probability
    }
}

impl fmt::Display for ProfiledMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 对应 `toString()`：`"{" + item.getName() + ", " + probability + "}"`。
        write!(f, "{{{}, {}}}", self.method.get_name(), self.probability)
    }
}

impl fmt::Debug for ProfiledMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 对齐 `toString()` 输出。
        write!(f, "{{{}, {}}}", self.method.get_name(), self.probability)
    }
}

/// 对应 `public final class JavaMethodProfile extends AbstractJavaProfile<ProfiledMethod, ResolvedJavaMethod>`。
pub struct JavaMethodProfile {
    inner: AbstractJavaProfile<ProfiledMethod, dyn ResolvedJavaMethod>,
}

impl JavaMethodProfile {
    /// 对应 `JavaMethodProfile(double notRecordedProbability, ProfiledMethod[] pitems)`。
    pub fn new(not_recorded_probability: f64, pitems: Vec<ProfiledMethod>) -> Self {
        Self {
            inner: AbstractJavaProfile::new(not_recorded_probability, pitems),
        }
    }

    /// 对应 `ProfiledMethod[] getMethods()`：委托 `super.getItems()`。
    pub fn get_methods(&self) -> &[ProfiledMethod] {
        self.inner.get_items()
    }

    /// 对应 `AbstractJavaProfile.getNotRecordedProbability()`。
    pub fn get_not_recorded_probability(&self) -> f64 {
        self.inner.get_not_recorded_probability()
    }

    /// 对应 `AbstractJavaProfile.findEntry(ResolvedJavaMethod)`。
    pub fn find_entry(
        &self,
        method: &(dyn ResolvedJavaMethod + 'static),
    ) -> Option<&ProfiledMethod> {
        self.inner.find_entry(method)
    }

    /// 对应 `AbstractJavaProfile.isIncluded(ResolvedJavaMethod)`。
    pub fn is_included(&self, method: &(dyn ResolvedJavaMethod + 'static)) -> bool {
        self.inner.is_included(method)
    }

    /// 对应 `AbstractJavaProfile.toString()`。
    pub fn profile_to_string(&self) -> String {
        self.inner.profile_to_string()
    }

    /// 对应 `AbstractJavaProfile.equals(Object)`：Java 入参为 `Object`，Rust 限定同型。
    pub fn profile_eq(&self, other: &JavaMethodProfile) -> bool {
        self.inner.profile_eq(&other.inner)
    }

    /// 对应 `AbstractJavaProfile.hashCode()`。
    pub fn profile_hash<H: Hasher>(&self, state: &mut H) {
        self.inner.profile_hash(state)
    }
}

impl fmt::Display for JavaMethodProfile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 对应 `toString()`：继承自 `AbstractJavaProfile.toString()`。
        f.write_str(&self.inner.profile_to_string())
    }
}

impl fmt::Debug for JavaMethodProfile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("JavaMethodProfile")
            .field(
                "notRecordedProbability",
                &self.inner.get_not_recorded_probability(),
            )
            .field("pitems", &self.inner.get_items())
            .finish()
    }
}
