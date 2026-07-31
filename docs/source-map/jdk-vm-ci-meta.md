# jdk.vm.ci.meta

## 源位置：JDK25 src.zip → `jdk.internal.vm.ci` 模块，`src/jdk.internal.vm.ci/share/classes/jdk/vm/ci/meta/`（不在 `/opt/graal`；共 53 文件）

## 状态：已完成

## 说明

JVMCI 元数据层。定义 Java 类型/方法/字段/常量/签名/profiling 等抽象接口与 `Unresolved*` 占位实现，是 RustCI 骨架的根接口集。

T4 已完成核心接口与关键 final class 子集的 1:1 镜像：Java 接口 → Rust trait，final class → struct+impl，继承 → supertrait，null 语义 → `Option<T>` 或对齐 Java nullable 引用语义（详见下方偏离记录）。Rust 代码位于 `rustci/crates/rustci-vm-ci/src/meta/`，共 48 个 `.rs` 文件（含 `mod.rs`/`mock_tests.rs`），Rust LoC 7239；对应 Java 源 LoC 7178。

自验：`cargo test -p rustci-vm-ci`（4 测试全过，trait 可被 mock 实现并通过编译）；`cargo fmt --check -p rustci-vm-ci`（无差异）。

## 映射表

### A. public 接口（23）

| Java 类（全限定名） | 职责 | 对应 Rust 文件 | 状态 | 负责人 |
|---|---|---|---|---|
| jdk.vm.ci.meta.Annotated | 可注解元素的公共基接口，提供注解访问 | rustci/crates/rustci-vm-ci/src/meta/annotated.rs | 已完成 | T4 |
| jdk.vm.ci.meta.Constant | 编译期/运行时常量值的根接口 | rustci/crates/rustci-vm-ci/src/meta/constant.rs | 已完成 | T4 |
| jdk.vm.ci.meta.ConstantPool | 常量池访问接口（解析类/字段/方法/签名） | rustci/crates/rustci-vm-ci/src/meta/constant_pool.rs | 已完成 | T4 |
| jdk.vm.ci.meta.ConstantReflectionProvider | 对常量做反射操作的提供者接口 | rustci/crates/rustci-vm-ci/src/meta/constant_reflection.rs | 已完成 | T4 |
| jdk.vm.ci.meta.InvokeTarget | 调用目标描述（方法句柄调用站点） | rustci/crates/rustci-vm-ci/src/meta/invoke_target.rs | 已完成 | T4 |
| jdk.vm.ci.meta.JavaConstant | Java 常量值（基本类型/null/对象）的表示 | rustci/crates/rustci-vm-ci/src/meta/java_constant.rs | 已完成 | T4 |
| jdk.vm.ci.meta.JavaField | Java 字段抽象（含 unresolved） | rustci/crates/rustci-vm-ci/src/meta/java_field.rs | 已完成 | T4 |
| jdk.vm.ci.meta.JavaMethod | Java 方法抽象（含 unresolved） | rustci/crates/rustci-vm-ci/src/meta/java_method.rs | 已完成 | T4 |
| jdk.vm.ci.meta.JavaType | Java 类型抽象（含 unresolved） | rustci/crates/rustci-vm-ci/src/meta/java_type.rs | 已完成 | T4 |
| jdk.vm.ci.meta.JavaValue | Java 值的根标记接口 | rustci/crates/rustci-vm-ci/src/meta/java_value.rs | 已完成 | T4 |
| jdk.vm.ci.meta.MemoryAccessProvider | 内存读取提供者（读常量对象字段） | rustci/crates/rustci-vm-ci/src/meta/memory_access.rs | 已完成 | T4 |
| jdk.vm.ci.meta.MetaAccessProvider | 元数据访问入口（类型/方法/字段解析） | rustci/crates/rustci-vm-ci/src/meta/meta_access.rs | 已完成 | T4 |
| jdk.vm.ci.meta.MethodHandleAccessProvider | 方法句柄访问提供者 | rustci/crates/rustci-vm-ci/src/meta/method_handle_access.rs | 已完成 | T4 |
| jdk.vm.ci.meta.ModifiersProvider | 修饰符（public/private/static…）提供者 | rustci/crates/rustci-vm-ci/src/meta/modifiers_provider.rs | 已完成 | T4 |
| jdk.vm.ci.meta.PlatformKind | 平台相关的值种类（INT/LONG/OBJECT…） | rustci/crates/rustci-vm-ci/src/meta/platform_kind.rs | 已完成 | T4 |
| jdk.vm.ci.meta.ProfilingInfo | 方法 profiling 信息接口 | rustci/crates/rustci-vm-ci/src/meta/profiling_info.rs | 已完成 | T4 |
| jdk.vm.ci.meta.ResolvedJavaField | 已解析字段接口 | rustci/crates/rustci-vm-ci/src/meta/resolved_java_field.rs | 已完成 | T4 |
| jdk.vm.ci.meta.ResolvedJavaMethod | 已解析方法接口 | rustci/crates/rustci-vm-ci/src/meta/resolved_java_method.rs | 已完成 | T4 |
| jdk.vm.ci.meta.ResolvedJavaType | 已解析类型接口 | rustci/crates/rustci-vm-ci/src/meta/resolved_java_type.rs | 已完成 | T4 |
| jdk.vm.ci.meta.SerializableConstant | 可序列化为字节数组的常量 | rustci/crates/rustci-vm-ci/src/meta/serializable_constant.rs | 已完成 | T4 |
| jdk.vm.ci.meta.Signature | 方法签名接口（参数/返回类型） | rustci/crates/rustci-vm-ci/src/meta/signature.rs | 已完成 | T4 |
| jdk.vm.ci.meta.SpeculationLog | 推测编译日志（speculation 失败回退） | rustci/crates/rustci-vm-ci/src/meta/speculation_log.rs | 已完成 | T4 |
| jdk.vm.ci.meta.VMConstant | VM 内部常量（需 VM 解析） | rustci/crates/rustci-vm-ci/src/meta/vm_constant.rs | 已完成 | T4 |

### B. 关键 final class（11）

| Java 类（全限定名） | 职责 | 对应 Rust 文件 | 状态 | 负责人 |
|---|---|---|---|---|
| jdk.vm.ci.meta.AnnotationData | 注解数据描述（名称 + 元素键值） | rustci/crates/rustci-vm-ci/src/meta/annotation_data.rs | 已完成 | T4 |
| jdk.vm.ci.meta.Assumptions | 编译期假设（无子类/唯一实现等）的容器与断言 | rustci/crates/rustci-vm-ci/src/meta/assumptions.rs | 已完成 | T4 |
| jdk.vm.ci.meta.DefaultProfilingInfo | ProfilingInfo 的默认实现（无数据时返回默认值） | rustci/crates/rustci-vm-ci/src/meta/default_profiling_info.rs | 已完成 | T4 |
| jdk.vm.ci.meta.EnumData | 枚举数据描述 | rustci/crates/rustci-vm-ci/src/meta/enum_data.rs | 已完成 | T4 |
| jdk.vm.ci.meta.ErrorData | 错误数据描述 | rustci/crates/rustci-vm-ci/src/meta/error_data.rs | 已完成 | T4 |
| jdk.vm.ci.meta.ExceptionHandler | 异常处理器表项（bci 范围 → handler bci） | rustci/crates/rustci-vm-ci/src/meta/exception_handler.rs | 已完成 | T4 |
| jdk.vm.ci.meta.JavaMethodProfile | 方法接收者 profiling（类型分布） | rustci/crates/rustci-vm-ci/src/meta/java_method_profile.rs | 已完成 | T4 |
| jdk.vm.ci.meta.JavaTypeProfile | 类型 profiling（接收者/参数类型分布） | rustci/crates/rustci-vm-ci/src/meta/java_type_profile.rs | 已完成 | T4 |
| jdk.vm.ci.meta.UnresolvedJavaField | 未解析字段的占位实现 | rustci/crates/rustci-vm-ci/src/meta/unresolved_java_field.rs | 已完成 | T4 |
| jdk.vm.ci.meta.UnresolvedJavaMethod | 未解析方法的占位实现 | rustci/crates/rustci-vm-ci/src/meta/unresolved_java_method.rs | 已完成 | T4 |
| jdk.vm.ci.meta.UnresolvedJavaType | 未解析类型的占位实现 | rustci/crates/rustci-vm-ci/src/meta/unresolved_java_type.rs | 已完成 | T4 |

### C. 支撑类（T4 为支撑上述 34 项而同步移植的依赖类/接口）

> 这些类不在 phase-1-tasks.md §3.2 T4 的预填清单中，但作为依赖（基类、枚举、工具、常量实现）必须同步移植以保证编译与 1:1 行为对齐。

| Java 类（全限定名） | 职责 | 对应 Rust 文件 | 状态 | 负责人 |
|---|---|---|---|---|
| jdk.vm.ci.meta.AbstractJavaProfile | profiling 容器抽象基类（JavaMethodProfile/JavaTypeProfile 的基类） | rustci/crates/rustci-vm-ci/src/meta/abstract_java_profile.rs | 已完成 | T4 |
| jdk.vm.ci.meta.AbstractProfiledItem | 单条 profiling 项抽象基类 | rustci/crates/rustci-vm-ci/src/meta/abstract_java_profile.rs | 已完成 | T4 |
| jdk.vm.ci.meta.DeoptimizationAction | 去优化动作枚举 | rustci/crates/rustci-vm-ci/src/meta/deoptimization.rs | 已完成 | T4 |
| jdk.vm.ci.meta.DeoptimizationReason | 去优化原因枚举 | rustci/crates/rustci-vm-ci/src/meta/deoptimization.rs | 已完成 | T4 |
| jdk.vm.ci.meta.EncodedSpeculationReason | SpeculationLog 的编码原因实现 | rustci/crates/rustci-vm-ci/src/meta/speculation_log.rs | 已完成 | T4 |
| jdk.vm.ci.meta.JavaKind | 类型基本种类枚举（Boolean/Int/Object/Void…） | rustci/crates/rustci-vm-ci/src/meta/java_kind.rs | 已完成 | T4 |
| jdk.vm.ci.meta.LineNumberTable | 行号表（bci → 源行号） | rustci/crates/rustci-vm-ci/src/meta/line_number_table.rs | 已完成 | T4 |
| jdk.vm.ci.meta.Local | 局部变量描述（调试/profiling 用） | rustci/crates/rustci-vm-ci/src/meta/local.rs | 已完成 | T4 |
| jdk.vm.ci.meta.LocalVariableTable | 局部变量表 | rustci/crates/rustci-vm-ci/src/meta/local_variable_table.rs | 已完成 | T4 |
| jdk.vm.ci.meta.MetaUtil | meta 工具集（内部名/Java 名转换、栈轨迹格式化） | rustci/crates/rustci-vm-ci/src/meta/meta_util.rs | 已完成 | T4 |
| jdk.vm.ci.meta.NullConstant | null 常量实现（JavaConstant 子类） | rustci/crates/rustci-vm-ci/src/meta/null_constant.rs | 已完成 | T4 |
| jdk.vm.ci.meta.PrimitiveConstant | 基本类型常量实现（JavaConstant 子类） | rustci/crates/rustci-vm-ci/src/meta/primitive_constant.rs | 已完成 | T4 |
| jdk.vm.ci.meta.RawConstant | 原始常量实现（Constant 子类） | rustci/crates/rustci-vm-ci/src/meta/raw_constant.rs | 已完成 | T4 |
| jdk.vm.ci.meta.TriState | 三态枚举（True/False/Unknown） | rustci/crates/rustci-vm-ci/src/meta/tri_state.rs | 已完成 | T4 |
| （Java 反射类型占位）`java.lang.Class`/`Method`/`Field`/`Constructor`/`Executable`/`Member`/`Annotation` 等在 JVMCI meta 接口签名中出现 | 反射对象的不透明标记（Rust 无直接等价物） | rustci/crates/rustci-vm-ci/src/meta/java_reflect.rs | 已完成 | T4 |
| （无 Java 对应；T4 内部辅助） | `SerializableConstant.serialize` 所需大端字节缓冲 | rustci/crates/rustci-vm-ci/src/meta/byte_buffer.rs | 已完成 | T4 |

> 上半部分（A）为 public 接口（23），B 为关键 final class（11），C 为支撑依赖（含 Java 反射占位与 T4 内部辅助）。开工顺序：先 `JavaType`/`JavaMethod`/`JavaField`/`Constant` 根接口，再 `Resolved*`，再 `Unresolved*` 占位，最后 `Assumptions`/profiling。

## 偏离记录

### 1. null 语义

- **Java nullable 引用 → Rust `Option<T>`**：所有 Java 可空参数/返回（如 `ProfilingInfo.getTypeProfile` 返回 nullable `JavaTypeProfile`、`Assumptions.recordTo` 的 nullable `target`、`UnresolvedJavaField`/`UnresolvedJavaMethod` 的 nullable `Throwable cause`、`MetaUtil.appendProfile` 的 nullable profile、`StackTraceElement.fileName`、`Local.type` 等）统一映射为 `Option<T>`，调用方需显式判空。
- **Java nullable 引用但语义为"恒非空单例/占位"**：如 `JavaConstant.NULL_POINTER`/`INT_0` 等静态单例常量，Rust 改用关联函数（`JavaConstant::null_pointer()`/`int_0()`）返回具体 `Copy` 类型（`NullConstant`/`PrimitiveConstant`），每次构造等值新值；Java 单例引用语义在值类型下行为一致（无 identity 差异）。
- **trait 对象的"null"**：Java 接口引用可为 null，Rust `&dyn Trait`/`Box<dyn Trait>` 不能为 null；需可空处一律用 `Option<&dyn Trait>`/`Option<Box<dyn Trait>>`（如 `Annotated.getAnnotationData` 返回 `Option<AnnotationData>`，`ConstantReflectionProvider` 多个方法返回 `Option<JavaConstant>`）。

### 2. 异常映射

- **Java 受检异常 → Rust `Result`/`Option`**：JVMCI meta 接口几乎不抛受检异常（解析失败改返回 `Unresolved*` 占位），故本期无 `Result` 返回；少数 Java 抛 `NullPointerException`/`IllegalArgumentException` 的路径（如 `JavaKind.format` 入参约束）以 `debug_assert!` 对齐。
- **Java 非受检 `UnsupportedOperationException`/`IllegalArgumentException` → Rust `panic!`**：接口默认方法抛 `UnsupportedOperationException`（如 `Annotated.getAnnotationData` 默认实现、`Assumptions.hashCode` 抛 `UnsupportedOperationException`）映射为 `panic!("UnsupportedOperationException")`，对齐运行时异常语义。`JavaKind.getMinValue`/`getMaxValue`/`getBitCount` 在非法变体上抛 `IllegalArgumentException` → `panic!`。
- **`UnresolvedJavaField`/`UnresolvedJavaMethod` 的 `Throwable cause`**：Java 字段类型为 `Throwable`，Rust 用 `Option<Box<dyn std::any::Any + Send + Sync>>` 占位（T7 绑定真实 JVM 异常对象时替换为具体类型）。

### 3. 引用相等/哈希（trait 对象）

- **Java `Object.equals`/`hashCode` 默认为引用相等**：对 `ResolvedJavaType`/`ResolvedJavaMethod`/`JavaConstant` 等 trait 对象，Rust 在 `Assumptions` 各子类（`NoFinalizableSubclass`/`ConcreteSubtype`/`LeafType`/`ConcreteMethod`）的 `PartialEq`/`Hash` 中用 `std::ptr::eq`/`std::ptr::addr_of` 做指针相等与哈希，对齐 Java 引用语义。
- **Java 子类覆写 `equals`/`hashCode` 为值相等**：`CallSiteTargetValue`（按 `callSite.equals`/`methodHandle.equals`）、`AnnotationData`/`EnumData`（按字段值）手动实现 `PartialEq`/`Eq`/`Hash`。`JavaConstant` trait 增设 `constant_equals(&dyn JavaConstant) -> bool` 方法，因 Rust trait 对象无多态 `equals`，由各实现（`NullConstant`/`PrimitiveConstant`）按类型+值比对。
- **`Assumptions` 容器**：Java `HashSet<Assumption>`（引用集）→ Rust `HashSet<AssumptionKey>`，`AssumptionKey` 包装 `Box<dyn Assumption>` 并以 `assumption_eq`/`assumption_hash`（逐型 downcast）做值相等/哈希，对齐各子类 `equals`/`hashCode`。`Assumption.hashCode()` Java 抛 `UnsupportedOperationException` → Rust 不实现 `Hash` 于 trait，仅在各子类实现。

### 4. trait 对象安全与 `Self: Sized` 约束

- **默认方法需 `&Self` 转 `&dyn Trait`**：`JavaType.get_elemental_type`、`ResolvedJavaType.is_leaf`/`get_elemental_type_resolved` 等默认方法体内需将 `&Self` 强转为 `&dyn JavaType`/`&dyn ResolvedJavaType`（trait 对象），Rust 要求 `Self: Sized` 方可做此 unsized 强转。已为这些方法增设 `where Self: Sized` 约束（不影响 trait 对象安全性，因方法仍有默认实现且非必需调用路径）。
- **泛型方法 → `dyn` 参数**：`AbstractProfiledItem.profiled_hash` 等原 Java 泛型方法改用 `dyn` trait 参数以保证 trait 对象安全。

### 5. `JavaKind` 枚举结构

- **Java `JavaKind` 是带每变体构造参数的 enum**（`typeChar`/`basicType`/`javaName`/`slotCount`/`isStackInt`/`primitiveJavaClass`/`boxedJavaClass`）。Rust 侧用无字段 enum + 私有 `KindData`（经 `data()` 取回）镜像每变体常量数据，保留所有原字段与行为。
- **`Class<?>` 字段映射为 `Option<JavaClass>`**：primitive 为 `Some`，Object/Illegal 为 `None`，保留 `isPrimitive`/`fromJavaClass` 的区分行为。
- **`format(Object)` 的 `Object` 参数**：映射为 `JavaObjectValue` 判别联合，覆盖原 Java `format` 通过 `instanceof` 分派的全部值域（boxed primitive / String / JavaType / Enum / FormatWithToString / Class / array / 其它），逐分支对齐 Java 行为。

### 6. Java 反射类型

- **Java `Class<?>`/`Method`/`Field`/`Constructor`/`Executable`/`Member`/`Annotation` 等** 在 JVMCI meta 接口签名中多处出现，Rust 无直接等价物。`java_reflect.rs` 定义不透明标记类型（`JavaClass`/`JavaMethod`/`JavaField` 等）与 `JavaReflectType` trait，仅携带最小元信息（如 `JavaClass` 的类名字符串），供 `JavaKind.fromJavaClass`/`toJavaClass`/`MetaUtil.getSimpleName` 等路径使用。T7 绑定真实 JVM 反射对象时替换为具体桥接类型。

### 7. 共享指针与所有权

- **Java 共享引用 → Rust `Box<dyn>` 所有权**：`Assumptions` 各子类、`AssumptionResult`、profiling 项等持有 `ResolvedJavaType`/`ResolvedJavaMethod`/`JavaConstant` 引用，Rust 侧用 `Box<dyn>` 持有（所有权转入）。`clone_box` 在记录路径下复制等值对象（集合成以值相等去重，行为与 Java 共享引用一致）。
- **T7 待替换**：`assumptions.rs` 中 `clone_type_box`/`clone_method_box`/`clone_const_box` 占位注释标注——T7 绑定真实运行时对象时应改用 `Rc`/`Arc` 共享指针对齐 Java 引用语义，避免 `Box` 所有权转移导致的使用约束。

### 8. subtrait 方法覆写

- **`JavaConstant` 重新声明 `isDefaultForKind()`/`toValueString()`**（Java 覆写等价，`toValueString` 带默认实现）。Rust 不支持 subtrait 覆写 supertrait 默认方法签名，且双 trait 同名方法致调用歧义；故 `JavaConstant` 不重新声明此二方法，统一由 `Constant` 提供（具体类型在 `Constant` impl 中提供等价实现）。

### 9. `Display`/`toString` 近似

- **Java `Object.toString`（`type@hash`）**：`Local`/`ExceptionHandler` 等的 `toString` 中 Java 经 `%s` 调 `JavaType.toString`（无覆写即 `Object.toString`）。Rust 侧 `dyn JavaType` 无 `Display`，改用 `to_java_name()` 作可读近似（偏离：输出 Java 名而非 `type@hash`）。`ResolvedJavaMethod.format(String)`/`ResolvedJavaType.toJavaName()` 等仍按 Java 格式串逐符对齐。

### 10. `ByteBuffer`（T4 内部辅助）

- **Java `java.nio.ByteBuffer`（大端）**：`SerializableConstant.serialize` 用之。Rust 无标准等价物，`byte_buffer.rs` 实现自定义大端 `ByteBuffer`（`put_*` 写方法），对齐 Java `serialize` 行为。仅 T4 内部使用，非 Java 类映射。

### 11. `MetaUtil.getSimpleName` 降级

- **`getSimpleName(Class<?>, boolean)`** Java 侧通过 `Class` 反射取 enclosing 类链；Rust 侧 `JavaClass` 仅携带类名，无法遍历 enclosing 类，`with_enclosing_class` 行为降级为返回 simple name（无 enclosing 前缀），匿名/局部类按 Java 逻辑解析名。T7 绑定真实反射对象后恢复完整行为。

### 12. `cargo fmt` 与命名

- **Rust 命名规范**：Java 方法 `getJavaKind` → Rust `get_java_kind`（snake_case）；Java 常量 `NULL_POINTER` → Rust 关联函数 `null_pointer()`（避免全大写常量与构造语义冲突）；Java 关键字字段名（`type`/`impl`）→ Rust `r#type`/`r#impl`。
- **模块文件命名**：Java 类 `JavaType` → Rust 文件 `java_type.rs`（snake_case 文件名），类型名保留 `JavaType`（PascalCase）。

## 自验结果

- `cd /workspace/rustci && cargo test -p rustci-vm-ci`：编译通过，4 测试全过（`mock_tests::unresolved_java_type_basic`、`unresolved_java_method_with_mock_signature`、`default_profiling_info_behavior`、`tri_state_display_and_name`），证明 trait 可被 mock 实现并通过编译。
- `cd /workspace/rustci && cargo fmt --check -p rustci-vm-ci`：无差异（通过）。
- 编译告警：4 条 `unused_variables`（`annotated.rs` 默认方法参数 `type1`/`type2`/`types`/`type_`，对应 Java 默认方法抛异常不读参数），非错误，保留以对齐 Java 签名。
