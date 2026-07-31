# 第一期任务清单 — GraalVM → Rust 移植项目

> 状态：Phase-1 任务拆分 v1
> 主协调者维护本文件；子代理（worker / verifier / fixer / re-verifier）按本文件认领任务。
> 配套：`/workspace/spec.md`（架构与纪律）、`/workspace/docs/source-map/README.md`（模块索引与回写规则）。
> 参考源只读：`/opt/graal`（oracle/graal HEAD）、`/opt/jdk-vm-ci-src`（JDK25 `jdk.vm.ci` Java 源，213 文件，含 `CompilerToVM.java` 132 native 声明、`HotSpotJVMCIRuntime.compile0`、`JVMCICompiler.compileMethod`，路径前缀 `jdk.internal.vm.ci/jdk/vm/ci/`）、`/opt/jdk-src`（openjdk/jdk master，**确认不含 jvmci C++ 源**，JEP410 演进，仅作其他 hotspot 参考）。`/opt` 下禁止 `Grep`/`Glob`，统一 `RunCommand`（`rg`/`find`/`wc`/`head`）访问。

---

## 1. 概述

第一期在单一 `dev` 分支上并行推进两条互不阻塞的工作路：

- **路线 A — RustCI 骨架**：以最小子集打通端到端编译链路。镜像 `jdk.vm.ci` 的 `meta` / `code` / `runtime` / `hotspot` 核心接口子集，再以 `rustci-bridge` cdylib 导出 7 个 JNI 符号，把 `compile0` 调度链路接通至 `JVMCICompiler.compileMethod`。**本期不实现 Graal 图编译**（图编译属第二期 compiler 子包移植），但调度链路、句柄传递、缓冲区回写、类型契约必须完整且与 Java 侧 1:1 对齐。
- **路线 B — `util/json` 完整移植**：`jdk.graal.compiler.util.json` 8 类 / 1685 LoC + 3 测试 / 616 LoC，非 MVP，忠实完整移植。

架构基调（务必遵循）：libgraal 风格，Rust `cdylib` 导出 JNI 符号供 HotSpot 经 `registerNativeMethods` 链接，**跳过 Java JVMCI 中间层**。Rust 侧接口 1:1 镜像 `jdk.vm.ci.*`，唯一变更为顶层模块名 `JVMCI → RustCI`（`jdk.vm.ci.meta` → `rustci_vm_ci::meta` 之类的命名空间映射）。不合并/拆分接口，不改继承与组合关系，不做 Rust 惯用化重构。

纪律（spec §7 重申）：禁偷懒实现（空实现 / 占位 / simple stub 全禁）；禁 TODO 注释（除非原 Java 源自带）；一切谨遵原 Java 实现，行为/边界/异常路径一致；不做未要求的重构、注释、docstring、类型标注；涉及 native 签名以 `/opt/jdk-src` C++ 源为核对基准，C++ 源缺失时基于 Java 侧 `native` 声明反推并标注（见 §6）。

---

## 2. cargo workspace 结构

Rust 代码置于 `/workspace/rustci/`（新建 cargo workspace）。crate 划分遵循"一个 Java 模块 / 包 → 一个 Rust crate"原则，依赖方向严格单向。

### 2.1 workspace 成员清单

```
/workspace/rustci/
├── Cargo.toml                      # workspace 根
└── crates/
    ├── rustci-collections/         # 镜像 org.graalvm.collections（sdk 模块）
    ├── rustci-util-json/           # 移植 jdk.graal.compiler.util.json
    ├── rustci-vm-ci/               # 镜像 jdk.vm.ci（meta/code/runtime/hotspot/services 子模块）
    └── rustci-bridge/              # cdylib，导出 7 个 JNI 符号
```

依赖图：`rustci-bridge → rustci-vm-ci → rustci-collections`；`rustci-util-json → rustci-collections`。`rustci-util-json` 与 `rustci-vm-ci` 互不依赖（两路并行）。

### 2.2 workspace 根 `Cargo.toml`

```toml
[workspace]
resolver = "2"
members = [
    "crates/rustci-collections",
    "crates/rustci-util-json",
    "crates/rustci-vm-ci",
    "crates/rustci-bridge",
]

[workspace.package]
edition = "2021"
version = "0.1.0"
# 上游多许可证：org.graalvm.collections = UPL-1.0；jdk.vm.ci / util.json = GPL-2.0 + Classpath Exception。
# 各 crate 保留上游 Oracle 版权与许可证头（逐文件），此处仅声明包级 SPDX。
license = "UPL-1.0 OR GPL-2.0-with-classpath-exception"
repository = "https://github.com/graalvm/graal"   # 上游溯源用
```

### 2.3 各 crate `Cargo.toml` 草案

#### `crates/rustci-collections/Cargo.toml`
```toml
[package]
name = "rustci-collections"
version.workspace = true
edition.workspace = true
license.workspace = true
description = "Rust 镜像 org.graalvm.collections；第一期：EconomicMap 族 trait + BTreeMap 参考实现"

[dependencies]
# 无外部依赖（第一期）；完整 EconomicMapImpl 移植（后续任务）可能引入 unsafe 内部实现

[lib]
name = "rustci_collections"
path = "src/lib.rs"
```

#### `crates/rustci-util-json/Cargo.toml`
```toml
[package]
name = "rustci-util-json"
version.workspace = true
edition.workspace = true
license.workspace = true
description = "Rust 移植 jdk.graal.compiler.util.json（8 类，完整移植）"

[dependencies]
rustci-collections = { path = "../rustci-collections" }

[lib]
name = "rustci_util_json"
path = "src/lib.rs"
```

#### `crates/rustci-vm-ci/Cargo.toml`
```toml
[package]
name = "rustci-vm-ci"
version.workspace = true
edition.workspace = true
license.workspace = true
description = "Rust 镜像 jdk.vm.ci（JVMCI→RustCI）；第一期核心接口子集"

[dependencies]
rustci-collections = { path = "../rustci-collections" }

[lib]
name = "rustci_vm_ci"
path = "src/lib.rs"
# 子模块（src/ 下目录）：meta/ code/ runtime/ hotspot/ services/
```

#### `crates/rustci-bridge/Cargo.toml`
```toml
[package]
name = "rustci-bridge"
version.workspace = true
edition.workspace = true
license.workspace = true
description = "Rust cdylib 桥接层，导出 7 个 JNI 符号供 HotSpot 链接"

[dependencies]
rustci-vm-ci = { path = "../rustci-vm-ci" }
rustci-collections = { path = "../rustci-collections" }
# JNI FFI：优先复用社区 jni crate 的 FFI 类型（JNIEnv/jclass/jlong/jbyteArray 等）。
# 若 jni crate API 与 spec §9.1 草案不匹配，允许在 crate 内 vendor 最小 FFI 类型定义（不算 stub）。
jni = "0.21"

[lib]
name = "rustci_bridge"
crate-type = ["cdylib", "rlib"]   # cdylib 供 HotSpot dlopen；rlib 供单测链接
path = "src/lib.rs"
```

> 说明：T1 仅产出上述 4 个 `Cargo.toml` + 各 crate 的 `src/lib.rs`（空根 + 模块声明，无任何业务实现），保证 `cargo build` / `cargo test` 通过。空 crate 根属骨架必需，不属"占位实现"。

---

## 3. 任务清单

### 3.1 总表

| ID | 标题 | 路线 | 前置 | 验收梯队 | 回写 source-map MD |
|----|------|------|------|----------|--------------------|
| T1 | cargo workspace 骨架 + 4 crate 空根打通编译 | 共用 | — | 第 1 梯（cargo test） | 无（README 备注 workspace 已建） |
| T2 | rustci-collections 暂替层（EconomicMap 族 trait + BTreeMap 参考实现） | B 前置 | T1 | 第 1 梯（cargo test） | `jdk-graal-compiler-util-collections.md` |
| T3 | rustci-util-json 完整移植（8 类 + 3 测试对拍） | B | T2 | 第 1 梯（cargo test） | `jdk-graal-compiler-util-json.md` |
| T4 | rustci-vm-ci `meta` 核心接口子集（~20 接口 + final class） | A | T1 | 第 2 梯（T6 集成） | `jdk-vm-ci-meta.md` |
| T5 | rustci-vm-ci `code` 核心接口子集（~15 接口 + final class） | A | T4 | 第 2 梯（T6 集成） | `jdk-vm-ci-code.md` |
| T6 | rustci-vm-ci `runtime` 核心接口（5 类） | A | T4, T5 | 第 2 梯（T6 集成） | `jdk-vm-ci-runtime.md` |
| T7 | rustci-vm-ci `hotspot` 核心子集 + CompilerToVM ~20 native 声明 | A | T6 | 第 3 梯（T9 集成） | `jdk-vm-ci-hotspot.md` |
| T8 | rustci-bridge cdylib 7 符号导出 + registerNativeMethods 注册表 | A | T7 | 第 3 梯（T9 集成） | `rustci-bridge.md` |
| T9 | compile0 调度链路打通（compile0 → JVMCICompiler.compileMethod 框架） | A | T8 | 第 3 梯（T9 集成） | `rustci-bridge.md` + `jdk-vm-ci-hotspot.md` |

### 3.2 每任务详情

---

#### T1 — cargo workspace 骨架 + 4 crate 空根打通编译

- **范围**
  - 新建 `/workspace/rustci/Cargo.toml`（见 §2.2）
  - 新建 4 个 crate 目录与 `Cargo.toml`（见 §2.3）：`crates/rustci-collections`、`crates/rustci-util-json`、`crates/rustci-vm-ci`、`crates/rustci-bridge`
  - 每个 crate 一个 `src/lib.rs`：仅 `#![...]` 属性 + 模块声明（如 `pub mod meta;`，对应模块文件可暂为空 `mod` 或留到后续任务），**不含任何业务实现**
  - `/workspace/.gitignore` 追加 `rustci/target/`、`rustci/Cargo.lock`（视仓库策略）
- **依赖**：无
- **原 Java 实现参考**：无（纯 Rust 骨架）
- **验收标准**
  - `cd /workspace/rustci && cargo build --workspace` 成功
  - `cargo test --workspace` 成功（无测试，仅编译通过）
  - `cargo fmt --check` 通过
  - `rustci-bridge` crate 产出 `cdylib`（`cargo build -p rustci-bridge` 后 `target/debug/librustci_bridge.so` 存在；符号集暂为空不扣分）
- **worker 闭环粒度**：纯建骨架，单一 worker 一轮即可；verifier 仅查 `cargo build/test/fmt`。闭环小，适合快速开局。

---

#### T2 — rustci-collections 暂替层（EconomicMap 族 trait + BTreeMap 参考实现）

- **范围**
  - `crates/rustci-collections/src/lib.rs` + 子模块文件，移植以下 Java 接口为 Rust trait（**1:1 镜像方法签名**，不省略 default method）：
    - `org.graalvm.collections.UnmodifiableEconomicMap<K,V>`（129 LoC）
    - `org.graalvm.collections.EconomicMap<K,V>`（314 LoC，含 `create`/`of`/`emptyMap`/`wrapMap`/`emptyCursor` 等静态工厂与 `putIfAbsent`/`putAll`/`trimToSize`/`computeIfAbsent` 等 default）
    - `org.graalvm.collections.UnmodifiableMapCursor<K,V>`（70 LoC）
    - `org.graalvm.collections.MapCursor<K,V>`（68 LoC）
    - `org.graalvm.collections.Equivalence`（132 LoC，含 `DEFAULT`/`IDENTITY`/`IDENTWITH_SYSTEM_HASHCODE` 枚举式常量与 `equals`/`hashCode`）
    - `org.graalvm.collections.Pair<L,R>`（175 LoC）
    - `org.graalvm.collections.UnmodifiableEconomicSet` / `EconomicSet`（139 + 240 LoC，最小接口面，供后续）
  - 提供一个 **BTreeMap 参考实现**（命名如 `BTreeEconomicMap<K,V>`），完整实现 `EconomicMap` trait 全部方法（含 `MapCursor` 迭代），**非占位**：`get`/`getOrDefault`/`put`/`putIfAbsent`/`putAll`/`remove`/`containsKey`/`size`/`isEmpty`/`clear`/`cursor`/`trimToSize`(no-op 合法)/`getEquivalenceStrategy`/静态工厂等行为与 Java `EconomicMap` 语义对齐
  - 同理 `BTreeEconomicSet` 实现 `EconomicSet`
  - 单元测试对拍 Java 语义：null value 支持、null key 拒绝（panic 或 Result，对齐 Java `UnsupportedOperationException`）、`computeIfAbsent` 惰性求值、cursor 遍历顺序、`putAll` 合并、`of(...)` 工厂
- **依赖**：T1
- **原 Java 实现参考**：`/opt/graal/sdk/src/org.graalvm.collections/src/org/graalvm/collections/` 下 `EconomicMap.java`、`UnmodifiableEconomicMap.java`、`MapCursor.java`、`UnmodifiableMapCursor.java`、`Equivalence.java`、`Pair.java`、`EconomicSet.java`、`UnmodifiableEconomicSet.java`、`EmptyMap.java`、`EmptySet.java`
- **验收标准**
  - `cargo test -p rustci-collections` 全绿
  - trait 方法集与 Java 接口 1:1（verifier 逐方法对照 Java 源签名）
  - `BTreeEconomicMap` 行为对拍：覆盖 null value、null key 拒绝、`computeIfAbsent`、cursor、`putAll`、工厂
  - **明确标注**：完整 `EconomicMapImpl`（941 LoC，分段数组+哈希压缩实现）移植列为后续任务（非本期），在 `jdk-graal-compiler-util-collections.md` 偏离记录列明"BTreeMap 暂替，待后续 EconomicMapImpl 完整移植"
- **worker 闭环粒度**：trait 镜像 + 一个完整 BTreeMap 实现 + 测试，规模适中（~900 LoC Java 参考 → Rust trait+impl+test），单 worker 一轮闭环可控；verifier 重点查方法集完整性与 null 语义。
- **回写**：`docs/source-map/jdk-graal-compiler-util-collections.md`（映射表填 Rust 文件列 + 状态 + 偏离记录）

---

#### T3 — rustci-util-json 完整移植（8 类 + 3 测试对拍）

- **范围**：完整移植 `jdk.graal.compiler.util.json` 全部 8 类（1685 LoC）：
  - `jdk.graal.compiler.util.json.JsonWriter`（411 LoC）
  - `jdk.graal.compiler.util.json.JsonPrettyWriter`（81 LoC，`JsonWriter` 子类）
  - `jdk.graal.compiler.util.json.JsonParser`（571 LoC）
  - `jdk.graal.compiler.util.json.JsonParserException`（70 LoC）
  - `jdk.graal.compiler.util.json.JsonBuilder`（329 LoC，含嵌套 `ObjectBuilder`/`ArrayBuilder`/`ValueBuilder`）
  - `jdk.graal.compiler.util.json.JsonFormatter`（95 LoC）
  - `jdk.graal.compiler.util.json.JsonPrintable`（39 LoC，接口）
  - `jdk.graal.compiler.util.json.JsonPrinter<T>`（89 LoC，接口）
  - Java `Writer`/`Reader`/`BufferedReader`/`CharBuffer` 适配为 Rust `io::Write`/`io::Read`/`BufRead`/等效字符流抽象；`IOException` → `io::Error`；`StringReader` 用 `Cursor<Vec<u8>>` 或等效
  - `JsonParser` 返回的 `EconomicMap`/`List` 结构用 T2 的 `rustci_collections::EconomicMap` trait + `BTreeEconomicMap`（`List` 用 `Vec`）
  - 移植上游 3 个测试（616 LoC）：`JsonParserTest`、`JsonWriterTest` 等（worker 在 `/opt/graal/compiler/src/jdk.graal.compiler.test/` 下定位，见参考路径），断言逐条对齐
- **依赖**：T2（EconomicMap trait）
- **原 Java 实现参考**：
  - 源：`/opt/graal/compiler/src/jdk.graal.compiler/src/jdk/graal/compiler/util/json/*.java`（8 文件）
  - 测试：`/opt/graal/compiler/src/jdk.graal.compiler.test/src/jdk/graal/compiler/util/json/` 下（worker 用 `find /opt/graal -path "*test*util/json*"` 定位）
- **验收标准**
  - `cargo test -p rustci-util-json` 全绿，3 个移植测试断言与 Java 一致
  - 8 类全部 1:1 移植，方法集、可见性、异常路径对齐（`JsonParser` 的转义/数字/utf8 边界、`JsonWriter` 的 quote/separator 状态机、`JsonBuilder` 嵌套闭合校验）
  - 行为对拍：用相同输入跑 Java 与 Rust，输出字节级一致（至少覆盖对象/数组/嵌套/数字/字符串转义/`null`/`true`/`false`/pretty vs compact）
- **worker 闭环粒度**：8 类 + 3 测试 = 单 worker 可控的最大完整移植单元；建议子任务内部按"JsonWriter → JsonPrettyWriter → JsonParser+Exception → JsonBuilder → JsonFormatter → JsonPrintable/JsonPrinter"顺序，但整体作为一个闭环交付与验收。verifier 重点查解析器边界与建造者嵌套校验。
- **回写**：`docs/source-map/jdk-graal-compiler-util-json.md`

---

#### T4 — rustci-vm-ci `meta` 核心接口子集

- **范围**：在 `crates/rustci-vm-ci/src/meta/` 下移植 `jdk.vm.ci.meta` 核心接口与 final class 子集（~20 项，spec §3.1）：
  - 根接口：`JavaType`、`JavaMethod`、`JavaField`、`JavaValue`、`Constant`、`JavaConstant`、`Annotated`、`ModifiersProvider`、`PlatformKind`、`InvokeTarget`
  - 已解析接口：`ResolvedJavaType`、`ResolvedJavaMethod`、`ResolvedJavaField`、`Signature`、`ConstantPool`
  - 提供者接口：`MetaAccessProvider`、`ConstantReflectionProvider`、`MemoryAccessProvider`、`MethodHandleAccessProvider`、`ProfilingInfo`、`SpeculationLog`
  - final class 子集：`UnresolvedJavaType`、`UnresolvedJavaMethod`、`UnresolvedJavaField`、`Assumptions`、`DefaultProfilingInfo`、`ExceptionHandler`、`JavaTypeProfile`、`JavaMethodProfile`、`AnnotationData`、`EnumData`、`ErrorData`、`SerializableConstant`、`VMConstant`
  - **1:1 镜像**：接口→Rust trait，final class→struct + impl，继承/组合关系保留（如 `ResolvedJavaType extends JavaType` → trait 继承）；`null` 语义用 `Option<T>` 或对齐 Java 的 nullable 引用语义（worker 在 MD 偏离记录说明取舍）
- **依赖**：T1（workspace）；与 T2/T3 互不依赖（可并行）
- **原 Java 实现参考**：JDK25 `src.zip` → `jdk.internal.vm.ci` 模块，`src/jdk.internal.vm.ci/share/classes/jdk/vm/ci/meta/`（**不在 `/opt/graal`**；见 §6 未确认项 #1，worker 须先落 JDK25 源）
- **验收标准**
  - `cargo test -p rustci-vm-ci` 编译通过（本期 meta 接口多为 trait 声明，测试以"trait 可被 mock 实现并通过编译"为主）
  - trait 方法集与 Java 接口 1:1（verifier 逐方法对照）
  - final class 字段与方法完整（非占位）
- **worker 闭环粒度**：~20 接口/class，trait 镜像为主，单 worker 一轮；verifier 重点查 trait 继承链与方法集。属于第 2 梯，T4-T5-T6 完成后做集成验收（见 §5）。
- **回写**：`docs/source-map/jdk-vm-ci-meta.md`

---

#### T5 — rustci-vm-ci `code` 核心接口子集

- **范围**：在 `crates/rustci-vm-ci/src/code/` 下移植 `jdk.vm.ci.code` 核心子集（~15 项）：
  - 接口：`CodeCacheProvider`、`CompilationRequestResult`、`CompiledCode`、`RegisterConfig`、`StackIntrospection`、`InspectedFrame`、`InspectedFrameVisitor`、`ValueKindFactory`、`CPUFeatureName`
  - final class：`Register`、`RegisterValue`、`StackSlot`、`DebugInfo`、`BytecodeFrame`、`Call`、`ConstantReference`、`DataPatch`、`DataSectionReference`、`Mark`、`Location`、`StackLockValue`、`VirtualObject`、`RegisterSaveLayout`、`ExceptionHandler`（code 层，与 meta 同名类区分）、`ImplicitExceptionDispatch`、`InvalidInstalledCodeException`、`ValueUtil`
  - 子包 `code.site` / `code.stack` 本期不展开，留空 `mod` 占位（不算占位实现，仅模块声明）
- **依赖**：T4（meta，code 接口引用 meta 类型如 `JavaMethod`/`ResolvedJavaMethod`）
- **原 Java 实现参考**：JDK25 `src.zip` → `jdk.internal.vm.ci`，`src/jdk.internal.vm.ci/share/classes/jdk/vm/ci/code/`（不在 `/opt/graal`；§6 #1）
- **验收标准**
  - `cargo test -p rustci-vm-ci` 编译通过
  - 接口/class 1:1 镜像，与 meta 的引用关系正确
- **worker 闭环粒度**：同 T4，trait+struct 镜像，单 worker 一轮。
- **回写**：`docs/source-map/jdk-vm-ci-code.md`

---

#### T6 — rustci-vm-ci `runtime` 核心接口

- **范围**：在 `crates/rustci-vm-ci/src/runtime/` 下移植 `jdk.vm.ci.runtime` 全部 5 类：
  - `JVMCI`（→ `rustci_vm_ci::runtime::JVMCI`，运行时获取/初始化入口）
  - `JVMCIBackend`（meta/code/services 提供者聚合）
  - `JVMCICompiler`（编译器接口，含 `compileMethod` 签名 — T9 compile0 的下游对接点）
  - `JVMCICompilerFactory`（编译器工厂选择）
  - `JVMCIRuntime`（宿主访问入口）
  - 命名映射：`JVMCI*` → `RustCI*`（spec §2.2），但 trait/类型名保留 `JVMCI` 前缀以 1:1 镜像？**决议**：模块名 `JVMCI→RustCI`，类型名保留 `JVMCI*` 原名以严格 1:1（worker 在 MD 偏离记录确认；若主协调者要求类型名亦改，回填此处）。
- **依赖**：T4, T5
- **原 Java 实现参考**：JDK25 `src.zip` → `jdk.internal.vm.ci`，`src/jdk.internal.vm.ci/share/classes/jdk/vm/ci/runtime/`（不在 `/opt/graal`；§6 #1）
- **验收标准**
  - `cargo test -p rustci-vm-ci` 编译通过
  - 5 类 trait/struct 1:1 镜像，`JVMCICompiler.compileMethod` 签名与 Java 一致（T9 对接基准）
  - **集成验收**（第 2 梯）：`cargo test --workspace` + 尝试 `mx build`（§6 #6，跑不动降级并记录）
  - 每 3 任务一次"是否偏离原 Java 实现"彻底复核（覆盖 T4-T5-T6）
- **worker 闭环粒度**：5 类，单 worker 一轮；本任务收尾触发第 2 梯集成验收。
- **回写**：`docs/source-map/jdk-vm-ci-runtime.md`

---

#### T7 — rustci-vm-ci `hotspot` 核心子集 + CompilerToVM ~20 native 声明

- **范围**：在 `crates/rustci-vm-ci/src/hotspot/` 下移植 `jdk.vm.ci.hotspot` 核心子集：
  - `HotSpotJVMCIRuntime`（含 `compile0` native 声明 — T9 的 Java 侧契约来源）
  - `HotSpotCompiledCode`、`HotSpotCompiledNmethod`、`HotSpotCompilationRequestResult`
  - `HotSpotVMConfigStore`、`HotSpotVMConfigAccess`、`VMField`、`VMFlag`
  - `CompilerToVM`：从 132 个 native 方法中**挑选编译/install/配置相关 ~20 个**（spec §3.1），在 Rust 侧声明为对应 `extern "C"` / trait 抽象。**本期仅声明契约与 Rust 侧 trait 转译，不实现 C++ 侧**（C++ 侧由 HotSpot 提供，RustCI 通过 bridge 调入/被调）
  - `HotSpotCodeCacheProvider`、`HotSpotMetaAccessProvider`、`HotSpotConstantReflectionProvider`、`HotSpotConstantPool`、`HotSpotResolvedJavaType`、`HotSpotResolvedObjectType`、`HotSpotResolvedJavaMethod`、`HotSpotResolvedJavaField`（接口/final class 子集，按 Java 源 1:1）
- **依赖**：T6（runtime）；T4, T5（meta/code 类型）
- **原 Java 实现参考**：JDK25 `src.zip` → `jdk.internal.vm.ci`，`src/jdk.internal.vm.ci/share/classes/jdk/vm/ci/hotspot/`（不在 `/opt/graal`；§6 #1）。C++ 侧 `compilerToVM.cpp` **确认在 openjdk/jdk master 与 oracle/graal 均不存在**（spec §10.3 / JEP410），基于 Java 侧 `CompilerToVM.java` 的 132 native 声明反推 C++ 契约，标注未确认项（§6 #2）。
- **验收标准**
  - `cargo test -p rustci-vm-ci` 编译通过
  - ~20 个 CompilerToVM native 方法签名与 Java 侧 `native` 声明 1:1（参数类型映射、返回类型映射在 MD 偏离记录列明）
  - 每个未确认的 C++ 签名在 `jdk-vm-ci-hotspot.md` 偏离记录标注"基于 Java 声明反推，C++ 待核对"
- **worker 闭环粒度**：hotspot 子集 + ~20 native 声明，规模偏大但以声明/镜像为主，单 worker 一轮；verifier 重点查 native 签名映射与未确认标注。
- **回写**：`docs/source-map/jdk-vm-ci-hotspot.md`

---

#### T8 — rustci-bridge cdylib 7 符号导出 + registerNativeMethods 注册表

- **范围**：在 `crates/rustci-bridge/src/` 下实现 spec §9 的 7 个 JNI 符号为 `#[no_mangle] pub extern "C"` 函数：
  - `JVMCI_OnLoad`（cdylib 加载入口）
  - `RustCIBridge_initialize`（RustCI 桥接初始化）
  - `attachThread` / `detachThread`（线程附加/分离）
  - `compile0`（编译分发入口，spec §9.1 草案签名；T9 填充调度体）
  - `installCode0`（代码安装）
  - `readConfiguration`（配置读取）
  - `registerNativeMethods`（动态链接 Java 桥接类 native 方法到 `Java_<pkg>_<Class>_<method>` JNI 符号）
  - 签名基于 Java 侧 `native` 声明反推（§6 #2/#3/#4）；`compile0` 以 spec §9.1 草案为准
  - `registerNativeMethods` 实现一个注册表（`JNINativeMethod` 数组），把 7 符号映射到 Java 桥接类方法名
  - **本期 compile0/installCode0 等的函数体可为"调度框架"**：参数解析 + 调用 rustci-vm-ci 侧对应 trait 方法（T9 细化 compile0）；**不得为空函数或直接返回常数的 stub**，必须有真实的参数→下游 trait 调用→返回值数据流
- **依赖**：T7（hotspot/CompilerToVM 契约）；T6（runtime）
- **原 Java 实现参考**：无直接 Java 源（Rust 原生桥接层）。反推参考：
  - Java 侧 `CompilerToVM.java` 的 `native` 声明（JDK25 src.zip，§6 #1）
  - `HotSpotJVMCIRuntime.compile0` Java native 声明
  - `/opt/graal/compiler/src/jdk.graal.compiler.libgraal/src/jdk/graal/compiler/libgraal/LibGraalEntryPoints.java`（含 `compileMethodInLibgraal` 测试入口，spec §9.1 草案参考）
  - `/opt/graal/truffle/src/com.oracle.truffle.runtime/src/com/oracle/truffle/runtime/hotspot/libgraal/LibGraal*.java`（Truffle 侧 libgraal 桥接参考，了解 isolate/handle 风格）
- **验收标准**
  - `cargo build -p rustci-bridge` 产出 `librustci_bridge.so`（cdylib）
  - `nm -D target/debug/librustci_bridge.so | grep -E "JVMCI_OnLoad|RustCIBridge_initialize|attachThread|detachThread|compile0|installCode0|readConfiguration|registerNativeMethods"` 7 符号全部可见（T8 后 `compile0` 符号存在但调度体可在 T9 细化）
  - `cargo test -p rustci-bridge` 通过（单测覆盖注册表映射、参数解析）
  - 签名偏离 spec §9.1 草案处，在 `rustci-bridge.md` 偏离记录列明原因与 Java 参考
- **worker 闭环粒度**：7 符号 + 注册表，FFI 密集，单 worker 一轮；verifier 重点查符号可见性与签名契约。
- **回写**：`docs/source-map/rustci-bridge.md`

---

#### T9 — compile0 调度链路打通

- **范围**：填充 `compile0` 函数体，接通端到端调度链路（spec §3.1 "最小子集打通编译链路"）：
  - JNI 入参解析：`isolate_thread`/`method_handle`/`entry_bci`/`compile_state`/`compile_id`/`options`/`failure_buf`/`time_mem_buf` → Rust 侧类型
  - 调用 `rustci_vm_ci::runtime::JVMCICompiler::compileMethod`（T6 声明的 trait），通过 `HotSpotJVMCIRuntime`（T7）取得 compiler 实例
  - 结果回写：`failure_buf`（失败信息）、`time_mem_buf`（编译耗时/内存）
  - 返回 `installedCode` handle（`jlong`）
  - **本期不实现 Graal 图编译**（图编译属第二期）；`compileMethod` 的具体 Graal 实现本期以"调度至 runtime/compiler 接口、句柄与缓冲区完整传递、类型契约对齐 Java `HotSpotJVMCIRuntime.compile0`"为准。verifier 验收调度链路完整性，不验收图编译正确性
  - `JVMCI_OnLoad` 内调用 `RustCIBridge_initialize` + `registerNativeMethods`，完成 cdylib 加载到符号注册的完整初始化路径
- **依赖**：T8（7 符号 + 注册表）；T7（hotspot/runtime）；T6（JVMCICompiler）
- **原 Java 实现参考**：
  - `HotSpotJVMCIRuntime.compile0` Java native 声明（JDK25 src.zip，§6 #1）
  - `LibGraalEntryPoints.compileMethodInLibgraal`（`/opt/graal/compiler/src/jdk.graal.compiler.libgraal/src/jdk/graal/compiler/libgraal/LibGraalEntryPoints.java`）
  - spec §9.1 草案
- **验收标准**
  - `cargo test --workspace` 全绿
  - `compile0` 单测：mock JNI 入参 → 验证参数解析、trait 调用、缓冲区回写、handle 返回的数据流完整（非 stub）
  - **集成验收**（第 3 梯）：`cargo test --workspace` + 尝试 `mx build`（§6 #6，跑不动降级为 `cargo test` + `nm` 符号检查 + 调度链路单测，记录降级原因）
  - 每 3 任务一次"是否偏离原 Java 实现"彻底复核（覆盖 T7-T8-T9）
  - `compile0` 签名与 spec §9.1 / Java 侧 `compile0` 声明双向核对定稿（spec §10.4），偏离入 `rustci-bridge.md` 偏离记录
- **worker 闭环粒度**：调度链路接通，跨 bridge + vm-ci，单 worker 一轮但 verifier 验收最严（端到端契约）；本任务收尾触发第 3 梯集成验收，标志第一期完成。
- **回写**：`docs/source-map/rustci-bridge.md`（compile0 行）+ `docs/source-map/jdk-vm-ci-hotspot.md`（compile0 调度）

---

## 4. 任务依赖图（文字版 DAG）

```
T1 (workspace 骨架)
├── T2 (collections 暂替) ──┐
│                            ├── T3 (util/json 完整移植)         [路线 B 完成]
│
└── T4 (vm-ci meta) ──┬── T5 (vm-ci code) ──┬── T6 (vm-ci runtime) ──┬── T7 (vm-ci hotspot + CompilerToVM ~20 native) ──┬── T8 (bridge 7 符号) ──┬── T9 (compile0 打通)  [路线 A 完成]
                       │                     │                         │                                                    │                        │
                       └─────────────────────┴─────────────────────────┘                                                    │                        │
                                                                                                                               ↑────────────────────────┘
                                                                                                                               (T9 依赖 T8 + T7 + T6)
```

- **并行点**：T1 完成后，T2（B 路）与 T4（A 路）可完全并行；T3 跟在 T2 后；T4→T5→T6→T7→T8→T9 串行（A 路强依赖链）。
- **集成验收点**：T3（第 1 梯末，纯 cargo test）、T6（第 2 梯末，cargo test + mx build 尝试）、T9（第 3 梯末，cargo test + mx build 尝试）。

---

## 5. 验收梯队对应

| 梯队 | 任务 | 验收手段 | 备注 |
|------|------|----------|------|
| 第 1 梯 | T1, T2, T3 | 仅 `cargo test`（+ `cargo build` / `cargo fmt --check`） | 前 3 任务纯 cargo test；T1-T3 完成标志 B 路线 + 骨架就绪 |
| 第 2 梯 | T4, T5, T6 | `cargo test --workspace` + 尝试 `mx build`（跑不动降级） + 彻底复核是否偏离原 Java 实现 | 覆盖 A 路线 meta/code/runtime 接口镜像；mx build 失败则降级为 cargo test + trait 方法集人工对照，记录降级原因 |
| 第 3 梯 | T7, T8, T9 | `cargo test --workspace` + 尝试 `mx build`（跑不动降级） + 彻底复核是否偏离原 Java 实现 + `nm` 符号检查 | 覆盖 hotspot + bridge + compile0；T9 完成标志第一期 A 路线端到端链路打通 |

**子代理闭环**（spec §6.2）：每个任务粒度适配一个 worker 闭环 — worker 写代码 → verifier（不同子代理）严格验收找全问题 → 主协调者汇总 → fixer（全新子代理）一次性修复 → re-verifier 验收 → 不通过则循环 fixer → re-verifier 直到通过。verifier 与 worker 必须不同子代理；fixer 必须全新子代理避免上下文污染。

---

## 6. 未确认项与处理策略

worker 遇到以下未确认项时，统一策略：**基于 Java 侧声明反推 + 在对应 source-map MD "偏离记录" 标注 + 不阻断实现**；C++ 签名以 Java `native` 声明为契约基准。每项核对结果由 worker 回填对应 MD 并通知主协调者升级 spec 版本。

1. **jdk.vm.ci Java 源不可直接访问**（最高优先级前置阻塞）
   - 现状：`/opt` 下无 `src.zip`；`find /opt -name "CompilerToVM.java"` / `JVMCICompiler.java` 均无命中。`jdk.vm.ci` 全部 213 文件（含 `CompilerToVM.java` 的 132 native 声明、`HotSpotJVMCIRuntime.compile0`、`JVMCICompiler.compileMethod`）不在 `/opt/graal` 也不在 `/opt/jdk-src`。
   - 处理：T4 启动时 worker 先落 JDK25 源（下载 OpenJDK25 `src.zip` 解压，或从 `github.com/openjdk/jdk` raw 取 `src/jdk.internal.vm.ci/share/classes/jdk/vm/ci/`），解压到 `/workspace/.jdk-src/`（gitignore）或 `/tmp`。落定后将实际路径回填 `jdk-vm-ci-*.md` 的"源位置"字段。
   - 阻断范围：T4-T9 全部依赖此项；T1-T3 不依赖（util/json 与 collections 源在 `/opt/graal` 内）。

2. **C++ JVMCI 源（`compilerToVM.cpp` 等）确认缺失**
   - 现状：spec §10.3 已确认 `compilerToVM.cpp` 在 openjdk/jdk master 与 oracle/graal 均不存在（JEP410 演进移除 Java JVMCI C++ 桥）。
   - 处理：基于 Java 侧 `CompilerToVM.java` 的 132 个 `native` 声明反推 C++ 契约（参数类型 / 返回类型 / 异常）。每个反推签名在 `jdk-vm-ci-hotspot.md` 偏离记录标注"基于 Java 声明反推，C++ 待核对"。不阻断 T7/T8。

3. **`JVMCI_OnLoad` 精确 ABI**（spec §10.1）
   - 现状：参数与返回契约未核对。
   - 处理：T8 实现时，优先核对 `/opt/jdk-src/src/hotspot/share/jvmci/` 下加载侧 C++（若存在）；无法核实则按 spec §9 + openjdk/jdk raw 反推，签名草案入 `rustci-bridge.md`，标注未确认。不阻断。

4. **HotSpot dlopen 路径/命名约定**（spec §10.2）
   - 现状：HotSpot 运行时定位并 `dlopen` 本 cdylib 的路径与命名未核对。
   - 处理：本期 cdylib 产物命名 `librustci.so`（Linux）/ `librustci.dylib`（macOS），worker 在 `rustci-bridge.md` 标注"命名待与 HotSpot 加载侧核对"。不阻断 T8/T9（仅影响真机集成，本期集成以 cargo + nm 为准）。

5. **`compile0` 签名终稿**（spec §10.4 / §9.1）
   - 现状：spec §9.1 为草案。
   - 处理：T8/T9 实现时以 spec §9.1 草案为准，与 `compileMethodInLibgraal`（`LibGraalEntryPoints.java`）+ `HotSpotJVMCIRuntime.compile0` Java 声明双向核对后定稿。偏离草案处入 `rustci-bridge.md` 偏离记录。不阻断。

6. **`mx build` 可运行性**（spec §6.3）
   - 现状：沙箱内 `mx` 工具链与 HotSpot 集成环境可能不可用。
   - 处理：第 2 / 第 3 梯集成验收时尝试 `mx build`；跑不动则降级为 `cargo test --workspace` + `nm -D` 符号检查 + 调度链路单测，并在本文件对应梯队列明降级原因。不阻断任务关闭。

7. **许可证头保留**
   - 现状：上游多许可证（`org.graalvm.collections` = UPL-1.0；`jdk.vm.ci` / `util.json` = GPL-2.0 + Classpath Exception）。
   - 处理：每个 Rust 文件逐文件保留上游 Oracle 版权与许可证头（移植即衍生，必须保留）。worker 在移植时把 Java 文件头的版权 + 许可证块以注释形式移植到对应 Rust 文件顶部。verifier 验收时抽查。

---

## 7. 给 worker 子代理的认领指引

1. 认领前：读 `/workspace/spec.md` §7（纪律）、本文件对应任务详情、对应 source-map MD。
2. 认领时：在对应 source-map MD 顶部把"状态"改为 `进行中`，"负责人"填自己标识。
3. 实现时：原 Java 实现参考路径见每任务"原 Java 实现参考"段；`/opt` 下用 `RunCommand`（`rg`/`find`/`wc`/`head`）访问，禁止 `Grep`/`Glob`。
4. 完成后：回写对应 source-map MD 映射表（Rust 文件列 + 状态 + 偏离记录），通知主协调者启动 verifier 闭环。
5. 遇未确认项：按 §6 策略处理，标注 + 不阻断，偏离入 MD。
