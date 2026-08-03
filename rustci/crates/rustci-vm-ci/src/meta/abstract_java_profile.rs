// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2013, Oracle and/or its affiliates. All rights reserved.
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
 * or visit www.oracle.com if you need additional information or have any
 * questions.
 */

//! 镜像 `jdk.vm.ci.meta.AbstractProfiledItem` 与 `AbstractJavaProfile`。
//!
//! 偏离记录：
//! - Java `AbstractProfiledItem<T>` 为抽象类（含 `item`/`probability` 字段与
//!   `compareTo`/`equals`/`hashCode`，`toString` 抽象）。Rust 侧以 trait
//!   `AbstractProfiledItem<U>` 镜像契约（`item`/`probability` 由实现者持有），
//!   `compare_to`/`profiled_equals`/`profiled_hash` 为默认方法，`Display` 对应
//!   Java 的 `toString` 抽象方法（由叶子类实现）。
//! - Java `equals`/`hashCode` 基于 `item.equals`/`item.hashCode`。Rust 侧 item 为
//!   trait 对象（`Rc<dyn ResolvedJavaType>`/`Rc<dyn ResolvedJavaMethod>`），无法直接
//!   调值等于；按指针地址比较/哈希，对齐 HotSpot 侧 `ResolvedJavaType.equals` 的引用
//!   相等语义。值相等绑定延至 T7。
//! - Java `findEntry(ResolvedJavaType)` 硬编码 `ResolvedJavaType` 参数（对
//!   `JavaMethodProfile` 语义上无效）。Rust 侧改用泛型 `&U` 以保持类型一致，方法集与
//!   行为对 `JavaTypeProfile` 忠实，`JavaMethodProfile` 路径同 Java 不会命中。
//! - Java `Comparable<AbstractProfiledItem<?>>` → Rust `compare_to(&dyn AbstractProfiledItem<U>)`。

use std::cmp::Ordering;
use std::fmt::Display;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

/// 镜像 `AbstractProfiledItem<T>`：带概率的 profiled 项，自然序为概率降序。
pub trait AbstractProfiledItem<U: ?Sized>: Display {
    fn item(&self) -> &U;
    fn probability(&self) -> f64;

    /// 对应 `AbstractProfiledItem.compareTo`：概率大者在前（操作数互换）。
    fn compare_to(&self, other: &dyn AbstractProfiledItem<U>) -> Ordering {
        let a = other.probability();
        let b = self.probability();
        a.partial_cmp(&b).unwrap_or(Ordering::Equal)
    }

    /// 对应 `AbstractProfiledItem.equals`：概率位级相等且 item 引用相等。
    fn profiled_equals(&self, other: &dyn AbstractProfiledItem<U>) -> bool {
        if self.probability().to_bits() != other.probability().to_bits() {
            return false;
        }
        item_ptr_eq(self.item(), other.item())
    }

    /// 对应 `AbstractProfiledItem.hashCode`。
    fn profiled_hash(&self, state: &mut dyn Hasher) {
        let temp = self.probability().to_bits();
        let prime = 31i64;
        let mut result: i64 = 1;
        result = prime.wrapping_mul(result).wrapping_add(temp as i64);
        let p = item_ptr_addr(self.item());
        result = prime.wrapping_mul(result).wrapping_add(p as i64);
        // 对齐 `(result as u64).hash(state)`：u64 的 Hash 写 8 字节小端。
        state.write_u64(result as u64);
    }
}

/// 比较两个 trait 对象引用是否指向同一对象（指针相等），对应 Java `==`/HotSpot 引用相等。
fn item_ptr_eq<U: ?Sized>(a: &U, b: &U) -> bool {
    let pa = a as *const U as *const ();
    let pb = b as *const U as *const ();
    std::ptr::eq(pa, pb)
}

fn item_ptr_addr<U: ?Sized>(a: &U) -> usize {
    a as *const U as *const () as usize
}

/// 镜像 `AbstractJavaProfile<T extends AbstractProfiledItem<U>, U>`：
/// 在某 BCI 处一组 profiled 项的概率容器。
pub struct AbstractJavaProfile<T, U: ?Sized> {
    not_recorded_probability: f64,
    pitems: Vec<T>,
    _u: PhantomData<U>,
}

impl<T: Clone, U: ?Sized> Clone for AbstractJavaProfile<T, U> {
    fn clone(&self) -> Self {
        Self {
            not_recorded_probability: self.not_recorded_probability,
            pitems: self.pitems.clone(),
            _u: PhantomData,
        }
    }
}

impl<T: AbstractProfiledItem<U>, U: ?Sized> AbstractJavaProfile<T, U> {
    /// 对应 `AbstractJavaProfile(double, T[])` 构造。
    pub fn new(not_recorded_probability: f64, pitems: Vec<T>) -> Self {
        debug_assert!(!not_recorded_probability.is_nan());
        let me = Self {
            not_recorded_probability,
            pitems,
            _u: PhantomData,
        };
        debug_assert!(me.is_sorted());
        let total = me.total_probability();
        debug_assert!((0.0..=1.0001).contains(&total), "{} {}", total, "ok");
        me
    }

    fn total_probability(&self) -> f64 {
        let mut total = self.not_recorded_probability;
        for item in &self.pitems {
            total += item.probability();
        }
        total
    }

    fn is_sorted(&self) -> bool {
        for i in 1..self.pitems.len() {
            if self.pitems[i - 1].probability() < self.pitems[i].probability() {
                return false;
            }
        }
        true
    }

    /// 未记录类型/方法的估计概率。
    pub fn get_not_recorded_probability(&self) -> f64 {
        self.not_recorded_probability
    }

    /// 内部 profiled 项切片（对应 `getItems()`，Java 返回数组）。
    pub fn get_items(&self) -> &[T] {
        &self.pitems
    }

    /// 对应 `findEntry`：按 item 引用相等查找。
    pub fn find_entry(&self, type_: &U) -> Option<&T> {
        self.pitems.iter().find(|pt| item_ptr_eq(pt.item(), type_))
    }

    /// 对应 `AbstractJavaProfile.toString`。
    pub fn profile_to_string(&self) -> String {
        let mut builder = String::new();
        builder.push_str(std::any::type_name::<Self>());
        builder.push('[');
        for pt in &self.pitems {
            builder.push_str(&pt.to_string());
            builder.push_str(", ");
        }
        builder.push_str(&self.not_recorded_probability.to_string());
        builder.push(']');
        builder
    }

    /// 对应 `isIncluded(U)`：未记录概率 > 0 时视为包含，否则按 item 引用相等判断。
    pub fn is_included(&self, item: &U) -> bool {
        if self.not_recorded_probability > 0.0 {
            return true;
        }
        for pitem in &self.pitems {
            if item_ptr_eq(pitem.item(), item) {
                return true;
            }
        }
        false
    }

    /// 对应 `equals`。
    pub fn profile_eq(&self, other: &AbstractJavaProfile<T, U>) -> bool {
        if self.not_recorded_probability != other.not_recorded_probability {
            return false;
        }
        if self.pitems.len() != other.pitems.len() {
            return false;
        }
        for (a, b) in self.pitems.iter().zip(other.pitems.iter()) {
            if !a.profiled_equals(b as &dyn AbstractProfiledItem<U>) {
                return false;
            }
        }
        true
    }

    /// 对应 `hashCode`。
    pub fn profile_hash<H: Hasher>(&self, state: &mut H) {
        let nrl = self.not_recorded_probability.to_bits() as i64;
        let h = nrl.wrapping_add((self.pitems.len() as i64) * 13);
        (h as u64).hash(state);
    }
}
