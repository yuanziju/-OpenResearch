# jdk.vm.ci.meta

## 源位置：JDK25 src.zip → `jdk.internal.vm.ci` 模块，`src/jdk.internal.vm.ci/share/classes/jdk/vm/ci/meta/`（不在 `/opt/graal`；共 53 文件）

## 状态：未开始

## 说明

JVMCI 元数据层。定义 Java 类型/方法/字段/常量/签名/profiling 等抽象接口与 `Unresolved*` 占位实现，是 RustCI 骨架的根接口集。第一期预填接口与关键 final class 清单，Rust 文件列暂空。

## 映射表

| Java 类（全限定名） | 职责 | 对应 Rust 文件 | 状态 | 负责人 |
|---|---|---|---|---|
| jdk.vm.ci.meta.Annotated | 可注解元素的公共基接口，提供注解访问 |  | 未开始 |  |
| jdk.vm.ci.meta.Constant | 编译期/运行时常量值的根接口 |  | 未开始 |  |
| jdk.vm.ci.meta.ConstantPool | 常量池访问接口（解析类/字段/方法/签名） |  | 未开始 |  |
| jdk.vm.ci.meta.ConstantReflectionProvider | 对常量做反射操作的提供者接口 |  | 未开始 |  |
| jdk.vm.ci.meta.InvokeTarget | 调用目标描述（方法句柄调用站点） |  | 未开始 |  |
| jdk.vm.ci.meta.JavaConstant | Java 常量值（基本类型/null/对象）的表示 |  | 未开始 |  |
| jdk.vm.ci.meta.JavaField | Java 字段抽象（含 unresolved） |  | 未开始 |  |
| jdk.vm.ci.meta.JavaMethod | Java 方法抽象（含 unresolved） |  | 未开始 |  |
| jdk.vm.ci.meta.JavaType | Java 类型抽象（含 unresolved） |  | 未开始 |  |
| jdk.vm.ci.meta.JavaValue | Java 值的根标记接口 |  | 未开始 |  |
| jdk.vm.ci.meta.MemoryAccessProvider | 内存读取提供者（读常量对象字段） |  | 未开始 |  |
| jdk.vm.ci.meta.MetaAccessProvider | 元数据访问入口（类型/方法/字段解析） |  | 未开始 |  |
| jdk.vm.ci.meta.MethodHandleAccessProvider | 方法句柄访问提供者 |  | 未开始 |  |
| jdk.vm.ci.meta.ModifiersProvider | 修饰符（public/private/static…）提供者 |  | 未开始 |  |
| jdk.vm.ci.meta.PlatformKind | 平台相关的值种类（INT/LONG/OBJECT…） |  | 未开始 |  |
| jdk.vm.ci.meta.ProfilingInfo | 方法 profiling 信息接口 |  | 未开始 |  |
| jdk.vm.ci.meta.ResolvedJavaField | 已解析字段接口 |  | 未开始 |  |
| jdk.vm.ci.meta.ResolvedJavaMethod | 已解析方法接口 |  | 未开始 |  |
| jdk.vm.ci.meta.ResolvedJavaType | 已解析类型接口 |  | 未开始 |  |
| jdk.vm.ci.meta.SerializableConstant | 可序列化为字节数组的常量 |  | 未开始 |  |
| jdk.vm.ci.meta.Signature | 方法签名接口（参数/返回类型） |  | 未开始 |  |
| jdk.vm.ci.meta.SpeculationLog | 推测编译日志（speculation 失败回退） |  | 未开始 |  |
| jdk.vm.ci.meta.VMConstant | VM 内部常量（需 VM 解析） |  | 未开始 |  |
| jdk.vm.ci.meta.AnnotationData | 注解数据描述（名称 + 元素键值） |  | 未开始 |  |
| jdk.vm.ci.meta.Assumptions | 编译期假设（无子类/唯一实现等）的容器与断言 |  | 未开始 |  |
| jdk.vm.ci.meta.DefaultProfilingInfo | ProfilingInfo 的默认实现（无数据时返回默认值） |  | 未开始 |  |
| jdk.vm.ci.meta.EnumData | 枚举数据描述 |  | 未开始 |  |
| jdk.vm.ci.meta.ErrorData | 错误数据描述 |  | 未开始 |  |
| jdk.vm.ci.meta.ExceptionHandler | 异常处理器表项（bci 范围 → handler bci） |  | 未开始 |  |
| jdk.vm.ci.meta.JavaMethodProfile | 方法接收者 profiling（类型分布） |  | 未开始 |  |
| jdk.vm.ci.meta.JavaTypeProfile | 类型 profiling（接收者/参数类型分布） |  | 未开始 |  |
| jdk.vm.ci.meta.UnresolvedJavaField | 未解析字段的占位实现 |  | 未开始 |  |
| jdk.vm.ci.meta.UnresolvedJavaMethod | 未解析方法的占位实现 |  | 未开始 |  |
| jdk.vm.ci.meta.UnresolvedJavaType | 未解析类型的占位实现 |  | 未开始 |  |

> 上半部分为 public 接口（23），下半部分（AnnotationData 起）为关键 final class（11）。开工时按依赖顺序：先 `JavaType`/`JavaMethod`/`JavaField`/`Constant` 根接口，再 `Resolved*`，再 `Unresolved*` 占位，最后 `Assumptions`/profiling。
