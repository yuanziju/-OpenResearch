# jdk.graal.compiler.debug

## 源位置：`/opt/graal/compiler/src/jdk.graal.compiler/src/jdk/graal/compiler/debug/`

## 状态：已验收

## 说明

调试/日志/度量框架：DebugContext、Scope、TimerKey、CounterKey、MemUseTrackerKey 等。26 文件完整移植，worker-verifier-fixer-reverifier 闭环通过。线程安全：使用 AtomicU64/AtomicBool/Mutex 替代 Cell。

## 映射表

| Java 类（全限定名） | 职责 | 对应 Rust 文件 | 状态 | 负责人 |
|---|---|---|---|---|
| `jdk.graal.compiler.debug.DebugContext` | 调试上下文 | `rustci/crates/rustci-compiler/src/debug/debug_context.rs` | 已验收 | — |
| `jdk.graal.compiler.debug.Scope` | 调试作用域 | `rustci/crates/rustci-compiler/src/debug/scope.rs` | 已验收 | — |
| `jdk.graal.compiler.debug.TimerKey` | 计时器键 | `rustci/crates/rustci-compiler/src/debug/timer_key.rs` | 已验收 | — |
| `jdk.graal.compiler.debug.CounterKey` | 计数器键 | `rustci/crates/rustci-compiler/src/debug/counter_key.rs` | 已验收 | — |
| `jdk.graal.compiler.debug.MemUseTrackerKey` | 内存追踪键 | `rustci/crates/rustci-compiler/src/debug/mem_use_tracker_key.rs` | 已验收 | — |
| `jdk.graal.compiler.debug.DebugCounter` | 计数器实现 | `rustci/crates/rustci-compiler/src/debug/debug_counter.rs` | 已验收 | — |
| `jdk.graal.compiler.debug.DebugTimer` | 计时器实现 | `rustci/crates/rustci-compiler/src/debug/debug_timer.rs` | 已验收 | — |
| `jdk.graal.compiler.debug.DebugMemUseTracker` | 内存追踪实现 | `rustci/crates/rustci-compiler/src/debug/debug_mem_use_tracker.rs` | 已验收 | — |
| `jdk.graal.compiler.debug.DebugHandler` | 调试处理器 | `rustci/crates/rustci-compiler/src/debug/debug_handler.rs` | 已验收 | — |
| `jdk.graal.compiler.debug.DebugOptions` | 调试选项 | `rustci/crates/rustci-compiler/src/debug/debug_options.rs` | 已验收 | — |
| `jdk.graal.compiler.debug.DebugConfig` | 调试配置 | `rustci/crates/rustci-compiler/src/debug/debug_config.rs` | 已验收 | — |
| `jdk.graal.compiler.debug.DebugFilter` | 调试过滤器 | `rustci/crates/rustci-compiler/src/debug/debug_filter.rs` | 已验收 | — |
| `jdk.graal.compiler.debug.DebugDumpHandler` | 转储处理器 | `rustci/crates/rustci-compiler/src/debug/debug_dump_handler.rs` | 已验收 | — |
| `jdk.graal.compiler.debug.DebugVerifyHandler` | 验证处理器 | `rustci/crates/rustci-compiler/src/debug/debug_verify_handler.rs` | 已验收 | — |
| `jdk.graal.compiler.debug.DebugMethodMetrics` | 方法度量 | `rustci/crates/rustci-compiler/src/debug/debug_method_metrics.rs` | 已验收 | — |
| `jdk.graal.compiler.debug.Assertions` | 断言工具 | `rustci/crates/rustci-compiler/src/debug/assertions.rs` | 已验收 | — |
| `jdk.graal.compiler.debug.GraalError` | 错误类型 | `rustci/crates/rustci-compiler/src/debug/graal_error.rs` | 已验收 | — |
| `jdk.graal.compiler.debug.GraalInternalError` | 内部错误 | `rustci/crates/rustci-compiler/src/debug/graal_internal_error.rs` | 已验收 | — |
| `jdk.graal.compiler.debug.GraalGraphError` | 图错误 | `rustci/crates/rustci-compiler/src/debug/graal_graph_error.rs` | 已验收 | — |
| `jdk.graal.compiler.debug.Indent` | 缩进工具 | `rustci/crates/rustci-compiler/src/debug/indent.rs` | 已验收 | — |
| `jdk.graal.compiler.debug.LogStream` | 日志流 | `rustci/crates/rustci-compiler/src/debug/log_stream.rs` | 已验收 | — |
| `jdk.graal.compiler.debug.Tty` | 终端输出 | `rustci/crates/rustci-compiler/src/debug/tty.rs` | 已验收 | — |
| `jdk.graal.compiler.debug.KeyRegistry` | 键注册表 | `rustci/crates/rustci-compiler/src/debug/key_registry.rs` | 已验收 | — |
| `jdk.graal.compiler.debug.CSVUtil` | CSV 工具 | `rustci/crates/rustci-compiler/src/debug/csv_util.rs` | 已验收 | — |
| `jdk.graal.compiler.debug.JavaMethodContext` | 方法上下文 | `rustci/crates/rustci-compiler/src/debug/java_method_context.rs` | 已验收 | — |
| `jdk.graal.compiler.debug.DiagnosticsOutputDirectory` | 诊断输出目录 | `rustci/crates/rustci-compiler/src/debug/diagnostics_output_directory.rs` | 已验收 | — |
| `jdk.graal.compiler.debug.DebugCloseable` | 可关闭调试资源 | `rustci/crates/rustci-compiler/src/debug/debug_closeable.rs` | 已验收 | — |

## 偏离记录

- `DebugCounter`/`DebugTimer` 使用 `AtomicU64`/`AtomicBool`/`Mutex` 替代 Java 的 `AtomicLong`/`AtomicBoolean`，确保线程安全
- `DebugContext` 的 `get_global_property` 残留 1 处 unsafe（全局属性查找），标注为 Rust 语言限制