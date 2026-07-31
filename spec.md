# GraalVM → Rust 编译器移植规格说明

> 状态：Spec v1（调研固化）
> 主协调者仅维护本文件及其派生 `.md`，其余后缀文件由子代理产出。
> 只读参考源：`/opt/graal`（oracle/graal HEAD，新版 `jdk.graal.compiler.*` 布局，已 libgraal 化）、`/opt/jdk-src`（openjdk/jdk 的 HotSpot C++ 源）。

---

## 1. 项目概述与目标

把 GraalVM 编译器（`compiler/` 下的 Java 实现）完整移植到 Rust，替代 libgraal 现有的 native-image 自举方案。

最终交付一个以 Rust 编写、可直接对接 HotSpot C++ 底层的 Graal 编译器：

- 行为与原 Java 实现等价，不引入语言层重设计；
- 以 Rust `cdylib` 形态被 HotSpot 加载，承担生产 JIT 编译职责；
- 移除对 native-image 自举链路的依赖。

非目标：不重写 Graal 的算法与图中间表示，不改 Graal 哲学；不实现新的优化管线。

## 2. 架构决策

### 2.1 libgraal 风格，跳过 Java 层

Rust `cdylib` 直接对接 HotSpot C++ 底层，跳过 Java JVMCI 中间层。Rust 侧不再经过 Java 桥接对象，而是以原生符号直接与 `CompilerToVM`、`HotSpotJVMCIRuntime` 等 C++ 实现交互。

### 2.2 全量 1:1 镜像 jdk.vm.ci

Rust 侧接口全量 1:1 镜像 `jdk.vm.ci.*`，唯一变更是顶层模块名 `JVMCI → RustCI`（即 `jdk.vm.ci.meta` → `rustci.meta` 之类的命名空间映射）。遵循 Graal 原哲学：忠实映射既有抽象，不做 Rust 惯用化重构，不合并/拆分接口，不改变继承与组合关系。

### 2.3 compiler 侧规模（基线）

`jdk.graal.compiler` 共 **494255 LoC / 2150 文件 / 24 子包**。`vm.ci` 引用 83% 集中在 `hotspot` / `lir` / `nodes` / `replacements` / `core`。零 `vm.ci` 依赖区：`util/json`、`util/args`、`graphio`、`util` 根集合——这些是首批可独立移植的叶子。

## 3. 第一期范围（并行两路）

第一期同时推进两条互不阻塞的工作路。

### 3.1 路线 A：RustCI 骨架（最小子集打通编译链路）

目标：以最小子集让端到端编译链路跑通，为后续全量镜像打地基。

包含：

- `JVMCICompiler` / `JVMCICompilerFactory` 接口；
- `CompilerToVM` 中编译 / install / 配置相关约 20 个 native 方法；
- `jdk.vm.ci.meta` / `code` / `runtime` 核心接口子集。

全量 213 文件的 1:1 镜像列入后续待办（见 §4），本期不追求覆盖。

### 3.2 路线 B：叶子模块完整移植 — `util/json/`

- 规模：8 文件 / 1685 LoC，`vm.ci` 仅 1 处引用且可降级处理；
- 测试：3 个 / 616 LoC；
- 定位：非 MVP，忠实完整移植，不简化、不占位。

两条路线在单一 `dev` 分支上并行，互不依赖。

## 4. 后续路线

### 4.1 全量 1:1 镜像 jdk.vm.ci（213 文件）

按模块分期推进，模块清单：

- `runtime`：`JVMCI` / `JVMCIBackend` / `JVMCICompiler` / `JVMCICompilerFactory` / `JVMCIRuntime`；
- `meta`：22 接口 + 若干 final class；
- `code`：9 接口 + 17 final class；
- `hotspot`：`HotSpotJVMCIRuntime` / `CompilerToVM`（132 native）/ 各 Provider / 各 `ResolvedJavaType` / `HotSpotCompiledCode` 等，共 76 文件；
- `services`、`common`；
- 架构相关：`amd64` / `aarch64` / `riscv64`。

### 4.2 compiler 其余 23 子包移植

依据 `vm.ci` 引用密度排序：`hotspot` → `lir` → `nodes` → `replacements` → `core` → 其余。每个子包按"接口镜像 → 实现移植 → 测试对齐"三步走，分期验收。

## 5. 环境与保命策略

- `/opt/graal`、`/opt/jdk-src`：持久预装的只读参考源，不在 git 控制下，沙箱重置不回滚。仅供查阅，禁止写入。
- `/workspace`：virtiofs git 卷。**沙箱重置会回滚到 git 状态**——`/workspace` 下所有产出必须及时 `commit + push` 保命，未提交的改动随时可能丢失。
- 工具约束：`/opt` 下不可用 `Grep`/`Glob`，统一通过 `RunCommand` 访问（如 `rg`、`ls`、`find`）。

## 6. 工作流

### 6.1 分支

- 只开一个 `dev` 分支；子代理不开单独分支，全部在 `dev` 上并行。
- 每个里程碑自动 `commit + push`（Conventional Commits，英文，面向社区）。

### 6.2 子代理闭环

```
worker 写代码
  → verifier（不同子代理）严格验收，找所有问题
  → 汇总给主协调者
  → 启动 fixer（全新子代理）一次性修复
  → re-verifier 验收
  → 不通过则循环 fixer → re-verifier，直到通过
```

- verifier 与 worker 必须是不同子代理；fixer 必须是全新子代理，避免上下文污染。
- 主协调者只碰 `.md` 文件，其他后缀文件由子代理碰。

### 6.3 验收梯队

- 前 3 个 todo：仅 `cargo test`。
- 每 3 个 todo：做一次集成验收（`cargo test` + 尝试 `mx build`，跑不动则降级并记录原因）。
- 每 3 个 todo：彻底验收一次是否偏离原 Java 实现。

## 7. 子代理纪律

- **禁止偷懒实现**：空实现 / 占位 / simple stub 一律禁止。
- **禁止写 TODO 注释**，除非原 Java 源码本身就有 TODO（届时原样保留语义）。
- **一切谨遵原 Java 实现已有的参考**：完整移植，非 MVP。行为、边界、异常路径与原实现一致。
- 不做未要求的重构、不添加未要求的注释 / docstring / 类型标注。
- 涉及 native 签名时，以 `/opt/jdk-src` 下 C++ 源为核对基准（见 §10 未确认项）。

## 8. source map 方案

按模块拆分多个派生 `.md`，每个对应一个移植单元，记录：

- 原 Java 源路径（`/opt/graal/...`）；
- Rust 目标路径（`/workspace/...`）；
- 文件 / LoC / 测试清单；
- 移植状态（未开始 / 进行中 / 已完成 / 已验收）；
- 偏离记录（如有，必须注明原因与对应 Java 参考）。

子代理在完成各自单元后**自行更新**对应 source map 文件，主协调者负责汇总与跨模块一致性。派生文件命名：`docs/sourcemap/<module>.md`（如 `docs/sourcemap/util-json.md`、`docs/sourcemap/rustci-meta.md`）。

## 9. Rust cdylib 对接符号集

HotSpot 加载 Rust `cdylib` 后依赖的最小对接符号集 **7 个**：

| 符号 | 作用 |
| --- | --- |
| `JVMCI_OnLoad` | cdylib 加载入口，完成初始化与注册 |
| `RustCIBridge_initialize` | RustCI 桥接初始化 |
| `attachThread` | 线程附加 |
| `detachThread` | 线程分离 |
| `compile0` | 对应 `JVMCICompiler.compileMethod`，编译分发入口 |
| `installCode0` | 代码安装 |
| `readConfiguration` | 配置读取 |
| `registerNativeMethods` | 动态链接 Java 桥接类 native 方法到 `Java_<pkg>_<Class>_<method>` JNI 符号 |

> 注：`registerNativeMethods` 在 libgraal 生产路径中用于把 Java 桥接类的 native 方法动态链接到 JNI 符号；graal 仓库无生产 JIT `@CEntryPoint`（仅测试 `compileMethodInLibgraal` 与 Truffle 入口），生产编译经此路径分发。

### 9.1 `compile0` 签名草案

参考测试入口 `compileMethodInLibgraal`，草案如下：

```
jlong compile0(
    JNIEnv*      env,
    jclass       clazz,
    jlong        isolate_thread,
    jlong        method_handle,
    jint         entry_bci,
    jlong        compile_state,
    jint         compile_id,
    jlong        options,
    jbyteArray   failure_buf,     // 失败信息回写缓冲
    jbyteArray   time_mem_buf      // 编译耗时 / 内存回写缓冲
);
// 返回：installedCode handle（jlong）
```

对应 Java 侧 `HotSpotJVMCIRuntime.compile0`（native 编译分发入口）。

## 10. 未确认项

以下项需在对应里程碑启动时核对，核对结果回填本节并升级 spec 版本：

1. **`JVMCI_OnLoad` 精确签名**：参数与返回契约需对照 `/opt/jdk-src` 下 HotSpot 加载侧 C++ 实现核对。
2. **HotSpot dlopen 路径**：HotSpot 在运行时定位并 `dlopen` 本 cdylib 的路径与命名约定，需核对 `/opt/jdk-src/src/hotspot/share/jvmci/` 下加载逻辑。
3. **`CompilerToVM` 的 132 个 native C++ 签名**：以 `/opt/jdk-src/src/hotspot/share/jvmci/compilerToVM.cpp` 等为基准逐个核对，本期骨架仅触及约 20 个，余者在后续里程碑按需核对。
4. **`compile0` 签名终稿**：草案见 §9.1，最终以 `compileMethodInLibgraal` 与 `HotSpotJVMCIRuntime.compile0` 双向核对后定稿。
