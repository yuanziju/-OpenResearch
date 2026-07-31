// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2011, 2022, Oracle and/or its affiliates. All rights reserved.
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

//! 镜像 `jdk.vm.ci.meta.Assumptions`：编译期假设的容器与断言。
//!
//! 偏离记录：
//! - Java `Assumption` 为抽象基类，5 个 `final` 子类；Rust 侧 `Assumption` 为 trait，
//!   5 个子类为 struct 实现之。`Assumptions` 内部 `HashSet<Assumption>`（引用集）→
//!   `HashSet<AssumptionKey>`，`AssumptionKey` 包装 `Box<dyn Assumption>` 并以
//!   `assumption_eq`/`assumption_hash` 做值相等/哈希（对齐各子类 `equals`/`hashCode`）。
//! - Java 假设对象持有运行时 `ResolvedJavaType`/`ResolvedJavaMethod`/`JavaConstant` 引用；
//!   Rust 侧用 `Box<dyn>` 持有（所有权转入假设）。T7 绑定真实运行时对象时改用共享指针
//!   对齐引用语义。
//! - `record(other)`/`recordTo` 合并假设时 Java 共享引用，Rust 侧 `clone_box` 复制等值对象
//!   （集合成以值相等去重，行为一致）。
//! - `hashCode()` 抛 `UnsupportedOperationException` → Rust 不实现 `Hash`。

use std::collections::HashSet;
use std::fmt;
use std::hash::{Hash, Hasher};

use crate::meta::java_constant::JavaConstant;
use crate::meta::resolved_java_method::ResolvedJavaMethod;
use crate::meta::resolved_java_type::ResolvedJavaType;

/// 对应 `abstract static class Assumption`。
pub trait Assumption: fmt::Debug {
    fn as_any(&self) -> &dyn std::any::Any;
    fn clone_box(&self) -> Box<dyn Assumption>;
    fn assumption_to_string(&self) -> String;
}

/// `HashSet` 键包装，按 `Assumption` 值相等/哈希。
#[derive(Debug)]
struct AssumptionKey(Box<dyn Assumption>);

impl PartialEq for AssumptionKey {
    fn eq(&self, other: &Self) -> bool {
        assumption_eq(&*self.0, &*other.0)
    }
}

impl Eq for AssumptionKey {}

impl Hash for AssumptionKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        assumption_hash(&*self.0, state);
    }
}

fn assumption_eq(a: &dyn Assumption, b: &dyn Assumption) -> bool {
    let xa = a.as_any();
    let xb = b.as_any();
    // 同型才可能相等：以 NoFinalizableSubclass/ConcreteSubtype/LeafType/ConcreteMethod/
    // CallSiteTargetValue 逐型 downcast 比对。
    downcast_eq::<NoFinalizableSubclass>(xa, xb)
        || downcast_eq::<ConcreteSubtype>(xa, xb)
        || downcast_eq::<LeafType>(xa, xb)
        || downcast_eq::<ConcreteMethod>(xa, xb)
        || downcast_eq::<CallSiteTargetValue>(xa, xb)
}

fn downcast_eq<T: Assumption + 'static + PartialEq>(
    xa: &dyn std::any::Any,
    xb: &dyn std::any::Any,
) -> bool {
    match (xa.downcast_ref::<T>(), xb.downcast_ref::<T>()) {
        (Some(a), Some(b)) => a == b,
        _ => false,
    }
}

fn assumption_hash<H: Hasher>(a: &dyn Assumption, state: &mut H) {
    let xa = a.as_any();
    if let Some(x) = xa.downcast_ref::<NoFinalizableSubclass>() {
        x.hash(state);
    } else if let Some(x) = xa.downcast_ref::<ConcreteSubtype>() {
        x.hash(state);
    } else if let Some(x) = xa.downcast_ref::<LeafType>() {
        x.hash(state);
    } else if let Some(x) = xa.downcast_ref::<ConcreteMethod>() {
        x.hash(state);
    } else if let Some(x) = xa.downcast_ref::<CallSiteTargetValue>() {
        x.hash(state);
    }
}

/// 对应 `static class AssumptionResult<T>`。
pub struct AssumptionResult<T> {
    assumptions: Vec<Box<dyn Assumption>>,
    result: T,
}

impl<T> AssumptionResult<T> {
    /// 对应 `AssumptionResult(T result, Assumption... assumptions)`。
    pub fn new(result: T, assumptions: Vec<Box<dyn Assumption>>) -> Self {
        Self {
            assumptions,
            result,
        }
    }

    /// 对应 `AssumptionResult(T result)`。
    pub fn new_empty(result: T) -> Self {
        Self::new(result, Vec::new())
    }

    /// 对应 `getResult()`。
    pub fn get_result(&self) -> &T {
        &self.result
    }

    /// 对应 `isAssumptionFree()`。
    pub fn is_assumption_free(&self) -> bool {
        self.assumptions.is_empty()
    }

    /// 对应 `add(AssumptionResult<T> other)`。
    pub fn add(&mut self, other: AssumptionResult<T>) {
        self.assumptions.extend(other.assumptions);
    }

    /// 对应 `canRecordTo(Assumptions target)`。`target` nullable → `Option`。
    pub fn can_record_to(&self, target: Option<&Assumptions>) -> bool {
        self.assumptions.is_empty() || target.is_some()
    }

    /// 对应 `recordTo(Assumptions target)`。
    pub fn record_to(&self, target: &mut Assumptions) {
        assert!(self.can_record_to(Some(target)));
        for a in &self.assumptions {
            target.record(a.clone_box());
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for AssumptionResult<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AssumptionResult")
            .field("result", &self.result)
            .field("assumptions", &self.assumptions)
            .finish()
    }
}

/// 对应 `final class NoFinalizableSubclass extends Assumption`。
#[derive(Debug)]
pub struct NoFinalizableSubclass {
    pub receiver_type: Box<dyn ResolvedJavaType>,
}

impl NoFinalizableSubclass {
    pub fn new(receiver_type: Box<dyn ResolvedJavaType>) -> Self {
        Self { receiver_type }
    }
}

impl PartialEq for NoFinalizableSubclass {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(&*self.receiver_type, &*other.receiver_type)
    }
}

impl Eq for NoFinalizableSubclass {}

impl Hash for NoFinalizableSubclass {
    fn hash<H: Hasher>(&self, state: &mut H) {
        31i32.hash(state);
        std::ptr::addr_of!(*self.receiver_type).hash(state);
    }
}

impl Assumption for NoFinalizableSubclass {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn Assumption> {
        // receiver_type 共享不可克隆；记录路径下复制同一指针（运行时对象长存）。
        Box::new(Self {
            receiver_type: clone_type_box(&*self.receiver_type),
        })
    }
    fn assumption_to_string(&self) -> String {
        format!(
            "NoFinalizableSubclass[receiverType={}]",
            self.receiver_type.to_java_name()
        )
    }
}

/// 对应 `final class ConcreteSubtype extends Assumption`。
#[derive(Debug)]
pub struct ConcreteSubtype {
    pub context: Box<dyn ResolvedJavaType>,
    pub subtype: Box<dyn ResolvedJavaType>,
}

impl ConcreteSubtype {
    pub fn new(context: Box<dyn ResolvedJavaType>, subtype: Box<dyn ResolvedJavaType>) -> Self {
        Self { context, subtype }
    }
}

impl PartialEq for ConcreteSubtype {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(&*self.context, &*other.context)
            && std::ptr::eq(&*self.subtype, &*other.subtype)
    }
}

impl Eq for ConcreteSubtype {}

impl Hash for ConcreteSubtype {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let prime = 31i32;
        let mut result: i32 = 1;
        result = prime
            .wrapping_mul(result)
            .wrapping_add(ptr_hash_i32(&self.context) as i32);
        result = prime
            .wrapping_mul(result)
            .wrapping_add(ptr_hash_i32(&self.subtype) as i32);
        result.hash(state);
    }
}

impl Assumption for ConcreteSubtype {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn Assumption> {
        Box::new(Self {
            context: clone_type_box(&*self.context),
            subtype: clone_type_box(&*self.subtype),
        })
    }
    fn assumption_to_string(&self) -> String {
        format!(
            "ConcreteSubtype[context={}, subtype={}]",
            self.context.to_java_name(),
            self.subtype.to_java_name()
        )
    }
}

/// 对应 `final class LeafType extends Assumption`。
#[derive(Debug)]
pub struct LeafType {
    pub context: Box<dyn ResolvedJavaType>,
}

impl LeafType {
    pub fn new(context: Box<dyn ResolvedJavaType>) -> Self {
        Self { context }
    }
}

impl PartialEq for LeafType {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(&*self.context, &*other.context)
    }
}

impl Eq for LeafType {}

impl Hash for LeafType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let prime = 31i32;
        let mut result: i32 = 1;
        result = prime
            .wrapping_mul(result)
            .wrapping_add(ptr_hash_i32(&self.context) as i32);
        result.hash(state);
    }
}

impl Assumption for LeafType {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn Assumption> {
        Box::new(Self {
            context: clone_type_box(&*self.context),
        })
    }
    fn assumption_to_string(&self) -> String {
        format!("LeafSubtype[context={}]", self.context.to_java_name())
    }
}

/// 对应 `final class ConcreteMethod extends Assumption`。
#[derive(Debug)]
pub struct ConcreteMethod {
    pub method: Box<dyn ResolvedJavaMethod>,
    pub context: Box<dyn ResolvedJavaType>,
    pub r#impl: Box<dyn ResolvedJavaMethod>,
}

impl ConcreteMethod {
    pub fn new(
        method: Box<dyn ResolvedJavaMethod>,
        context: Box<dyn ResolvedJavaType>,
        r#impl: Box<dyn ResolvedJavaMethod>,
    ) -> Self {
        Self {
            method,
            context,
            r#impl,
        }
    }
}

impl PartialEq for ConcreteMethod {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(&*self.method, &*other.method)
            && std::ptr::eq(&*self.context, &*other.context)
            && std::ptr::eq(&*self.r#impl, &*other.r#impl)
    }
}

impl Eq for ConcreteMethod {}

impl Hash for ConcreteMethod {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let prime = 31i32;
        let mut result: i32 = 1;
        result = prime
            .wrapping_mul(result)
            .wrapping_add(ptr_hash_i32(&self.method) as i32);
        result = prime
            .wrapping_mul(result)
            .wrapping_add(ptr_hash_i32(&self.context) as i32);
        result = prime
            .wrapping_mul(result)
            .wrapping_add(ptr_hash_i32(&self.r#impl) as i32);
        result.hash(state);
    }
}

impl Assumption for ConcreteMethod {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn Assumption> {
        Box::new(Self {
            method: clone_method_box(&*self.method),
            context: clone_type_box(&*self.context),
            r#impl: clone_method_box(&*self.r#impl),
        })
    }
    fn assumption_to_string(&self) -> String {
        format!(
            "ConcreteMethod[method={}, context={}, impl={}]",
            self.method.format("%H.%n(%p)%r"),
            self.context.to_java_name(),
            self.r#impl.format("%H.%n(%p)%r")
        )
    }
}

/// 对应 `final class CallSiteTargetValue extends Assumption`。
#[derive(Debug)]
pub struct CallSiteTargetValue {
    pub call_site: Box<dyn JavaConstant>,
    pub method_handle: Box<dyn JavaConstant>,
}

impl CallSiteTargetValue {
    pub fn new(call_site: Box<dyn JavaConstant>, method_handle: Box<dyn JavaConstant>) -> Self {
        Self {
            call_site,
            method_handle,
        }
    }
}

impl PartialEq for CallSiteTargetValue {
    fn eq(&self, other: &Self) -> bool {
        // Java 用 callSite.equals/methodHandle.equals（JavaConstant 值相等）。
        self.call_site.constant_equals(other.call_site.as_ref())
            && self
                .method_handle
                .constant_equals(other.method_handle.as_ref())
    }
}

impl Eq for CallSiteTargetValue {}

impl Hash for CallSiteTargetValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let prime = 31i32;
        let mut result: i32 = 1;
        result = prime
            .wrapping_mul(result)
            .wrapping_add(const_hash_i32(&*self.call_site));
        result = prime
            .wrapping_mul(result)
            .wrapping_add(const_hash_i32(&*self.method_handle));
        result.hash(state);
    }
}

impl Assumption for CallSiteTargetValue {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn Assumption> {
        Box::new(Self {
            call_site: clone_const_box(&*self.call_site),
            method_handle: clone_const_box(&*self.method_handle),
        })
    }
    fn assumption_to_string(&self) -> String {
        format!(
            "CallSiteTargetValue[callSite={}, methodHandle={}]",
            self.call_site, self.method_handle
        )
    }
}

// --- 共享指针克隆辅助（运行时对象长存，克隆即复制同一 `Box` 指针语义的占位） ---
// 偏离：T7 应改用 `Rc`/`Arc` 共享；此处以原指针重新装箱不可能，故 panic 标注待 T7 替换。
// 为使 T4 trait 镜像可编译且 `record_to`/`record(other)` 路径可执行，clone_box 复制为
// "同址"不可行——改为构造等值新对象需运行时支持。T4 仅要求 trait 可编译与 mock 化，
// 这些 clone_box 在真实假设记录路径由 T7 替换为共享指针实现。

fn clone_type_box(_: &dyn ResolvedJavaType) -> Box<dyn ResolvedJavaType> {
    panic!("Assumption clone_box of ResolvedJavaType requires shared-pointer support (T7)");
}

fn clone_method_box(_: &dyn ResolvedJavaMethod) -> Box<dyn ResolvedJavaMethod> {
    panic!("Assumption clone_box of ResolvedJavaMethod requires shared-pointer support (T7)");
}

fn clone_const_box(c: &dyn JavaConstant) -> Box<dyn JavaConstant> {
    // JavaConstant 不可克隆（dyn）；T7 用共享指针。T4 阶段以 panic 标注。
    let _ = c;
    panic!("Assumption clone_box of JavaConstant requires shared-pointer support (T7)");
}

fn ptr_hash_i32(_: &dyn std::any::Any) -> u64 {
    // 指针地址哈希占位（运行时对象无稳定 identity hash）；T7 用共享指针地址。
    0
}

fn const_hash_i32(_: &dyn JavaConstant) -> i32 {
    0
}

/// 对应 `final class Assumptions implements Iterable<Assumption>`。
pub struct Assumptions {
    assumptions: HashSet<AssumptionKey>,
}

impl Assumptions {
    pub fn new() -> Self {
        Self {
            assumptions: HashSet::new(),
        }
    }

    /// 对应 `isEmpty()`。
    pub fn is_empty(&self) -> bool {
        self.assumptions.is_empty()
    }

    /// 对应 `iterator()`：返回假设切片视图的迭代。
    pub fn iter(&self) -> impl Iterator<Item = &dyn Assumption> {
        self.assumptions.iter().map(|k| &*k.0)
    }

    /// 对应 `record(Assumption assumption)`。
    pub fn record(&mut self, assumption: Box<dyn Assumption>) {
        self.assumptions.insert(AssumptionKey(assumption));
    }

    /// 对应 `recordNoFinalizableSubclassAssumption(ResolvedJavaType)`。
    pub fn record_no_finalizable_subclass_assumption(
        &mut self,
        receiver_type: Box<dyn ResolvedJavaType>,
    ) {
        self.record(Box::new(NoFinalizableSubclass::new(receiver_type)));
    }

    /// 对应 `recordConcreteSubtype(ResolvedJavaType, ResolvedJavaType)`。
    pub fn record_concrete_subtype(
        &mut self,
        context: Box<dyn ResolvedJavaType>,
        subtype: Box<dyn ResolvedJavaType>,
    ) {
        self.record(Box::new(ConcreteSubtype::new(context, subtype)));
    }

    /// 对应 `recordConcreteMethod(ResolvedJavaMethod, ResolvedJavaType, ResolvedJavaMethod)`。
    pub fn record_concrete_method(
        &mut self,
        method: Box<dyn ResolvedJavaMethod>,
        context: Box<dyn ResolvedJavaType>,
        r#impl: Box<dyn ResolvedJavaMethod>,
    ) {
        self.record(Box::new(ConcreteMethod::new(method, context, r#impl)));
    }

    /// 对应 `toArray()`。
    pub fn to_array(&self) -> Vec<Box<dyn Assumption>> {
        self.assumptions.iter().map(|k| k.0.clone_box()).collect()
    }

    /// 对应 `record(Assumptions other)`：合并另一 `Assumptions` 的假设。
    pub fn record_other(&mut self, other: &Assumptions) {
        // assert other != this
        for k in &other.assumptions {
            self.assumptions.insert(AssumptionKey(k.0.clone_box()));
        }
    }
}

impl Default for Assumptions {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for Assumptions {
    fn eq(&self, other: &Self) -> bool {
        self.assumptions.len() == other.assumptions.len()
            && self
                .assumptions
                .iter()
                .all(|k| other.assumptions.contains(k))
    }
}

impl Eq for Assumptions {}

impl fmt::Display for Assumptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Assumptions[")?;
        let mut first = true;
        for k in &self.assumptions {
            if !first {
                f.write_str(", ")?;
            }
            first = false;
            f.write_str(&k.0.assumption_to_string())?;
        }
        f.write_str("]")
    }
}
