# jdk.vm.ci.code

## 源位置：JDK25 src.zip → `jdk.internal.vm.ci` 模块，`src/jdk.internal.vm.ci/share/classes/jdk/vm/ci/code/`（不在 `/opt/graal`；共 30 文件，另含 code.site 12、code.stack 4）

## 状态：未开始

## 说明

JVMCI 代码层。定义寄存器/栈帧/调试信息/代码缓存/已安装代码等抽象，是 RustCI 表达编译产物的核心。第一期预填接口与关键 final class 清单，Rust 文件列暂空。

## 映射表

| Java 类（全限定名） | 职责 | 对应 Rust 文件 | 状态 | 负责人 |
|---|---|---|---|---|
| jdk.vm.ci.code.CPUFeatureName | CPU 特性名标记接口 |  | 未开始 |  |
| jdk.vm.ci.code.CodeCacheProvider | 代码缓存访问提供者（安装/查询已安装代码） |  | 未开始 |  |
| jdk.vm.ci.code.CompilationRequestResult | 编译请求结果接口 |  | 未开始 |  |
| jdk.vm.ci.code.CompiledCode | 已编译代码抽象（待安装） |  | 未开始 |  |
| jdk.vm.ci.code.InspectedFrame | 可检视的栈帧接口（含本地/操作数栈信息） |  | 未开始 |  |
| jdk.vm.ci.code.InspectedFrameVisitor | 栈帧访问器（遍历栈） |  | 未开始 |  |
| jdk.vm.ci.code.RegisterConfig | 寄存器分配配置（调用约定/保留寄存器） |  | 未开始 |  |
| jdk.vm.ci.code.StackIntrospection | 栈内省接口 |  | 未开始 |  |
| jdk.vm.ci.code.ValueKindFactory | ValueKind 工厂接口 |  | 未开始 |  |
| jdk.vm.ci.code.BytecodeFrame | 字节码栈帧快照（含局部变量/操作数栈/锁） |  | 未开始 |  |
| jdk.vm.ci.code.Call | 调用站点描述 |  | 未开始 |  |
| jdk.vm.ci.code.ConstantReference | 常量引用（DataPatch 目标） |  | 未开始 |  |
| jdk.vm.ci.code.DataPatch | 数据修补项（引用待解析常量） |  | 未开始 |  |
| jdk.vm.ci.code.DataSectionReference | 数据段引用 |  | 未开始 |  |
| jdk.vm.ci.code.DebugInfo | 调试信息（栈帧+引用映射+虚拟对象） |  | 未开始 |  |
| jdk.vm.ci.code.ExceptionHandler | 代码级异常处理器表项 |  | 未开始 |  |
| jdk.vm.ci.code.ImplicitExceptionDispatch | 隐式异常分发描述 |  | 未开始 |  |
| jdk.vm.ci.code.InvalidInstalledCodeException | 已安装代码失效异常 |  | 未开始 |  |
| jdk.vm.ci.code.Location | 位置描述（寄存器/栈槽） |  | 未开始 |  |
| jdk.vm.ci.code.Mark | 代码标记（记录调用点等） |  | 未开始 |  |
| jdk.vm.ci.code.Register | 寄存器描述（编号/名/类别） |  | 未开始 |  |
| jdk.vm.ci.code.RegisterSaveLayout | 寄存器保存布局（调用点保存区） |  | 未开始 |  |
| jdk.vm.ci.code.RegisterValue | 寄存器值 |  | 未开始 |  |
| jdk.vm.ci.code.StackLockValue | 栈上锁对象描述 |  | 未开始 |  |
| jdk.vm.ci.code.StackSlot | 栈槽描述 |  | 未开始 |  |
| jdk.vm.ci.code.ValueUtil | Value 工具类（类型断言/转换） |  | 未开始 |  |
| jdk.vm.ci.code.VirtualObject | 虚拟对象描述（逃逸分析） |  | 未开始 |  |

> 上半部分为 public 接口（9），下半部分（BytecodeFrame 起）为关键 final class（17）。注意 `jdk.vm.ci.code.ExceptionHandler` 与 meta 包同名类区分（本表为代码层 bci/pc 范围）。子包 `code.site`（12）/`code.stack`（4）本表未展开，开工时按需增行。
