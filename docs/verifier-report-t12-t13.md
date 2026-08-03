# GraalVM→Rust 迁移验证报告: T12 core + T13 nodes

**生成时间**: 2026-08-03
**Java 参考**: `/opt/graal/compiler/src/jdk.graal.compiler/src/jdk/graal/compiler/`
**Rust 代码**: `/workspace/rustci/crates/rustci-compiler/src/`

---

## T12 core Issues

### 覆盖范围
- Java 核心源文件: 10 个类 (不含 package-info.java)
- Rust 核心源文件: 10 个 .rs + mod.rs + phases.rs
- 对应关系: 一一对应

---

### Class: ArchitectureSpecific
Java 路径: `core/ArchitectureSpecific.java`
Rust 路径: `core/architecture_specific.rs`

**状态: OK** — 接口匹配正确。Java `interface ArchitectureSpecific { String getArchitecture(); }` → Rust `trait ArchitectureSpecific { fn get_architecture(&self) -> &str; }`。

---

### Class: CompilationPrinter
Java 路径: `core/CompilationPrinter.java`
Rust 路径: `core/compilation_printer.rs`

- [MISSING] public method `finish(CompilationResult, InstalledCode)`: Java 中打印编译统计信息的核心方法，Rust 中完全缺失。
- [MISSING] public static method `close()`: Java 中关闭共享 CSV 流的方法。
- [MISSING] public static method `printingToCSV()`: Java 中检查 CSV 流是否打开的方法。
- [MISSING] static field `csvStream` (volatile PrintStream): Java 中的共享 CSV 流。Rust 中用独立的 `CsvState` 结构体替代，但未与 `CompilationPrinter` 关联。
- [MISSING] private method `initializeStream(String)` / `maybePrintCSV(long, long, long, int, int, InstalledCode)`: 虽然这些是 private，但 `begin()` 和 `finish()` 依赖它们。
- [SIGNATURE_MISMATCH] `begin()`: Java 签名为 `begin(OptionValues, CompilationIdentifier, Object, int)` → `CompilationPrinter`；Rust 签名为 `begin(CompilationIdentifier, CompilationSource, i32, bool, bool) -> Option<Self>`。参数类型完全不同。
- [DESIGN] `source` 字段: Java 使用 `Object`（可为 `JavaMethod` 或 `ForeignCallSignature`），Rust 使用 `CompilationSource` 枚举。这是合理的设计选择，但 `CompilationSource` 定义在 `compilation_printer.rs` 中而非独立的 `core/common` 包中。
- [DESIGN] `CompilationIdentifier` 定义在 `compilation_watch_dog.rs` 中，被 `compilation_printer.rs` 引用——Java 中 `CompilationIdentifier` 在 `core/common` 包中。

---

### Class: CompilationWatchDog
Java 路径: `core/CompilationWatchDog.java`
Rust 路径: `core/compilation_watch_dog.rs`

- [MISSING] `implements Runnable`: Java `CompilationWatchDog` 实现 `Runnable`（含 `run()` 方法），`AutoCloseable`（含 `close()` 方法）。Rust 中 `CompilationWatchDog` 只是一个普通 struct，没有任何等效的 trait 实现。
- [MISSING] `run()` 方法: 这是看门狗的核心逻辑——每 1 秒采样一次堆栈，检测长时间运行和卡住的编译。Rust 中完全缺失。
- [MISSING] `close()` → `stopCompilation()` 方法: 停止看门狗并取消调度任务。
- [MISSING] 字段: `watchedThread`, `lastStackTrace`, `singleShotExecutor`, `task`, `debug`, `vmExitDelayNS`。Rust 只有 `compilation`, `start_delay_secs`, `vm_exit_delay_secs`。
- [MISSING] static field `CURRENT_THREAD_LABEL`: 用于日志输出的共享标签对象。
- [MISSING] inner class `Options`: 包含 `CompilationWatchDogStartDelay` 和 `CompilationWatchDogVMExitDelay` 选项键。Rust 用 `WatchDogOptions` 替代，但只有常量没有完整的 OptionKey 语义。
- [MISSING] `toString()` 方法: 返回 `"WatchDog[" + watchedThread.getName() + "]"`。
- [MISSING] private methods: `recordStackTrace`, `trace`, `seconds`, `schedule`, `createExecutor`, `stopCompilation`。
- [SIGNATURE_MISMATCH] `watch()`: Java 签名为 `watch(CompilationIdentifier, OptionValues, boolean, EventHandler, ThreadFactory) → CompilationWatchDog`；Rust 签名为 `watch(CompilationIdentifier, i32, i32) -> Option<Self>`。
- [SIGNATURE_MISMATCH] `EventHandler.on_long_compilation()`: Java 签名为 `onLongCompilation(CompilationWatchDog, Thread, CompilationIdentifier, long, StackTraceElement[])`；Rust 签名为 `on_long_compilation(&self, &CompilationIdentifier, Duration, &[String])`。
- [SIGNATURE_MISMATCH] `EventHandler.on_stuck_compilation()`: Java 签名为 `onStuckCompilation(CompilationWatchDog, Thread, CompilationIdentifier, StackTraceElement[], long stuckTime)`；Rust 签名为 `on_stuck_compilation(&self, &CompilationIdentifier, &[String], Duration)`。
- [SIGNATURE_MISMATCH] `EventHandler.on_exception()`: Java 签名为 `onException(Throwable)`；Rust 签名为 `on_exception(&self, &dyn Error)`。

---

### Class: CompilationWrapper
Java 路径: `core/CompilationWrapper.java` (602 行)
Rust 路径: `core/compilation_wrapper.rs` (118 行)

- [MAJOR] 整个类从 Java 抽象类变为 Rust trait: Java `abstract class CompilationWrapper<T>` 有 2 个字段、1 个构造函数、15+ 个方法（含 `Failure` 内部类）。Rust `trait CompilationWrapper<T>` 只有 5 个方法，无字段，无状态。
- [MISSING] 构造函数 `CompilationWrapper(DiagnosticsOutputDirectory, Map<ExceptionAction, Integer>)`: 初始化 outputDirectory 和 problemsHandledPerAction。
- [MISSING] 字段: `outputDirectory`, `problemsHandledPerAction`。
- [MISSING] `Failure` 内部类: 包含 `cause` 和 `debug` 字段，`handle(boolean)` 方法。
- [MISSING] protected method `lookupAction(OptionValues, Throwable)`: 根据选项值和异常原因查找要采取的操作。
- [MISSING] protected method `handleFailure(DebugContext, Throwable)`: 诊断/重试逻辑的核心——约 100 行复杂逻辑。
- [MISSING] protected method `dumpOnError(DebugContext, Throwable)`: 错误时转储的钩子。
- [MISSING] protected abstract method `createRetryDebugContext(DebugContext, OptionValues, PrintStream)`: 创建重试编译的调试上下文。
- [MISSING] protected abstract method `parseRetryOptions(String[], EconomicMap)`: 解析重试选项。
- [MISSING] protected abstract method `exitHostVM(int)`: 在嵌入 Graal 的运行时中调用 System.exit。
- [MISSING] protected method `requestExitVMOnCompilationFailure()`: 请求 VM 退出的钩子。
- [MISSING] private static method `isNonFailureBailout(OptionValues, Throwable)`: 判断 bailout 是否应视为非失败。
- [MISSING] private static method `detectCompilationFailureRateTooHigh(OptionValues, Throwable)`: 检测系统性编译失败。
- [MISSING] private static method `getCompilationPeriodStart(long)`: 获取编译周期开始时间。
- [MISSING] private method `adjustAction(OptionValues, ExceptionAction, Throwable)`: 根据问题计数调整操作。
- [MISSING] private method `performDiagnosticRetry(...)`: 执行诊断重试编译。
- [MISSING] private method `getRetryOptions(OptionValues, String)`: 获取重试编译选项。
- [MISSING] private method `postRetry(ExceptionAction, T)`: 重试后处理。
- [MISSING] private static method `finalizeRetryLog(String, ByteArrayOutputStream, PrintStream)`: 完成重试日志。
- [MISSING] private method `maybeExitVM(ExceptionAction)`: 可能退出 VM。
- [MISSING] static fields: `totalCompilations`, `failedCompilations`, `compilationPeriodStart`, `COMPILATION_FAILURE_DETECTION_PERIOD_NS`, `MIN_COMPILATIONS_FOR_FAILURE_DETECTION`。
- [MISSING] `printCompilationFailureActionAlternatives` static method。
- [DESIGN] `run()` 方法: Java 使用 `try-catch` + `totalCompilations.incrementAndGet()`；Rust 使用 `catch_unwind`。机制不同，且 Rust 缺少计数器递增。
- [DESIGN] `ExceptionAction.quieter()`: Java 实现为 `int index = Math.max(ordinal() - 1, 0); return VALUES[index];`；Rust 实现为 match 语句，功能等价。OK。

---

### Class: CompilerThread
Java 路径: `core/CompilerThread.java`
Rust 路径: `core/compiler_thread.rs`

- [MISSING] `extends Thread`: Java `CompilerThread` 继承 `java.lang.Thread`，是真正的线程。Rust 只是一个数据 struct，不含线程行为。
- [MISSING] 构造函数 `CompilerThread(Runnable, String)`: Java 接收 `Runnable` 并调用 `super(r)`；Rust 只有 `new(name_prefix)` 无 Runnable 参数。
- [MISSING] `run()` 方法: Java 重写 `run()` 设置上下文类加载器后调用 `super.run()`；Rust 无此方法。
- [MISSING] 线程行为: Java 设置 `setPriority(MAX_PRIORITY)`, `setDaemon(true)`, 使用 `namePrefix + "-" + threadId` 命名；Rust 只存储 `name_prefix` 字符串。
- [以 Rust 替换] Rust 用 `MAX_PRIORITY: i32 = 10` 常量；Java 使用 `Thread.MAX_PRIORITY`。语义等价。

---

### Class: CompilerThreadFactory
Java 路径: `core/CompilerThreadFactory.java`
Rust 路径: `core/compiler_thread_factory.rs`

- [MISSING] `implements ThreadFactory`: Java 实现 `java.util.concurrent.ThreadFactory`；Rust 无此 trait。
- [SIGNATURE_MISMATCH] `newThread(Runnable)` → `Thread`: Java 签名为 `newThread(Runnable r) → Thread`，返回 `new CompilerThread(r, threadNamePrefix)`；Rust 签名为 `new_thread(&self) → CompilerThread`，无 Runnable 参数，返回 `CompilerThread` 而非 `Thread`。
- [DESIGN] `threadNamePrefix` 字段: Java 为 `protected final String`；Rust 为 `pub String`。OK。

---

### Class: GraalCompiler
Java 路径: `core/GraalCompiler.java`
Rust 路径: `core/graal_compiler.rs`

- [MISSING] `Request<T>` record: Java 的 `record Request<T extends CompilationResult>` 包含 14 个组件字段：`graph`, `installedCodeOwner`, `providers`, `backend`, `graphBuilderSuite`, `optimisticOpts`, `profilingInfo`, `suites`, `lirSuites`, `compilationResult`, `factory`, `entryPointDecorator`, `requestedCrashHandler`, `verifySourcePositions`。Rust 的 `CompilationRequest` 只有 4 个字段：`id`, `source`, `entry_bci`, `verify_source_positions`。这是不完整的等价映射。
- [MISSING] `Request.execute()` 方法: Java 调用 `GraalCompiler.compile(this)`。
- [MISSING] static fields: `CompilerTimer`, `CompilerMemory`, `FrontEnd` (TimerKey/MemUseTrackerKey)。
- [MISSING] private static method `checkForHeapDump(Request<T>, DebugContext)`: 检查 `DumpHeapAfter` 选项并执行堆转储。
- [MISSING] private static method `checkForRequestedDelay(StructuredGraph)`: 检查 `InjectedCompilationDelay` 选项并延迟编译。
- [MISSING] private static method `match(StructuredGraph, String)`: 使用 `GraphFilter` 进行方法模式匹配。
- [STUB] `compile(CompilationRequest)`: Java 方法执行完整的编译流程（前端 + 后端 + 崩溃检查 + 延迟检查 + 堆转储检测）；Rust 版本只创建一个空的 `CompilationResult`，不执行任何实际编译。
- [STUB] `emit_front_end(CompilationRequest)`: Java 方法执行解析、高/中/低层优化、死代码消除、调度等；Rust 版本为空函数体（只有注释）。
- [DESIGN] `CompilationResult` 类型: Java 使用泛型 `<T extends CompilationResult>`；Rust 使用具体 struct `CompilationResult`。不完全等价。
- [DESIGN] `RequestedCrashHandler` → Rust: 作为 trait 保留，OK。

---

### Class: GraalCompilerOptions
Java 路径: `core/GraalCompilerOptions.java`
Rust 路径: `core/graal_compiler_options.rs`

- [DESIGN] 选项类型: Java 使用 `OptionKey<Boolean>`, `OptionKey<String>`, `EnumOptionKey<ExceptionAction>`, `OptionKey<Integer>`, `PhaseFilterKey` 等富类型；Rust 使用原生常量（`bool`, `Option<&str>`, `i32`, `ExceptionAction`）。这是简化设计，但丢失了 OptionKey 的元数据（help 文本、type、stability 等）。
- [MISSING] `DumpHeapAfter` 字段: Java 为 `PhaseFilterKey`（支持阶段过滤）；Rust 为 `&str` 常量 `"<compilation>"`。不支持阶段过滤。
- 所有 9 个选项常量都存在，映射关系正确。

---

### Class: Instrumentation
Java 路径: `core/Instrumentation.java`
Rust 路径: `core/instrumentation.rs`

**状态: OK** — 空接口 → 空 trait，完全匹配。

---

### Class: LIRGenerationPhase
Java 路径: `core/LIRGenerationPhase.java`
Rust 路径: `core/lir_generation_phase.rs`

- [MISSING] `extends LIRPhase<LIRGenerationContext>`: Java 继承自 `LIRPhase`；Rust 无此继承链。
- [MISSING] inner class `LIRGenerationContext`: Java 的 `LIRGenerationContext` 包含 `nodeLirBuilder`, `lirGen`, `graph`, `schedule` 四个字段；Rust 的 `LIRGenerationContext` 只有 `graph_name` 和 `node_count` 两个字段。
- [STUB] `run()`: Java 的 `run(TargetDescription, LIRGenerationResult, LIRGenerationContext)` 执行实际的 LIR 生成（matchBlock + emitBlock 循环 + beforeRegisterAllocation + SSA 验证 + 计数器更新）；Rust 的 `run(&mut self, &LIRGenerationContext, usize)` 只是简单赋值 `instruction_count = block_count as u64`。这是占位实现。
- [MISSING] static fields: `instructionCounter` (CounterKey), `nodeCount` (CounterKey)。
- [MISSING] private static methods: `emitBlock`, `matchBlock`, `verifyPredecessors`, `isProcessed`。

---

### Extra: phases.rs
Rust 路径: `core/phases.rs`

此文件覆盖了多个 Java 类（来自 `core/phases/` 子包）：
- `BaseTier.java`, `HighTier.java`, `MidTier.java`, `LowTier.java`
- `CEOptimization.java`, `CommunityCompilerConfiguration.java`, `EconomyCompilerConfiguration.java`
- `EconomyHighTier.java`, `EconomyMidTier.java`, `EconomyLowTier.java`, `EconomyMarkFixReadsPhase.java`

- [MISSING] 所有 Java 类中的具体阶段（phase）列表：Java 的 `HighTier`, `MidTier`, `LowTier` 等类在构造函数中注册了具体的优化阶段；Rust 的对应结构体只创建空的 `PhaseSuite`，没有注册任何阶段。
- [MISSING] `CEOptimization` 枚举值: Java 枚举包含 `getOption()`, `getPhaseType()`, `getPhaseClass()` 等方法；Rust 只实现了 `name()` 方法。
- [MISSING] `EconomyMarkFixReadsPhase` 的 `apply` 方法在 Java 中执行实际的图标记操作；Rust 中为空函数体。
- [DESIGN] `PhaseSuite`, `BaseTier`, 各 Tier 类型: 这是合理的 Rust 翻译，但所有 phase suite 都是空的，没有填充任何实际阶段。

---

## T13 nodes Issues

### 覆盖范围
- Java 顶级源文件: 约 106 个类/接口
- Rust 顶级源文件: 30 个 .rs + mod.rs
- 已移植: 30 个核心类（约 28%）
- 未移植: 约 76 个类（包括 `AbstractDeoptimizeNode`, `AbstractFixedGuardNode`, `AbstractLocalNode`, `AbstractStateSplit`, `ArithmeticOperation`, `BeginStateSplitNode`, `BinaryOpLogicNode`, `BreakpointNode`, `CallTargetNode`, `Cancellable`, `CanonicalizableLocation`, `CompanionObjectEncoder`, `CompressionNode`, `ComputeObjectAddressNode`, `ConditionAnchorNode`, `DeadEndNode`, `DeoptBarrier`, `DeoptBciSupplier`, `DeoptimizeNode`, `DeoptimizingFixedWithNextNode`, `DeoptimizingGuard`, `DirectCallTargetNode`, `DynamicDeoptimizeNode`, `DynamicPiNode`, `EncodedGraph`, `EntryMarkerNode`, `EntryProxyNode`, `FieldLocationIdentity`, `FixedAccessNodeInterface`, `FixedGlobalValueNumberable`, `FixedGuardNode`, `FixedNodeInterface`, `FixedWithNextNodeInterface`, `FloatingAnchoredNode`, `FloatingGuardedNode`, `FullInfopointNode`, `GetObjectAddressNode`, `GuardPhiNode`, `GuardProxyNode`, `GuardedValueNode`, `ImplicitNullCheckNode`, `IndirectCallTargetNode`, `InliningLogCodec`, `Invokable`, `Invoke`, `InvokeNode`, `InvokeWithExceptionNode`, `LIRLowerableLogicNode`, `LogicConstantNode`, `LogicNegationNode`, `LogicNode`, `LoweredCallTargetNode`, `MemoryMapControlSinkNode`, `MemoryProxyNode`, `MultiReturnNode`, `NamedLocationIdentity`, `NodeClassMap`, `NodeView`, `NonFoldingConstantNode`, `OptimizationLogCodec`, `PauseNode`, `PhiNode`, `PiArrayNode`, `PluginReplacementInterface`, `PluginReplacementNode`, `PluginReplacementWithExceptionNode`, `PrefetchAllocateNode`, `ProfileData`, `ProxyNode`, `ReadArgumentNode`, `SafepointNode`, `ShortCircuitOrNode`, `SnippetAnchorNode`, `SpinWaitNode`, `StateSplit`, `StaticDeoptimizingNode`, `TypeCheckHints`, `UnaryOpLogicNode`, `UnreachableBeginNode`, `UnreachableControlSinkNode`, `UnreachableNode`, `UnwindNode`, `ValueNodeInterface`, `ValuePhiNode`, `ValueProxyNode`, `VirtualState`, `WithExceptionNode` 等）

---

### Class: AbstractBeginNode
Java 路径: `nodes/AbstractBeginNode.java`
Rust 路径: `nodes/abstract_begin_node.rs`

- [MISSING] public method `prepareDelete()`: 准备删除节点，疏散锚定值。
- [MISSING] public method `prepareDelete(FixedNode)`: 带疏散起点的准备删除。
- [MISSING] public method `verifyNode()`: 验证节点（前驱必须非空或是 graph.start() 或 AbstractMergeNode）。
- [MISSING] public method `generate(NodeLIRBuilderTool)`: LIR 生成（处理 speculation fence）。
- [MISSING] public method `isUsedAsGuardInput()`: 检查是否用作守卫输入。**Rust 中有此方法**（在 trait 中声明），但 Java 版本遍历所有用法检查 InputType——Rust 实现 `BeginNode` 时直接返回 `false`，未实现实际逻辑。
- [MISSING] public method `guards()`: 返回 `NodeIterable<GuardNode>`。
- [MISSING] public method `anchored()`: 返回 `NodeIterable<Node>`。
- [MISSING] public method `hasAnchored()`: 是否锚定值。
- [MISSING] public method `getBlockNodes()`: 返回 `NodeIterable<FixedNode>`。
- [MISSING] public method `setHasSpeculationFence()`: 设置 speculation fence。
- [MISSING] public method `mustNotMoveAttachedGuards()`: 优化器是否允许移动守卫。
- [MISSING] 字段 `hasSpeculationFence` (boolean): Java 有；Rust trait 无此字段（各实现 struct 自行定义）。
- [MISSING] inner class `BlockNodeIterator`: 实现 `Iterator<FixedNode>` 的私有迭代器。
- [MISSING] private method `evacuateAnchored(FixedNode)`: 疏散锚定节点。
- [SIGNATURE_MISMATCH] `prev_begin`: Java 签名为 `prevBegin(FixedNode) → AbstractBeginNode`（返回 null 或 AbstractBeginNode）；Rust 签名为 `prev_begin(from: &dyn FixedNode) -> Option<&dyn AbstractBeginNode>`。语义等价但类型不同。
- [DESIGN] Java 为抽象类继承 `FixedWithNextNode`，实现 `LIRLowerable`, `GuardingNode`, `AnchoringNode`, `IterableNodeType`；Rust 为 trait 继承 `FixedWithNextNode + GuardingNode`。缺少 `LIRLowerable`, `AnchoringNode`, `IterableNodeType`。

---

### Class: AbstractEndNode
Java 路径: `nodes/AbstractEndNode.java`
Rust 路径: `nodes/abstract_end_node.rs`

- [MISSING] public method `generate(NodeLIRBuilderTool)`: 调用 `gen.visitEndNode(this)`。
- [MISSING] public method `merge()`: 返回 `AbstractMergeNode`。**Rust 中有声明**但返回 `Option<&dyn AbstractMergeNode>`——Java 返回可能为 null 的非 Optional 类型。
- [MISSING] public method `verifyNode()`: 验证 getUsageCount() <= 1。
- [MISSING] public method `cfgSuccessors()`: 返回 CFG 后继列表。
- [DESIGN] Java 实现 `LIRLowerable`；Rust trait 未实现等效接口。

---

### Class: AbstractMergeNode
Java 路径: `nodes/AbstractMergeNode.java`
Rust 路径: `nodes/abstract_merge_node.rs`

- [MISSING] 字段 `ends` (NodeInputList<EndNode>): Java 的 `@Input(Association) protected NodeInputList<EndNode> ends` ——核心字段，存储所有前向结束节点。Rust trait 无此字段声明。
- [MISSING] public method `generate(NodeLIRBuilderTool)`: 调用 `gen.visitMerge(this)`。
- [MISSING] public method `isPhiAtMerge(Node)`: 判断节点是否是合并到此 merge 的 phi。
- [MISSING] public method `removeEnd(AbstractEndNode)`: 移除结束节点及相关 phi 条目——约 15 行复杂逻辑。
- [MISSING] protected method `deleteEnd(AbstractEndNode)`: 从 ends 列表中移除。
- [MISSING] public method `forwardEnds()`: 返回 `NodeInputList<EndNode>`。
- [MISSING] public method `phis()`: 返回 `NodeIterable<PhiNode>`，包含去重逻辑。
- [MISSING] public method `valuePhis()`: 返回 `NodeIterable<ValuePhiNode>`。
- [MISSING] public method `memoryPhis()`: 返回 `NodeIterable<MemoryPhiNode>`。
- [MISSING] public method `anchored()`: 重写，过滤掉 phi 节点。
- [MISSING] public method `simplify(SimplifierTool)`: 约 50 行简化逻辑——处理 merge 拆分、phi 复制等。
- [MISSING] public static method `duplicateReturnThroughMerge(MergeNode)`: 约 30 行——通过 merge 复制返回指令。
- [MISSING] protected method `verifyState()`: 验证 stateAfter 不为 null。
- [MISSING] public method `verifyNode()`: 验证帧状态。
- [DESIGN] Java 继承 `BeginStateSplitNode`，实现 `IterableNodeType`, `Simplifiable`, `LIRLowerable`；Rust trait 继承 `AbstractBeginNode`。`BeginStateSplitNode` 继承链缺失。

---

### Class: BeginNode
Java 路径: `nodes/BeginNode.java`
Rust 路径: `nodes/begin_node.rs`

- [MISSING] `implements Simplifiable`: Java `BeginNode` 实现 `Simplifiable`；Rust 无等效 trait。
- [MISSING] `simplify(SimplifierTool)` 方法: 如果前驱不是 ControlSplitNode，则移除 BeginNode 并重连控制流边。
- [MISSING] `prepareDelete()`, `verifyNode()`, `guards()`, `anchored()`, `hasAnchored()`, `getBlockNodes()`, `setHasSpeculationFence()`, `mustNotMoveAttachedGuards()`: 继承自 `AbstractBeginNode` 的方法，Rust 中未实现。
- [STUB] `begin(FixedWithNextNode)` 方法: 直接 `panic!("begin() requires graph context")`——这是占位实现，应立即修复。
- [DESIGN] Java 的 `BeginNode` 是 `final class`，Rust 是具体 struct。OK。
- [DESIGN] 字段 `speculation_fence` 在 Rust 中为 `pub speculation_fence: bool`，Java 中为 `private boolean hasSpeculationFence`。Java 中通过 `hasSpeculationFence()` 和 `setHasSpeculationFence()` 访问；Rust 直接公开字段 + `has_speculation_fence()` trait 方法。

---

### Class: ConstantNode
Java 路径: `nodes/ConstantNode.java`
Rust 路径: `nodes/constant_node.rs`

- [MISSING] `extends FloatingNode`: Java 继承 `FloatingNode`；Rust 只实现 `ValueNode` trait。
- [MISSING] `implements LIRLowerable`: Java 实现 `LIRLowerable` 接口；Rust 无等效实现。
- [MISSING] `implements ArrayLengthProvider`: Java 实现 `ArrayLengthProvider`（提供 `length()` 方法）；Rust 无等效实现。
- [MISSING] public static factory method `forConstant(JavaConstant, MetaAccessProvider)`: 创建任意 JavaConstant 的 ConstantNode。
- [MISSING] public static factory method `forConstant(JavaConstant, int, boolean, MetaAccessProvider)`: 创建稳定数组的 ConstantNode。
- [MISSING] public static factory method `forPrimitive(JavaConstant, Stamp)`: 创建基本类型 ConstantNode。
- [MISSING] public static factory method `forObject(JavaConstant, ResolvedJavaType, MetaAccessProvider)`: 创建对象 ConstantNode。
- [MISSING] public static factory method `forFloatingStamp(Stamp)`: 创建浮动 stamp 的 ConstantNode。
- [MISSING] public method `getStableDimension()`: 获取稳定数组维度。**Rust 中有此方法**，OK。
- [MISSING] public method `isDefaultStable()`: 默认元素是否稳定。**Rust 中有此方法**，OK。
- [MISSING] public method `getValue()`: 获取常量值。**Rust 中有此方法**，OK。
- [MISSING] public method `isArrayLength()`: 检查是否为数组长度。
- [MISSING] public method `length()`: 来自 `ArrayLengthProvider`。
- [OK] Rust 添加了方便的工厂方法: `for_int`, `for_long`, `for_float`, `for_double`, `for_boolean`, `for_null`, `default_for_kind`, `new_stable`。这些是合理的 Rust 惯用法。
- [DESIGN] `value` 字段: Java 使用 `protected Constant value`（`Constant` 是 JVMCI 类型）；Rust 使用 `Box<dyn JavaConstant>`。不完全等价。

---

### Class: ControlSinkNode
Java 路径: `nodes/ControlSinkNode.java`
Rust 路径: `nodes/control_sink_node.rs`

**状态: OK** — Java `abstract class ControlSinkNode extends FixedNode` → Rust `trait ControlSinkNode: FixedNode {}`。匹配正确。

---

### Class: ControlSplitNode
Java 路径: `nodes/ControlSplitNode.java`
Rust 路径: `nodes/control_split_node.rs`

- [MISSING] public method `blockSuccessorCount()`: 返回块后继数量。
- [MISSING] public method `blockSuccessor(int)`: 获取指定索引的块后继。
- [MISSING] public method `setBlockSuccessor(int, AbstractBeginNode)`: 设置块后继。
- [MISSING] public method `successors()`: 返回 `NodeIterable<AbstractBeginNode>`。
- [MISSING] public method `successorBlockMap()`: 返回 `BlockMap<HIRBlock>`。
- [MISSING] public method `setSuccessorProbabilities(double[])`: 设置后继概率数组。
- [MISSING] public method `getSuccessorProbability(int)`: 获取指定索引的后继概率。
- [OK] `probability`, `getPrimarySuccessor`, `getSuccessorCount`, `successorProbabilities` 在 Rust 中已声明。

---

### Class: DeoptimizingNode
Java 路径: `nodes/DeoptimizingNode.java`
Rust 路径: `nodes/deoptimizing_node.rs`

- [OK] `DeoptimizingNode` trait 及其子 trait `DeoptBefore`, `DeoptAfter`, `DeoptDuring` 的方法签名基本匹配。
- [MISSING] `DeoptDuring.computeStateDuring(FrameState)`: Java 版本有默认实现（从 stateAfter 复制并设置 duringCall=true, rethrowException=false）；Rust 版本声明为 trait 方法但无默认实现。
- [MISSING] `DeoptDuring.canUseAsStateDuring()`: Java 中 `DeoptBefore` 有此方法，不在 `DeoptDuring` 中。Rust 将其放在 `DeoptBefore` 中，正确。
- [DESIGN] `stateBefore()` 在 Java 中返回 `FrameState`（可为 null）；Rust 返回 `Option<&FrameState>`。语义等价。

---

### Class: EndNode
Java 路径: `nodes/EndNode.java`
Rust 路径: `nodes/end_node.rs`

- [OK] 基本结构匹配。`merge` 字段在 Rust 中为 `Option<Box<dyn AbstractMergeNode>>`，在 Java 中通过 `usages().first()` 查询。
- [DESIGN] Java 中 `merge()` 方法通过 `(AbstractMergeNode) usages().first()` 动态查找；Rust 中直接存储引用。设计方式不同，但 Rust 方式更高效。

---

### Class: FixedNode
Java 路径: `nodes/FixedNode.java`
Rust 路径: `nodes/fixed_node.rs`

- [OK] Java `abstract class FixedNode extends ValueNode implements FixedNodeInterface` → Rust `trait FixedNode: ValueNode`。
- [MISSING] `asFixedNode()` Java 方法：Rust 中有等效的 `as_fixed_node()` 默认方法。OK。

---

### Class: FixedWithNextNode
Java 路径: `nodes/FixedWithNextNode.java`
Rust 路径: `nodes/fixed_with_next_node.rs`

- [OK] Java `abstract class FixedWithNextNode extends FixedNode implements FixedWithNextNodeInterface` → Rust `trait FixedWithNextNode: FixedNode`。
- [OK] `next()` 和 `setNext()` 方法已声明。

---

### Class: FrameState
Java 路径: `nodes/FrameState.java`
Rust 路径: `nodes/frame_state.rs`

- [MISSING] `extends VirtualState`: Java 继承 `VirtualState`；Rust 只实现 `ValueNode`。
- [MISSING] `implements IterableNodeType`: Java 标记接口。
- [MISSING] public method `hasExactlyOneUsage()`: 是否有恰好一个用法。
- [MISSING] public method `hasNoUsages()`: 是否无用法。
- [MISSING] public static method `create(FrameState, int, int, int, boolean)`: 创建帧状态的工厂方法。
- [MISSING] public static method `create(int, int, int, int, boolean)`: 另一个工厂方法。
- [MISSING] public method `topFrameSize()`: 顶层帧大小。
- [MISSING] public method `hashCode()` / `equals()`: 自定义相等性。
- [OK] 核心方法 `localAt`, `stackAt`, `lockAt`, `setLocalAt`, `setStackAt`, `setLockAt`, `isValidForDeoptimization`, `setValidForDeoptimization`, `outerFrameState`, `setOuterFrameState` 已实现。
- [OK] `StackState` 枚举及其 `of` 方法已正确映射。

---

### Class: GraphDecoder
Java 路径: `nodes/GraphDecoder.java`
Rust 路径: `nodes/graph_decoder.rs`

- [STUB] `decode()`: 返回 `Ok(0)`，不执行实际解码。Java 版本有约 500 行解码逻辑。
- [MISSING] 大量 Java 方法: `decodeNode`, `decodeFixedNode`, `decodeFloatingNode`, `readProperties`, `readEdges`, `makeStubNode`, `registerNode`, `getNodeClass` 等。
- [MISSING] inner class `MethodScope`: Java 的 MethodScope 有更丰富的字段（`methodData`, `method`, `encodedGraph`, `callerScope`, `loopExplosionPlugin`, `invocationPlugin`, `inlineInvokePlugin`, `parameterPlugin` 等）。
- [DESIGN] 作为简化版，保留核心概念是合理的。

---

### Class: GraphEncoder
Java 路径: `nodes/GraphEncoder.java`
Rust 路径: `nodes/graph_encoder.rs`

- [OK] 核心常量已正确映射: `NULL_ORDER_ID`, `START_NODE_ORDER_ID`, `FIRST_NODE_ORDER_ID`, `MAX_INDEX_1_BYTE`, `MAX_INDEX_2_BYTES`, `BEGIN_NEXT_ORDER_ID_OFFSET`。
- [MISSING] `encode(StructuredGraph)` 方法: Java 的核心编码方法；Rust 中无此方法。
- [MISSING] `getEncodedGraph(byte[], Object[])` 方法: 获取编码后的图数据。
- [MISSING] `verifyEncoding(StructuredGraph, byte[], Object[])` 方法: 验证编码。
- [MISSING] `prepareEncoding(StructuredGraph)` 方法: 准备编码。
- [MISSING] `finishEncoding(StructuredGraph)` 方法: 完成编码。

---

### Class: GraphState
Java 路径: `nodes/GraphState.java`
Rust 路径: `nodes/graph_state.rs`

- [OK] 枚举 `StageFlag`, `GuardsStage`, `FrameStateVerification`, `FrameStateVerificationFeature`, `MandatoryStages` 已正确映射。
- [OK] 核心方法 `isBeforeStage`, `isAfterStage`, `setAfterStage`, `getGuardsStage`, `setGuardsStage`, `getFrameStateVerification`, `weakenFrameStateVerification`, `forceDisableFrameStateVerification`, `setAfterFSA`, `requiresFutureStages` 已实现。
- [MISSING] `getMandatoryStages()` 方法: 获取强制阶段。
- [MISSING] `checkIfStageIsReachable(StageFlag)` 方法: 检查阶段是否可达。
- [MISSING] `isDuringStage(StageFlag)` 方法: 是否正在指定阶段中。
- [MISSING] `copy()` 方法: 复制图状态。
- [MISSING] `getFutureRequiredStages()` 方法: 获取未来需要的阶段。
- [MISSING] `addFutureStageRequirement(StageFlag)` 方法: 添加未来阶段需求。
- [OK] `GuardsStage` 的辅助方法 `allows_floating_guards`, `allows_guard_insertion`, `are_frame_states_at_deopts`, `are_deopts_fixed`, `reached_guards_stage` 已实现。

---

### Class: GuardNode
Java 路径: `nodes/GuardNode.java`
Rust 路径: `nodes/guard_node.rs`

- [MISSING] `extends FloatingAnchoredNode`: Java 继承 `FloatingAnchoredNode`；Rust 只实现 `ValueNode` 和 `GuardingNode`。
- [MISSING] `implements Canonicalizable`: Java 实现 `Canonicalizable`；Rust 无等效实现。
- [MISSING] `implements DeoptimizingGuard`: Java 实现 `DeoptimizingGuard`（提供 `getSpeculation()` 等方法）；Rust 无等效实现。
- [MISSING] `implements IterableNodeType`: Java 标记接口。
- [MISSING] public method `getSpeculation()`: 获取推测对象。
- [MISSING] public method `setSpeculation(Speculation)`: 设置推测对象。
- [MISSING] public method `getAnchor()`: 获取锚定节点（来自 `FloatingAnchoredNode`）。
- [MISSING] public method `setAnchor(AnchoringNode)`: 设置锚定节点。
- [MISSING] public method `setDeoptimizationReason(DeoptimizationReason, DeoptimizationAction)`: 组合设置原因和动作。
- [OK] 核心方法 `getCondition`, `setCondition`, `isNegated`, `getReason`, `getAction`, `setAction`, `setReason`, `negate`, `deoptsOnTrue` 已实现。
- [DESIGN] `condition` 字段: Java 中为 `LogicNode`；Rust 中使用 `Box<dyn ValueNode>`。类型精度降低。

---

### Class: GuardingNode
Java 路径: `nodes/extended/GuardingNode.java`（在 extended 包中）
Rust 路径: `nodes/guarding_node.rs`

- [OK] 接口 → trait 映射正确。
- [NOTE] Java 中 `GuardingNode` 在 `jdk.graal.compiler.nodes.extended` 包中，不在 `nodes` 顶级包中。Rust 将其放在 `nodes` 模块中——这是合理的重组。

---

### Class: IfNode
Java 路径: `nodes/IfNode.java`
Rust 路径: `nodes/if_node.rs`

- [MISSING] `implements Simplifiable`: Java 实现 `Simplifiable`；Rust 无等效 trait。
- [MISSING] `implements LIRLowerable`: Java 实现 `LIRLowerable`；Rust 无等效 trait。
- [MISSING] public method `simplify(SimplifierTool)`: 约 100 行简化逻辑——常量折叠、交换、消除等。
- [MISSING] public method `generate(NodeLIRBuilderTool)`: LIR 生成。
- [MISSING] public method `getNegatedCondition()`: 获取取反条件。
- [MISSING] public method `eliminate(boolean)`: 消除 if 分支。
- [MISSING] public method `swapSuccessors()`: 交换后继。
- [MISSING] public method `isLoopExit()`: 是否为循环出口。
- [MISSING] public method `setIsLoopExit(boolean)`: 设置循环出口标志。
- [MISSING] public static method `checkIfCondition(IfNode)`: 检查 if 条件。
- [OK] 核心方法 `trueSuccessor`, `falseSuccessor`, `setTrueSuccessor`, `setFalseSuccessor`, `condition`, `setCondition`, `getTrueSuccessorProbability`, `setTrueSuccessorProbability`, `getSuccessor` 已实现。
- [OK] `ControlSplitNode` trait 方法已实现。

---

### Class: InliningLog
Java 路径: `nodes/InliningLog.java`
Rust 路径: `nodes/inlining_log.rs`

- [OK] `InliningDecision`（对应 Java 的 `Decision`）, `Callsite`, `InliningLog` 结构体已正确映射。
- [MISSING] public method `getRootCallsite()`: 获取根调用点。
- [MISSING] public method `getInlinedMethods()`: 获取已内联的方法映射。
- [MISSING] public method `getLeafCallsites()`: 获取叶子调用点。**Rust 中有此方法**，OK。
- [MISSING] public method `formatAsTree(boolean)`: 格式化为树形结构。
- [MISSING] public method `logInliningTree()`: 记录内联树到调试日志。
- [MISSING] public method `getCallsiteFor(ResolvedJavaMethod)`: 获取指定方法的调用点。
- [MISSING] public method `addCallsite(Callsite)`: 添加调用点。
- [MISSING] public method `addRootCallsite(Callsite)`: 添加根调用点。
- [MISSING] public method `popInliningDecision()`: 弹出内联决策。
- [MISSING] public static method `getDecisionString(Decision)`: 获取决策字符串。
- [DESIGN] `InliningDecision` 对应 Java 的 `Decision` 内部类——名称更改需注意。

---

### Class: LoopBeginNode
Java 路径: `nodes/LoopBeginNode.java`
Rust 路径: `nodes/loop_begin_node.rs`

- [OK] 字段 `unroll_factor`, `peelings`, `unswitches`, `can_never_overflow`, `rotated`, `osr_loop`, `next_end_index`, `overflow_guard` 已正确映射。
- [OK] 枚举 `SafepointState`, `LoopType` 已正确映射。
- [OK] 核心方法已实现: `next_end_index`, `get_loop_end_count`, `can_ends_safepoint`, `can_exits_safepoint`, `is_simple_loop`, `set_pre_loop`, `is_pre_loop`, `set_main_loop`, `is_main_loop`, `set_post_loop`, `is_post_loop`, `can_overflow`, `set_can_never_overflow`, `is_rotated`, `set_rotated`, `get_unroll_factor`, `set_unroll_factor`, `peelings`, `increment_peelings`, `unswitches`, `increment_unswitches`, `is_osr_loop`, `mark_osr_loop`, `get_overflow_guard`, `set_overflow_guard`, `forward_end`。
- [MISSING] public method `loopEnds()`: 返回 `NodeIterable<LoopEndNode>`。
- [MISSING] public method `loopExits()`: 返回 `NodeIterable<LoopExitNode>`。
- [MISSING] public method `getLoopEnd(int)`: 获取指定索引的 LoopEnd。
- [MISSING] public method `removeLoopEnd(LoopEndNode)`: 移除 LoopEnd。
- [MISSING] public method `setSafepointState(SafepointState, SafepointState)`: 组合设置 safepoint 状态。
- [MISSING] public method `disableSafepoint()`: 禁用 safepoint。
- [MISSING] public method `isCounted()`: 是否为计数循环。
- [MISSING] public method `canBeCounted()`: 是否可变为计数循环。
- [OK] `AbstractMergeNode` trait 方法已实现。

---

### Class: LoopEndNode
Java 路径: `nodes/LoopEndNode.java`
Rust 路径: `nodes/loop_end_node.rs`

- [OK] 核心方法 `loop_begin`, `set_loop_begin`, `end_index`, `set_end_index`, `get_safepoint_state`, `set_safepoint_state` 已实现。
- [MISSING] `canSafepoint()` 方法: 检查 safepoint 状态。
- [OK] `AbstractEndNode` trait 方法已实现。

---

### Class: LoopExitNode
Java 路径: `nodes/LoopExitNode.java`
Rust 路径: `nodes/loop_exit_node.rs`

- [MISSING] `implements Simplifiable`: Java 实现 `Simplifiable`；Rust 无等效 trait。
- [MISSING] `simplify(SimplifierTool)` 方法: 如果循环已经消失则移除 LoopExit。
- [MISSING] `proxyPoint()` 方法: 返回代理点。
- [MISSING] `getLoopBegin()` 方法: 获取循环头。**Rust 中有 `loop_begin()` 方法**，OK。
- [OK] 核心字段 `loop_begin` 和方法已实现。

---

### Class: MergeNode
Java 路径: `nodes/MergeNode.java`
Rust 路径: `nodes/merge_node.rs`

- [OK] 枚举 `DuplicationHint` 已正确映射。
- [OK] 核心方法 `get_duplication_hint`, `set_duplication_hint`, `remove_merge_if_degenerated` 已实现。
- [MISSING] `simplify(SimplifierTool)` 方法: 与 AbstractMergeNode 的 simplify 类似。
- [OK] `AbstractMergeNode` trait 方法已实现。

---

### Class: OptimizationLog / OptimizationLogImpl
Java 路径: `nodes/OptimizationLog.java`, `nodes/OptimizationLogImpl.java`
Rust 路径: `nodes/optimization_log.rs`, `nodes/optimization_log_impl.rs`

- [OK] `OptimizationLog` trait 和 `OptimizationEntry` trait 已正确映射。
- [OK] `OptimizationEntryDummy` 和 `OptimizationLogEntry` 已实现。
- [OK] `OptimizationLogImpl` 的常量属性键已定义。
- [DESIGN] Java 的 `OptimizationLog` 有更多方法（`isEnabled()`, `report()`, `reportInlining()`, `willNotOptimize()`, `inline()`, `partialEscape()`, `clear()`, `flush()` 等）；Rust 版本简化但保留了核心概念。

---

### Class: ParameterNode
Java 路径: `nodes/ParameterNode.java`
Rust 路径: `nodes/parameter_node.rs`

- [MISSING] `extends AbstractLocalNode`: Java 继承 `AbstractLocalNode`；Rust 只实现 `ValueNode`。
- [MISSING] `implements IterableNodeType`: Java 标记接口。
- [MISSING] `implements UncheckedInterfaceProvider`: Java 接口（提供 `uncheckedStamp()` 方法）。
- [MISSING] public method `getStackKind()`: **Rust 中有 `get_stack_kind()`**，OK。
- [MISSING] public method `stamp()`: 获取 stamp。
- [OK] 核心字段 `index` 和方法 `index()`, `unchecked_kind()` 已实现。

---

### Class: PiNode
Java 路径: `nodes/PiNode.java`
Rust 路径: `nodes/pi_node.rs`

- [MISSING] `extends FloatingGuardedNode`: Java 继承 `FloatingGuardedNode`；Rust 只实现 `ValueNode`。
- [MISSING] `implements Canonicalizable`: Java 实现 `Canonicalizable`；Rust 无等效 trait。
- [MISSING] public method `canonical(CanonicalizerTool)`: 规范化逻辑。
- [MISSING] public method `piStamp()`: 获取 pi stamp。
- [MISSING] public method `strengthProtect()`: 强度保护。
- [MISSING] public method `setStrengthProtect(boolean)`: 设置强度保护。
- [MISSING] public static method `create(ValueNode, Stamp, GuardingNode)`: 创建 PiNode 的工厂方法。
- [MISSING] public static method `create(ValueNode, ValueNode, GuardingNode)`: 另一个工厂方法。
- [OK] 核心字段 `object`, `guard`, `target_kind`, `intrinsify_op` 和对应方法已实现。
- [OK] `IntrinsifyOp` 枚举已正确映射。

---

### Class: ReturnNode
Java 路径: `nodes/ReturnNode.java`
Rust 路径: `nodes/return_node.rs`

- [MISSING] `extends MemoryMapControlSinkNode`: Java 继承 `MemoryMapControlSinkNode`；Rust 实现 `ControlSinkNode`。
- [MISSING] `implements LIRLowerable`: Java 实现 `LIRLowerable`；Rust 无等效 trait。
- [MISSING] public method `generate(NodeLIRBuilderTool)`: LIR 生成。
- [MISSING] public method `getMemoryMap()`: 获取内存映射（来自 `MemoryMapControlSinkNode`）。
- [MISSING] public method `setMemoryMap(MemoryMap)`: 设置内存映射。
- [OK] 核心字段 `result` 和方法 `result()` 已实现。

---

### Class: SimplifyingGraphDecoder
Java 路径: `nodes/SimplifyingGraphDecoder.java`
Rust 路径: `nodes/simplifying_graph_decoder.rs`

- [STUB] `decode()`: 返回 `Ok(0)`，不执行实际解码+简化。Java 版本有约 200 行解码+规范化逻辑。
- [MISSING] `processNode(Node)` 方法: 处理单个节点的规范化。
- [MISSING] `canonicalizeFixedNode(FixedNode)` 方法: 规范化固定节点。
- [MISSING] `handleCanonicalization(Node, Node)` 方法: 处理规范化结果。
- [OK] 结构体继承 `GraphDecoder`（通过组合），正确。

---

### Class: StartNode
Java 路径: `nodes/StartNode.java`
Rust 路径: `nodes/start_node.rs`

- [MISSING] `implements SingleMemoryKill`: Java 实现 `SingleMemoryKill`；Rust 无等效 trait。
- [MISSING] public method `getKilledLocationIdentity()`: 获取被杀死的内存位置标识。
- [MISSING] public method `stateAfter()`: 获取 stateAfter。
- [MISSING] public method `setStateAfter(FrameState)`: 设置 stateAfter。
- [MISSING] public method `hasStateAfter()`: 是否有 stateAfter。
- [OK] 核心结构体已实现 `AbstractBeginNode` trait。

---

### Class: StructuredGraph
Java 路径: `nodes/StructuredGraph.java`
Rust 路径: `nodes/structured_graph.rs`

- [MISSING] `extends Graph`: Java 继承 `Graph`（来自 `jdk.graal.compiler.graph` 包）；Rust 是独立 struct。
- [MISSING] `implements JavaMethodContext`: Java 接口。
- [MISSING] 大量 public 方法: `getDebug()`, `getOptions()`, `isFrozen()`, `verifySourcePositions(boolean)`, `getLastSchedule()`, `logInliningTree()`, `maybeCompress()`, `reduceTrivialMerge(AbstractMergeNode)`, `add(Class<T>, T)`, `addWithoutUnique(T)`, `addOrUnique(T)`, `addOrUniqueWithInputs(T)`, `method()`, `getProfilingInfo()`, `getAssumptions()`, `recordAssumption(Assumption)`, `hasValueProxies()`, `setHasValueProxies(boolean)`, `clearAllStateAfter()`, `getInliningLog()`, `setInliningLog(InliningLog)`, `getSpeculationLog()`, `copy()`, `copy(String)`, `getNodeCount()` 等。
- [OK] 核心字段 `start`, `name`, `graph_state`, `allow_assumptions`, `node_count` 已定义。
- [OK] 基本方法 `start()`, `get_graph_state()`, `get_graph_state_mut()`, `get_node_count()`, `increment_node_count()` 已实现。
- [DESIGN] `AllowAssumptions` 枚举及其 `if_true` 方法已正确映射。

---

### Class: ValueNode
Java 路径: `nodes/ValueNode.java`
Rust 路径: `nodes/value_node.rs`

- [MISSING] `extends Node`: Java 继承 `Node`（来自 graph 包）；Rust 是独立 trait。
- [MISSING] `implements ValueNodeInterface`: Java 接口。
- [MISSING] public method `asValueNode()`: 返回自身作为 ValueNode。
- [MISSING] public method `stamp()`: 获取 stamp。
- [MISSING] public method `getStableDimension()`: 获取稳定维度。
- [MISSING] public method `isDefaultStable()`: 默认元素是否稳定。
- [MISSING] public method `getPi()`: 获取 PiNode 引用。
- [MISSING] public method `isArrayLength()`: 是否为数组长度。
- [MISSING] public method `constantOptimizations()`: 常量优化。
- [MISSING] public method `getNodeClass()`: 获取节点类。
- [MISSING] public method `isAllowedToBeCombined()`: 是否允许组合。
- [MISSING] public method `getArithmeticOp()`: 获取算术操作。
- [MISSING] public method `getNullCheck()`: 获取 null 检查。
- [MISSING] public method `getUncheckedStamp()`: 获取未检查的 stamp。
- [MISSING] public method `setUncheckedStamp(Stamp)`: 设置未检查的 stamp。
- [MISSING] public method `usages()`: 获取用法。
- [MISSING] public method `hasUsages()`: 是否有用法。
- [OK] 核心方法 `get_stack_kind`, `is_constant`, `is_null_constant`, `is_default_constant`, `infer_stamp` 已声明为 trait 方法。

---

## 总结

### T12 core: 共发现 **约 85 个问题**
- 严重问题 (STUB/空实现): 4 (GraalCompiler.compile, GraalCompiler.emit_front_end, LIRGenerationPhase.run, BeginNode.begin)
- 缺失方法/字段: 约 45 个
- 签名不匹配: 约 10 个
- 设计差异（需注意但不一定是错误）: 约 15 个
- Java 类完全未移植: 0（核心类全部有对应）
- 额外子包内容（phases）: 所有 phases 类合并到单个 phases.rs，但 phase suite 为空（无实际阶段注册）

### T13 nodes: 共发现 **约 200+ 个问题**
- 严重问题 (STUB/空实现): 3 (GraphDecoder.decode, SimplifyingGraphDecoder.decode, BeginNode.begin)
- 缺失方法/字段: 约 150+ 个
- 签名不匹配: 约 5 个
- 设计差异: 约 20 个
- Java 类完全未移植: 约 76 个（约 72% 的顶级 nodes 类尚未移植）
- 继承链不完整: 多处 Java 继承链（如 FloatingNode → FloatingAnchoredNode → GuardNode）在 Rust 中丢失

### 总体评估
- **T12 core**: 移植覆盖率约 90%（9/10 核心类有对应），但实现深度不足——多数方法的逻辑被简化或省略。`CompilationWrapper` 从有状态的抽象类退化为无状态的 trait，丢失了大量故障处理逻辑。`GraalCompiler` 的 `compile` 和 `emit_front_end` 是空壳。
- **T13 nodes**: 移植覆盖率约 28%（30/106 类有对应），核心控制流节点（BeginNode, EndNode, MergeNode, LoopBeginNode 等）已移植，但大量业务节点（Invoke, Phi, Deoptimize, Guard 系列等）缺失。已移植的类中，Java 的继承链和接口实现在 Rust 中大量简化或丢失。
- **建议优先级**:
  1. 修复所有 STUB 实现（标有空函数体的方法）
  2. 补全 `CompilationWrapper` 的完整故障处理逻辑
  3. 补全 `GraalCompiler.compile` 和 `emit_front_end` 的实际编译流程
  4. 移植缺失的 76 个 nodes 顶级类
  5. 补全已移植类的继承链和缺失方法