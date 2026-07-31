# jdk.vm.ci.runtime

## 源位置：JDK25 src.zip → `jdk.internal.vm.ci` 模块，`src/jdk.internal.vm.ci/share/classes/jdk/vm/ci/runtime/`（不在 `/opt/graal`；共 6 文件）

## 状态：未开始

## 说明

JVMCI 运行时入口层。提供 JVMCI 运行时获取、后端聚合、编译器接口与工厂选择，是 RustCI 骨架对宿主的顶层入口。第一期预填全部 5 个关键类型，Rust 文件列暂空。

## 映射表

| Java 类（全限定名） | 职责 | 对应 Rust 文件 | 状态 | 负责人 |
|---|---|---|---|---|
| jdk.vm.ci.runtime.JVMCI | JVMCI 入口（运行时获取/初始化） |  | 未开始 |  |
| jdk.vm.ci.runtime.JVMCIBackend | JVMCI 后端（meta/code/services 提供者聚合） |  | 未开始 |  |
| jdk.vm.ci.runtime.JVMCICompiler | JVMCI 编译器接口 |  | 未开始 |  |
| jdk.vm.ci.runtime.JVMCICompilerFactory | JVMCI 编译器工厂（选择编译器） |  | 未开始 |  |
| jdk.vm.ci.runtime.JVMCIRuntime | JVMCI 运行时（宿主访问入口） |  | 未开始 |  |

> 依赖 meta/code 子包；建议在 meta/code 骨架就位后再实现本包（顶层入口性质）。
