# 《Rust程序设计》课程实践报告

小组成员：安锐博，斯文

## 项目：`Partrick's Parabox`简易版

**GitHub仓库地址：[Patrick-s-Parabox-Rust](https://github.com/1-rambo/Patrick-s-Parabox-Rust)**

### 概览

我们用`Rust`语言重制了推箱子解谜游戏`Partrick's Parabox`的简易版，共1000+行`Rust`代码。它基于`Bevy`游戏引擎的`0.16.1`版本开发。
我们为这个项目做了以下工作：

- 阅读`Bevy`游戏引擎的相关文档，学习了其中基本的接口使用方法和`ECS`(Entity, Compenent, System)游戏开发逻辑。
- 完成游戏的主体设计和开发，包括游戏的基本逻辑、图形界面、音效等。

### 背景：`Bevy`游戏引擎

`Bevy`是一个开源的`Rust`游戏引擎，它为开发者提供了一个跨平台的框架，用于开发游戏。`Bevy`引擎的一大特色是`ECS`机制；`ECS`是指`Entity-Component-System`（实体-组件-系统）。`Entity`是实体或对象，由`Component`构成；每一个`Component`都是一个`struct`（仅含一个整型参数）；`System`则是指程序的执行机制或函数，`System`和`Component`的关系是通过`World`和`Entity`索引建立的。`Bevy`引擎内置了很多`System`的`plugin`，开发者可以通过`plugin`自由组合并从外部模块接入所需要的功能，来设计玩法、界面并使用计算机中的各种设备。