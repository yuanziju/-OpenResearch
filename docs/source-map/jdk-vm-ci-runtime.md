# jdk.vm.ci.runtime

## 源位置：JDK25 src.zip → `jdk.internal.vm.ci` 模块，`src/jdk.internal.vm.ci/share/classes/jdk/vm/ci/runtime/`（不在 `/opt/graal`；共 6 文件）

## 状态：已完成

## 说明

JVMCI 运行时入口层。提供 JVMCI 运行时获取、后端聚合、编译器接口与工厂选择，是 RustCI 骨架对宿主的顶层入口。全部 5 个类型已移植。

## 映射表

| Java 类（全限定名） | 职责 | 对应 Rust 文件 | 状态 | 负责人 |
|---|---|---|---|---|
| jdk.vm.ci.runtime.JVMCI | JVMCI 入口（运行时获取/初始化） | crates/rustci-vm-ci/src/runtime/jvmci.rs | 已完成 | T6 worker |
| jdk.vm.ci.runtime.JVMCIBackend | JVMCI 后端（meta/code/services 提供者聚合） | crates/rustci-vm-ci/src/runtime/jvmci_backend.rs | 已完成 | T6 worker |
| jdk.vm.ci.runtime.JVMCICompiler | JVMCI 编译器接口 | crates/rustci-vm-ci/src/runtime/jvmci_compiler.rs | 已完成 | T6 worker |
| jdk.vm.ci.runtime.JVMCICompilerFactory | JVMCI 编译器工厂（选择编译器） | crates/rustci-vm-ci/src/runtime/jvmci_compiler_factory.rs | 已完成 | T6 worker |
| jdk.vm.ci.runtime.JVMCIRuntime | JVMCI 运行时（宿主访问入口） | crates/rustci-vm-ci/src/runtime/jvmci_runtime.rs | 已完成 | T6 worker |

模块声明：`crates/rustci-vm-ci/src/runtime/mod.rs`；测试：`crates/rustci-vm-ci/src/runtime/mock_tests.rs`。

## 偏离记录

### 1. 命名：类型名保留 `JVMCI` 前缀（spec §2.2 决议）

模块名 `JVMCI → RustCI`（`rustci_vm_ci::runtime`），类型名保留 `JVMCI`/`JVMCIBackend`/`JVMCICompiler`/`JVMCICompilerFactory`/`JVMCIRuntime` 不改名，以 1:1 镜像 Java 源类型名。方法名/参数名/字段名/方法签名 1:1 保留。

### 2. `JVMCI`（class）→ `struct JVMCI` + `impl`

- **`private static volatile JVMCIRuntime runtime` → `thread_local`**：`dyn JVMCIRuntime` 不带 `Send + Sync` bound，无法存入 `static`。语义偏差：每线程首次 `get_runtime` 各自调用 `initializeRuntime` 并缓存，与 Java 进程级共享单例不等价。
- **`private static native JVMCIRuntime initializeRuntime()` → 函数指针注册**：Java native 方法经 JNI 链接；Rust 经 `JVMCI::register_initialize_runtime(fn) -> Result<(), fn>` 注册（VM 在 `JVMCI_OnLoad` 调用）。这是 native 边界机制（非 stub）：VM 提供实现，Rust 侧声明契约。
- **`getRuntime` 双重检查锁定 + `synchronized` → `thread_local` 每线程独立缓存**：单线程内无竞态，无需锁。返回 `&'static dyn JVMCIRuntime`（`Box::leak` 后缓存于 `thread_local`），对齐 Java 返回单例引用语义。每线程首次调用 leak 一次 Box（Java 单例亦永不释放）。
- **`UnsatisfiedLinkError → UnsupportedOperationException`**：Rust 检查 `INITIALIZE_RUNTIME` 是否已注册；未注册时 panic（对齐 Java `throw UnsupportedOperationException`）。Java 消息含 `java.home`/`java.vm.name`；Rust 无对应系统属性，消息以注册状态替代。
- **`private static boolean initializing`**：保留为 `thread_local`（`#[allow(dead_code)]`）。`getRuntime` 不读此字段（与 Java 一致），供未来 HotSpot 移植侧检测初始化重入。
- **`public static void initialize()`**：Java 空方法体强制触发静态初始化；Rust 无静态初始化语义，保留为 no-op 以对齐 API。

### 3. `JVMCICompiler`（interface）→ `trait JVMCICompiler`

- **`int INVOCATION_ENTRY_BCI = -1` → 模块级 `pub const INVOCATION_ENTRY_BCI: i32 = -1`**：Rust trait 关联 `const` 使 trait 非 dyn 兼容（`dyn JVMCICompiler` 不合法），而 `JVMCIRuntime::get_compiler` 返回 `Box<dyn JVMCICompiler>` 需要 dyn 兼容。模块级 const 的访问路径 `jvmci_compiler::INVOCATION_ENTRY_BCI` 对齐 Java `JVMCICompiler.INVOCATION_ENTRY_BCI` 的类型名访问语义。
- **`compileMethod(CompilationRequest)` → `compile_method(&self, request: &CompilationRequest) -> Box<dyn CompilationRequestResult>`**：Java 持对象引用，Rust 借用 `&CompilationRequest`（struct，非 trait）；返回 `Box<dyn CompilationRequestResult>` 对齐 Java 返回接口引用。
- **`default boolean isGCSupported(int)` / `default boolean isIntrinsicSupported(int)`**：保留为 trait 默认实现（`true` / `false`），1:1 对齐。

### 4. `JVMCICompilerFactory`（interface）→ `trait JVMCICompilerFactory`

- **`default void printProperties(PrintStream out)` → `fn print_properties(&self, _out: &mut dyn std::io::Write) {}`**：Java `java.io.PrintStream` → Rust `&mut dyn std::io::Write`（对齐 Rust IO 抽象）。
- **`createCompiler(JVMCIRuntime)` → `create_compiler(&self, runtime: &dyn JVMCIRuntime) -> Box<dyn JVMCICompiler>`**：Java 持接口引用，Rust 借用 `&dyn JVMCIRuntime`；返回 `Box<dyn JVMCICompiler>` 对齐 Java 返回接口引用。
- **`default void onSelection()`**：保留为 trait 默认空实现，1:1 对齐。

### 5. `JVMCIRuntime`（interface）→ `trait JVMCIRuntime`

- **`<T extends Architecture> JVMCIBackend getJVMCIBackend(Class<T> arch)` → `fn get_jvmci_backend(&self, arch: &Architecture) -> Option<&JVMCIBackend>`**：Java 泛型 `<T extends Architecture>` + `Class<T>` 无 Rust 对应；Rust 用 `&Architecture` 参数（Architecture 为 struct，非 trait，不可用 `&dyn`）。Java 返回 `JVMCIBackend`/`null`；Rust 返回 `Option<&JVMCIBackend>`（`None` 表达 null）。
- **`getHostJVMCIBackend()` / `getJVMCIBackend()` 返回 `&JVMCIBackend`（引用）而非 `Box<dyn JVMCIBackend>`**：JVMCIBackend 为 struct（非 trait），`dyn JVMCIBackend` 不合法；且 JVMCIBackend 持非 `Clone` 的 `Box<dyn ...>` 字段，无法每次构造 owned `Box`。返回引用对齐 Java 返回存储后端引用语义。
- **`getCompiler()` → `get_compiler(&self) -> Box<dyn JVMCICompiler>`**：返回 `Box<dyn JVMCICompiler>` 对齐 Java 返回接口引用。

### 6. `JVMCIBackend`（class）→ `struct JVMCIBackend` + `impl`

- 4 个 `final` 字段（`metaAccess`/`codeCache`/`constantReflection`/`stackIntrospection`）为 trait 对象 `Box<dyn ...>`（Java 持接口引用，Rust 持所有权）。
- `getTarget()` 委托 `codeCache.getTarget()`，返回 owned `TargetDescription`（对齐 `CodeCacheProvider::get_target` 签名）。
- 其余 getter 返回 `&dyn ...`（引用，对齐 Java 返回字段引用语义）。
