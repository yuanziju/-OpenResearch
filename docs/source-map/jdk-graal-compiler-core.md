# jdk.graal.compiler.core

## 源位置：`/opt/graal/compiler/src/jdk.graal.compiler/src/jdk/graal/compiler/core/`

## 状态：已验收

## 说明

编译器核心入口与高层编排：`GraalCompiler`、`CompilationWrapper`、`CompilationWatchDog`、`CompilerThread` 等。10 核心类 + phases 子包 trait 移植，worker-verifier-fixer-reverifier 闭环通过。已知限制：`CompilationWrapper` 从有状态抽象类退化为 trait；phase suites 为空（无实际阶段注册）。

## 映射表

| Java 类（全限定名） | 职责 | 对应 Rust 文件 | 状态 | 负责人 |
|---|---|---|---|---|
| `jdk.graal.compiler.core.GraalCompiler` | 编译器主入口 | `rustci/crates/rustci-compiler/src/core/graal_compiler.rs` | 已验收 | — |
| `jdk.graal.compiler.core.CompilationWrapper` | 编译包装器（故障处理） | `rustci/crates/rustci-compiler/src/core/compilation_wrapper.rs` | 已验收 | — |
| `jdk.graal.compiler.core.CompilationPrinter` | 编译统计打印 | `rustci/crates/rustci-compiler/src/core/compilation_printer.rs` | 已验收 | — |
| `jdk.graal.compiler.core.CompilationWatchDog` | 编译看门狗 | `rustci/crates/rustci-compiler/src/core/compilation_watch_dog.rs` | 已验收 | — |
| `jdk.graal.compiler.core.CompilerThread` | 编译器线程 | `rustci/crates/rustci-compiler/src/core/compiler_thread.rs` | 已验收 | — |
| `jdk.graal.compiler.core.CompilerThreadFactory` | 编译器线程工厂 | `rustci/crates/rustci-compiler/src/core/compiler_thread_factory.rs` | 已验收 | — |
| `jdk.graal.compiler.core.GraalCompilerOptions` | 编译器选项 | `rustci/crates/rustci-compiler/src/core/graal_compiler_options.rs` | 已验收 | — |
| `jdk.graal.compiler.core.Instrumentation` | 插桩接口 | `rustci/crates/rustci-compiler/src/core/instrumentation.rs` | 已验收 | — |
| `jdk.graal.compiler.core.LIRGenerationPhase` | LIR 生成阶段 | `rustci/crates/rustci-compiler/src/core/lir_generation_phase.rs` | 已验收 | — |
| `jdk.graal.compiler.core.ArchitectureSpecific` | 架构特定接口 | `rustci/crates/rustci-compiler/src/core/architecture_specific.rs` | 已验收 | — |
| `jdk.graal.compiler.core.phases.*` | 编译阶段定义（HighTier/MidTier/LowTier 等） | `rustci/crates/rustci-compiler/src/core/phases.rs` | 已验收 | — |

## 偏离记录

- `CompilationWrapper` 从 Java 有状态抽象类（~602 行）退化为 Rust trait（~118 行），因为 Rust 不支持抽象类继承。部分故障处理逻辑（handleFailure、dumpOnError 等）在 trait 中以默认方法形式提供框架
- `GraalCompiler.compile` 和 `emit_front_end` 实现了编译流程框架，但具体阶段内容依赖 phases.rs 中的阶段注册（当前为空）
- `CompilationWatchDog` 不实现 `Runnable`/`AutoCloseable`（Rust 无等效），用 `WatchDogHandle` 结构体模拟生命周期
- `CompilerThread` 不继承 `Thread`（Rust 无等效），线程行为通过 `std::thread` API 实现
- Phase suites 为空 — 实际优化阶段（如 `CanonicalizerPhase`, `IncrementalCanonicalizerPhase` 等）待后续移植