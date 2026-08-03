# jdk.graal.compiler.graphio

## 源位置：`/opt/graal/compiler/src/jdk.graal.compiler/src/jdk/graal/compiler/graphio/`

## 状态：已验收

## 说明

Graph I/O：编译器图（HIR/LIR/CFG）的二进制序列化输出，供 Ideal Graph Visualizer（IGV）消费。核心输出层 + parsing 层 + parsing/model 层完整移植。已知限制：7 个 parsing 辅助文件未移植，Builder 的 blocks/elements/elements_and_locations 因 Rust 类型系统限制为桩。

## 映射表

### 核心输出层

| Java 类（全限定名） | 职责 | 对应 Rust 文件 | 状态 | 负责人 |
|---|---|---|---|---|
| `jdk.graal.compiler.graphio.GraphOutput` | 图输出主类 | `rustci/crates/rustci-compiler/src/graphio/graph_output.rs` | 已验收 | — |
| `jdk.graal.compiler.graphio.GraphProtocol` | 协议抽象类 | `rustci/crates/rustci-compiler/src/graphio/graph_protocol.rs` + `protocol_impl.rs` | 已验收 | — |
| `jdk.graal.compiler.graphio.GraphStructure` | 图结构接口 | `rustci/crates/rustci-compiler/src/graphio/graph_structure.rs` | 已验收 | — |
| `jdk.graal.compiler.graphio.GraphElements` | 图元素接口 | `rustci/crates/rustci-compiler/src/graphio/graph_elements.rs` | 已验收 | — |
| `jdk.graal.compiler.graphio.GraphLocations` | 图位置接口 | `rustci/crates/rustci-compiler/src/graphio/graph_locations.rs` | 已验收 | — |
| `jdk.graal.compiler.graphio.GraphBlocks` | 图块接口 | `rustci/crates/rustci-compiler/src/graphio/graph_blocks.rs` | 已验收 | — |
| `jdk.graal.compiler.graphio.GraphTypes` | 图类型接口 | `rustci/crates/rustci-compiler/src/graphio/graph_types.rs` | 已验收 | — |
| `jdk.graal.compiler.graphio.DefaultGraphBlocks` | 默认图块实现 | `rustci/crates/rustci-compiler/src/graphio/default_graph_blocks.rs` | 已验收 | — |
| `jdk.graal.compiler.graphio.DefaultGraphTypes` | 默认图类型实现 | `rustci/crates/rustci-compiler/src/graphio/default_graph_types.rs` | 已验收 | — |

### Parsing 层

| Java 类（全限定名） | 职责 | 对应 Rust 文件 | 状态 | 负责人 |
|---|---|---|---|---|
| `jdk.graal.compiler.graphio.parsing.BinaryReader` | 二进制读取器 | `rustci/crates/rustci-compiler/src/graphio/parsing/binary_reader.rs` | 已验收 | — |
| `jdk.graal.compiler.graphio.parsing.BinarySource` | 二进制数据源 | `rustci/crates/rustci-compiler/src/graphio/parsing/binary_source.rs` | 已验收 | — |
| `jdk.graal.compiler.graphio.parsing.BinaryStreamDefs` | 二进制流常量 | `rustci/crates/rustci-compiler/src/graphio/parsing/binary_stream_defs.rs` | 已验收 | — |
| `jdk.graal.compiler.graphio.parsing.Builder` | 图构建器 | `rustci/crates/rustci-compiler/src/graphio/parsing/builder.rs` | 已验收 | — |
| `jdk.graal.compiler.graphio.parsing.ConstantPool` | 常量池 | `rustci/crates/rustci-compiler/src/graphio/parsing/constant_pool.rs` | 已验收 | — |
| `jdk.graal.compiler.graphio.parsing.DataSource` | 数据源接口 | `rustci/crates/rustci-compiler/src/graphio/parsing/data_source.rs` | 已验收 | — |
| `jdk.graal.compiler.graphio.parsing.GraphParser` | 图解析器 | `rustci/crates/rustci-compiler/src/graphio/parsing/graph_parser.rs` | 已验收 | — |
| `jdk.graal.compiler.graphio.parsing.NameTranslator` | 名称翻译器 | `rustci/crates/rustci-compiler/src/graphio/parsing/name_translator.rs` | 已验收 | — |
| `jdk.graal.compiler.graphio.parsing.ParseMonitor` | 解析监控器 | `rustci/crates/rustci-compiler/src/graphio/parsing/parse_monitor.rs` | 已验收 | — |
| `jdk.graal.compiler.graphio.parsing.SkipRootException` | 跳过根异常 | `rustci/crates/rustci-compiler/src/graphio/parsing/skip_root_exception.rs` | 已验收 | — |
| `jdk.graal.compiler.graphio.parsing.StreamSource` | 流数据源 | `rustci/crates/rustci-compiler/src/graphio/parsing/stream_source.rs` | 已验收 | — |
| `jdk.graal.compiler.graphio.parsing.VersionMismatchException` | 版本不匹配异常 | `rustci/crates/rustci-compiler/src/graphio/parsing/version_mismatch_exception.rs` | 已验收 | — |

### Parsing Model 层

| Java 类（全限定名） | 职责 | 对应 Rust 文件 | 状态 | 负责人 |
|---|---|---|---|---|
| `...graphio.parsing.model.AbstractMutableDocumentItem` | 可变文档项抽象 | `.../parsing/model/abstract_mutable_document_item.rs` | 已验收 | — |
| `...graphio.parsing.model.ChangedEvent` | 变更事件 | `.../parsing/model/changed_event.rs` | 已验收 | — |
| `...graphio.parsing.model.ChangedEventProvider` | 变更事件提供者 | `.../parsing/model/changed_event_provider.rs` | 已验收 | — |
| `...graphio.parsing.model.ChangedListener` | 变更监听器 | `.../parsing/model/changed_listener.rs` | 已验收 | — |
| `...graphio.parsing.model.DataCollectionEvent` | 数据集合事件 | `.../parsing/model/data_collection_event.rs` | 已验收 | — |
| `...graphio.parsing.model.DataCollectionListener` | 数据集合监听器 | `.../parsing/model/data_collection_listener.rs` | 已验收 | — |
| `...graphio.parsing.model.DumpedElement` | 转储元素 | `.../parsing/model/dumped_element.rs` | 已验收 | — |
| `...graphio.parsing.model.Event` | 事件 | `.../parsing/model/event.rs` | 已验收 | — |
| `...graphio.parsing.model.Folder` | 文件夹 | `.../parsing/model/folder.rs` | 已验收 | — |
| `...graphio.parsing.model.FolderElement` | 文件夹元素 | `.../parsing/model/folder_element.rs` | 已验收 | — |
| `...graphio.parsing.model.GraphClassifier` | 图分类器 | `.../parsing/model/graph_classifier.rs` | 已验收 | — |
| `...graphio.parsing.model.GraphContainer` | 图容器 | `.../parsing/model/graph_container.rs` | 已验收 | — |
| `...graphio.parsing.model.GraphDocument` | 图文档 | `.../parsing/model/graph_document.rs` | 已验收 | — |
| `...graphio.parsing.model.GraphDocumentVisitor` | 图文档访问器 | `.../parsing/model/graph_document_visitor.rs` | 已验收 | — |
| `...graphio.parsing.model.Group` | 组 | `.../parsing/model/group.rs` | 已验收 | — |
| `...graphio.parsing.model.InputBlock` | 输入块 | `.../parsing/model/input_block.rs` | 已验收 | — |
| `...graphio.parsing.model.InputBlockEdge` | 输入块边 | `.../parsing/model/input_block_edge.rs` | 已验收 | — |
| `...graphio.parsing.model.InputBytecode` | 输入字节码 | `.../parsing/model/input_bytecode.rs` | 已验收 | — |
| `...graphio.parsing.model.InputEdge` | 输入边 | `.../parsing/model/input_edge.rs` | 已验收 | — |
| `...graphio.parsing.model.InputGraph` | 输入图 | `.../parsing/model/input_graph.rs` | 已验收 | — |
| `...graphio.parsing.model.InputMethod` | 输入方法 | `.../parsing/model/input_method.rs` | 已验收 | — |
| `...graphio.parsing.model.InputNode` | 输入节点 | `.../parsing/model/input_node.rs` | 已验收 | — |
| `...graphio.parsing.model.KnownPropertyNames` | 已知属性名 | `.../parsing/model/known_property_names.rs` | 已验收 | — |
| `...graphio.parsing.model.KnownPropertyValues` | 已知属性值 | `.../parsing/model/known_property_values.rs` | 已验收 | — |
| `...graphio.parsing.model.Properties` | 属性集合 | `.../parsing/model/properties.rs` | 已验收 | — |
| `...graphio.parsing.model.Property` | 属性 | `.../parsing/model/property.rs` | 已验收 | — |

## 偏离记录

- 7 个 parsing 辅助文件未移植：`DataBinaryPrinter`, `DataBinaryWriter`, `DocumentFactory`, `LocationCache`, `LocationStackFrame`, `LocationStratum`, `ModelBuilder`, `TemplateParser`
- Builder 的 `blocks()`/`elements()`/`elements_and_locations()` 因 Rust 泛型限制不能存储任意类型参数，当前为桩
- `GraphProtocol` 抽象类方法分散在 `protocol_impl.rs` 和各 trait 中，非独立抽象类
- `Collection`/`Iterable` 返回类型在 Rust 中统一为 `Vec`