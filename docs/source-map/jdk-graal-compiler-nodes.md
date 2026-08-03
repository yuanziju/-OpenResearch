# jdk.graal.compiler.nodes

## 源位置：`/opt/graal/compiler/src/jdk.graal.compiler/src/jdk/graal/compiler/nodes/`

## 状态：已验收

## 说明

`ValueNode` 体系与 HIR 节点类型，编译器图的核心节点集。顶层约 106 个 Java 类，已移植 30 个核心控制流和基础节点类。worker-verifier-fixer-reverifier 闭环通过。已知限制：约 76 个顶级类未移植，继承链在多处简化。

## 映射表

| Java 类（全限定名） | 职责 | 对应 Rust 文件 | 状态 | 负责人 |
|---|---|---|---|---|
| `jdk.graal.compiler.nodes.ValueNode` | 值节点抽象基类 | `rustci/crates/rustci-compiler/src/nodes/value_node.rs` | 已验收 | — |
| `jdk.graal.compiler.nodes.FixedNode` | 固定节点抽象基类 | `rustci/crates/rustci-compiler/src/nodes/fixed_node.rs` | 已验收 | — |
| `jdk.graal.compiler.nodes.FixedWithNextNode` | 带后继的固定节点 | `rustci/crates/rustci-compiler/src/nodes/fixed_with_next_node.rs` | 已验收 | — |
| `jdk.graal.compiler.nodes.ControlSinkNode` | 控制流汇节点 | `rustci/crates/rustci-compiler/src/nodes/control_sink_node.rs` | 已验收 | — |
| `jdk.graal.compiler.nodes.ControlSplitNode` | 控制流分叉节点 | `rustci/crates/rustci-compiler/src/nodes/control_split_node.rs` | 已验收 | — |
| `jdk.graal.compiler.nodes.AbstractBeginNode` | 开始节点抽象基类 | `rustci/crates/rustci-compiler/src/nodes/abstract_begin_node.rs` | 已验收 | — |
| `jdk.graal.compiler.nodes.AbstractEndNode` | 结束节点抽象基类 | `rustci/crates/rustci-compiler/src/nodes/abstract_end_node.rs` | 已验收 | — |
| `jdk.graal.compiler.nodes.AbstractMergeNode` | 合并节点抽象基类 | `rustci/crates/rustci-compiler/src/nodes/abstract_merge_node.rs` | 已验收 | — |
| `jdk.graal.compiler.nodes.BeginNode` | 开始节点 | `rustci/crates/rustci-compiler/src/nodes/begin_node.rs` | 已验收 | — |
| `jdk.graal.compiler.nodes.EndNode` | 结束节点 | `rustci/crates/rustci-compiler/src/nodes/end_node.rs` | 已验收 | — |
| `jdk.graal.compiler.nodes.MergeNode` | 合并节点 | `rustci/crates/rustci-compiler/src/nodes/merge_node.rs` | 已验收 | — |
| `jdk.graal.compiler.nodes.IfNode` | 条件分支节点 | `rustci/crates/rustci-compiler/src/nodes/if_node.rs` | 已验收 | — |
| `jdk.graal.compiler.nodes.LoopBeginNode` | 循环头节点 | `rustci/crates/rustci-compiler/src/nodes/loop_begin_node.rs` | 已验收 | — |
| `jdk.graal.compiler.nodes.LoopEndNode` | 循环尾节点 | `rustci/crates/rustci-compiler/src/nodes/loop_end_node.rs` | 已验收 | — |
| `jdk.graal.compiler.nodes.LoopExitNode` | 循环出口节点 | `rustci/crates/rustci-compiler/src/nodes/loop_exit_node.rs` | 已验收 | — |
| `jdk.graal.compiler.nodes.StartNode` | 方法入口节点 | `rustci/crates/rustci-compiler/src/nodes/start_node.rs` | 已验收 | — |
| `jdk.graal.compiler.nodes.ReturnNode` | 返回节点 | `rustci/crates/rustci-compiler/src/nodes/return_node.rs` | 已验收 | — |
| `jdk.graal.compiler.nodes.ConstantNode` | 常量节点 | `rustci/crates/rustci-compiler/src/nodes/constant_node.rs` | 已验收 | — |
| `jdk.graal.compiler.nodes.ParameterNode` | 参数节点 | `rustci/crates/rustci-compiler/src/nodes/parameter_node.rs` | 已验收 | — |
| `jdk.graal.compiler.nodes.PiNode` | Pi 节点（类型窄化） | `rustci/crates/rustci-compiler/src/nodes/pi_node.rs` | 已验收 | — |
| `jdk.graal.compiler.nodes.GuardNode` | 守卫节点 | `rustci/crates/rustci-compiler/src/nodes/guard_node.rs` | 已验收 | — |
| `jdk.graal.compiler.nodes.GuardingNode` | 守卫节点接口 | `rustci/crates/rustci-compiler/src/nodes/guarding_node.rs` | 已验收 | — |
| `jdk.graal.compiler.nodes.DeoptimizingNode` | 去优化节点接口 | `rustci/crates/rustci-compiler/src/nodes/deoptimizing_node.rs` | 已验收 | — |
| `jdk.graal.compiler.nodes.FrameState` | 帧状态节点 | `rustci/crates/rustci-compiler/src/nodes/frame_state.rs` | 已验收 | — |
| `jdk.graal.compiler.nodes.StructuredGraph` | 结构化图 | `rustci/crates/rustci-compiler/src/nodes/structured_graph.rs` | 已验收 | — |
| `jdk.graal.compiler.nodes.GraphState` | 图状态 | `rustci/crates/rustci-compiler/src/nodes/graph_state.rs` | 已验收 | — |
| `jdk.graal.compiler.nodes.GraphEncoder` | 图编码器 | `rustci/crates/rustci-compiler/src/nodes/graph_encoder.rs` | 已验收 | — |
| `jdk.graal.compiler.nodes.GraphDecoder` | 图解码器 | `rustci/crates/rustci-compiler/src/nodes/graph_decoder.rs` | 已验收 | — |
| `jdk.graal.compiler.nodes.SimplifyingGraphDecoder` | 简化图解码器 | `rustci/crates/rustci-compiler/src/nodes/simplifying_graph_decoder.rs` | 已验收 | — |
| `jdk.graal.compiler.nodes.InliningLog` | 内联日志 | `rustci/crates/rustci-compiler/src/nodes/inlining_log.rs` | 已验收 | — |
| `jdk.graal.compiler.nodes.OptimizationLog` | 优化日志接口 | `rustci/crates/rustci-compiler/src/nodes/optimization_log.rs` | 已验收 | — |
| `jdk.graal.compiler.nodes.OptimizationLogImpl` | 优化日志实现 | `rustci/crates/rustci-compiler/src/nodes/optimization_log_impl.rs` | 已验收 | — |

## 未移植类（约 76 个，待后续任务）

包括但不限于：`InvokeNode`, `InvokeWithExceptionNode`, `PhiNode`, `ValuePhiNode`, `ProxyNode`, `ValueProxyNode`, `GuardPhiNode`, `GuardProxyNode`, `DeoptimizeNode`, `DynamicDeoptimizeNode`, `FixedGuardNode`, `AbstractFixedGuardNode`, `ConditionAnchorNode`, `LogicNode`, `LogicConstantNode`, `LogicNegationNode`, `ShortCircuitOrNode`, `BinaryOpLogicNode`, `UnaryOpLogicNode`, `CallTargetNode`, `DirectCallTargetNode`, `IndirectCallTargetNode`, `LoweredCallTargetNode`, `AbstractLocalNode`, `AbstractStateSplit`, `BeginStateSplitNode`, `FloatingNode`, `FloatingAnchoredNode`, `FloatingGuardedNode`, `MemoryMapControlSinkNode`, `EncodedGraph`, `NodeClassMap`, `NodeView`, `NonFoldingConstantNode`, `FieldLocationIdentity`, `NamedLocationIdentity`, `EntryMarkerNode`, `EntryProxyNode`, `FullInfopointNode`, `GetObjectAddressNode`, `ComputeObjectAddressNode`, `CompressionNode`, `PauseNode`, `PrefetchAllocateNode`, `ReadArgumentNode`, `SafepointNode`, `SnippetAnchorNode`, `SpinWaitNode`, `UnreachableBeginNode`, `UnreachableControlSinkNode`, `UnreachableNode`, `UnwindNode`, `WithExceptionNode`, `MultiReturnNode`, `DeadEndNode`, `BreakpointNode`, `MemoryProxyNode`, `GuardedValueNode`, `PiArrayNode`, `PluginReplacementNode`, `PluginReplacementWithExceptionNode`, `SnippetAnchorNode`, `DynamicPiNode`, `TypeCheckHints`, `ProfileData`, `InliningLogCodec`, `OptimizationLogCodec`, `CompanionObjectEncoder`, `Cancellable`, `CanonicalizableLocation`, `DeoptBarrier`, `DeoptBciSupplier`, `DeoptimizingFixedWithNextNode`, `DeoptimizingGuard`, `FixedAccessNodeInterface`, `FixedGlobalValueNumberable`, `FixedNodeInterface`, `FixedWithNextNodeInterface`, `ImplicitNullCheckNode`, `Invokable`, `Invoke`, `LIRLowerableLogicNode`, `PluginReplacementInterface`, `StateSplit`, `StaticDeoptimizingNode`, `ValueNodeInterface`, `VirtualState`

## 偏离记录

- 继承链简化：Java 多层继承（如 `FloatingNode → FloatingAnchoredNode → GuardNode`）在 Rust 中通过 trait 组合实现，部分中间层丢失
- `GraphDecoder.decode` 和 `SimplifyingGraphDecoder.decode` 实现了框架，但具体解码逻辑（如 `decodeNode`, `decodeFixedNode` 等）为简化实现
- `StructuredGraph` 不继承 `Graph`（来自 `jdk.graal.compiler.graph` 包，尚未移植），作为独立 struct 实现
- 约 72% 顶级 nodes 类尚未移植，列为后续任务