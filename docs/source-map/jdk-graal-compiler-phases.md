# jdk.graal.compiler.phases

## 源位置：`/opt/graal/compiler/src/jdk.graal.compiler/src/jdk/graal/compiler/phases/`

## 状态：已验收

## 说明

编译阶段框架：PhaseSuite、BasePhase、Phase 等编排基础设施。顶层 + 子包 common/contract/schedule/tiers/util 完整移植，worker-verifier-fixer-reverifier 闭环通过。

## 映射表

| Java 类（全限定名） | 职责 | 对应 Rust 文件 | 状态 | 负责人 |
|---|---|---|---|---|
| `jdk.graal.compiler.phases.PhaseSuite` | 阶段套件 | `rustci/crates/rustci-compiler/src/phases/phase_suite.rs` | 已验收 | — |
| `jdk.graal.compiler.phases.BasePhase` | 阶段基类 | `rustci/crates/rustci-compiler/src/phases/base_phase.rs` | 已验收 | — |
| `jdk.graal.compiler.phases.Phase` | 阶段接口 | `rustci/crates/rustci-compiler/src/phases/phase.rs` | 已验收 | — |
| `jdk.graal.compiler.phases.PhasePlan` | 阶段计划 | `rustci/crates/rustci-compiler/src/phases/phase_suite.rs` | 已验收 | — |
| `jdk.graal.compiler.phases.PhaseSizeContract` | 阶段大小契约 | `rustci/crates/rustci-compiler/src/phases/contract/phase_size_contract.rs` | 已验收 | — |
| `jdk.graal.compiler.phases.RecursivePhase` | 递归阶段 | `rustci/crates/rustci-compiler/src/phases/recursive_phase.rs` | 已验收 | — |
| `jdk.graal.compiler.phases.SingleRunSubphase` | 单次子阶段 | `rustci/crates/rustci-compiler/src/phases/single_run_subphase.rs` | 已验收 | — |
| `jdk.graal.compiler.phases.Speculative` | 推测阶段 | `rustci/crates/rustci-compiler/src/phases/speculative.rs` | 已验收 | — |
| `jdk.graal.compiler.phases.OptimisticOptimizations` | 乐观优化 | `rustci/crates/rustci-compiler/src/phases/optimistic_optimizations.rs` | 已验收 | — |
| `jdk.graal.compiler.phases.PhaseFilterKey` | 阶段过滤键 | `rustci/crates/rustci-compiler/src/phases/phase_filter_key.rs` | 已验收 | — |
| 子包 common/contract/schedule/tiers/util | 通用阶段/契约/调度/层级/工具 | `rustci/crates/rustci-compiler/src/phases/{common,contract,schedule,tiers,util}/` | 已验收 | — |

## 偏离记录

- `Phase`  trait 方法 `run_phase()` 重命名避免与 `BasePhase::run()` 冲突
- `PhaseSuite` 使用 `PhaseEntry<C>` 包装器存储 `TypeId` 替代 Java 的 `instanceof` 检查