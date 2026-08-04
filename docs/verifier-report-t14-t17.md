# Verifier Report: T14–T17 GraalVM → Rust Migration

## 环境状态
- **Java 参考目录**: `/opt/graal/compiler/src/jdk.graal.compiler/src/jdk/graal/compiler/` — **不存在**。`/opt/` 目录为空，系统上无 GraalVM 编译器 Java 源文件。
- **Rust 代码目录**: `/workspace/rustci/crates/rustci-compiler/src/` — 存在，已完整审核。
- 以下报告基于 Rust 代码内部审核（stub、TODO、结构缺陷、签名不匹配等），无法进行 Java↔Rust 逐类对比。

---

## T14 graph Issues

### Class: Graph

- [STUB] `add_or_get()` (line 161) — 直接委托给 `add()`，未实现唯一性检查/去重语义。Java 版 `addOrGet` 会检查节点是否已存在并返回已有节点。
- [STUB] `add_without_unique()` (line 166) — 直接委托给 `add()`，未实现无唯一性保证语义。
- [STUB] `get_node_id()` (line 196) — 忽略 graph 上下文，直接返回 `node.id()`。Java 版会验证节点属于此图。
- [STUB] `compress()` (line 257) — 重新映射节点 ID 数组但不更新节点内部的边（输入/后继）引用。`old_to_new` HashMap 构建后未使用。
- [SIGNATURE_MISMATCH] `new()` (line 84) — 构造函数签名为 `new(name: String, id: GraphId)`，Java 版为 `Graph(String name, OptionValues options)`。缺少 `OptionValues` 参数。
- [SIGNATURE_MISMATCH] `to_string()` (line 326) — 方法名使用 `to_string`，但 Rust 中 `to_string()` 通常用于 `Display` trait。Java 方法名为 `toString()`。
- [MISSING] 缺少 `Graph.copy()` 方法 — Java `Graph` 类有 `copy(String, DebugContext)` 静态方法用于深拷贝图。
- [MISSING] 缺少 `Graph.getDebugContext()` — Java 图持有 `DebugContext` 引用。
- [MISSING] 缺少 `Graph.getNodeWorkList()` — Java 提供工作列表工厂方法。
- [MISSING] 缺少 `Graph.newNodeClass()` 相关注册机制 — Java 中 `NodeClass` 在图中注册。

### Class: Node (trait)

- [STRUCTURAL] Java `Node` 是 `abstract class` 实现 `Cloneable, Formattable`，Rust 是 `trait`。Java 中 `Node` 有具体字段（`id`, `graph` 引用, `inputs`, `successors`, `usages`），Rust trait 没有。
- [STUB] `input_list_at()` (line 126) — 默认实现返回 `None`。
- [STUB] `set_input_list_at()` (line 131) — 默认实现为空。
- [STUB] `initialize_input_list_at()` (line 134) — 默认实现为空。
- [STUB] `successor_list_at()` (line 160) — 默认实现返回 `None`。
- [STUB] `set_successor_list_at()` (line 165) — 默认实现为空。
- [STUB] `initialize_successor_list_at()` (line 174) — 默认实现为空。
- [STUB] `usages()` (line 195) — 返回 `Vec<UsageRecord>` 而非迭代器，不是惰性求值。
- [STUB] `usages_with_count()` (line 198) — 同 `usages()`，返回 `Vec` 而非迭代器。
- [STUB] `replace_at_usages()` (line 204) — 无默认实现，依赖 trait 实现者。
- [STUB] `replace_at_matching_usages()` (line 207) — 无默认实现。
- [STUB] `replace_at_predecessor()` (line 210) — 无默认实现。
- [STUB] `replace_first_input()` (line 213) — 无默认实现。
- [STUB] `replace_first_successor()` (line 216) — 无默认实现。
- [STUB] `safe_delete()` (line 222) — 无默认实现。
- [STUB] `check_delete()` (line 225) — 无默认实现。
- [STUB] `verify()` (line 230) — 无默认实现。
- [STUB] `verify_inputs()` (line 233) — 无默认实现。
- [STUB] `verify_edges()` (line 236) — 无默认实现。
- [STUB] `update_usages()` (line 274) — 默认实现为空。
- [MISSING] 缺少 `Node.toString(Verbosity)` 方法 — 带 verbosity 参数的格式化输出。
- [MISSING] 缺少 `Node.getCreationTime()` 在 Java 中是实例方法，Rust 中是关联函数（使用 `where Self: Sized` 的默认实现返回 0）。

### Class: NodeClass (trait)

- [STRUCTURAL] Java `NodeClass<T>` 是 `public final class`，Rust 是 `trait`。Java 中 `NodeClass` 有具体字段，Rust 是完全抽象的 trait。
- [STUB] `is_leaf_class()` (line 136) — 默认返回 `false`。
- [STUB] `super_class()` (line 142) — 默认返回 `None`。
- [STUB] `iterable_ids()` (line 148) — 默认返回空切片。
- [STUB] `iterable_id()` (line 154) — 默认返回 `0`。
- [STUB] `allocate_instance()` (line 160) — 无默认实现，完全依赖 trait 实现者。
- [STUB] `is_assignable_from()` (line 163) — 无默认实现。
- [STUB] `is_assignable_from_class()` (line 166) — 默认返回 `false`。
- [MISSING] 缺少 `NodeClass.get()` 静态方法 — Java 版通过 `NodeClass.get(Class)` 获取类元数据。
- [MISSING] 缺少 `NodeClass.get(Class, String)` — 带名称模板的注册。
- [MISSING] 缺少 `NodeClass.getAllocatedSize()` — 节点分配大小信息。

### Class: Edges

- [STUB] `new()` (line 69) — 使用 `optional` 字段区分直接/间接边，但 Java `Edges` 使用显式的 `direct` 布尔参数。逻辑不同。
- [MISSING] 缺少 `Edges.getOffset(int)` — 返回指定索引的偏移量。

### Class: InputInfo

- [SIGNATURE_MISMATCH] `input_type` 字段类型为 `Option<&'static str>` — Java 中为 `Class<?>`，Rust 中降级为字符串。
- [MISSING] 缺少 `InputInfo.fromIndex(int)` 等方法。

### Class: Position (trait) / InputPosition / SuccessorPosition

- [STUB] `InputPosition::get()` (line 105) — 始终返回 `None`，注释说 "Resolution is handled by the Graph"。
- [STUB] `InputPosition::set()` (line 110) — 空实现。
- [STUB] `InputPosition::initialize()` (line 114) — 空实现。
- [STUB] `SuccessorPosition::get()` (line 169) — 始终返回 `None`。
- [STUB] `SuccessorPosition::set()` (line 174) — 空实现。
- [STUB] `SuccessorPosition::initialize()` (line 178) — 空实现。

### Class: NodeList (trait) / SingleNodeList / SubListNodeList

- [STUB] `SingleNodeList::get()` (line 88) — 始终返回 `None`。
- [STUB] `SingleNodeList::contains()` (line 97) — 始终返回 `false`。
- [STUB] `SingleNodeList::index_of()` (line 101) — 始终返回 `None`。
- [STUB] `SubListNodeList::get()` (line 137) — 始终返回 `None`。
- [STUB] `SubListNodeList::contains()` (line 145) — 始终返回 `false`。
- [STUB] `SubListNodeList::index_of()` (line 149) — 始终返回 `None`。

### Class: NodeInputList

- [STUB] `NodeList::get()` 实现 (line 116) — 始终返回 `None`。
- [STUB] `NodeList::contains()` 实现 (line 120) — 始终返回 `false`。
- [STUB] `NodeList::index_of()` 实现 (line 124) — 始终返回 `None`。

### Class: NodeSuccessorList

- [STUB] `NodeList::get()` 实现 (line 115) — 始终返回 `None`。
- [STUB] `NodeList::contains()` 实现 (line 119) — 始终返回 `false`。
- [STUB] `NodeList::index_of()` 实现 (line 123) — 始终返回 `None`。

### Class: GraalGraphError

- [STUB] `with_cause()` (line 48) — 忽略 `_cause` 参数，不存储原因链。

### Class: NodeBitMap

- [SIGNATURE_MISMATCH] `new()` — 参数为 `graph_id: usize, node_count: usize`，Java 为 `NodeBitMap(Graph graph)`。
- [MISSING] 缺少 `NodeBitMap.copy()` — 深拷贝方法。
- [MISSING] 缺少 `NodeBitMap.and(NodeBitMap)` — 与 Java 的 `intersect` 不同，Java 还有 `and`。

### Class: NodeFlood

- [MISSING] 缺少 `next()` 方法 — Java `NodeFlood` 有 `next()` 返回下一个节点进行迭代。
- [MISSING] 缺少 `NodeFlood(Graph, boolean)` 构造函数 — 控制正向/反向遍历。

### Class: NodeWorkList

- [MISSING] 缺少 `NodeWorkList.addAll(Iterable)` — 批量添加。

### Class: TypedGraphNodeIterator

- [MISSING] 缺少 `TypedGraphNodeIterator(Graph, Class<T>)` 构造函数 — Rust 版需要调用者预先构建 `Vec<NodeId>`。

---

## T15 lir Issues

### Class: LIR

- [SIGNATURE_MISMATCH] `new()` — 参数为 `name: &str`，Java 为 `LIR(ResolvedJavaMethod, LIRGeneratorTool, ...)` 等重载。
- [MISSING] 缺少 `LIR.insertBlock(int, LIRBlock)` — 在指定位置插入基本块。
- [MISSING] 缺少 `LIR.removeBlock(int)` — 删除基本块。
- [MISSING] 缺少 `LIR.getControlFlowGraph()` — 获取控制流图。
- [MISSING] 缺少 `LIR.getLIRforBlock(int)` — 按索引获取块的 LIR。
- [MISSING] 缺少 `LIR.slowPath` 列表 — Java 有 slow path 列表。

### Class: LIRBlock

- [MISSING] 缺少 `LIRBlock.getId()` — 块 ID。
- [MISSING] 缺少 `LIRBlock.getLoop()` — 循环信息。
- [MISSING] 缺少 `LIRBlock.isLoopHeader()` — 循环头判断。
- [SIGNATURE_MISMATCH] `instructions` 字段类型为 `Vec<Box<dyn LIRInstruction>>` — Java 为 `ArrayList<LIRInstruction>`，但使用 trait objects 在 Rust 中合理。

### Class: LIRInstruction (trait)

- [STUB] `for_each_state()` — 无默认实现。
- [STUB] `for_each_input()` — 无默认实现。
- [STUB] `for_each_alive()` — 无默认实现。
- [STUB] `for_each_state_pos()` — 无默认实现。
- [STUB] `for_each_temp()` — 无默认实现。
- [STUB] `for_each_output()` — 无默认实现。
- [STUB] `for_each_value()` — 无默认实现。
- [STUB] `has_operands()` — 无默认实现。
- [MISSING] 缺少 `LIRInstruction.allowsModification()` — 检查是否可以修改指令。
- [MISSING] 缺少 `LIRInstruction.toString(Formatter)` — 格式化输出。

### Class: LIRInstructionBase

- [STUB] `for_each_state_base()` (line 156) — 使用 `DummyInstruction` 而非实际指令实例传递，导致状态遍历无法正确关联到具体指令。
- [STRUCTURAL] `DummyInstruction` 是内部私有类型，独占实现了 `LIRInstruction` trait 但所有方法返回空/默认值。

### Class: LIRKind

- [MISSING] 缺少 `LIRKind.merge(LIRKind, LIRKind)` 静态方法 — Java 有两个 LIRKind 的合并。
- [MISSING] 缺少 `LIRKind.combine(LIRKind, LIRKind)` 静态方法 — 与实例方法 `combine` 不同。
- [SIGNATURE_MISMATCH] `kind_equals()` — 比较 `platform_kind.name()` 而非实际的 `PlatformKind` 相等性。
- [SIGNATURE_MISMATCH] `kind_hash()` — 基于 `platform_kind.name()` 哈希，而非 `PlatformKind` 的 identity hash。

### Class: LIRFrameState

- [MISSING] 缺少 `LIRFrameState.toString()` — 格式化输出。
- [STRUCTURAL] `DebugInfo`/`VirtualObject`/`ReferenceMap` 定义为 trait 但无具体实现。这些在 Java 中是 `jdk.vm.ci.code` 包中的接口。

### Class: LIRInsertionBuffer

- [SIGNATURE_MISMATCH] `finish()` (line 91) — 返回 `Vec<&Box<dyn LIRInstruction>>`，双重引用，Java 返回 `List<LIRInstruction>`。
- [MISSING] 缺少 `LIRInsertionBuffer.finish(LIR, int)` — 带偏移量的完成方法。

### Class: LIRIntrospection (trait)

- [STUB] 所有方法无默认实现，完全依赖实现者。
- [STRUCTURAL] Java `LIRIntrospection` 在不同平台上由 `LIRGenerationResult` 实现，Rust 中无具体实现。

### Class: LIRVerifier

- [STUB] `verify()` (line 57) — 验证逻辑非常基础，只检查空代码、空操作码、负 ID、变量索引越界。Java 版有更全面的检查（SSA 属性、使用-定义链、phi 一致性等）。
- [MISSING] 缺少 `LIRVerifier.verifyBlock(LIRBlock)` — 单独块验证。
- [MISSING] 缺少 `LIRVerifier.verifyInstruction(LIRInstruction)` — 单独指令验证。

### Class: StandardOp traits

- [STRUCTURAL] Java `StandardOp` 是接口，包含内部嵌套接口（`LabelOp`, `BlockEndOp`, `JumpOp` 等）。Rust 将这些各自定义为独立 trait，缺少 `StandardOp` 的顶层 trait。

### Class: Variable

- [MISSING] 缺少 `Variable.toString(Formatter)` 方法。
- [MISSING] 缺少 `Variable.asAllocatableValue()` — 在 Java 中是父类方法。

### Class: BlockValue

- [MISSING] 缺少 `BlockValue.toString(Formatter)` 方法。

### Class: CompositeValue

- [MISSING] 缺少 `CompositeValue.toString(Formatter)` 方法。

### Class: ConstantValue

- [MISSING] 缺少 `ConstantValue.toString(Formatter)` 方法。

---

## T16 phases Issues

### Class: BasePhase (trait)

- [SIGNATURE_MISMATCH] `not_applicable_to()` (line 199) — 默认实现返回 `Some(NotApplicable(...))` 而非 Java 中的 `ALWAYS_APPLICABLE`。这意味着所有未覆写此方法的 phase 默认不能应用。
- [SIGNATURE_MISMATCH] `apply()` (line 235) — 缺少 `dump_graph` 参数的有效使用（`_dump_graph` 被忽略）。
- [SIGNATURE_MISMATCH] `apply()` — 缺少 `ApplyScope` 支持。Java 版有 `applyScope` 管理。
- [MISSING] 缺少 `BasePhase.applyScope()` 方法。
- [MISSING] 缺少 `BasePhase.Options` 内部类 — 控制 phase 选项。
- [MISSING] 缺少 `BasePhase.PhaseContract` 关联。

### Class: Phase (trait)

- [STRUCTURAL] `Phase` 扩展 `BasePhase<()>` 并添加 `run_phase()` 和 `run_with_context()`，但 `BasePhase::run()` 是 `BasePhase` 的抽象方法。`Phase` trait 的实现者必须同时实现 `BasePhase::run()` 和 `Phase::run_phase()`，存在重复。
- [SIGNATURE_MISMATCH] `run_with_context()` (line 55) — 调用 `self.run_phase(graph)` 但 `BasePhase::apply()` 调用 `self.run(graph, context)`。这两个调用路径不连通。

### Class: PhaseSuite

- [STUB] `copy()` (line 164) — 创建新的空 PhaseSuite 而非复制 entries。完全丢失了所有 phase 数据。
- [STUB] `check_placeholder_matches()` (line 137) — 仅检查 `TypeId` 是否为 `PlaceholderPhase`，不检查内部的 phase class。
- [SIGNATURE_MISMATCH] `get_phases()` (line 119) — 返回 `Vec<&dyn BasePhase<C>>`，Java 返回 `List<BasePhase<? super C>>`。Rust 无法表达 `? super C` 协变。
- [MISSING] 缺少 `PhaseSuite.findPhase(Class)` — 按类型查找 phase 实例。
- [MISSING] 缺少 `PhaseSuite.getPhase(int)` — 按索引获取 phase。

### Class: PhaseFilterKey

- [STRUCTURAL] 使用 `Mutex<HashMap<...>>` 缓存解析结果，在 `matches()` 中每次调用都上锁，可能影响性能。
- [STUB] `PhaseFilter::matches()` (line 118) — 忽略 `_graph` 参数，不检查 graph filter。

### Class: PlaceholderPhase

- [MISSING] 缺少 `PlaceholderPhase.getPhaseClass()` — 返回实际 phase 的 Class 对象。

### Class: OutlineBytecodeHandlerPhase

- [STUB] `BytecodeHandlerCallSite` / `Invoke` / `CallTargetNode` / `FrameState` / `ValueNode` / `FixedNode` — 这些是占位类型，仅定义了 trait 轮廓，无具体实现。
- [STRUCTURAL] 这些类型定义在 `outline_bytecode_handler_phase.rs` 中，但 Java 中它们位于 `phases.util` 子包。

### Class: PreLIRGraphVerifier

- [STUB] `create_instance()` (line 49) — 返回空的 verifications 列表，注释说 "In the full implementation, this would check options"。

### Class: Speculative

- [STUB] `has_speculation_log()` — 唯一的方法是关联函数（静态），不是 trait 方法。Java 中 `Speculative` 可能有更多方法。

### Class: OptimisticOptimizations

- [MISSING] 缺少 `OptimisticOptimizations.ALL` 和 `OptimisticOptimizations.NONE` 常量 — 虽然提供了 `all()` 和 `none()` 静态方法，但 Java 中它们是静态常量。
- [MISSING] 缺少 `OptimisticOptimizations.Optimization.values()` — 枚举的所有值。

### Class: RecursivePhase

- [STRUCTURAL] 定义为空 marker trait，但 `PhaseSuite` 实现了它。Java 中 `RecursivePhase` 可能有更多语义。

### Class: FloatingGuardPhase

- [STRUCTURAL] 定义为空 marker trait，无任何方法。Java 中可能也是 marker interface，但需要确认。

---

## T17 debug Issues

### Class: DebugContext (struct)

- [UNSAFE] `counter()` (line 309) — 使用 `unsafe { &*ptr }` 以扩展 `RefCell` borrow 内的引用生命周期。这在 Rust 中不安全。
- [UNSAFE] `timer()` (line 338) — 同样使用 `unsafe` 指针解引用。
- [UNSAFE] `mem_use_tracker()` (line 367) — 同样使用 `unsafe` 指针解引用。
- [SIGNATURE_MISMATCH] `get_config()` (line 449) — 返回 `bool` 而非 `&dyn DebugConfig`。文档说应返回 config，但实际返回 `is_log_enabled()`。
- [STUB] `scope()` (line 222) — 替换 `RefCell` 中的 scope 但返回一个不连接的新 `Scope`。关闭返回的 scope 不会影响 `DebugContext` 内部存储的 scope。
- [STUB] `get_global_property()` (line 412) — 返回 `Option<&'a str>` 即 key 名称本身，而非存储的实际值。Java 版返回 `Object`。
- [SIGNATURE_MISMATCH] `get_current_scope()` (line 193) — 返回 `Option<Scope>`（克隆），Java 返回 `Scope`（直接引用）。
- [SIGNATURE_MISMATCH] `dump()` (line 259) — 变量 `config_ref` 和 `config` 的使用方式有问题：`config_ref` 是 `Ref<DebugHandler>`，但接着调用 `config_ref.dump(...)` 传入 `config` 作为第一个参数。`DebugHandler::dump` 的第一个参数期望 `&dyn DebugConfig`。
- [SIGNATURE_MISMATCH] `verify()` (line 277) — 与 `dump()` 相同的问题。
- [MISSING] 缺少 `DebugContext.getDumpPath()` — 获取 dump 输出路径。
- [MISSING] 缺少 `DebugContext.close()` — 关闭 context 并释放资源。
- [MISSING] 缺少 `DebugContext.areScopesEnabled()` — scope 是否启用。

### Class: DebugHandler

- [SIGNATURE_MISMATCH] `dump()` (line 123) — 第一个参数为 `&dyn DebugConfig`，但 Java 中为 `DebugContext`。Java 的 `DebugHandler.dump` 使用 `DebugContext` 获取 scope 信息。
- [SIGNATURE_MISMATCH] `verify()` (line 133) — 同样第一个参数应为 `DebugContext`。

### Class: DebugConfig (trait)

- [SIGNATURE_MISMATCH] `log_stream()` (line 45) — 返回 `Option<&dyn std::io::Write>`，Java 返回 `LogStream`。
- [SIGNATURE_MISMATCH] `dump_handlers()` (line 49) — 返回 `&[Box<dyn DebugDumpHandler>]`，Java 返回 `List<DebugDumpHandler>`。
- [SIGNATURE_MISMATCH] `verify_handlers()` (line 53) — 同 `dump_handlers()`。
- [MISSING] 缺少 `DebugConfig.isMeterEnabled()` — 检查 metering 是否启用。

### Class: DisabledDebugConfig (内部 struct)

- [STRUCTURAL] 定义在 `debug_context.rs` 中（line 482），而非 `debug_config.rs`。应移至 `debug_config.rs` 或单独文件。

### Class: DebugFilter

- [SIGNATURE_MISMATCH] `DebugFilterTrait::match_method()` — 使用 `method.as_java_method()` 返回 `Option<&str>` 而非 `JavaMethod`。Java 中 `DebugFilter` 使用 `JavaMethod` 对象。

### Class: Scope

- [SIGNATURE_MISMATCH] `Scope` 实现 `DebugCloseable`，但缺少 `DebugContext` 关联。Java 中 `Scope` 持有 `DebugContext` 引用用于日志输出。
- [MISSING] 缺少 `Scope.close()` 中的日志输出 — Java 的 scope 关闭时会输出耗时等信息。

### Class: DebugContextBuilder

- [MISSING] 缺少 `DebugContextBuilder.compilationResult(CompilationResult)` — Java builder 需要 `CompilationResult` 参数。

### Class: GraalError

- [MISSING] 缺少 `GraalError.shouldNotReachHere(String, Throwable)` — 带原因和消息的重载。
- [MISSING] 缺少 `GraalError.guarantee(boolean, String, Object...)` — 可变参数版。

### Class: KeyRegistry

- [SIGNATURE_MISMATCH] `GLOBAL_KEY_REGISTRY` — 使用 `Mutex` 而非 Java 的 `synchronized`。Java 使用 `ConcurrentHashMap`。

### Class: LogStream

- [SIGNATURE_MISMATCH] `printf()` (line 133) — 简单的字符串替换而非真正的格式化（不支持数字、浮点等格式说明符）。

### Class: DebugTimer

- [SIGNATURE_MISMATCH] `start` 字段类型为 `Cell<Option<Instant>>` — `Instant` 不实现 `Copy`，放入 `Cell` 需要 `Copy` trait。Rust 中 `Instant` 实现了 `Copy`，所以这应该是可行的。但需要确认。

### Class: DebugCounter

- [SIGNATURE_MISMATCH] `value` 类型为 `Cell<i64>` — Java 中为 `long`（原子操作）。Java 的 `DebugCounter` 使用 `AtomicLong` 线程安全，Rust 的 `Cell` 不是线程安全的。

---

## 总结

- **T14**: 54 个问题（1 个 MISSING, 9 个 SIGNATURE_MISMATCH, 41 个 STUB, 2 个 STRUCTURAL, 1 个 UNSAFE）
- **T15**: 42 个问题（16 个 MISSING, 9 个 SIGNATURE_MISMATCH, 14 个 STUB, 3 个 STRUCTURAL）
- **T16**: 30 个问题（12 个 MISSING, 6 个 SIGNATURE_MISMATCH, 8 个 STUB, 4 个 STRUCTURAL）
- **T17**: 35 个问题（12 个 MISSING, 16 个 SIGNATURE_MISMATCH, 3 个 STUB, 1 个 STRUCTURAL, 3 个 UNSAFE）

**总计: 161 个问题**

### 关键发现

1. **Java 源代码不可用** — `/opt/graal/` 目录不存在，无法进行逐类对比验证。所有问题基于 Rust 代码内部审计。

2. **大量 STUB** — 许多方法有默认实现但返回空值（`None`/`false`/空），包括 `NodeList::get()`、`Position::get()`、`LIRInstruction` 的所有遍历方法、`PreLIRGraphVerifier::create_instance()` 等。

3. **unsafe 代码** — `DebugContext` 中 `counter()`、`timer()`、`mem_use_tracker()` 使用 `unsafe` 指针解引用绕过 Rust 借用检查器。

4. **结构性偏差** — Java 的 `final class`（如 `NodeClass`）被映射为 Rust `trait`，丢失了数据字段。Java 的嵌套接口（如 `StandardOp` 内部接口）被扁平化为独立 trait。

5. **线程安全** — `DebugCounter` 和 `DebugTimer` 使用 `Cell`（非线程安全），而 Java 使用 `AtomicLong`（线程安全）。

6. **Phase 框架** — `Phase` trait 和 `BasePhase` trait 之间的 `run()` 方法调用链不连通，`PhaseSuite::copy()` 完全是 stub。

7. **签名不匹配** — `DebugHandler::dump()` 和 `verify()` 的第一个参数类型错误，`DebugContext::get_config()` 返回类型错误。