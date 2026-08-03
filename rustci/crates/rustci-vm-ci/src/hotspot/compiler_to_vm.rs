// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2009, 2025, Oracle and/or its affiliates. All rights reserved.
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

//! 镜像 `jdk.vm.ci.hotspot.CompilerToVM`：HotSpot 编译器到 VM 的 native 接口。
//!
//! 偏离记录：
//! - Java `CompilerToVM` 是 final class，含 ~132 个 native 方法声明 + 若干 Java 包装方法。
//!   Rust 侧只移植 native 方法声明为 `extern "C"` 函数签名，不实现（native 边界契约）。
//! - Java 类型（如 `HotSpotResolvedJavaMethodImpl`、`HotSpotObjectConstantImpl` 等）→
//!   Rust 用 `*mut std::ffi::c_void` 不透明指针表示（native 边界无类型信息，实现侧负责
//!   指针转换）。Rust 侧用 `*mut std::ffi::c_void` 是因为 JNI 层传递的是 jobject/jlong，
//!   在 Rust 侧统一为不透明指针。
//! - Java 数组类型 → Rust 用对应的 `*mut std::ffi::c_void` 指针。
//! - Java `String` → Rust `*const std::os::raw::c_char`（JNI 层 UTF-8）。
//! - Java `boolean` → Rust `u8`（JNI `jboolean` is `unsigned char`）。
//! - Java `byte` → Rust `i8`（JNI `jbyte` is `signed char`）。
//! - Java `char` → Rust `u16`（JNI `jchar` is `unsigned short`）。
//! - Java `int` → Rust `i32`（JNI `jint`）。
//! - Java `long` → Rust `i64`（JNI `jlong`）。
//! - 方法名 camelCase → snake_case。

use std::ffi::c_void;

// =====================================================================
// 1. Method bytecode / metadata
// =====================================================================

extern "C" {
    /// 对应 `native byte[] getBytecode(HotSpotResolvedJavaMethodImpl, long)`。
    pub fn get_bytecode(method: *mut c_void, method_pointer: i64) -> *mut c_void;

    /// 对应 `native int getExceptionTableStart(HotSpotResolvedJavaMethodImpl, long)`。
    pub fn get_exception_table_start(method: *mut c_void, method_pointer: i64) -> i32;

    /// 对应 `native int getExceptionTableLength(HotSpotResolvedJavaMethodImpl, long)`。
    pub fn get_exception_table_length(method: *mut c_void, method_pointer: i64) -> i32;

    /// 对应 `native long getLocalVariableTableStart(HotSpotResolvedJavaMethodImpl, long)`。
    pub fn get_local_variable_table_start(method: *mut c_void, method_pointer: i64) -> i64;

    /// 对应 `native int getLocalVariableTableLength(HotSpotResolvedJavaMethodImpl, long)`。
    pub fn get_local_variable_table_length(method: *mut c_void, method_pointer: i64) -> i32;

    /// 对应 `native long[] getLineNumberTable(HotSpotResolvedJavaMethodImpl, long)`。
    pub fn get_line_number_table(method: *mut c_void, method_pointer: i64) -> *mut c_void;

    /// 对应 `native boolean hasNeverInlineDirective(HotSpotResolvedJavaMethodImpl, long)`。
    pub fn has_never_inline_directive(method: *mut c_void, method_pointer: i64) -> u8;

    /// 对应 `native boolean shouldInlineMethod(HotSpotResolvedJavaMethodImpl, long)`。
    pub fn should_inline_method(method: *mut c_void, method_pointer: i64) -> u8;

    /// 对应 `native void getOopMapAt(HotSpotResolvedJavaMethodImpl, long, int, long[])`。
    pub fn get_oop_map_at(method: *mut c_void, method_pointer: i64, bci: i32, oop_map: *mut c_void);
}

// =====================================================================
// 2. Constant pool operations
// =====================================================================

extern "C" {
    /// 对应 `native Object lookupConstantInPool(HotSpotConstantPool, int, boolean)`。
    pub fn lookup_constant_in_pool(
        constant_pool: *mut c_void,
        index: i32,
        resolve: u8,
    ) -> *mut c_void;

    /// 对应 `native HotSpotResolvedJavaMethodImpl lookupMethodInPool(HotSpotConstantPool, int, byte, HotSpotResolvedJavaMethodImpl)`。
    pub fn lookup_method_in_pool(
        constant_pool: *mut c_void,
        index: i32,
        opcode: i8,
        caller: *mut c_void,
    ) -> *mut c_void;

    /// 对应 `native int bootstrapArgumentIndexAt(HotSpotConstantPool, int, int)`。
    pub fn bootstrap_argument_index_at(
        constant_pool: *mut c_void,
        bss_index: i32,
        index: i32,
    ) -> i32;

    /// 对应 `native Object lookupKlassInPool(HotSpotConstantPool, int)`。
    pub fn lookup_klass_in_pool(constant_pool: *mut c_void, index: i32) -> *mut c_void;

    /// 对应 `native String lookupNameInPool(HotSpotConstantPool, int, int)`。
    pub fn lookup_name_in_pool(
        constant_pool: *mut c_void,
        raw_index: i32,
        opcode: i32,
    ) -> *const std::os::raw::c_char;

    /// 对应 `native String lookupSignatureInPool(HotSpotConstantPool, int, int)`。
    pub fn lookup_signature_in_pool(
        constant_pool: *mut c_void,
        raw_index: i32,
        opcode: i32,
    ) -> *const std::os::raw::c_char;

    /// 对应 `native int rawIndexToConstantPoolIndex(HotSpotConstantPool, int, int)`。
    pub fn raw_index_to_constant_pool_index(
        constant_pool: *mut c_void,
        raw_index: i32,
        opcode: i32,
    ) -> i32;

    /// 对应 `native int decodeIndyIndexToCPIndex(HotSpotConstantPool, int, boolean)`。
    pub fn decode_indy_index_to_cp_index(
        constant_pool: *mut c_void,
        raw_index: i32,
        resolve: u8,
    ) -> i32;

    /// 对应 `native int decodeFieldIndexToCPIndex(HotSpotConstantPool, int)`。
    pub fn decode_field_index_to_cp_index(constant_pool: *mut c_void, raw_index: i32) -> i32;

    /// 对应 `native int decodeMethodIndexToCPIndex(HotSpotConstantPool, int)`。
    pub fn decode_method_index_to_cp_index(constant_pool: *mut c_void, raw_index: i32) -> i32;

    /// 对应 `native int getNumIndyEntries(HotSpotConstantPool)`。
    pub fn get_num_indy_entries(constant_pool: *mut c_void) -> i32;

    /// 对应 `native HotSpotResolvedObjectTypeImpl resolveTypeInPool(HotSpotConstantPool, int)`。
    pub fn resolve_type_in_pool(constant_pool: *mut c_void, index: i32) -> *mut c_void;

    /// 对应 `native HotSpotResolvedObjectTypeImpl resolveFieldInPool(HotSpotConstantPool, int, HotSpotResolvedJavaMethodImpl, byte, int[])`。
    pub fn resolve_field_in_pool(
        constant_pool: *mut c_void,
        raw_index: i32,
        method: *mut c_void,
        opcode: i8,
        info: *mut c_void,
    ) -> *mut c_void;

    /// 对应 `native Object resolveBootstrapMethod(HotSpotConstantPool, int)`。
    pub fn resolve_bootstrap_method(constant_pool: *mut c_void, index: i32) -> *mut c_void;

    /// 对应 `native void resolveInvokeDynamic(HotSpotConstantPool, int)`。
    pub fn resolve_invoke_dynamic(constant_pool: *mut c_void, index: i32);

    /// 对应 `native void resolveInvokeHandleInPool(HotSpotConstantPool, int)`。
    pub fn resolve_invoke_handle_in_pool(constant_pool: *mut c_void, raw_index: i32);

    /// 对应 `native HotSpotConstantPool getConstantPool(HotSpotResolvedJavaMethodImpl)`。
    pub fn get_constant_pool(method: *mut c_void) -> *mut c_void;

    /// 对应 `native String getUncachedStringInPool(HotSpotConstantPool, int)`。
    pub fn get_uncached_string_in_pool(
        constant_pool: *mut c_void,
        index: i32,
    ) -> *const std::os::raw::c_char;

    /// 对应 `native JavaConstant lookupAppendixInPool(HotSpotConstantPool, int, int)`。
    pub fn lookup_appendix_in_pool(
        constant_pool: *mut c_void,
        raw_index: i32,
        opcode: i32,
    ) -> *mut c_void;

    /// 对应 `native String[] getSignaturePolymorphicHolders()`。
    pub fn get_signature_polymorphic_holders() -> *mut c_void;
}

// =====================================================================
// 3. Code installation / invalidation
// =====================================================================

extern "C" {
    /// 对应 `native int installCode(HotSpotCompiledCode, HotSpotInstalledCode, long, byte[])`。
    pub fn install_code(
        compiled_code: *mut c_void,
        installed_code: *mut c_void,
        failed_speculations_address: i64,
        speculations: *mut c_void,
    ) -> i32;

    /// 对应 `native void invalidateHotSpotNmethod(HotSpotNmethod, boolean)`。
    pub fn invalidate_hot_spot_nmethod(nmethod: *mut c_void, deoptimize: u8);

    /// 对应 `native Object executeHotSpotNmethod(Object[], HotSpotNmethod)`。
    pub fn execute_hot_spot_nmethod(args: *mut c_void, nmethod_mirror: *mut c_void) -> *mut c_void;

    /// 对应 `native void updateHotSpotNmethod(HotSpotNmethod)`。
    pub fn update_hot_spot_nmethod(nmethod_mirror: *mut c_void);

    /// 对应 `native byte[] getCode(HotSpotInstalledCode)`。
    pub fn get_code(code: *mut c_void) -> *mut c_void;
}

// =====================================================================
// 4. Type resolution / lookup
// =====================================================================

extern "C" {
    /// 对应 `native HotSpotResolvedObjectTypeImpl lookupType(String, HotSpotResolvedObjectTypeImpl, boolean)`。
    pub fn lookup_type(
        name: *const std::os::raw::c_char,
        accessing_class: *mut c_void,
        resolve: u8,
    ) -> *mut c_void;

    /// 对应 `native HotSpotResolvedObjectTypeImpl getArrayType(char, HotSpotResolvedObjectTypeImpl, long)`。
    pub fn get_array_type(type_char: u16, klass: *mut c_void, klass_pointer: i64) -> *mut c_void;

    /// 对应 `native void ensureInitialized(HotSpotResolvedObjectTypeImpl, long)`。
    pub fn ensure_initialized(klass: *mut c_void, klass_pointer: i64);

    /// 对应 `native void ensureLinked(HotSpotResolvedObjectTypeImpl, long)`。
    pub fn ensure_linked(klass: *mut c_void, klass_pointer: i64);

    /// 对应 `native HotSpotResolvedObjectTypeImpl[] getInterfaces(HotSpotResolvedObjectTypeImpl, long)`。
    pub fn get_interfaces(klass: *mut c_void, klass_pointer: i64) -> *mut c_void;

    /// 对应 `native HotSpotResolvedJavaType getComponentType(HotSpotResolvedObjectTypeImpl, long)`。
    pub fn get_component_type(klass: *mut c_void, klass_pointer: i64) -> *mut c_void;

    /// 对应 `native HotSpotResolvedObjectTypeImpl getResolvedJavaType(HotSpotResolvedObjectTypeImpl, long, long, boolean)`。
    pub fn get_resolved_java_type(
        base: *mut c_void,
        base_pointer: i64,
        displacement: i64,
        compressed: u8,
    ) -> *mut c_void;

    /// 对应 `native Integer getResolvedJavaMethod(HotSpotResolvedObjectTypeImpl, long, long, boolean)`。
    pub fn get_resolved_java_method(
        base: *mut c_void,
        base_pointer: i64,
        displacement: i64,
        compressed: u8,
    ) -> *mut c_void;
}

// =====================================================================
// 5. Field / method reflection
// =====================================================================

extern "C" {
    /// 对应 `native JavaConstant readStaticFieldValue(HotSpotResolvedObjectTypeImpl, long, long, char)`。
    pub fn read_static_field_value(
        declaring_klass: *mut c_void,
        declaring_klass_pointer: i64,
        offset: i64,
        type_char: u16,
    ) -> *mut c_void;

    /// 对应 `native JavaConstant readFieldValue(HotSpotObjectConstantImpl, HotSpotResolvedObjectTypeImpl, long, long, char)`。
    pub fn read_field_value(
        object: *mut c_void,
        expected_type: *mut c_void,
        expected_type_pointer: i64,
        offset: i64,
        type_char: u16,
    ) -> *mut c_void;

    /// 对应 `native boolean isInstance(HotSpotResolvedObjectTypeImpl, long, HotSpotObjectConstantImpl)`。
    pub fn is_instance(klass: *mut c_void, klass_pointer: i64, object: *mut c_void) -> u8;

    /// 对应 `native boolean isAssignableFrom(HotSpotResolvedObjectTypeImpl, long, HotSpotResolvedObjectTypeImpl, long)`。
    pub fn is_assignable_from(
        klass: *mut c_void,
        klass_pointer: i64,
        subklass: *mut c_void,
        subklass_pointer: i64,
    ) -> u8;

    /// 对应 `native HotSpotResolvedJavaType asJavaType(HotSpotObjectConstantImpl)`。
    pub fn as_java_type(object: *mut c_void) -> *mut c_void;

    /// 对应 `native String asString(HotSpotObjectConstantImpl)`。
    pub fn as_string(object: *mut c_void) -> *const std::os::raw::c_char;

    /// 对应 `native boolean equals(HotSpotObjectConstantImpl, long, HotSpotObjectConstantImpl, long)`。
    pub fn equals(x: *mut c_void, x_handle: i64, y: *mut c_void, y_handle: i64) -> u8;

    /// 对应 `native HotSpotObjectConstantImpl getJavaMirror(HotSpotResolvedObjectTypeImpl, long)`。
    pub fn get_java_mirror(type_: *mut c_void, klass_pointer: i64) -> *mut c_void;

    /// 对应 `native int getArrayLength(HotSpotObjectConstantImpl)`。
    pub fn get_array_length(object: *mut c_void) -> i32;

    /// 对应 `native Object readArrayElement(HotSpotObjectConstantImpl, int)`。
    pub fn read_array_element(object: *mut c_void, index: i32) -> *mut c_void;

    /// 对应 `native boolean isInternedString(HotSpotObjectConstantImpl)`。
    pub fn is_interned_string(object: *mut c_void) -> u8;

    /// 对应 `native int getIdentityHashCode(HotSpotObjectConstantImpl)`。
    pub fn get_identity_hash_code(object: *mut c_void) -> i32;

    /// 对应 `native Object unboxPrimitive(HotSpotObjectConstantImpl)`。
    pub fn unbox_primitive(object: *mut c_void) -> *mut c_void;

    /// 对应 `native HotSpotObjectConstantImpl boxPrimitive(Object)`。
    pub fn box_primitive(source: *mut c_void) -> *mut c_void;

    /// 对应 `native ResolvedJavaMethod[] getDeclaredConstructors(HotSpotResolvedObjectTypeImpl, long)`。
    pub fn get_declared_constructors(klass: *mut c_void, klass_pointer: i64) -> *mut c_void;

    /// 对应 `native ResolvedJavaMethod[] getDeclaredMethods(HotSpotResolvedObjectTypeImpl, long)`。
    pub fn get_declared_methods(klass: *mut c_void, klass_pointer: i64) -> *mut c_void;

    /// 对应 `native ResolvedJavaMethod[] getAllMethods(HotSpotResolvedObjectTypeImpl, long)`。
    pub fn get_all_methods(klass: *mut c_void, klass_pointer: i64) -> *mut c_void;

    /// 对应 `native HotSpotResolvedObjectTypeImpl.FieldInfo[] getDeclaredFieldsInfo(HotSpotResolvedObjectTypeImpl, long)`。
    pub fn get_declared_fields_info(klass: *mut c_void, klass_pointer: i64) -> *mut c_void;

    /// 对应 `native Executable asReflectionExecutable(HotSpotResolvedJavaMethodImpl, long)`。
    pub fn as_reflection_executable(method: *mut c_void, method_pointer: i64) -> *mut c_void;

    /// 对应 `native Field asReflectionField(HotSpotResolvedObjectTypeImpl, long, int)`。
    pub fn as_reflection_field(
        holder: *mut c_void,
        holder_pointer: i64,
        field_index: i32,
    ) -> *mut c_void;

    /// 对应 `native boolean isTrustedForIntrinsics(HotSpotResolvedObjectTypeImpl, long)`。
    pub fn is_trusted_for_intrinsics(klass: *mut c_void, klass_pointer: i64) -> u8;
}

// =====================================================================
// 6. Speculation / failed speculations
// =====================================================================

extern "C" {
    /// 对应 `native byte[][] getFailedSpeculations(long, byte[][])`。
    pub fn get_failed_speculations(
        failed_speculations_address: i64,
        current_failures: *mut c_void,
    ) -> *mut c_void;

    /// 对应 `native long getFailedSpeculationsAddress(HotSpotResolvedJavaMethodImpl, long)`。
    pub fn get_failed_speculations_address(method: *mut c_void, method_pointer: i64) -> i64;

    /// 对应 `native void releaseFailedSpeculations(long)`。
    pub fn release_failed_speculations(failed_speculations_address: i64);

    /// 对应 `native boolean addFailedSpeculation(long, byte[])`。
    pub fn add_failed_speculation(failed_speculations_address: i64, speculation: *mut c_void)
        -> u8;
}

// =====================================================================
// 7. Thread / runtime
// =====================================================================

extern "C" {
    /// 对应 `native boolean isCurrentThreadAttached()`。
    pub fn is_current_thread_attached() -> u8;

    /// 对应 `native long getCurrentJavaThread()`。
    pub fn get_current_java_thread() -> i64;

    /// 对应 `native boolean attachCurrentThread(byte[], boolean, long[])`。
    pub fn attach_current_thread(name: *mut c_void, as_daemon: u8, java_vm_info: *mut c_void)
        -> u8;

    /// 对应 `native boolean detachCurrentThread(boolean)`。
    pub fn detach_current_thread(release: u8) -> u8;

    /// 对应 `native void callSystemExit(int)`。
    pub fn call_system_exit(status: i32);

    /// 对应 `native long[] registerNativeMethods(Class<?>)`。
    pub fn register_native_methods(clazz: *mut c_void) -> *mut c_void;

    /// 对应 `native long translate(Object, boolean)`。
    pub fn translate(obj: *mut c_void, call_post_translation: u8) -> i64;

    /// 对应 `native Object unhand(long)`。
    pub fn unhand(handle: i64) -> *mut c_void;

    /// 对应 `native void clearOopHandle(long)`。
    pub fn clear_oop_handle(handle: i64);

    /// 对应 `native void releaseClearedOopHandles()`。
    pub fn release_cleared_oop_handles();

    /// 对应 `native boolean updateCompilerThreadCanCallJava(boolean)`。
    pub fn update_compiler_thread_can_call_java(new_state: u8) -> u8;

    /// 对应 `native int getCompilationActivityMode()`。
    pub fn get_compilation_activity_mode() -> i32;

    /// 对应 `native boolean isCompilerThread()`。
    pub fn is_compiler_thread() -> u8;

    /// 对应 `native void compileToBytecode(HotSpotObjectConstantImpl)`。
    pub fn compile_to_bytecode(lambda_form: *mut c_void);

    /// 对应 `native Object getFlagValue(String)`。
    pub fn get_flag_value(name: *const std::os::raw::c_char) -> *mut c_void;
}

// =====================================================================
// 8. Thread-local storage
// =====================================================================

extern "C" {
    /// 对应 `native void setThreadLocalObject(int, Object)`。
    pub fn set_thread_local_object(id: i32, value: *mut c_void);

    /// 对应 `native Object getThreadLocalObject(int)`。
    pub fn get_thread_local_object(id: i32) -> *mut c_void;

    /// 对应 `native void setThreadLocalLong(int, long)`。
    pub fn set_thread_local_long(id: i32, value: i64);

    /// 对应 `native long getThreadLocalLong(int)`。
    pub fn get_thread_local_long(id: i32) -> i64;
}

// =====================================================================
// 9. JFR / compiler events
// =====================================================================

extern "C" {
    /// 对应 `native long ticksNow()`。
    pub fn ticks_now() -> i64;

    /// 对应 `native int registerCompilerPhase(String)`。
    pub fn register_compiler_phase(phase_name: *const std::os::raw::c_char) -> i32;

    /// 对应 `native void notifyCompilerPhaseEvent(long, int, int, int)`。
    pub fn notify_compiler_phase_event(start_time: i64, phase: i32, compile_id: i32, level: i32);

    /// 对应 `native void notifyCompilerInliningEvent(int, HotSpotResolvedJavaMethodImpl, long, HotSpotResolvedJavaMethodImpl, long, boolean, String, int)`。
    pub fn notify_compiler_inlining_event(
        compile_id: i32,
        caller: *mut c_void,
        caller_pointer: i64,
        callee: *mut c_void,
        callee_pointer: i64,
        succeeded: u8,
        message: *const std::os::raw::c_char,
        bci: i32,
    );
}

// =====================================================================
// 10. Annotation data
// =====================================================================

extern "C" {
    /// 对应 `native byte[] getEncodedClassAnnotationData(HotSpotResolvedObjectTypeImpl, long, Object, int, long)`。
    pub fn get_encoded_class_annotation_data(
        type_: *mut c_void,
        klass_pointer: i64,
        filter: *mut c_void,
        filter_length: i32,
        filter_klass_pointers: i64,
    ) -> *mut c_void;

    /// 对应 `native byte[] getEncodedExecutableAnnotationData(HotSpotResolvedJavaMethodImpl, long, Object, int, long)`。
    pub fn get_encoded_executable_annotation_data(
        method: *mut c_void,
        method_pointer: i64,
        filter: *mut c_void,
        filter_length: i32,
        filter_klass_pointers: i64,
    ) -> *mut c_void;

    /// 对应 `native byte[] getEncodedFieldAnnotationData(HotSpotResolvedObjectTypeImpl, long, int, Object, int, long)`。
    pub fn get_encoded_field_annotation_data(
        holder: *mut c_void,
        klass_pointer: i64,
        field_index: i32,
        filter_types: *mut c_void,
        filter_length: i32,
        filter_klass_pointers: i64,
    ) -> *mut c_void;
}

// =====================================================================
// 11. Debug / logging / misc
// =====================================================================

extern "C" {
    /// 对应 `native long getMaxCallTargetOffset(long)`。
    pub fn get_max_call_target_offset(address: i64) -> i64;

    /// 对应 `native boolean shouldDebugNonSafepoints()`。
    pub fn should_debug_non_safepoints() -> u8;

    /// 对应 `native void writeDebugOutput(long, int, boolean)`。
    pub fn write_debug_output(buffer: i64, length: i32, flush: u8);

    /// 对应 `native void flushDebugOutput()`。
    pub fn flush_debug_output();

    /// 对应 `native long[] collectCounters()`。
    pub fn collect_counters() -> *mut c_void;

    /// 对应 `native int getCountersSize()`。
    pub fn get_counters_size() -> i32;

    /// 对应 `native boolean setCountersSize(int)`。
    pub fn set_counters_size(new_size: i32) -> u8;

    /// 对应 `native String disassembleCodeBlob(InstalledCode)`。
    pub fn disassemble_code_blob(installed_code: *mut c_void) -> *const std::os::raw::c_char;

    /// 对应 `native int interpreterFrameSize(BytecodeFrame)`。
    pub fn interpreter_frame_size(pos: *mut c_void) -> i32;

    /// 对应 `native void resetCompilationStatistics()`。
    pub fn reset_compilation_statistics();

    /// 对应 `native int getSymbol(long)`。
    pub fn get_symbol(symbol_pointer: i64) -> i32;
}
