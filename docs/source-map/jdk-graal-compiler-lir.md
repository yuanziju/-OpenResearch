# jdk.graal.compiler.lir

## 源位置：`/opt/graal/compiler/src/jdk.graal.compiler/src/jdk/graal/compiler/lir/`

## 状态：已验收

## 说明

Low-Level IR 层：HIR（nodes）和机器码之间的中间表示。22 文件 + 6 子包声明，worker-verifier-fixer-reverifier 闭环通过。

## 映射表

| Java 类（全限定名） | 职责 | 对应 Rust 文件 | 状态 | 负责人 |
|---|---|---|---|---|
| `jdk.graal.compiler.lir.LIR` | LIR 图主类 | `rustci/crates/rustci-compiler/src/lir/lir.rs` | 已验收 | — |
| `jdk.graal.compiler.lir.LIRInstruction` | LIR 指令基类 | `rustci/crates/rustci-compiler/src/lir/lir_instruction.rs` | 已验收 | — |
| `jdk.graal.compiler.lir.LIRKind` | 值类型 | `rustci/crates/rustci-compiler/src/lir/lir_kind.rs` | 已验收 | — |
| `jdk.graal.compiler.lir.Variable` | SSA 变量/虚拟寄存器 | `rustci/crates/rustci-compiler/src/lir/variable.rs` | 已验收 | — |
| `jdk.graal.compiler.lir.CompositeValue` | 复合值 | `rustci/crates/rustci-compiler/src/lir/composite_value.rs` | 已验收 | — |
| `jdk.graal.compiler.lir.ConstantValue` | 常量值 | `rustci/crates/rustci-compiler/src/lir/constant_value.rs` | 已验收 | — |
| `jdk.graal.compiler.lir.LabelRef` | 标签引用 | `rustci/crates/rustci-compiler/src/lir/label_ref.rs` | 已验收 | — |
| `jdk.graal.compiler.lir.BlockValue` | 跨块值引用 | `rustci/crates/rustci-compiler/src/lir/block_value.rs` | 已验收 | — |
| `jdk.graal.compiler.lir.LIRFrameState` | 帧状态 | `rustci/crates/rustci-compiler/src/lir/lir_frame_state.rs` | 已验收 | — |
| `jdk.graal.compiler.lir.LIRInsertionBuffer` | 插入缓冲区 | `rustci/crates/rustci-compiler/src/lir/lir_insertion_buffer.rs` | 已验收 | — |
| `jdk.graal.compiler.lir.LIRIntrospection` | 内省接口 | `rustci/crates/rustci-compiler/src/lir/lir_introspection.rs` | 已验收 | — |
| `jdk.graal.compiler.lir.LIRValueClass` | 值分类 | `rustci/crates/rustci-compiler/src/lir/lir_value_class.rs` | 已验收 | — |
| `jdk.graal.compiler.lir.LIRVerifier` | LIR 验证器 | `rustci/crates/rustci-compiler/src/lir/lir_verifier.rs` | 已验收 | — |
| `jdk.graal.compiler.lir.StandardOp` | 标准操作接口 | `rustci/crates/rustci-compiler/src/lir/standard_op.rs` | 已验收 | — |
| `jdk.graal.compiler.lir.ValueProcedure` | 值过程 | `rustci/crates/rustci-compiler/src/lir/value_procedure.rs` | 已验收 | — |
| 子包 asm/dfa/gen/phases/ssa/stackslotalloc | 子包声明 | `rustci/crates/rustci-compiler/src/lir/{asm,dfa,gen,phases,ssa,stackslotalloc}/mod.rs` | 已验收 | — |

## 偏离记录

- 子包（asm/dfa/gen/phases/ssa/stackslotalloc）仅声明模块结构，具体类待后续移植