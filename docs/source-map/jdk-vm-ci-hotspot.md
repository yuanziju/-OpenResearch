# jdk.vm.ci.hotspot

## 源位置：JDK25 src.zip → `jdk.internal.vm.ci` 模块，`src/jdk.internal.vm.ci/share/classes/jdk/vm/ci/hotspot/`（不在 `/opt/graal`；共 76 文件，另含 hotspot.{amd64,aarch64,riscv64}）

## 状态：已完成

## 说明

HotSpot 特定实现层。将 meta/code 抽象绑定到 HotSpot 内部结构，核心是 `CompilerToVM`（132 个 native 方法）与 `HotSpotVMConfigStore`/`HotSpotVMConfigAccess`（VM 字段/偏移缓存）。是 RustCI 骨架中与宿主 VM 耦合最深的部分，第一期仅预填关键类型清单，native 桥接细节留待 rustci-bridge.md 协同。

## 偏离记录

- **Java class → Rust trait**：`HotSpotJVMCIRuntime`、`HotSpotCodeCacheProvider`、`HotSpotMetaAccessProvider`、`HotSpotConstantReflectionProvider`、`HotSpotMemoryAccessProvider`、`HotSpotConstantPool`、`HotSpotResolvedJavaType`、`HotSpotResolvedObjectType`、`HotSpotResolvedJavaMethod`、`HotSpotCompiledCode`、`HotSpotCompiledNmethod`、`HotSpotInstalledCode`、`HotSpotNmethod` 等 Java 类/接口 → Rust trait（1:1 镜像方法签名，具体实现留待 HotSpot 后端绑定）。
- **Java final class → Rust struct**：`HotSpotCompilationRequestResult`、`HotSpotCompilationRequest`、`HotSpotSpeculationLog`、`HotSpotResolvedObjectTypeImpl`、`HotSpotResolvedJavaMethodImpl` → Rust struct（只持有元数据字段，native 方法声明移至 `CompilerToVM`）。
- **CompilerToVM**：Java `final class CompilerToVM` 含 132 个 native 方法 → Rust `extern "C"` 块（仅声明函数签名，不提供实现）。方法名 camelCase→snake_case。Java 类型映射：`jobject`→`*mut c_void`、`jstring`→`*const c_char`、`jboolean`→`u8`、`jlong`→`i64`、`jint`→`i32`、`jbyte`→`i8`。
- **HotSpotResolvedJavaField**：本期未移植（第一阶段范围外）。
- **HotSpotVMConfigStore / HotSpotVMConfigAccess / VMField / VMFlag**：本期未移植（第一阶段范围外，需 VM 内部字段偏移缓存）。
- **子包 hotspot.{amd64,aarch64,riscv64}**：本期未移植（架构相关，留待后续）。

## 映射表

| Java 类（全限定名） | 职责 | 对应 Rust 文件 | 状态 | 负责人 |
|---|---|---|---|---|
| jdk.vm.ci.hotspot.HotSpotJVMCIRuntime | HotSpot JVMCI 运行时实现 | crates/rustci-vm-ci/src/hotspot/hotspot_jvmci_runtime.rs | 已完成 |  |
| jdk.vm.ci.hotspot.HotSpotCompilationRequestResult | HotSpot 编译请求结果 | crates/rustci-vm-ci/src/hotspot/hotspot_compilation_request_result.rs | 已完成 |  |
| jdk.vm.ci.hotspot.HotSpotCompiledCode | HotSpot 已编译代码 | crates/rustci-vm-ci/src/hotspot/hotspot_compiled_code.rs | 已完成 |  |
| jdk.vm.ci.hotspot.HotSpotCompiledNmethod | HotSpot 已编译 nmethod | crates/rustci-vm-ci/src/hotspot/hotspot_compiled_nmethod.rs | 已完成 |  |
| jdk.vm.ci.hotspot.CompilerToVM | HotSpot Compiler→VM 桥（132 个 native 方法） | crates/rustci-vm-ci/src/hotspot/compiler_to_vm.rs | 已完成 |  |
| jdk.vm.ci.hotspot.HotSpotCodeCacheProvider | HotSpot 代码缓存提供者 | crates/rustci-vm-ci/src/hotspot/hotspot_code_cache_provider.rs | 已完成 |  |
| jdk.vm.ci.hotspot.HotSpotMetaAccessProvider | HotSpot 元数据访问提供者 | crates/rustci-vm-ci/src/hotspot/hotspot_meta_access_provider.rs | 已完成 |  |
| jdk.vm.ci.hotspot.HotSpotConstantReflectionProvider | HotSpot 常量反射提供者 | crates/rustci-vm-ci/src/hotspot/hotspot_constant_reflection_provider.rs | 已完成 |  |
| jdk.vm.ci.hotspot.HotSpotConstantPool | HotSpot 常量池 | crates/rustci-vm-ci/src/hotspot/hotspot_constant_pool.rs | 已完成 |  |
| jdk.vm.ci.hotspot.HotSpotResolvedJavaType | HotSpot 已解析类型基类 | crates/rustci-vm-ci/src/hotspot/hotspot_resolved_java_type.rs | 已完成 |  |
| jdk.vm.ci.hotspot.HotSpotResolvedObjectType | HotSpot 已解析对象类型 | crates/rustci-vm-ci/src/hotspot/hotspot_resolved_object_type.rs | 已完成 |  |
| jdk.vm.ci.hotspot.HotSpotResolvedJavaMethod | HotSpot 已解析方法 | crates/rustci-vm-ci/src/hotspot/hotspot_resolved_java_method.rs | 已完成 |  |
| jdk.vm.ci.hotspot.HotSpotNmethod | HotSpot nmethod | crates/rustci-vm-ci/src/hotspot/hotspot_nmethod.rs | 已完成 |  |
| jdk.vm.ci.hotspot.HotSpotInstalledCode | HotSpot 已安装代码 | crates/rustci-vm-ci/src/hotspot/hotspot_installed_code.rs | 已完成 |  |
| jdk.vm.ci.hotspot.HotSpotSpeculationLog | HotSpot 推测日志 | crates/rustci-vm-ci/src/hotspot/hotspot_speculation_log.rs | 已完成 |  |
| jdk.vm.ci.hotspot.HotSpotMemoryAccessProvider | HotSpot 内存访问提供者 | crates/rustci-vm-ci/src/hotspot/hotspot_memory_access_provider.rs | 已完成 |  |
| jdk.vm.ci.hotspot.HotSpotResolvedObjectTypeImpl | HotSpot 已解析对象类型实现 | crates/rustci-vm-ci/src/hotspot/hotspot_resolved_object_type_impl.rs | 已完成 |  |
| jdk.vm.ci.hotspot.HotSpotResolvedJavaMethodImpl | HotSpot 已解析方法实现 | crates/rustci-vm-ci/src/hotspot/hotspot_resolved_java_method_impl.rs | 已完成 |  |
| jdk.vm.ci.hotspot.HotSpotCompilationRequest | HotSpot 编译请求 | crates/rustci-vm-ci/src/hotspot/hotspot_compilation_request.rs | 已完成 |  |

> 依赖 meta/code/runtime 子包。`CompilerToVM` 的 132 native 方法对接 rustci-bridge.md（Rust cdylib 7 符号）的子集，需主协调者协调映射范围。子包 `hotspot.{amd64,aarch64,riscv64}` 本表未展开，开工时按需增行。