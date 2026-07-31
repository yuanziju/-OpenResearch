# jdk.vm.ci.hotspot

## 源位置：JDK25 src.zip → `jdk.internal.vm.ci` 模块，`src/jdk.internal.vm.ci/share/classes/jdk/vm/ci/hotspot/`（不在 `/opt/graal`；共 76 文件，另含 hotspot.{amd64,aarch64,riscv64}）

## 状态：未开始

## 说明

HotSpot 特定实现层。将 meta/code 抽象绑定到 HotSpot 内部结构，核心是 `CompilerToVM`（132 个 native 方法）与 `HotSpotVMConfigStore`/`HotSpotVMConfigAccess`（VM 字段/偏移缓存）。是 RustCI 骨架中与宿主 VM 耦合最深的部分，第一期仅预填关键类型清单，native 桥接细节留待 rustci-bridge.md 协同。

## 映射表

| Java 类（全限定名） | 职责 | 对应 Rust 文件 | 状态 | 负责人 |
|---|---|---|---|---|
| jdk.vm.ci.hotspot.HotSpotJVMCIRuntime | HotSpot JVMCI 运行时实现 |  | 未开始 |  |
| jdk.vm.ci.hotspot.HotSpotCompilationRequestResult | HotSpot 编译请求结果 |  | 未开始 |  |
| jdk.vm.ci.hotspot.HotSpotCompiledCode | HotSpot 已编译代码 |  | 未开始 |  |
| jdk.vm.ci.hotspot.HotSpotCompiledNmethod | HotSpot 已编译 nmethod |  | 未开始 |  |
| jdk.vm.ci.hotspot.CompilerToVM | HotSpot Compiler→VM 桥（132 个 native 方法） |  | 未开始 |  |
| jdk.vm.ci.hotspot.HotSpotCodeCacheProvider | HotSpot 代码缓存提供者 |  | 未开始 |  |
| jdk.vm.ci.hotspot.HotSpotMetaAccessProvider | HotSpot 元数据访问提供者 |  | 未开始 |  |
| jdk.vm.ci.hotspot.HotSpotConstantReflectionProvider | HotSpot 常量反射提供者 |  | 未开始 |  |
| jdk.vm.ci.hotspot.HotSpotConstantPool | HotSpot 常量池 |  | 未开始 |  |
| jdk.vm.ci.hotspot.HotSpotResolvedJavaType | HotSpot 已解析类型基类 |  | 未开始 |  |
| jdk.vm.ci.hotspot.HotSpotResolvedObjectType | HotSpot 已解析对象类型 |  | 未开始 |  |
| jdk.vm.ci.hotspot.HotSpotResolvedJavaMethod | HotSpot 已解析方法 |  | 未开始 |  |
| jdk.vm.ci.hotspot.HotSpotResolvedJavaField | HotSpot 已解析字段 |  | 未开始 |  |
| jdk.vm.ci.hotspot.HotSpotVMConfigStore | VM 配置存储（字段/偏移量缓存） |  | 未开始 |  |
| jdk.vm.ci.hotspot.HotSpotVMConfigAccess | VM 配置访问器（按名取字段） |  | 未开始 |  |
| jdk.vm.ci.hotspot.VMField | VM 字段描述（偏移/地址/值） |  | 未开始 |  |
| jdk.vm.ci.hotspot.VMFlag | VM 标志描述 |  | 未开始 |  |

> 依赖 meta/code/runtime 子包。`CompilerToVM` 的 132 native 方法对接 rustci-bridge.md（Rust cdylib 7 符号）的子集，需主协调者协调映射范围。子包 `hotspot.{amd64,aarch64,riscv64}` 本表未展开，开工时按需增行。
