// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2018, 2024, Oracle and/or its affiliates. All rights reserved.
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
 * Please contact Oracle, 500 Oracle Parkway, Redwood Shores, CA 94065 USA
 * or visit www.oracle.com if you need additional information or have
 * questions.
 */

//! 镜像 `jdk.vm.ci.runtime.JVMCI`：JVMCI 入口（运行时获取/初始化）。
//!
//! 偏离记录：
//! - Java `private static volatile JVMCIRuntime runtime` → Rust `thread_local`（`dyn JVMCIRuntime`
//!   不带 `Send + Sync` bound，无法存入 `static`）。语义偏差：每线程首次 `get_runtime` 各自
//!   调用 `initializeRuntime` 并缓存，与 Java 进程级共享单例不等价。
//! - Java `private static native JVMCIRuntime initializeRuntime()` 经 JNI 链接 native 方法 →
//!   Rust 经函数指针注册（`register_initialize_runtime`，VM 在 `JVMCI_OnLoad` 调用）。这是
//!   native 边界机制（非 stub）：VM 提供实现，Rust 侧声明契约。
//! - Java `getRuntime` 双重检查锁定 + `synchronized` → Rust `thread_local` 每线程独立缓存
//!   （单线程内无竞态，无需锁）。
//! - Java `UnsatisfiedLinkError` → Rust 检查 `INITIALIZE_RUNTIME` 是否已注册；未注册时 panic
//!   （对齐 Java `throw UnsupportedOperationException`）。
//! - Java 消息含 `java.home`/`java.vm.name`；Rust 无对应系统属性，消息以注册状态替代。
//! - 返回 `&'static dyn JVMCIRuntime`（`Box::leak` 后缓存于 `thread_local`），对齐 Java 返回
//!   单例引用语义。每线程首次调用 leak 一次 Box，生命周期与线程相同（Java 单例亦永不释放）。
//! - Java `private static boolean initializing` 字段保留（`thread_local`），用于 1:1 镜像；
//!   `getRuntime` 不检查此字段（与 Java 一致），供未来 HotSpot 移植侧检测重入。

use std::cell::Cell;
use std::sync::OnceLock;

use crate::runtime::jvmci_runtime::JVMCIRuntime;

/// VM 注册的 `initializeRuntime` 实现类型。对应 Java
/// `private static native JVMCIRuntime initializeRuntime()` 的 native 边界。
type InitializeRuntimeFn = fn() -> Box<dyn JVMCIRuntime>;

static INITIALIZE_RUNTIME: OnceLock<InitializeRuntimeFn> = OnceLock::new();

thread_local! {
    /// 对应 `private static volatile JVMCIRuntime runtime`。
    static RUNTIME: Cell<Option<&'static dyn JVMCIRuntime>> = const { Cell::new(None) };

    /// 对应 `private static boolean initializing`。`getRuntime` 不读此字段（与 Java 一致），
    /// 保留供未来 HotSpot 移植侧检测初始化重入。
    #[allow(dead_code)]
    static INITIALIZING: Cell<bool> = const { Cell::new(false) };
}

/// 对应 `public final class JVMCI`。
pub struct JVMCI;

impl JVMCI {
    /// 对应 `public static void initialize()`：Java 空方法体，强制触发静态初始化。
    /// Rust 无静态初始化语义，本方法为 no-op，保留以对齐 API。
    pub fn initialize() {}

    /// VM 注册 native `initializeRuntime` 实现。对应 Java
    /// `private static native JVMCIRuntime initializeRuntime()`。
    ///
    /// 偏离：Java native 方法经 JNI 链接；Rust 经函数指针注册（VM 在 `JVMCI_OnLoad` 调用）。
    /// 返回 `Err(existing)` 表示已注册过（`OnceLock` 单次赋值语义）。
    pub fn register_initialize_runtime(f: InitializeRuntimeFn) -> Result<(), InitializeRuntimeFn> {
        INITIALIZE_RUNTIME.set(f)
    }

    /// 对应 `public static JVMCIRuntime getRuntime()`：双重检查锁定单例。
    pub fn get_runtime() -> &'static dyn JVMCIRuntime {
        RUNTIME.with(|r| {
            if let Some(rt) = r.get() {
                return rt;
            }
            INITIALIZING.with(|init| init.set(true));
            let result = match INITIALIZE_RUNTIME.get() {
                Some(f) => Ok(f()),
                None => Err(()),
            };
            INITIALIZING.with(|init| init.set(false));
            match result {
                Ok(rt) => {
                    let rt_ref: &'static dyn JVMCIRuntime = Box::leak(rt);
                    r.set(Some(rt_ref));
                    rt_ref
                }
                Err(()) => panic!(
                    "JVMCI runtime not available: initializeRuntime not registered.\n\
                     This indicates the VM-side runtime initializer was not set via \
                     JVMCI::register_initialize_runtime before get_runtime was called."
                ),
            }
        })
    }
}
