// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2015, 2025, Oracle and/or its affiliates. All rights reserved.
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

//! 镜像 `jdk.vm.ci.hotspot.HotSpotJVMCIRuntime`：HotSpot JVMCI 运行时入口。
//!
//! 偏离记录：
//! - Java `final class HotSpotJVMCIRuntime implements JVMCIRuntime` → Rust `pub trait HotSpotJVMCIRuntime`。
//!   本期只移植公共接口方法签名，具体实现（单例、初始化、内部类）留待 HotSpot 后端绑定。
//! - 方法名 camelCase → snake_case。

use std::ffi::c_void;

use crate::code::compilation_request_result::CompilationRequestResult;
use crate::code::compiled_code::CompiledCode;
use crate::code::installed_code::InstalledCode;
use crate::meta::java_kind::JavaKind;
use crate::meta::resolved_java_type::ResolvedJavaType;

/// 对应 `final class HotSpotJVMCIRuntime implements JVMCIRuntime`。
pub trait HotSpotJVMCIRuntime {
    /// 对应 `static HotSpotJVMCIRuntime runtime()`。
    fn runtime() -> *mut c_void;

    /// 对应 `CompilerToVM getCompilerToVM()`。
    fn get_compiler_to_vm(&self) -> *mut c_void;

    /// 对应 `void compileMethod(HotSpotResolvedJavaMethod, int, int, long)`。
    fn compile_method(
        &self,
        method: *mut c_void,
        entry_bci: i32,
        compile_id: i32,
        compile_state: i64,
    ) -> Box<dyn CompilationRequestResult>;

    /// 对应 `HotSpotResolvedObjectTypeImpl fromClass(Class<?>)`。
    #[allow(clippy::wrong_self_convention)]
    fn from_class(&self, clazz: *mut c_void) -> Box<dyn ResolvedJavaType>;

    /// 对应 `HotSpotResolvedObjectTypeImpl lookupType(String, HotSpotResolvedObjectType, boolean)`。
    fn lookup_type(
        &self,
        name: &str,
        accessing_class: &dyn ResolvedJavaType,
        resolve: bool,
    ) -> Option<Box<dyn ResolvedJavaType>>;

    /// 对应 `void notifyInstall(HotSpotCodeCacheProvider, InstalledCode, CompiledCode)`。
    fn notify_install(
        &self,
        code_cache_provider: *mut c_void,
        installed_code: &InstalledCode,
        compiled_code: &dyn CompiledCode,
    );

    /// 对应 `int writeDebugOutput(byte[], int, int, boolean, boolean)`。
    fn write_debug_output(
        &self,
        bytes: &[u8],
        offset: i32,
        length: i32,
        flush: bool,
        can_throw: bool,
    ) -> i32;

    /// 对应 `OutputStream getLogStream()`。
    fn get_log_stream(&self) -> *mut c_void;

    /// 对应 `long[] collectCounters()`。
    fn collect_counters(&self) -> Vec<i64>;

    /// 对应 `int getCountersSize()`。
    fn get_counters_size(&self) -> i32;

    /// 对应 `boolean setCountersSize(int)`。
    fn set_counters_size(&self, new_size: i32) -> bool;

    /// 对应 `int getArrayBaseOffset(JavaKind)`。
    fn get_array_base_offset(&self, kind: JavaKind) -> i32;

    /// 对应 `int getArrayIndexScale(JavaKind)`。
    fn get_array_index_scale(&self, kind: JavaKind) -> i32;

    /// 对应 `long[] registerNativeMethods(Class<?>)`。
    fn register_native_methods(&self, clazz: *mut c_void) -> Vec<i64>;

    /// 对应 `long translate(Object, boolean)`。
    fn translate(&self, obj: *mut c_void, call_post_translation: bool) -> i64;

    /// 对应 `<T> T unhand(Class<T>, long)`。
    fn unhand(&self, handle: i64) -> *mut c_void;

    /// 对应 `boolean isCurrentThreadAttached()`。
    fn is_current_thread_attached(&self) -> bool;

    /// 对应 `long getCurrentJavaThread()`。
    fn get_current_java_thread(&self) -> i64;

    /// 对应 `boolean attachCurrentThread(boolean, long[])`。
    fn attach_current_thread(&self, as_daemon: bool, java_vm_info: &mut [i64]) -> bool;

    /// 对应 `boolean detachCurrentThread(boolean)`。
    fn detach_current_thread(&self, release: bool) -> bool;

    /// 对应 `void exitHotSpot(int)`。
    fn exit_hot_spot(&self, status: i32);

    /// 对应 `int getCompilationActivityMode()`。
    fn get_compilation_activity_mode(&self) -> i32;
}
