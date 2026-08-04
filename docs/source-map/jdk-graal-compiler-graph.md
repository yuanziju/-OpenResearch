# jdk.graal.compiler.graph

## 源位置：`/opt/graal/compiler/src/jdk.graal.compiler/src/jdk/graal/compiler/graph/`

## 状态：已验收

## 说明

Graal 编译器 IR 图的基础设施层。提供节点、边、图的抽象——所有 nodes 和 core 模块的底层依赖。19 文件完整移植，worker-verifier-fixer-reverifier 闭环通过。

## 映射表

| Java 类（全限定名） | 职责 | 对应 Rust 文件 | 状态 | 负责人 |
|---|---|---|---|---|
| `jdk.graal.compiler.graph.Graph` | 图数据结构 | `rustci/crates/rustci-compiler/src/graph/graph.rs` | 已验收 | — |
| `jdk.graal.compiler.graph.Node` | 节点基类 | `rustci/crates/rustci-compiler/src/graph/node.rs` | 已验收 | — |
| `jdk.graal.compiler.graph.NodeClass` | 节点元数据 | `rustci/crates/rustci-compiler/src/graph/node_class.rs` | 已验收 | — |
| `jdk.graal.compiler.graph.NodeList` | 节点列表 | `rustci/crates/rustci-compiler/src/graph/node_list.rs` | 已验收 | — |
| `jdk.graal.compiler.graph.NodeInputList` | 节点输入列表 | `rustci/crates/rustci-compiler/src/graph/node_input_list.rs` | 已验收 | — |
| `jdk.graal.compiler.graph.NodeSuccessorList` | 节点后继列表 | `rustci/crates/rustci-compiler/src/graph/node_successor_list.rs` | 已验收 | — |
| `jdk.graal.compiler.graph.NodeBitMap` | 节点位图 | `rustci/crates/rustci-compiler/src/graph/node_bit_map.rs` | 已验收 | — |
| `jdk.graal.compiler.graph.NodeFlood` | 节点泛洪遍历 | `rustci/crates/rustci-compiler/src/graph/node_flood.rs` | 已验收 | — |
| `jdk.graal.compiler.graph.NodeWorkList` | 节点工作列表 | `rustci/crates/rustci-compiler/src/graph/node_work_list.rs` | 已验收 | — |
| `jdk.graal.compiler.graph.Position` | 节点边位置 | `rustci/crates/rustci-compiler/src/graph/position.rs` | 已验收 | — |
| `jdk.graal.compiler.graph.IterableNodeType` | 可迭代节点类型标记 | `rustci/crates/rustci-compiler/src/graph/iterable_node_type.rs` | 已验收 | — |
| `jdk.graal.compiler.graph.TypedGraphNodeIterator` | 类型化节点迭代器 | `rustci/crates/rustci-compiler/src/graph/typed_graph_node_iterator.rs` | 已验收 | — |
| `jdk.graal.compiler.graph.NodeUsageIterator` | 节点用法迭代器 | `rustci/crates/rustci-compiler/src/graph/node_usage_iterator.rs` | 已验收 | — |
| `jdk.graal.compiler.graph.NodeUsageWithCountIterator` | 带计数的用法迭代器 | `rustci/crates/rustci-compiler/src/graph/node_usage_with_count_iterator.rs` | 已验收 | — |
| `jdk.graal.compiler.graph.NodeUnionFind` | 节点并查集 | `rustci/crates/rustci-compiler/src/graph/node_union_find.rs` | 已验收 | — |
| `jdk.graal.compiler.graph.NodeSourcePosition` | 源码位置 | `rustci/crates/rustci-compiler/src/graph/node_source_position.rs` | 已验收 | — |
| `jdk.graal.compiler.graph.NodeSourcePositionFilter` | 源码位置过滤器 | `rustci/crates/rustci-compiler/src/graph/node_source_position_filter.rs` | 已验收 | — |
| `jdk.graal.compiler.graph.GraalGraphError` | 图错误 | `rustci/crates/rustci-compiler/src/graph/graal_graph_error.rs` | 已验收 | — |

## 偏离记录

- 节点引用使用 `NodeId`（usize）替代 `&dyn Node`，因为 Rust trait 对象无法持有 Graph 所有的 Vec<Box<dyn Node>> 的引用
- `Node` 从 Java abstract class 映射为 Rust trait（带默认方法），`NodeClass` 同理