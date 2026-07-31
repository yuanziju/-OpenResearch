# jdk.graal.compiler.util.json

## 源位置：`/opt/graal/compiler/src/jdk.graal.compiler/src/jdk/graal/compiler/util/json/`（oracle/graal HEAD，新布局；8 类，源 LoC 1685）

## 状态：未开始

## 说明

JSON 读写叶子模块，第一期完整移植目标。无外部 graal 依赖（仅 JDK `Writer`/`Reader` + 内部 `EconomicMap`），是理想的零依赖 Rust 移植起点。8 个类按 读写分层：`JsonWriter`/`JsonPrettyWriter`（写）/`JsonParser`+`JsonParserException`（读）/`JsonBuilder`（高层建造）/`JsonFormatter`（Map/List 工具）/`JsonPrintable`+`JsonPrinter`（回调接口）。条目已预填，Rust 文件列暂空。

## 映射表

| Java 类（全限定名） | 职责 | 对应 Rust 文件 | 状态 | 负责人 |
|---|---|---|---|---|
| jdk.graal.compiler.util.json.JsonWriter | `Writer` 包装，向字符流写 JSON token；`AutoCloseable`；JsonBuilder 的高层入口 |  | 未开始 |  |
| jdk.graal.compiler.util.json.JsonPrettyWriter | `JsonWriter` 子类，按值分隔符换行 + 缩进的 pretty-print（仅用于减少合并冲突的文件输出） |  | 未开始 |  |
| jdk.graal.compiler.util.json.JsonParser | 从字符流解析 JSON 为 `EconomicMap`/`List` 等结构 |  | 未开始 |  |
| jdk.graal.compiler.util.json.JsonParserException | `JsonParser` 解析出错时抛出（`RuntimeException`） |  | 未开始 |  |
| jdk.graal.compiler.util.json.JsonBuilder | 逐步构建结构化 JSON 的建造者（嵌套 `ObjectBuilder`/`ArrayBuilder`/`ValueBuilder`，直写底层 `Writer`，无中间对象） |  | 未开始 |  |
| jdk.graal.compiler.util.json.JsonFormatter | 围绕 `JsonWriter` 的工具封装，简化 `Map`/`List` → JSON 转换（不可实例化） |  | 未开始 |  |
| jdk.graal.compiler.util.json.JsonPrintable | 可转 JSON 表示的抽象接口（需显式调 `printJson`） |  | 未开始 |  |
| jdk.graal.compiler.util.json.JsonPrinter\<T\> | 数组元素打印回调接口（`printCollection` 使用） |  | 未开始 |  |

> 建议实现顺序：`JsonWriter` → `JsonPrettyWriter` → `JsonParser`+`JsonParserException` → `JsonBuilder` → `JsonFormatter` → `JsonPrintable`/`JsonPrinter`。
> **依赖**：`JsonParser` 返回 `org.graalvm.collections.EconomicMap`，该包位于 sdk 模块（非 compiler），移植见 [jdk-graal-compiler-util-collections.md](jdk-graal-compiler-util-collections.md)；若第一期需独立可编译，可在 Rust 侧先用 `BTreeMap`/`Vec` 暂替并在此备注。
