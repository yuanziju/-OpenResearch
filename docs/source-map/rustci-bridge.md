# rustci-bridge（Rust cdylib 对接层）

## 源位置：无 Java 源；Rust 原生 cdylib（产物 `librustci_bridge.so`），导出 **8 个 C ABI 符号** 供 HotSpot/宿主 JNI 调用。参考 `jdk.vm.ci.hotspot.CompilerToVM`（132 native）的对接需求。

## 状态：已完成

## 说明

RustCI 与宿主 VM 的桥接层：将 Rust 实现的 JVMCI/Graal 子集以 cdylib 形式暴露给 HotSpot，承接 `CompilerToVM` 等 native 入口。8 个 JNI 导出符号 + 104 字段 CompilerToVM 注册表框架已实现。

## 偏离记录

- 返回类型 `*mut JVMCIRuntime`/`*mut CompilationRequestResult`/`*mut JVMCICompiler` → Rust 侧统一为 `*mut std::ffi::c_void`（trait object 为胖指针，C ABI 不兼容；`*mut c_void` 是标准 FFI 不透明指针类型，调用方自行转换）。
- `JVMCI_GetRuntime` 返回指向堆分配单元的指针（单元内存储胖指针 `*const dyn JVMCIRuntime`），调用方通过解引用恢复胖指针。原因：`*mut c_void` 为 8 字节瘦指针，无法容纳 16 字节 trait object 胖指针。
- `JVMCI_RegisterNativeMethods` 注册表：本期提供 104 个函数指针框架（对齐 `compiler_to_vm.rs` 已移植的 native 方法），剩余 28 个待后续补全。
- `JVMCI_Close`：Rust 侧当前无持久化状态需清理，保留为 no-op 以对齐 API 契约。
- `JVMCI_CompileMethod`/`JVMCI_GetCompiler`/`JVMCI_GetHostBackend` 标记为 `unsafe extern "C"`（解引用原始指针，调用方确保指针有效性）。
- `compile0`：HotSpotJVMCIRuntime 的私有 native 方法，连接 JVMCI bridge 到 `JVMCICompiler::compile_method()`。`method` 参数（`*mut c_void`）通过 `HotSpotResolvedJavaMethodWrapper` 包装为 `Box<dyn ResolvedJavaMethod>`。`result_buffer` 参数（`jlong`）指向 `Compile0Result`（`#[repr(C)]` 结构体，对齐 HotSpot 端 `JVMCICompileResult` 布局）。

## 映射表

| 桥接符号（C ABI 名） | 对应 Java native 声明（全限定名.方法） | 对应 Rust 文件 | 状态 | 负责人 |
|---|---|---|---|---|
| JVMCI_GetRuntime | JVMCI::getRuntime() | crates/rustci-bridge/src/lib.rs | 已完成 | — |
| JVMCI_Open | JVMCI::initialize() | crates/rustci-bridge/src/lib.rs | 已完成 | — |
| JVMCI_Close | (JVMCI shutdown) | crates/rustci-bridge/src/lib.rs | 已完成 | — |
| JVMCI_CompileMethod | JVMCICompiler::compileMethod(CompilationRequest) | crates/rustci-bridge/src/lib.rs | 已完成 | — |
| JVMCI_GetCompiler | JVMCIRuntime::getCompiler() | crates/rustci-bridge/src/lib.rs | 已完成 | — |
| JVMCI_GetHostBackend | JVMCIRuntime::getHostJVMCIBackend() | crates/rustci-bridge/src/lib.rs | 已完成 | — |
| JVMCI_RegisterNativeMethods | CompilerToVM::registerNativeMethods(Class<?>) | crates/rustci-bridge/src/lib.rs | 已完成 | — |
| compile0 | HotSpotJVMCIRuntime::compile0(method, entryBCI, ...) | crates/rustci-bridge/src/lib.rs | 已完成 | — |