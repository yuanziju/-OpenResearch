# jdk.graal.compiler.core

## 源位置：`/opt/graal/compiler/src/jdk.graal.compiler/src/jdk/graal/compiler/core/`

## 状态：未开始

## 说明

编译器核心入口与高层编排：`GraalCompiler`、`PhaseSuite`、LIR 生成/发射流程入口、目标描述（`TargetDescription`）聚合等。依赖 graph / nodes / lir / phases / debug。第三期范围，本期仅建框架，不预填条目；开工时由子代理列出 public 接口与 final class 全限定名后回填。

## 映射表

| Java 类（全限定名） | 职责 | 对应 Rust 文件 | 状态 | 负责人 |
|---|---|---|---|---|
