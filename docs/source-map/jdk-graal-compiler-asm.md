# jdk.graal.compiler.asm

## 源位置：`/opt/graal/compiler/src/jdk.graal.compiler/src/jdk/graal/compiler/asm/`

## 状态：未开始

## 说明

抽象汇编器：`Assembler`、`Buffer`、`Label`、`NumUtil` 等目标无关基类，目标特定汇编器（amd64/aarch64）在此基础上扩展。依赖少。第三期范围，本目录顶层约 8 个 .java；本期仅建框架，不预填条目；开工时由子代理列出 public 接口与 final class 全限定名后回填。

## 映射表

| Java 类（全限定名） | 职责 | 对应 Rust 文件 | 状态 | 负责人 |
|---|---|---|---|---|
