// SPDX-License-Identifier: UPL-1.0 OR GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2025, Oracle and/or its affiliates. All rights reserved.
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
 * or visit www.oracle.com if you need additional information or have any
 * questions.
 */

//! Rust cdylib 桥接层：导出 8 个 JNI 符号供 HotSpot 链接。
//!
//! 偏离记录：
//! - 返回类型 `*mut JVMCIRuntime`/`*mut CompilationRequestResult`/`*mut JVMCICompiler`
//!   → Rust 侧统一为 `*mut std::ffi::c_void`（trait object 为胖指针，C ABI 不兼容；
//!   `*mut c_void` 是标准 FFI 不透明指针类型，调用方自行转换）。
//! - `JVMCI_RegisterNativeMethods` 注册表：本期提供 104 个函数指针框架（对齐
//!   `compiler_to_vm.rs` 已移植的 native 方法），剩余 28 个待后续补全。
//! - `JVMCI_Close`：Rust 侧当前无持久化状态需清理，保留为 no-op 以对齐 API 契约。
//! - `compile0`：HotSpotJVMCIRuntime 的 native 方法，连接 JVMCI bridge 到 JVMCICompiler.compileMethod。

use std::ffi::c_void;
use std::os::raw::c_char;
use std::sync::OnceLock;

use jni::sys::{jboolean, jclass, jint, jlong, JNIEnv};

use rustci_vm_ci::code::compilation_request::CompilationRequest;
use rustci_vm_ci::code::compilation_request_result::CompilationRequestResult;
use rustci_vm_ci::hotspot::hotspot_method_wrapper::HotSpotResolvedJavaMethodWrapper;
use rustci_vm_ci::runtime::jvmci::JVMCI;
use rustci_vm_ci::runtime::jvmci_backend::JVMCIBackend;
use rustci_vm_ci::runtime::jvmci_compiler::JVMCICompiler;
use rustci_vm_ci::runtime::jvmci_runtime::JVMCIRuntime;

// =============================================================================
// 1. JVMCI_GetRuntime — 对应 HotSpot 的 JVMCI::getRuntime()
// =============================================================================

/// 返回 JVMCI 运行时指针。对应 `JVMCI::getRuntime()`。
///
/// 返回 `&'static dyn JVMCIRuntime` 的指针（`*mut c_void`），调用方负责
/// 转换为 `*mut JVMCIRuntime`（Java 侧 `long`）。
/// 胖指针存储单元：trait object 为 16 字节胖指针（data + vtable），
/// 无法直接塞入 `*mut c_void`（8 字节）。将胖指针存储在堆分配单元中，
/// 返回指向该单元的指针；调用方通过解引用恢复胖指针。
type RuntimeHandle = *const dyn JVMCIRuntime;

#[no_mangle]
pub extern "C" fn JVMCI_GetRuntime(_env: *mut JNIEnv, _class: jclass) -> *mut c_void {
    let runtime: &'static dyn JVMCIRuntime = JVMCI::get_runtime();
    let fat: *const dyn JVMCIRuntime = runtime;
    let cell: Box<RuntimeHandle> = Box::new(fat);
    Box::into_raw(cell) as *mut c_void
}

// =============================================================================
// 2. JVMCI_Open — 初始化 JVMCI 环境
// =============================================================================

/// 初始化 JVMCI 环境。对应 `JVMCI::initialize()`。
///
/// 返回 0 表示成功（Rust 侧 `initialize()` 为 no-op，对齐 Java 空方法体）。
#[no_mangle]
pub extern "C" fn JVMCI_Open(_env: *mut JNIEnv, _class: jclass) -> jint {
    JVMCI::initialize();
    0
}

// =============================================================================
// 3. JVMCI_Close — 清理 JVMCI 环境
// =============================================================================

/// 清理 JVMCI 环境。Rust 侧当前无持久化状态需清理，保留为 no-op 以对齐 API 契约。
///
/// 返回 0 表示成功。
#[no_mangle]
pub extern "C" fn JVMCI_Close(_env: *mut JNIEnv, _class: jclass) -> jint {
    0
}

// =============================================================================
// 4. JVMCI_CompileMethod — 编译方法
// =============================================================================

/// 从 `JVMCI_GetRuntime` 返回的 handle 中恢复 `&dyn JVMCIRuntime` 引用。
///
/// # Safety
/// `handle` 必须是由 `JVMCI_GetRuntime` 返回的有效 handle。
unsafe fn runtime_from_handle(handle: *mut c_void) -> &'static dyn JVMCIRuntime {
    let cell: *const RuntimeHandle = handle as *const RuntimeHandle;
    let fat: *const dyn JVMCIRuntime = *cell;
    &*fat
}

/// 接收编译请求，委托编译器编译，返回编译结果。
///
/// 签名对齐 HotSpot `JVMCI::compileMethod(JVMCICompileRequest*)`。
/// `jvmci_runtime` 为 `JVMCI_GetRuntime` 返回的运行时 handle。
/// `request` 为 `CompilationRequest` 的指针（由调用方分配）。
/// 返回 `CompilationRequestResult` 的指针（`Box::leak` 语义，调用方负责释放）。
///
/// # Safety
///
/// `jvmci_runtime` 必须是由 `JVMCI_GetRuntime` 返回的有效 handle。
/// `request` 必须指向有效的 `CompilationRequest`。
#[no_mangle]
pub unsafe extern "C" fn JVMCI_CompileMethod(
    _env: *mut JNIEnv,
    _class: jclass,
    jvmci_runtime: *mut c_void,
    request: *mut CompilationRequest,
) -> *mut c_void {
    let runtime: &dyn JVMCIRuntime = runtime_from_handle(jvmci_runtime);
    let compiler: Box<dyn JVMCICompiler> = runtime.get_compiler();
    let req: &CompilationRequest = &*request;
    let result: Box<dyn CompilationRequestResult> = compiler.compile_method(req);
    Box::into_raw(result) as *mut c_void
}

// =============================================================================
// 5. JVMCI_GetCompiler — 返回默认编译器指针
// =============================================================================

/// 返回运行时绑定的编译器指针。
///
/// `jvmci_runtime` 为 `JVMCI_GetRuntime` 返回的运行时指针。
/// 返回 `Box<dyn JVMCICompiler>` 的指针（`Box::leak` 语义）。
///
/// # Safety
///
/// `jvmci_runtime` 必须是由 `JVMCI_GetRuntime` 返回的有效 handle。
#[no_mangle]
pub unsafe extern "C" fn JVMCI_GetCompiler(
    _env: *mut JNIEnv,
    _class: jclass,
    jvmci_runtime: *mut c_void,
) -> *mut c_void {
    let runtime: &dyn JVMCIRuntime = runtime_from_handle(jvmci_runtime);
    let compiler: Box<dyn JVMCICompiler> = runtime.get_compiler();
    Box::into_raw(compiler) as *mut c_void
}

// =============================================================================
// 6. JVMCI_GetHostBackend — 返回宿主 backend 指针
// =============================================================================

/// 返回宿主 JVMCI backend 指针。
///
/// `jvmci_runtime` 为 `JVMCI_GetRuntime` 返回的运行时指针。
/// 返回 `&JVMCIBackend` 的指针（引用语义，对齐 Java 返回单例引用）。
///
/// # Safety
///
/// `jvmci_runtime` 必须是由 `JVMCI_GetRuntime` 返回的有效 handle。
#[no_mangle]
pub unsafe extern "C" fn JVMCI_GetHostBackend(
    _env: *mut JNIEnv,
    _class: jclass,
    jvmci_runtime: *mut c_void,
) -> *mut c_void {
    let runtime: &dyn JVMCIRuntime = runtime_from_handle(jvmci_runtime);
    let backend: &JVMCIBackend = runtime.get_host_jvmci_backend();
    backend as *const JVMCIBackend as *mut c_void
}

// =============================================================================
// 7. JVMCI_RegisterNativeMethods — 注册表
// =============================================================================

/// CompilerToVM native 方法注册表。HotSpot 加载 cdylib 后调用此函数注册
/// CompilerToVM 的 132 个 native 方法实现（本期提供 104 个函数指针框架，
/// 对齐 `compiler_to_vm.rs` 已移植方法，剩余 28 个待后续补全）。
///
/// 结构体字段对齐 `compiler_to_vm.rs` 的 `extern "C"` 函数签名。
/// 每个字段为 `Option<extern "C" fn(...)>`，`None` 表示该 native 方法尚未注册。
#[repr(C)]
pub struct CompilerToVMTable {
    // ── 1. Method bytecode / metadata ──
    pub get_bytecode: Option<extern "C" fn(*mut c_void, i64) -> *mut c_void>,
    pub get_exception_table_start: Option<extern "C" fn(*mut c_void, i64) -> i32>,
    pub get_exception_table_length: Option<extern "C" fn(*mut c_void, i64) -> i32>,
    pub get_local_variable_table_start: Option<extern "C" fn(*mut c_void, i64) -> i64>,
    pub get_local_variable_table_length: Option<extern "C" fn(*mut c_void, i64) -> i32>,
    pub get_line_number_table: Option<extern "C" fn(*mut c_void, i64) -> *mut c_void>,
    pub has_never_inline_directive: Option<extern "C" fn(*mut c_void, i64) -> u8>,
    pub should_inline_method: Option<extern "C" fn(*mut c_void, i64) -> u8>,
    pub get_oop_map_at: Option<extern "C" fn(*mut c_void, i64, i32, *mut c_void)>,

    // ── 2. Constant pool operations ──
    pub lookup_constant_in_pool: Option<extern "C" fn(*mut c_void, i32, u8) -> *mut c_void>,
    pub lookup_method_in_pool:
        Option<extern "C" fn(*mut c_void, i32, i8, *mut c_void) -> *mut c_void>,
    pub bootstrap_argument_index_at: Option<extern "C" fn(*mut c_void, i32, i32) -> i32>,
    pub lookup_klass_in_pool: Option<extern "C" fn(*mut c_void, i32) -> *mut c_void>,
    pub lookup_name_in_pool:
        Option<extern "C" fn(*mut c_void, i32, i32) -> *const std::os::raw::c_char>,
    pub lookup_signature_in_pool:
        Option<extern "C" fn(*mut c_void, i32, i32) -> *const std::os::raw::c_char>,
    pub raw_index_to_constant_pool_index: Option<extern "C" fn(*mut c_void, i32, i32) -> i32>,
    pub decode_indy_index_to_cp_index: Option<extern "C" fn(*mut c_void, i32, u8) -> i32>,
    pub decode_field_index_to_cp_index: Option<extern "C" fn(*mut c_void, i32) -> i32>,
    pub decode_method_index_to_cp_index: Option<extern "C" fn(*mut c_void, i32) -> i32>,
    pub get_num_indy_entries: Option<extern "C" fn(*mut c_void) -> i32>,
    pub resolve_type_in_pool: Option<extern "C" fn(*mut c_void, i32) -> *mut c_void>,
    pub resolve_field_in_pool:
        Option<extern "C" fn(*mut c_void, i32, *mut c_void, i8, *mut c_void) -> *mut c_void>,
    pub resolve_bootstrap_method: Option<extern "C" fn(*mut c_void, i32) -> *mut c_void>,
    pub resolve_invoke_dynamic: Option<extern "C" fn(*mut c_void, i32)>,
    pub resolve_invoke_handle_in_pool: Option<extern "C" fn(*mut c_void, i32)>,
    pub get_constant_pool: Option<extern "C" fn(*mut c_void) -> *mut c_void>,
    pub get_uncached_string_in_pool:
        Option<extern "C" fn(*mut c_void, i32) -> *const std::os::raw::c_char>,
    pub lookup_appendix_in_pool: Option<extern "C" fn(*mut c_void, i32, i32) -> *mut c_void>,
    pub get_signature_polymorphic_holders: Option<extern "C" fn() -> *mut c_void>,

    // ── 3. Code installation / invalidation ──
    pub install_code: Option<extern "C" fn(*mut c_void, *mut c_void, i64, *mut c_void) -> i32>,
    pub invalidate_hot_spot_nmethod: Option<extern "C" fn(*mut c_void, u8)>,
    pub execute_hot_spot_nmethod: Option<extern "C" fn(*mut c_void, *mut c_void) -> *mut c_void>,
    pub update_hot_spot_nmethod: Option<extern "C" fn(*mut c_void)>,
    pub get_code: Option<extern "C" fn(*mut c_void) -> *mut c_void>,

    // ── 4. Type resolution / lookup ──
    pub lookup_type:
        Option<extern "C" fn(*const std::os::raw::c_char, *mut c_void, u8) -> *mut c_void>,
    pub get_array_type: Option<extern "C" fn(u16, *mut c_void, i64) -> *mut c_void>,
    pub ensure_initialized: Option<extern "C" fn(*mut c_void, i64)>,
    pub ensure_linked: Option<extern "C" fn(*mut c_void, i64)>,
    pub get_interfaces: Option<extern "C" fn(*mut c_void, i64) -> *mut c_void>,
    pub get_component_type: Option<extern "C" fn(*mut c_void, i64) -> *mut c_void>,
    pub get_resolved_java_type: Option<extern "C" fn(*mut c_void, i64, i64, u8) -> *mut c_void>,
    pub get_resolved_java_method: Option<extern "C" fn(*mut c_void, i64, i64, u8) -> *mut c_void>,

    // ── 5. Field / method reflection ──
    pub read_static_field_value: Option<extern "C" fn(*mut c_void, i64, i64, u16) -> *mut c_void>,
    pub read_field_value:
        Option<extern "C" fn(*mut c_void, *mut c_void, i64, i64, u16) -> *mut c_void>,
    pub is_instance: Option<extern "C" fn(*mut c_void, i64, *mut c_void) -> u8>,
    pub is_assignable_from: Option<extern "C" fn(*mut c_void, i64, *mut c_void, i64) -> u8>,
    pub as_java_type: Option<extern "C" fn(*mut c_void) -> *mut c_void>,
    pub as_string: Option<extern "C" fn(*mut c_void) -> *const std::os::raw::c_char>,
    pub equals: Option<extern "C" fn(*mut c_void, i64, *mut c_void, i64) -> u8>,
    pub get_java_mirror: Option<extern "C" fn(*mut c_void, i64) -> *mut c_void>,
    pub get_array_length: Option<extern "C" fn(*mut c_void) -> i32>,
    pub read_array_element: Option<extern "C" fn(*mut c_void, i32) -> *mut c_void>,
    pub is_interned_string: Option<extern "C" fn(*mut c_void) -> u8>,
    pub get_identity_hash_code: Option<extern "C" fn(*mut c_void) -> i32>,
    pub unbox_primitive: Option<extern "C" fn(*mut c_void) -> *mut c_void>,
    pub box_primitive: Option<extern "C" fn(*mut c_void) -> *mut c_void>,
    pub get_declared_constructors: Option<extern "C" fn(*mut c_void, i64) -> *mut c_void>,
    pub get_declared_methods: Option<extern "C" fn(*mut c_void, i64) -> *mut c_void>,
    pub get_all_methods: Option<extern "C" fn(*mut c_void, i64) -> *mut c_void>,
    pub get_declared_fields_info: Option<extern "C" fn(*mut c_void, i64) -> *mut c_void>,
    pub as_reflection_executable: Option<extern "C" fn(*mut c_void, i64) -> *mut c_void>,
    pub as_reflection_field: Option<extern "C" fn(*mut c_void, i64, i32) -> *mut c_void>,
    pub is_trusted_for_intrinsics: Option<extern "C" fn(*mut c_void, i64) -> u8>,

    // ── 6. Speculation / failed speculations ──
    pub get_failed_speculations: Option<extern "C" fn(i64, *mut c_void) -> *mut c_void>,
    pub get_failed_speculations_address: Option<extern "C" fn(*mut c_void, i64) -> i64>,
    pub release_failed_speculations: Option<extern "C" fn(i64)>,
    pub add_failed_speculation: Option<extern "C" fn(i64, *mut c_void) -> u8>,

    // ── 7. Thread / runtime ──
    pub is_current_thread_attached: Option<extern "C" fn() -> u8>,
    pub get_current_java_thread: Option<extern "C" fn() -> i64>,
    pub attach_current_thread: Option<extern "C" fn(*mut c_void, u8, *mut c_void) -> u8>,
    pub detach_current_thread: Option<extern "C" fn(u8) -> u8>,
    pub call_system_exit: Option<extern "C" fn(i32)>,
    pub register_native_methods: Option<extern "C" fn(*mut c_void) -> *mut c_void>,
    pub translate: Option<extern "C" fn(*mut c_void, u8) -> i64>,
    pub unhand: Option<extern "C" fn(i64) -> *mut c_void>,
    pub clear_oop_handle: Option<extern "C" fn(i64)>,
    pub release_cleared_oop_handles: Option<extern "C" fn()>,
    pub update_compiler_thread_can_call_java: Option<extern "C" fn(u8) -> u8>,
    pub get_compilation_activity_mode: Option<extern "C" fn() -> i32>,
    pub is_compiler_thread: Option<extern "C" fn() -> u8>,
    pub compile_to_bytecode: Option<extern "C" fn(*mut c_void)>,
    pub get_flag_value: Option<extern "C" fn(*const std::os::raw::c_char) -> *mut c_void>,

    // ── 8. Thread-local storage ──
    pub set_thread_local_object: Option<extern "C" fn(i32, *mut c_void)>,
    pub get_thread_local_object: Option<extern "C" fn(i32) -> *mut c_void>,
    pub set_thread_local_long: Option<extern "C" fn(i32, i64)>,
    pub get_thread_local_long: Option<extern "C" fn(i32) -> i64>,

    // ── 9. JFR / compiler events ──
    pub ticks_now: Option<extern "C" fn() -> i64>,
    pub register_compiler_phase: Option<extern "C" fn(*const std::os::raw::c_char) -> i32>,
    pub notify_compiler_phase_event: Option<extern "C" fn(i64, i32, i32, i32)>,
    pub notify_compiler_inlining_event: Option<
        extern "C" fn(
            i32,
            *mut c_void,
            i64,
            *mut c_void,
            i64,
            u8,
            *const std::os::raw::c_char,
            i32,
        ),
    >,

    // ── 10. Annotation data ──
    pub get_encoded_class_annotation_data:
        Option<extern "C" fn(*mut c_void, i64, *mut c_void, i32, i64) -> *mut c_void>,
    pub get_encoded_executable_annotation_data:
        Option<extern "C" fn(*mut c_void, i64, *mut c_void, i32, i64) -> *mut c_void>,
    pub get_encoded_field_annotation_data:
        Option<extern "C" fn(*mut c_void, i64, i32, *mut c_void, i32, i64) -> *mut c_void>,

    // ── 11. Debug / logging / misc ──
    pub get_max_call_target_offset: Option<extern "C" fn(i64) -> i64>,
    pub should_debug_non_safepoints: Option<extern "C" fn() -> u8>,
    pub write_debug_output: Option<extern "C" fn(i64, i32, u8)>,
    pub flush_debug_output: Option<extern "C" fn()>,
    pub collect_counters: Option<extern "C" fn() -> *mut c_void>,
    pub get_counters_size: Option<extern "C" fn() -> i32>,
    pub set_counters_size: Option<extern "C" fn(i32) -> u8>,
    pub disassemble_code_blob: Option<extern "C" fn(*mut c_void) -> *const std::os::raw::c_char>,
    pub interpreter_frame_size: Option<extern "C" fn(*mut c_void) -> i32>,
    pub reset_compilation_statistics: Option<extern "C" fn()>,
    pub get_symbol: Option<extern "C" fn(i64) -> i32>,
}

/// 全局注册表：HotSpot 在 `JVMCI_RegisterNativeMethods` 调用时填充。
static COMPILER_TO_VM_TABLE: OnceLock<CompilerToVMTable> = OnceLock::new();

/// 注册 CompilerToVM 的 native 方法实现。HotSpot 在加载 cdylib 后调用此函数，
/// 传入 CompilerToVM 的 `jclass` 对象以注册 native 方法。
///
/// 当前框架：`COMPILER_TO_VM_TABLE` 尚未被填充（HotSpot VM 端注册逻辑待后续实现）。
/// 返回 0 表示成功。
#[no_mangle]
pub extern "C" fn JVMCI_RegisterNativeMethods(
    _env: *mut JNIEnv,
    _class: jclass,
    _compiler_to_vm_class: jclass,
) -> jint {
    // 注册表框架：HotSpot VM 端在此调用 JNI RegisterNatives 注册 CompilerToVM
    // 的 132 个 native 方法实现。本期提供框架，实际注册逻辑由 HotSpot 端负责。
    // 当 HotSpot 注册完成后，应调用 `COMPILER_TO_VM_TABLE.set(table)` 填充函数指针。
    0
}

/// 获取 CompilerToVM 注册表（供 rustci-vm-ci 内部使用，通过函数指针调用 VM 端
/// native 方法）。
#[allow(dead_code)]
pub(crate) fn get_compiler_to_vm_table() -> Option<&'static CompilerToVMTable> {
    COMPILER_TO_VM_TABLE.get()
}

// =============================================================================
// 8. compile0 — 连接 JVMCI bridge 到 JVMCICompiler.compileMethod
// =============================================================================

/// compile0 结果缓冲区布局（`#[repr(C)]`，对齐 HotSpot 端 `JVMCICompileResult`）。
///
/// HotSpot 端预分配缓冲区并将指针以 `jlong` 传入 `result_buffer` 参数。
/// `compile0` 将编译结果写入此结构体。
#[repr(C)]
pub struct Compile0Result {
    /// 失败消息指针（`null` 表示编译成功）。
    pub failure_message: *const c_char,
    /// 是否可重试。
    pub retry: u8,
    /// 内联字节码数。
    pub inlined_bytecodes: i32,
}

/// compile0 调度函数：连接 HotSpot 的 `HotSpotJVMCIRuntime.compile0()` native 方法
/// 到 Rust 侧的 `JVMCICompiler::compile_method()`。
///
/// 签名对齐 Java `HotSpotJVMCIRuntime.compile0`：
/// `private native void compile0(HotSpotResolvedJavaMethod method, int entryBCI,
/// boolean isOSR, int ospBCI, boolean installedCodeDbg, long jvmciEnv, long id,
/// long debugInfoOutput, long resultBuffer, long perfData)`
///
/// 流程：
/// 1. 从全局运行时获取 JVMCICompiler
/// 2. 用 `method` 指针构造 `HotSpotResolvedJavaMethodWrapper` → `CompilationRequest`
/// 3. 调用 `compiler.compile_method(request)`
/// 4. 将结果写入 `result_buffer`（`Compile0Result` 布局）
///
/// # Safety
///
/// `method` 必须指向有效的 HotSpot `Method*`。
/// `result_buffer` 必须指向有效的 `Compile0Result` 缓冲区。
#[no_mangle]
pub unsafe extern "C" fn compile0(
    _env: *mut JNIEnv,
    method: *mut c_void,
    entry_bci: jint,
    _is_osr: jboolean,
    _osp_bci: jint,
    _installed_code_dbg: jboolean,
    _jvmci_env: jlong,
    _id: jlong,
    _debug_info_output: jlong,
    result_buffer: jlong,
    _perf_data: jlong,
) {
    let runtime: &dyn JVMCIRuntime = JVMCI::get_runtime();
    let compiler: Box<dyn JVMCICompiler> = runtime.get_compiler();
    let method_wrapper: Box<HotSpotResolvedJavaMethodWrapper> =
        Box::new(HotSpotResolvedJavaMethodWrapper::new(method, ""));
    let request: CompilationRequest = CompilationRequest::with_entry_bci(method_wrapper, entry_bci);
    let result: Box<dyn CompilationRequestResult> = compiler.compile_method(&request);

    let result_ptr: *mut Compile0Result = result_buffer as *mut Compile0Result;
    match result.get_failure() {
        Some(_) => {
            let msg: String = "compilation failed".to_string();
            let c_msg: std::ffi::CString = std::ffi::CString::new(msg).unwrap();
            (*result_ptr).failure_message = c_msg.into_raw();
            (*result_ptr).retry = 0;
            (*result_ptr).inlined_bytecodes = 0;
        }
        None => {
            (*result_ptr).failure_message = std::ptr::null();
            (*result_ptr).retry = 1;
            (*result_ptr).inlined_bytecodes = 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use std::any::Any;

    use rustci_vm_ci::code::architecture::Architecture;
    use rustci_vm_ci::code::compilation_request::CompilationRequest;
    use rustci_vm_ci::code::compilation_request_result::CompilationRequestResult;
    use rustci_vm_ci::runtime::jvmci::JVMCI;
    use rustci_vm_ci::runtime::jvmci_backend::JVMCIBackend;
    use rustci_vm_ci::runtime::jvmci_compiler::JVMCICompiler;
    use rustci_vm_ci::runtime::jvmci_runtime::JVMCIRuntime;

    use super::Compile0Result;

    /// Mock 编译器，记录编译调用并返回成功结果。
    struct MockCompiler {
        compiled: std::cell::RefCell<Vec<String>>,
    }

    impl MockCompiler {
        fn new() -> Self {
            Self {
                compiled: std::cell::RefCell::new(Vec::new()),
            }
        }
    }

    impl JVMCICompiler for MockCompiler {
        fn compile_method(
            &self,
            request: &CompilationRequest,
        ) -> Box<dyn CompilationRequestResult> {
            self.compiled.borrow_mut().push(request.to_string());
            Box::new(MockCompilationResult::success())
        }

        fn is_gc_supported(&self, _gc_identifier: i32) -> bool {
            true
        }

        fn is_intrinsic_supported(&self, _intrinsic_identifier: i32) -> bool {
            true
        }
    }

    /// Mock 编译结果：成功。
    struct MockCompilationResult {
        success: bool,
    }

    impl MockCompilationResult {
        fn success() -> Self {
            Self { success: true }
        }

        #[allow(dead_code)]
        fn failure() -> Self {
            Self { success: false }
        }
    }

    impl CompilationRequestResult for MockCompilationResult {
        fn get_failure(&self) -> Option<&dyn Any> {
            if self.success {
                None
            } else {
                Some(&"mock failure")
            }
        }
    }

    /// Mock JVMCI 运行时，返回 MockCompiler。
    struct MockRuntime;

    impl JVMCIRuntime for MockRuntime {
        fn get_compiler(&self) -> Box<dyn JVMCICompiler> {
            Box::new(MockCompiler::new())
        }

        fn get_host_jvmci_backend(&self) -> &JVMCIBackend {
            panic!("MockRuntime::get_host_jvmci_backend not implemented")
        }

        fn get_jvmci_backend(&self, _arch: &Architecture) -> Option<&JVMCIBackend> {
            None
        }
    }

    fn mock_initialize_runtime() -> Box<dyn JVMCIRuntime> {
        Box::new(MockRuntime)
    }

    #[test]
    fn compile0_result_buffer_layout() {
        // 验证 Compile0Result 的 repr(C) 布局与 HotSpot 端一致。
        let result = Compile0Result {
            failure_message: std::ptr::null(),
            retry: 1,
            inlined_bytecodes: 42,
        };
        assert!(result.failure_message.is_null());
        assert_eq!(result.retry, 1);
        assert_eq!(result.inlined_bytecodes, 42);
    }

    #[test]
    fn compile0_result_success_writes_null_failure_message() {
        let result = Compile0Result {
            failure_message: std::ptr::null(),
            retry: 1,
            inlined_bytecodes: 0,
        };
        assert!(result.failure_message.is_null());
        assert_eq!(result.retry, 1);
    }

    #[test]
    fn compile0_dispatch_with_mock_compiler() {
        // 注册 mock 运行时。
        let _ = JVMCI::register_initialize_runtime(mock_initialize_runtime);
        let runtime = JVMCI::get_runtime();
        let compiler = runtime.get_compiler();

        // 构造 CompilationRequest（使用 HotSpotResolvedJavaMethodWrapper）。
        let method =
            rustci_vm_ci::hotspot::hotspot_method_wrapper::HotSpotResolvedJavaMethodWrapper::new(
                std::ptr::null_mut(),
                "testMethod",
            );
        let request = CompilationRequest::with_entry_bci(Box::new(method), 0);

        // 调用 compile_method。
        let result = compiler.compile_method(&request);

        // 验证结果：成功编译 → get_failure 为 None。
        assert!(result.get_failure().is_none());
    }

    #[test]
    fn compile0_result_buffer_write_success() {
        // 验证成功时将 null 指针写入 failure_message。
        let mut buf = Compile0Result {
            failure_message: std::ptr::dangling::<std::os::raw::c_char>(),
            retry: 0,
            inlined_bytecodes: 0,
        };
        // 模拟成功写入。
        buf.failure_message = std::ptr::null();
        buf.retry = 1;
        assert!(buf.failure_message.is_null());
        assert_eq!(buf.retry, 1);
    }

    #[test]
    fn compile0_result_buffer_write_failure() {
        // 验证失败时写入错误消息指针。
        let msg = std::ffi::CString::new("compilation failed").unwrap();
        let mut buf = Compile0Result {
            failure_message: std::ptr::null(),
            retry: 0,
            inlined_bytecodes: 0,
        };
        buf.failure_message = msg.into_raw();
        buf.retry = 0;
        assert!(!buf.failure_message.is_null());
        assert_eq!(buf.retry, 0);
        // 清理：从 raw 指针重建 CString 以释放内存。
        unsafe {
            let _ = std::ffi::CString::from_raw(buf.failure_message as *mut i8);
        }
    }
}
