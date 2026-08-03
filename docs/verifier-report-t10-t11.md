# Verifier Report: T10 (util_args) + T11 (graphio)

## T10 — jdk.graal.compiler.util.args

### Class: BooleanValue
- 无重大问题。构造函数映射正确，`parseValue` 逻辑匹配。

### Class: Command
- **[SIGNATURE_MISMATCH]** `add_positional` (Rust line 89): Java 返回 `OptionValue<T>`，Rust 返回 `()`。Java 的方法签名是 `public <T> OptionValue<T> addPositional(OptionValue<T> argument)`。
- **[SIGNATURE_MISMATCH]** `add_named` (Rust line 95): Java 返回 `OptionValue<T>`，Rust 返回 `()`。Java 签名：`public <T> OptionValue<T> addNamed(String optionName, OptionValue<T> argument)`。
- **[SIGNATURE_MISMATCH]** `add_command_group` (Rust line 101): Java 返回 `CommandGroup<C>`，Rust 返回 `()`。Java 签名：`public <C extends Command> CommandGroup<C> addCommandGroup(CommandGroup<C> group)`。
- **[SIGNATURE_MISMATCH]** `collect_options` (Rust line 296): Java 存储 `List<OptionValue<?>>` 和 `List<Pair<String, OptionValue<?>>>`（即实际 OptionValue 引用），Rust 存储 `Vec<String>` 和 `Vec<Pair<String, String>>`（仅存储名称字符串）。这导致 Rust 无法对各个选项调用 `print_help` 或 `print_usage`。
- **[SIGNATURE_MISMATCH]** `parse` (Rust line 132): Java 返回 `int` 并显式声明抛出 `CommandParsingException, HelpRequestedException`，Rust 返回 `Result<usize, Box<dyn Error>>`。Rust 版本中 CommandParsingException 的解包存在差异。
- **[SIGNATURE_MISMATCH]** `print_usage` (Rust line 288): Java 接受 `PrintWriter`，Rust 接受 `&mut dyn fmt::Write`。功能等价，但签名不同。

### Class: CommandGroup
- **[SIGNATURE_MISMATCH]** 缺少类型参数：Java 有 `CommandGroup<C extends Command>`，Rust 的 `CommandGroup` 没有类型参数，使用 `Box<dyn AnyOptionValue>` 和 `Box<Command>` 进行类型擦除。
- **[SIGNATURE_MISMATCH]** `add_command` (Rust line 106): Java 返回 `C`，Rust 返回 `()`。Java 签名：`public C addCommand(C command)`。
- **[SIGNATURE_MISMATCH]** `get_selected_command` (Rust line 116): Java 返回 `C`（即 `Command`），Rust 返回 `Option<std::cell::Ref<'_, Box<Command>>>`。Rust 版本存在根本性错误——它创建了一个无法到达的 `Ref::map`，导致永远无法返回有效的命令引用。
- **[SIGNATURE_MISMATCH]** `print_usage` (Rust line 158): Java 覆盖了 `OptionValue.printUsage(PrintWriter, boolean)`，Rust 实现了 `AnyOptionValue::print_usage`。功能等价，但 Java 版本在值已设置时委托给 `value.printUsage(writer)`，Rust 版本也委托给内部命令。
- **[SIGNATURE_MISMATCH]** `print_help` (Rust line 170): Java 覆盖了 `OptionValue.printHelp`，Rust 实现了 `AnyOptionValue::print_help`。功能等价。

### Class: CommandParsingException
- **[SIGNATURE_MISMATCH]** 字段 `command` (Rust line 36): Java 存储 `Command command` 对象，Rust 存储 `command_name: String`（仅名称）。Java 有 `private final Command command`。
- **[SIGNATURE_MISMATCH]** `get_command_name` (Rust line 47): Java 有 `public Command getCommand()` 返回 `Command` 对象，Rust 有 `pub fn get_command_name(&self) -> &str` 仅返回名称字符串。
- **[SIGNATURE_MISMATCH]** 构造函数 (Rust line 40): Java 构造函数接受 `Exception cause` 并调用 `super(cause)`，使 `getCause()` 可用。Rust 仅存储格式化的消息字符串，丢失了因果链。

### Class: DoubleValue
- 无重大问题。构造函数和 `parseValue` 逻辑正确映射。

### Class: Flag
- **[SIGNATURE_MISMATCH]** `clear()` (Rust line 80): Java 设置 `value = false`（即 `Boolean.FALSE`，继承自 `OptionValue<Boolean>`，`value` 字段类型为 `Boolean`）。Rust 设置 `self.value = Some(false)`。从语义上讲，两者在调用 `clear()` 后 `is_set()` 都返回 `true`，因此功能等价，但 Java 的 `OptionValue.clear()` 默认设置 `value = null`，而 `Flag` 覆盖了它。Rust 的 `clear()` 直接设置 `Some(false)` 是合理的。

### Class: HelpRequestedException
- **[SIGNATURE_MISMATCH]** 字段 `command` (Rust line 37): Java 存储 `private final Command command`，Rust 存储 `command_name: String` 和一个未使用的 `command: *const ()` 指针。
- **[SIGNATURE_MISMATCH]** `get_command_name` (Rust line 49): Java 有 `public Command getCommand()`，Rust 有 `pub fn get_command_name(&self) -> &str`。与 CommandParsingException 相同的问题。

### Class: IntegerValue
- **[SIGNATURE_MISMATCH]** 错误消息 (Rust line 87): 当解析失败时，错误消息显示 "invalid double value"，而不是 "invalid integer value"。Java 源代码 (IntegerValue.java) 存在相同的 bug（它复制了 DoubleValue 的错误消息）。因此 Rust 的"bug"实际上与 Java 完全匹配。
- **[SIGNATURE_MISMATCH]** `getValue()` (Rust line 67): Java 返回 `Integer`（可为 null），Rust 返回 `Option<i32>`。功能等价。

### Class: InvalidArgumentException
- 无重大问题。构造函数 `InvalidArgumentException(String name, String reason)` 正确映射。

### Class: ListValue
- **[SIGNATURE_MISMATCH]** 缺少类型参数：Java 有 `ListValue<T> extends OptionValue<List<T>>`，Rust 没有类型参数，使用 `Vec<Box<dyn Any>>`。
- **[SIGNATURE_MISMATCH]** `parse_value` (Rust line 99-104): Java 在成功解析时将 `inner.value` 添加到列表中：`value.add(inner.value)`。Rust 将内部选项的**名称**推入列表：`v.push(Box::new(self.inner.get_name().to_string()))`。这是**严重的逻辑错误**——应该存储实际解析的值，而不是名称字符串。
- **[SIGNATURE_MISMATCH]** `get_values` (Rust line 78): Java 从 `OptionValue<List<T>>` 继承 `getValue()` 返回 `List<T>`。Rust 返回 `Option<&Vec<Box<dyn Any>>>`。对于未设置的情况，语义不同（Java 返回 defaultValue，Rust 返回 None）。

### Class: MissingArgumentException
- 无重大问题。构造函数 `MissingArgumentException(String argumentName)` 正确映射。

### Class: MultiChoiceValue
- **[SIGNATURE_MISMATCH]** 缺少类型参数：Java 有 `MultiChoiceValue<T> extends OptionValue<T>`，Rust 没有类型参数，使用 `Box<dyn Any>`。
- **[SIGNATURE_MISMATCH]** `add_choice` (Rust line 82): Java 返回 `MultiChoiceValue<T>`（返回 `this` 以支持链式调用），Rust 返回 `()`。Java 签名：`public MultiChoiceValue<T> addChoice(String name, T choiceValue, String help)`。
- **[STUB]** `add_choice` (Rust line 85-87): Java 在 `choiceValue.equals(defaultValue)` 时设置 `defaultChoice = name`。Rust 代码有一个注释 `// no-op: first choice becomes default`，但实际上并未实现默认选择追踪。
- **[SIGNATURE_MISMATCH]** `parse_value` (Rust line 109): Java 设置 `value = choices.get(arg)`（存储实际的 T 值），Rust 设置 `self.value = Some(Box::new(arg.to_string()))`（存储**名称字符串**而不是实际选择值）。这是**严重的逻辑错误**。

### Class: OptionValue (作为 trait AnyOptionValue)
- **[MISSING]** `repeated()` 方法 (Java OptionValue.java): Java 有 `public OptionValue<List<T>> repeated()` 将单个选项值转换为列表值。Rust 的 `AnyOptionValue` trait 或任何具体类型均未实现此方法。
- **[SIGNATURE_MISMATCH]** `get_usage` (Rust option_value.rs line 84): Java 的 `getUsage(boolean)` 是 `public final`，使用 `StringWriter` 和 `PrintWriter`。Rust 版本使用简单的 `String` 缓冲区。功能等价。
- **[SIGNATURE_MISMATCH]** `print_indented` (Rust option_value.rs line 43): Java 有 `private static void printIndentedLine` 和 `static void printIndented`。Rust 将两者合并为一个公共函数，这是可以接受的，但失去了可见性差异。

### Class: Program
- **[STRUCTURAL]** Java 的 `Program extends Command`，Rust 的 `Program` 通过组合方式包装 `Command`（字段 `command: Command`）。这意味着 Rust 的 `Program` 无法直接作为 `Command` 使用——调用者必须通过 `command_mut()` 访问。
- **[EXTRA]** `command_mut()` (Rust line 54): Java 中没有此方法。Java 的 `Program` 继承了所有 `Command` 方法，无需额外访问器。这是组合模式的必要变通方法。
- **[SIGNATURE_MISMATCH]** `print_help` (Rust line 97): Java 的 `printHelp(PrintWriter)` 对每个选项调用 `arg.printHelp(writer, 2)` 和 `arg.getUsage(false)`。Rust 版本仅打印名称（`print_indented(writer, arg, 1)`）——它**不打印任何帮助文本或使用详情**。这是因为 `collect_options` 仅存储名称字符串，而不是 `OptionValue` 引用。
- **[SIGNATURE_MISMATCH]** `parse_and_validate` (Rust line 60): Java 在 `catch (InvalidArgumentException | MissingArgumentException e)` 中捕获这些异常并将其包装为 `new CommandParsingException(e, this)`。Rust 版本直接使用 `UnknownArgumentException` 创建 `CommandParsingException`，跳过了正确的包装。在 `parse` 方法中，`InvalidArgumentException` 和 `MissingArgumentException` 被作为 `Err` 返回，但此处的 `parse_and_validate` 不处理那些情况。

### Class: StringValue
- 无重大问题。正确映射。

### Class: UnknownArgumentException
- 无重大问题。构造函数正确映射。

---

## T11 — jdk.graal.compiler.graphio

### 整体架构问题
- **[MISSING]** 整个 `parsing/model/` 子目录（25 个 Java 文件）在 Rust 中**完全缺失**。没有 Rust 对应文件。关键类包括：`AbstractMutableDocumentItem`, `ChangedEvent`, `ChangedEventProvider`, `ChangedListener`, `DataCollectionEvent`, `DataCollectionListener`, `DumpedElement`, `Event`, `Folder`, `FolderElement`, `GraphClassifier`, `GraphContainer`, `GraphDocument`, `GraphDocumentVisitor`, `Group`, `InputBlock`, `InputBlockEdge`, `InputBytecode`, `InputEdge`, `InputGraph`, `InputMethod`, `InputNode`, `KnownPropertyNames`, `KnownPropertyValues`, `Properties`, `Property`。
- **[MISSING]** 以下 parsing Java 文件缺少 Rust 对应文件：`DataBinaryPrinter.java`, `DataBinaryWriter.java`, `DocumentFactory.java`, `LocationCache.java`, `LocationStackFrame.java`, `LocationStratum.java`, `ModelBuilder.java`, `TemplateParser.java`。
- **[MISSING]** `GraphJavadocSnippets.java` 没有 Rust 对应文件（这是 javadoc 示例代码，可以接受省略）。

### Class: GraphOutput
- **[SIGNATURE_MISMATCH]** 类型参数：Java 有 `<G, M>`（2 个类型参数），Rust 有 `<G, N, C, P, B, M, F, S, SP, L>`（10 个类型参数）。Java 的 `GraphOutput` 是一个薄包装器，将大部分类型参数委托给 `GraphProtocol`。Rust 暴露了所有内部类型参数。
- **[SIGNATURE_MISMATCH]** `begin_group` (Rust line 56): Java 接受 `M method`（非可选），Rust 接受 `method: Option<&M>`。Java 总是传递一个方法（可能为 null），Rust 使用 `Option`。
- **[SIGNATURE_MISMATCH]** `write` (Rust line 92): Java 接受 `ByteBuffer src` 并返回写入的字节数 `int`，Rust 接受 `&[u8]` 并返回 `io::Result<()>`（不返回字节数）。
- **[MISSING]** `isOpen()` (Java GraphOutput.java): Java 实现 `WritableByteChannel` 并覆盖 `isOpen()`。Rust 没有此方法。
- **[SIGNATURE_MISMATCH]** `close()` (Rust line 87): Java 的 `close()` 返回 `void`（实现 `Closeable`），Rust 返回 `io::Result<()>`。
- **[SIGNATURE_MISMATCH]** `new_builder` (Rust line 49): Java 的 `newBuilder` 是 `public static <G, N, C, P> Builder<G, N, ?> newBuilder(GraphStructure<G, N, C, P> structure)`，Rust 的 `new_builder` 有额外的类型参数 `<C2, P2>`，且接受 `Box<dyn GraphStructure<G, N, C2, P2>>`。
- **[MISSING]** `ATTR_VM_ID` 常量：Java 有 `public static final String ATTR_VM_ID = "vm.uuid"`。Rust 在 `graph_protocol.rs` 中有此常量，但 `GraphOutput` 结构体本身没有暴露它。

### Class: Builder (GraphOutput.Builder)
- **[SIGNATURE_MISMATCH]** 类型参数：Java 有 `<G, N, M>`，Rust 有 `<G, N, M, C, P>`。
- **[STUB]** `blocks` 方法 (Rust line 191): 完全忽略其参数 `_graph_blocks`。块未存储在构建器中。Java 将 `graphBlocks` 存储在 `this.blocks` 字段中。
- **[STUB]** `elements` 方法 (Rust line 199): 完全忽略其参数 `_graph_elements`。元素未存储。Java 创建一个 `ElementsAndLocations` 包装器并存储它。
- **[STUB]** `elements_and_locations` 方法 (Rust line 219): 完全忽略两个参数。Java 存储两者。
- **[MISSING]** `build(GraphOutput<?, ?> parent)` 方法 (Java GraphOutput.java): Java 的 Builder 有两个 `build` 重载：一个接受 `WritableByteChannel`，另一个接受父 `GraphOutput` 以创建共享通道和常量池的嵌套输出。Rust 仅有 `build(writer)` 变体。
- **[MISSING]** `ElementsAndLocations` 内部类 (Java GraphOutput.java): Java 有一个私有的 `ElementsAndLocations<M, P, L>` 内部类，用于捆绑元素和位置。Rust 没有此结构。
- **[MISSING]** `StackLocations` 内部类 (Java GraphOutput.java): Java 有一个私有的 `StackLocations<M, P>` 内部类，实现 `GraphLocations`，委托给 `GraphElements.methodStackTraceElement`。Rust 没有此结构。
- **[MISSING]** `DEFAULT_MAJOR_VERSION` 和 `DEFAULT_MINOR_VERSION` 作为 Builder 常量 (Java GraphOutput.java): Java 在 Builder 中定义了 `private static final int DEFAULT_MAJOR_VERSION = 8` 和 `DEFAULT_MINOR_VERSION = 0`。Rust 将这些常量放在 `graph_protocol.rs` 中。

### Class: GraphProtocol
- **[MISSING]** 整个抽象类 `GraphProtocol<Graph, Node, NodeClass, Edges, Block, ResolvedJavaMethod, ResolvedJavaField, Signature, NodeSourcePosition, Location>` 在 Rust 中**完全缺失**。Rust 的 `graph_protocol.rs` 仅包含协议常量（操作码、池类型等）。
- **[MISSING]** 以下 `protected abstract` 方法缺失（约 50 个抽象方法）：`findGraph`, `findMethod`, `findNode`, `findNodeClass`, `findClassForNode`, `findJavaClass`, `findEnumClass`, `findNameTemplate`, `findClassEdges`, `findNodeId`, `hasPredecessor`, `findNodesCount`, `findNodes`, `findNodeProperties`, `findBlockNodes`, `findBlockId`, `findBlocks`, `findBlockSuccessors`, `formatTitle`, `findSize`, `isDirect`, `findName`, `findType`, `findNodes`, `findEnumOrdinal`, `findEnumTypeValues`, `findJavaTypeName`, `findMethodCode`, `findMethodModifiers`, `findMethodSignature`, `findMethodName`, `findMethodDeclaringClass`, `findFieldModifiers`, `findFieldTypeName`, `findFieldName`, `findFieldDeclaringClass`, `findJavaField`, `findSignature`, `findSignatureParameterCount`, `findSignatureParameterTypeName`, `findSignatureReturnTypeName`, `findNodeSourcePosition`, `findNodeSourcePositionMethod`, `findNodeSourcePositionCaller`, `findNodeSourcePositionBCI`, `findLocation`, `findLocationFile`, `findLocationLine`, `findLocationURI`, `findLocationLanguage`, `findLocationStart`, `findLocationEnd`。
- **[MISSING]** 所有私有实现方法缺失：`writeVersion`, `flushEmbedded`, `flush`, `ensureAvailable`, `writeByte`, `writeInt`, `writeLong`, `writeDouble`, `writeFloat`, `writeShort`, `writeString`, `writeBytes`, `writeBytesRaw`, `writeInts`, `writeDoubles`, `writePoolObject`, `findPoolType`, `writeGraph`, `writeNodes`, `writeEdges`, `classForNode`, `writeNodeRef`, `writeBlocks`, `writeEdgesInfo`, `addPoolEntry`, `writePropertyObject`, `writeProperties`, `isFound`, `reportBadToString`, `checkToString`。
- **[MISSING]** `ConstantPool` 内部类 (Java GraphProtocol.java): Java 有一个带有 `WeakHashMap` 的复杂 `ConstantPool` 内部类，用于有限常量池。Rust 的 `protocol_impl.rs` 中有一个简化版本，但缺少 `WeakHashMap` 语义和 POOL_STRING 转发逻辑。

### Class: ProtocolImpl
- **[STUB]** `write_pool_object` (Rust protocol_impl.rs line 329): 仅处理 `POOL_STRING` 和 `POOL_NULL`。Java 处理所有池类型：FIELD、SIGNATURE、NODE_SOURCE_POSITION、NODE_CLASS、NODE、METHOD、ENUM、CLASS、STRING。Rust 版本丢失了所有其他池类型的编码逻辑。
- **[STUB]** `write_property_object` (Rust line 469): 仅处理 `i32`, `i64`, `f64`, `f32`, `bool`, `Vec<f64>`, `Vec<i32>`。Java 还处理 `Integer`, `Long`, `Double`, `Float`, `Boolean`, 原始数组（`double[]`, `int[]`）、对象数组，以及**子图**（`PROPERTY_SUBGRAPH`）。Rust 的 `else` 分支将所有未知类型作为 `PROPERTY_POOL` 转储，这是不正确的。
- **[STUB]** `write_properties_doc` (Rust line 504): 仅写入键（两次），**不写入值**。Java 调用 `writePropertyObject(graph, entry.getValue())` 写入每个属性值。Rust 的版本仅写入键两次（`write_pool_object(key)` 后跟 `write_byte(PROPERTY_POOL)` 和另一个 `write_pool_object(key)` - 值完全丢失）。
- **[MISSING]** `write_edges_info` (Java GraphProtocol.java): 写入节点类边信息的私有无方法。Rust 中完全缺失。
- **[MISSING]** `add_pool_entry` (Java GraphProtocol.java): 写入各种类型池条目（FIELD、SIGNATURE 等）的私有方法。Rust 中完全缺失。
- **[MISSING]** `find_pool_type` (Java GraphProtocol.java): 确定对象池类型的私有方法。Rust 中完全缺失。
- **[SIGNATURE_MISMATCH]** `write_bytes` (Rust line 297): Java 使用 `writeInt(-1)` 处理 null 字节数组，Rust 不处理 null 情况。
- **[SIGNATURE_MISMATCH]** `write_ints`/`write_doubles` (Rust line 313-327): Java 直接写入缓冲区（`buffer.asIntBuffer().put(b)`），Rust 逐个元素写入。功能等价但效率较低。
- **[MISSING]** `reportBadToString` 和 `checkToString` 辅助方法 (Java GraphProtocol.java): 用于检测 `toString()` 不一致的静态方法。Rust 中缺失。
- **[SIGNATURE_MISMATCH]** `ConstantPool` (Rust line 22-58): Java 使用 `WeakHashMap<Object, Object>` 实现弱引用语义，Rust 使用 `HashMap<Vec<u8>, u16>`。Rust 版本不支持 POOL_STRING 转发（其中对象映射到其 toString 表示，然后映射到池 ID）。
- **[SIGNATURE_MISMATCH]** `format_title` (Rust line 538): Java 使用 `String.format(format, args)`，Rust 使用手动字符串替换，仅处理 `%s` 和 `%d` 模式。Java 支持完整的 `String.format` 语法。

### Class: DefaultGraphBlocks
- **[SIGNATURE_MISMATCH]** 单例模式 (Rust line 18): Java 使用 `private static final DefaultGraphBlocks DEFAULT` 并有私有构造函数。Rust 有 `pub fn new()` 和 `pub fn empty()`，两者都返回新实例。Java 的 `empty()` 返回 `GraphBlocks<G, B, N>`（类型转换），Rust 的 `empty()` 返回 `Self`。
- **[EXTRA]** `new()` 方法 (Rust line 18): Java 有私有构造函数，Rust 有公共 `new()`。Java 的 `DefaultGraphBlocks` 是单例，禁止外部实例化。
- **[EXTRA]** `default_instance()` 不在 Rust 中，但 Java 有 `static final GraphTypes DEFAULT`。Rust 的 `DefaultGraphTypes` 有 `default_instance()`，但 `DefaultGraphBlocks` 没有。

### Class: DefaultGraphTypes
- **[EXTRA]** `new()` (Rust line 20): Java 有私有构造函数，Rust 有公共 `new()`。
- **[EXTRA]** `default_instance()` (Rust line 24): Java 有 `static final GraphTypes DEFAULT`，但 Rust 使用 `default_instance()` 方法。
- **[SIGNATURE_MISMATCH]** `enum_class` (Rust line 36): Java 返回 `Class<?>`（可为 null），Rust 返回 `Option<Box<dyn Any>>`。
- **[SIGNATURE_MISMATCH]** `type_name` (Rust line 48): Java 返回 `String`（可为 null），Rust 返回 `Option<String>`。

### Class: GraphBlocks (trait)
- **[SIGNATURE_MISMATCH]** `blocks` (Rust line 14): Java 返回 `Collection<? extends B>`，Rust 返回 `Vec<B>`。Java 允许迭代器语义，Rust 强制具体集合类型。
- **[SIGNATURE_MISMATCH]** `block_nodes` (Rust line 20): Java 返回 `Collection<? extends N>`，Rust 返回 `Vec<N>`。
- **[SIGNATURE_MISMATCH]** `block_successors` (Rust line 23): Java 返回 `Collection<? extends B>`，Rust 返回 `Vec<B>`。

### Class: GraphElements (trait)
- **[SIGNATURE_MISMATCH]** `method_code` (Rust line 17): Java 返回 `byte[]`（可为 null），Rust 返回 `Vec<u8>`（不可为 null）。
- **[SIGNATURE_MISMATCH]** `method_declaring_class` (Rust line 29): Java 返回 `Object`，Rust 返回 `Box<dyn Any>`。
- **[SIGNATURE_MISMATCH]** `field_declaring_class` (Rust line 44): Java 返回 `Object`，Rust 返回 `Box<dyn Any>`。
- **[SIGNATURE_MISMATCH]** `method_stack_trace_element` (Rust line 71): Java 返回 `java.lang.StackTraceElement`，Rust 返回自定义的 `StackTraceElement` 结构体。Rust 的 `StackTraceElement` 有字段 `class_name`, `method_name`, `file_name`, `line_number`，与 Java 的 `StackTraceElement` 匹配，但它们是不同的类型。

### Class: GraphLocations (trait)
- **[SIGNATURE_MISMATCH]** `method_location` (Rust line 17): Java 返回 `Iterable<L>`，Rust 返回 `Vec<L>`。
- **[SIGNATURE_MISMATCH]** `location_uri` (Rust line 23): Java 返回 `URI` 并声明抛出 `URISyntaxException`，Rust 返回 `Option<String>`。Java 方法可以抛出受检异常，Rust 不处理此情况。
- **[SIGNATURE_MISMATCH]** `location_language` (Rust line 20): Java 返回 `String`，Rust 返回 `Option<String>`。

### Class: GraphStructure (trait)
- **[SIGNATURE_MISMATCH]** `nodes` (Rust line 25): Java 返回 `Iterable<? extends N>`，Rust 返回 `Vec<N>`。
- **[SIGNATURE_MISMATCH]** `edge_nodes` (Rust line 78): Java 返回 `Collection<? extends N>`（可为 null），Rust 返回 `Option<Vec<N>>`。
- **[SIGNATURE_MISMATCH]** `node_class_type` (Rust line 57): Java 返回 `Object`，Rust 返回 `Box<dyn Any>`。
- **[SIGNATURE_MISMATCH]** `edge_type` (Rust line 75): Java 返回 `Object`，Rust 返回 `Box<dyn Any>`。

### Class: GraphTypes (trait)
- 无重大问题。Java 接口方法正确映射到 Rust trait 方法。

### Class: BinaryReader (parsing)
- Java 文件存在，Rust 文件存在。未进行详细逐方法比较（parsing 文件范围过大）。

### Class: BinarySource (parsing)
- Java 文件存在，Rust 文件存在。未进行详细比较。

### Parsing 缺失文件
- **[MISSING]** `DataBinaryPrinter.java` → 无 Rust 文件
- **[MISSING]** `DataBinaryWriter.java` → 无 Rust 文件
- **[MISSING]** `DocumentFactory.java` → 无 Rust 文件
- **[MISSING]** `LocationCache.java` → 无 Rust 文件
- **[MISSING]** `LocationStackFrame.java` → 无 Rust 文件
- **[MISSING]** `LocationStratum.java` → 无 Rust 文件
- **[MISSING]** `ModelBuilder.java` → 无 Rust 文件（但 `builder.rs` 存在，应进行比较）
- **[MISSING]** `TemplateParser.java` → 无 Rust 文件

### Parsing model/ 子目录
- **[MISSING]** 整个 `graphio/parsing/model/` 包（25 个 Java 文件）在 Rust 中**完全缺失**。没有对应的 Rust 模块或文件。所有以下类都缺失：`AbstractMutableDocumentItem`, `ChangedEvent`, `ChangedEventProvider`, `ChangedListener`, `DataCollectionEvent`, `DataCollectionListener`, `DumpedElement`, `Event`, `Folder`, `FolderElement`, `GraphClassifier`, `GraphContainer`, `GraphDocument`, `GraphDocumentVisitor`, `Group`, `InputBlock`, `InputBlockEdge`, `InputBytecode`, `InputEdge`, `InputGraph`, `InputMethod`, `InputNode`, `KnownPropertyNames`, `KnownPropertyValues`, `Properties`, `Property`。

---

## 总结

### T10 (util_args): 约 40 个问题
- **6 个 SIGNATURE_MISMATCH**：返回类型不匹配（`add_positional`, `add_named`, `add_command_group`, `add_choice`, `get_selected_command`, `getCommand`/`get_command_name`）
- **4 个 SIGNATURE_MISMATCH**：类型参数缺失（`CommandGroup`, `ListValue`, `MultiChoiceValue`）
- **3 个 SIGNATURE_MISMATCH**：存储值错误（`ListValue.parse_value` 存储名称而非值，`MultiChoiceValue.parse_value` 存储名称而非值，`collect_options` 存储名称而非 OptionValue 引用）
- **2 个 SIGNATURE_MISMATCH**：异常存储（`CommandParsingException`, `HelpRequestedException` 存储命令名称字符串而非 Command 对象）
- **1 个 MISSING**：`OptionValue.repeated()` 方法
- **1 个 STUB**：`MultiChoiceValue.add_choice` 默认选择追踪
- **1 个 EXTRA**：`Program.command_mut()` 访问器
- **1 个 STRUCTURAL**：Program 使用组合而非继承

### T11 (graphio): 约 60+ 个问题（不含 parsing 详细分析）
- **1 个 MISSING**：整个 `GraphProtocol` 抽象类（~50 个抽象方法，约 500 行私有实现）
- **3 个 MISSING**：内部类（`ElementsAndLocations`, `StackLocations`, `ConstantPool`）
- **4 个 STUB**：构建器方法（`blocks`, `elements`, `elements_and_locations` 忽略其参数）
- **1 个 STUB**：`write_pool_object` 仅处理字符串（缺失 8 种池类型）
- **1 个 STUB**：`write_property_object` 缺少多种类型处理（Integer, Long, 数组, 子图）
- **1 个 STUB**：`write_properties_doc` 不写入属性值
- **8 个 MISSING**：parsing 文件（`DataBinaryPrinter`, `DataBinaryWriter`, `DocumentFactory`, `LocationCache`, `LocationStackFrame`, `LocationStratum`, `ModelBuilder` 对应文件, `TemplateParser`）
- **25 个 MISSING**：model/ 子目录文件
- **多个 SIGNATURE_MISMATCH**：trait 方法返回 `Vec` 而非 `Collection`/`Iterable`，`Box<dyn Any>` 而非 `Object`，`Option<String>` 而非 `String`，`Option<&M>` 而非 `M`
- **1 个 MISSING**：`GraphOutput.Builder.build(parent)` 重载
- **1 个 MISSING**：`GraphOutput.isOpen()` 方法
- **多个 MISSING**：`GraphProtocol` 私有方法（`addPoolEntry`, `findPoolType`, `writeEdgesInfo` 等）