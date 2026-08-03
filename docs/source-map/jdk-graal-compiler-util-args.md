# jdk.graal.compiler.util.args

## 源位置：`/opt/graal/compiler/src/jdk.graal.compiler/src/jdk/graal/compiler/util/args/`

## 状态：已验收

## 说明

命令行参数解析工具。16 类完整移植，worker-verifier-fixer-reverifier 闭环通过。已知限制：泛型类型参数因 Rust 语言限制使用 `Box<dyn Any>` 替代；`Program` 因 Rust 无继承使用组合模式。

## 映射表

| Java 类（全限定名） | 职责 | 对应 Rust 文件 | 状态 | 负责人 |
|---|---|---|---|---|
| `jdk.graal.compiler.util.args.Program` | 程序入口，继承 Command | `rustci/crates/rustci-compiler/src/util_args/program.rs` | 已验收 | — |
| `jdk.graal.compiler.util.args.Command` | 命令定义，选项注册与解析 | `rustci/crates/rustci-compiler/src/util_args/command.rs` | 已验收 | — |
| `jdk.graal.compiler.util.args.CommandGroup` | 命令组，子命令选择 | `rustci/crates/rustci-compiler/src/util_args/command_group.rs` | 已验收 | — |
| `jdk.graal.compiler.util.args.OptionValue` | 选项值抽象基类 | `rustci/crates/rustci-compiler/src/util_args/option_value.rs` | 已验收 | — |
| `jdk.graal.compiler.util.args.BooleanValue` | 布尔选项值 | `rustci/crates/rustci-compiler/src/util_args/boolean_value.rs` | 已验收 | — |
| `jdk.graal.compiler.util.args.DoubleValue` | 双精度浮点选项值 | `rustci/crates/rustci-compiler/src/util_args/double_value.rs` | 已验收 | — |
| `jdk.graal.compiler.util.args.IntegerValue` | 整数选项值 | `rustci/crates/rustci-compiler/src/util_args/integer_value.rs` | 已验收 | — |
| `jdk.graal.compiler.util.args.StringValue` | 字符串选项值 | `rustci/crates/rustci-compiler/src/util_args/string_value.rs` | 已验收 | — |
| `jdk.graal.compiler.util.args.ListValue` | 列表选项值 | `rustci/crates/rustci-compiler/src/util_args/list_value.rs` | 已验收 | — |
| `jdk.graal.compiler.util.args.MultiChoiceValue` | 多选选项值 | `rustci/crates/rustci-compiler/src/util_args/multi_choice_value.rs` | 已验收 | — |
| `jdk.graal.compiler.util.args.Flag` | 布尔开关（继承 BooleanValue） | `rustci/crates/rustci-compiler/src/util_args/flag.rs` | 已验收 | — |
| `jdk.graal.compiler.util.args.CommandParsingException` | 解析异常 | `rustci/crates/rustci-compiler/src/util_args/command_parsing_exception.rs` | 已验收 | — |
| `jdk.graal.compiler.util.args.HelpRequestedException` | 帮助请求异常 | `rustci/crates/rustci-compiler/src/util_args/help_requested_exception.rs` | 已验收 | — |
| `jdk.graal.compiler.util.args.InvalidArgumentException` | 无效参数异常 | `rustci/crates/rustci-compiler/src/util_args/invalid_argument_exception.rs` | 已验收 | — |
| `jdk.graal.compiler.util.args.MissingArgumentException` | 缺失参数异常 | `rustci/crates/rustci-compiler/src/util_args/missing_argument_exception.rs` | 已验收 | — |
| `jdk.graal.compiler.util.args.UnknownArgumentException` | 未知参数异常 | `rustci/crates/rustci-compiler/src/util_args/unknown_argument_exception.rs` | 已验收 | — |

## 偏离记录

- 泛型类型参数（`CommandGroup<C>`, `ListValue<T>`, `MultiChoiceValue<T>`）因 Rust 无 Java 泛型继承，使用 `Box<dyn Any>` 替代
- `Program extends Command` → Rust 组合模式（`Program` 包含 `Command` 字段），通过 `command_mut()` 访问
- 返回值类型：`add_positional`/`add_named`/`add_command_group`/`add_choice`/`add_command` 返回 `Rc<RefCell<...>>` 而非具体类型引用