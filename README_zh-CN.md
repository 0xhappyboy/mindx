<h1 align="center">mindx</h1>
<h4 align="center">一个实验性类脑仿生联想记忆系统,所有知识仅从输入中发育,无任何预置人类知识。</h4>
<p align="center">
  <a href="https://github.com/0xhappyboy/mindx/blob/main/LICENSE"><img src="https://img.shields.io/badge/License-Apache2.0-d1d1f6.svg?style=flat&labelColor=1C2C2E&color=BEC5C9&logo=googledocs&label=license&logoColor=BEC5C9" alt="License"></a>
  <a href="https://crates.io/crates/mindx"><img src="https://img.shields.io/badge/crates-mindx-20B2AA.svg?style=flat&labelColor=0F1F2D&color=FFD700&logo=rust&logoColor=FFD700"></a>
  <a href="https://crates.io/crates/mindx"><img src="https://img.shields.io/crates/d/mindx?style=flat&labelColor=0F1F2D&color=20B2AA&logo=rust&logoColor=white&label=downloads" alt="Crates.io Downloads"></a>
</p>
<p align="center">
<a href="./README_zh-CN.md">简体中文</a> | <a href="./README.md">English</a>
</p>

## mindx 是什么

mindx 是一个实验性的联想记忆系统。

它的核心假设是：

> 智能不应被"编程"进去，而应从经历中"长"出来。

系统内部只有四样东西：

- 稀疏联想矩阵 `W`（初始为空）
- Hebbian 学习规则
- 程序合成引擎
- 一个输入接口

没有词表，没有规则，没有数字表, 所有知识都从 `Mind::input` 中来。

## 核心机制

mindx 由 16 个机制组成，每个机制独立成模块：

| 模块             | 作用                       |
| ---------------- | -------------------------- |
| `entry`          | 唯一公开接口 `Mind::input` |
| `input`          | 输入总调度，按顺序尝试解析 |
| `tokenize`       | 分词（用发现的词优先匹配） |
| `word_discovery` | 从字对频率中发现词         |
| `vector`         | 稀疏向量表示（VSA 风格）   |
| `associate`      | 联想传播                   |
| `hebbian`        | Hebbian 学习               |
| `decay`          | 权重衰减（遗忘）           |
| `rule_def`       | 规则定义解析               |
| `example`        | 例子解析与触发归纳         |
| `induction`      | 程序合成引擎               |
| `equivalence`    | 等价类                     |
| `number`         | 数字解析与计算             |
| `query`          | 联想查询与解码             |
| `stats`          | 状态统计                   |
| `html`           | HTML 清洗与分词辅助        |

## 理论支撑

mindx 的设计受以下论文和理论工作启发。它们分别对应系统的不同机制层。

### 白板说与发育框架

> **A Developmental Framework for Authentic Machine Intelligence: From Embodied Instinct to Emergent Personhood**
> Joshua Knoechelman, 2025
> https://zenodo.org/records/15875750
>
> 启发：真正的智能不能被直接编程，必须在白板状态下通过长期发育"培育"出来。

### Hebbian 学习与持续学习

> **Continual Learning with Hebbian Plasticity in Sparse and Predictive Coding Networks**
> 2024
> https://ar5iv.labs.arxiv.org/html/2407.17305
>
> 启发：稀疏网络中的学习等价于局部 Hebbian 机制，且天然适合持续学习。

### VSA（向量符号架构）

> **Cross-Layer Design of Vector-Symbolic Computing**
> ACM TECS, 2026
> https://dl.acm.org/doi/10.1145/3807784
>
> 启发：VSA 的绑定、捆绑、置换操作，以及高维向量的准正交性。

> **Classification using hyperdimensional computing: a review**
> Springer, 2025
> https://link.springer.com/article/10.1007/s10462-025-11181-2
>
> 启发：超维计算的信息表示与编码方法。

### 归纳与程序合成（AIXI）

> **Universal Algorithmic Intelligence: A Mathematical Top→Down Approach**
> Marcus Hutter, 2007
> https://researchportalplus.anu.edu.au/en/publications/universal-algorithmic-intelligence-a-mathematical-topdown-approac/
>
> 启发：不预设环境知识、从历史中推断规律的理论最优智能体模型。

> **On the computability of Solomonoff induction and AIXI**
> Leike & Hutter, 2018
> https://researchportalplus.anu.edu.au/en/publications/on-the-computability-of-solomonoff-induction-and-aixi/
>
> 启发：AIXI 不可计算性的量化分析，及其可计算近似。

### 人工生命

> **Toward a Living AI: A Self-Evolving System Based on Conway's Game of Life**
> Zenodo
> https://zenodo.org/records/15417438
>
> 启发：以元胞自动机为基底的"活 AI"，涌现、自组织、开放结局演化。

## 具体对应关系

| 论文方向          | 对应 mindx 的机制            |
| ----------------- | ---------------------------- |
| 白板说与发育框架  | 零预置知识、从输入发育       |
| Hebbian 持续学习  | 稀疏联想、在线学习、衰减     |
| VSA / 超维计算    | 稀疏向量表示、组合操作       |
| AIXI / Solomonoff | 程序合成、从例子归纳规则     |
| 人工生命          | 持续运行、自维持、可演化方向 |
