# Source Map 索引 — GraalVM → Rust 移植项目

## 项目说明

本目录是 **GraalVM（oracle/graal HEAD，`jdk.graal.compiler.*` 新布局）→ Rust** 移植项目的源码映射索引。每个模块一个 MD 文件，记录 Java 源类 → Rust 对应文件的映射、职责、状态与负责人，供主协调者与各子代理协同。

- **只读参考源**：`/opt/graal`（oracle/graal HEAD）。仅可用 `RunCommand` 访问（`Grep`/`Glob` 不可用于 `/opt`）。
- **jdk.vm.ci 模块源**：JDK25 `src.zip`（`jdk.internal.vm.ci` 模块），**不在 `/opt/graal` 内**，共 213 文件。
- **Rust 代码**：`/workspace` 下独立新仓库结构（graal-rust 项目）。
- **第一期范围**：
  1. RustCI 骨架（镜像 `jdk.vm.ci` 核心子集 `meta` / `code` / `runtime` / `hotspot`）；
  2. `jdk.graal.compiler.util.json` 叶子模块完整移植（8 类，源 LoC 1685）。

**分工**：主协调者只维护本目录下的 `.md` 文件；子代理负责具体模块的 Rust 实现并回写对应 MD 的映射表与状态。

---

## Workspace 骨架状态

| 任务 | 范围 | 状态 |
|---|---|---|
| T1 | cargo workspace 骨架 + 4 crate 空根打通编译（rustci-collections / rustci-util-json / rustci-vm-ci / rustci-bridge；cdylib `librustci_bridge.so` 已产出） | 已完成 |

---

## 模块清单总表

> 列：模块 / 源 LoC / Rust LoC / 状态 / MD 文件。源 LoC 未统计者记 `—`，子代理开工时填写实测值。

### A. jdk.vm.ci 核心（JDK25 src.zip → jdk.internal.vm.ci）

| # | 模块 | 源 LoC | Rust LoC | 状态 | MD 文件 |
|---|---|---|---|---|---|
| 1 | jdk.vm.ci.meta | — | — | 未开始 | [jdk-vm-ci-meta.md](jdk-vm-ci-meta.md) |
| 2 | jdk.vm.ci.code | — | — | 未开始 | [jdk-vm-ci-code.md](jdk-vm-ci-code.md) |
| 3 | jdk.vm.ci.runtime | — | — | 未开始 | [jdk-vm-ci-runtime.md](jdk-vm-ci-runtime.md) |
| 4 | jdk.vm.ci.hotspot | — | — | 未开始 | [jdk-vm-ci-hotspot.md](jdk-vm-ci-hotspot.md) |
| 5 | jdk.vm.ci.services | — | — | 未开始 | [jdk-vm-ci-services.md](jdk-vm-ci-services.md) |

### B. jdk.graal.compiler 叶子工具（/opt/graal/compiler/src/jdk.graal.compiler/）

| # | 模块 | 源 LoC | Rust LoC | 状态 | MD 文件 |
|---|---|---|---|---|---|
| 6 | jdk.graal.compiler.util.json | 1685 | — | 未开始 | [jdk-graal-compiler-util-json.md](jdk-graal-compiler-util-json.md) |
| 7 | jdk.graal.compiler.util.args | — | — | 未开始 | [jdk-graal-compiler-util-args.md](jdk-graal-compiler-util-args.md) |
| 8 | jdk.graal.compiler.util.collections | — | — | 未开始 | [jdk-graal-compiler-util-collections.md](jdk-graal-compiler-util-collections.md) |
| 9 | jdk.graal.compiler.graphio | — | — | 未开始 | [jdk-graal-compiler-graphio.md](jdk-graal-compiler-graphio.md) |

### C. jdk.graal.compiler 核心与图（/opt/graal/compiler/src/jdk.graal.compiler/）

| # | 模块 | 源 LoC | Rust LoC | 状态 | MD 文件 |
|---|---|---|---|---|---|
| 10 | jdk.graal.compiler.core | — | — | 未开始 | [jdk-graal-compiler-core.md](jdk-graal-compiler-core.md) |
| 11 | jdk.graal.compiler.nodes | — | — | 未开始 | [jdk-graal-compiler-nodes.md](jdk-graal-compiler-nodes.md) |
| 12 | jdk.graal.compiler.graph | — | — | 未开始 | [jdk-graal-compiler-graph.md](jdk-graal-compiler-graph.md) |
| 13 | jdk.graal.compiler.lir | — | — | 未开始 | [jdk-graal-compiler-lir.md](jdk-graal-compiler-lir.md) |
| 14 | jdk.graal.compiler.phases | — | — | 未开始 | [jdk-graal-compiler-phases.md](jdk-graal-compiler-phases.md) |
| 15 | jdk.graal.compiler.debug | — | — | 未开始 | [jdk-graal-compiler-debug.md](jdk-graal-compiler-debug.md) |

### D. jdk.graal.compiler 后端与平台（/opt/graal/compiler/src/jdk.graal.compiler/）

| # | 模块 | 源 LoC | Rust LoC | 状态 | MD 文件 |
|---|---|---|---|---|---|
| 16 | jdk.graal.compiler.hotspot | — | — | 未开始 | [jdk-graal-compiler-hotspot.md](jdk-graal-compiler-hotspot.md) |
| 17 | jdk.graal.compiler.replacements | — | — | 未开始 | [jdk-graal-compiler-replacements.md](jdk-graal-compiler-replacements.md) |
| 18 | jdk.graal.compiler.runtime | — | — | 未开始 | [jdk-graal-compiler-runtime.md](jdk-graal-compiler-runtime.md) |
| 19 | jdk.graal.compiler.asm | — | — | 未开始 | [jdk-graal-compiler-asm.md](jdk-graal-compiler-asm.md) |
| 20 | jdk.graal.compiler.bytecode | — | — | 未开始 | [jdk-graal-compiler-bytecode.md](jdk-graal-compiler-bytecode.md) |
| 21 | jdk.graal.compiler.code | — | — | 未开始 | [jdk-graal-compiler-code.md](jdk-graal-compiler-code.md) |

### E. jdk.graal.compiler 优化与扩展（/opt/graal/compiler/src/jdk.graal.compiler/）

| # | 模块 | 源 LoC | Rust LoC | 状态 | MD 文件 |
|---|---|---|---|---|---|
| 22 | jdk.graal.compiler.loop | — | — | 未开始 | [jdk-graal-compiler-loop.md](jdk-graal-compiler-loop.md) |
| 23 | jdk.graal.compiler.virtual | — | — | 未开始 | [jdk-graal-compiler-virtual.md](jdk-graal-compiler-virtual.md) |
| 24 | jdk.graal.compiler.vector | — | — | 未开始 | [jdk-graal-compiler-vector.md](jdk-graal-compiler-vector.md) |
| 25 | jdk.graal.compiler.truffle | — | — | 未开始 | [jdk-graal-compiler-truffle.md](jdk-graal-compiler-truffle.md) |
| 26 | jdk.graal.compiler.printer | — | — | 未开始 | [jdk-graal-compiler-printer.md](jdk-graal-compiler-printer.md) |
| 27 | jdk.graal.compiler.java | — | — | 未开始 | [jdk-graal-compiler-java.md](jdk-graal-compiler-java.md) |

### F. jdk.graal.compiler 元数据/底层（/opt/graal/compiler/src/jdk.graal.compiler/）

| # | 模块 | 源 LoC | Rust LoC | 状态 | MD 文件 |
|---|---|---|---|---|---|
| 28 | jdk.graal.compiler.api | — | — | 未开始 | [jdk-graal-compiler-api.md](jdk-graal-compiler-api.md) |
| 29 | jdk.graal.compiler.annotation | — | — | 未开始 | [jdk-graal-compiler-annotation.md](jdk-graal-compiler-annotation.md) |
| 30 | jdk.graal.compiler.nodeinfo | — | — | 未开始 | [jdk-graal-compiler-nodeinfo.md](jdk-graal-compiler-nodeinfo.md) |
| 31 | jdk.graal.compiler.word | — | — | 未开始 | [jdk-graal-compiler-word.md](jdk-graal-compiler-word.md) |
| 32 | jdk.graal.compiler.serviceprovider | — | — | 未开始 | [jdk-graal-compiler-serviceprovider.md](jdk-graal-compiler-serviceprovider.md) |

### G. libgraal 与桥接层

| # | 模块 | 源 LoC | Rust LoC | 状态 | MD 文件 |
|---|---|---|---|---|---|
| 33 | jdk.graal.compiler.libgraal | — | — | 未开始 | [jdk-graal-compiler-libgraal.md](jdk-graal-compiler-libgraal.md) |
| 34 | rustci-bridge（Rust cdylib 对接层，7 符号） | — | — | 未开始 | [rustci-bridge.md](rustci-bridge.md) |

---

## 状态图例

| 状态 | 含义 |
|---|---|
| 未开始 | 模块尚未启动移植；映射表可能已预填源类清单（第一期模块），但无 Rust 产出 |
| 进行中 | 子代理已认领并正在实现；映射表逐步补全 Rust 文件列 |
| 已完成 | 该模块全部 Java 类已映射到 Rust 文件并实现，自测通过 |
| 已验收 | 已完成 + 经主协调者验收（接口/测试/文档齐备） |

类级状态（映射表“状态”列）同上四态，逐类标注。

---

## 更新规则

1. **认领**：子代理开工前，将目标 MD 顶部“状态”改为 `进行中`，并在“负责人”列填入自己的标识。
2. **逐类回写**：每完成一个 Java 类的 Rust 实现，在对应映射表行填入“对应 Rust 文件”（相对 `/workspace` 的路径）、把该行“状态”改为 `已完成` 或 `进行中`。
3. **模块收尾**：模块全部行 `已完成` 后，将 MD 顶部“状态”改为 `已完成`，并在 README 总表同步该行的“Rust LoC”与“状态”。
4. **验收**：主协调者验收后将 MD 与 README 状态改为 `已验收`。
5. **源 LoC**：子代理开工时用 `RunCommand`（`wc -l`）实测源 LoC 并回填 README 总表“源 LoC”列；`/opt/graal` 路径见各 MD“源位置”。
6. **不跨界**：子代理只修改自己模块的 MD 与 Rust 文件；跨模块依赖在“负责人”列备注，由主协调者协调。
7. **保持表头统一**：新增条目须遵循各 MD 既定的五列格式，不增删列。
