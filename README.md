<h1 align="center">mindx</h1>
<h4 align="center">An experimental brain-inspired associative memory system, All knowledge develops solely from input, No pretrained human knowledge of any kind.</h4>
<p align="center">
  <a href="https://github.com/0xhappyboy/mindx/blob/main/LICENSE"><img src="https://img.shields.io/badge/License-Apache2.0-d1d1f6.svg?style=flat&labelColor=1C2C2E&color=BEC5C9&logo=googledocs&label=license&logoColor=BEC5C9" alt="License"></a>
  <a href="https://crates.io/crates/mindx"><img src="https://img.shields.io/badge/crates-mindx-20B2AA.svg?style=flat&labelColor=0F1F2D&color=FFD700&logo=rust&logoColor=FFD700"></a>
  <a href="https://crates.io/crates/mindx"><img src="https://img.shields.io/crates/d/mindx?style=flat&labelColor=0F1F2D&color=20B2AA&logo=rust&logoColor=white&label=downloads" alt="Crates.io Downloads"></a>
</p>
<p align="center">
<a href="./README_zh-CN.md">简体中文</a> | <a href="./README.md">English</a>
</p>

## What is mindx

mindx is an experimental associative memory system.

Its core hypothesis:

> Intelligence should not be programmed in; it should grow out of experience.

Internally, the system has only four things:

- A sparse association matrix `W` (initially empty)
- A Hebbian learning rule
- A program synthesis engine
- A single input interface

There is no vocabulary, no rules, no number table. All knowledge comes from `Mind::input`.

## Core Mechanisms

mindx consists of 16 mechanisms, each in its own module:

| Module           | Role                                             |
| ---------------- | ------------------------------------------------ |
| `entry`          | Single public interface `Mind::input`            |
| `input`          | Dispatch, tries parsers in order                 |
| `tokenize`       | Tokenization (longest match on discovered words) |
| `word_discovery` | Discover words from character-pair frequencies   |
| `vector`         | Sparse vector representation (VSA-style)         |
| `associate`      | Associative propagation                          |
| `hebbian`        | Hebbian learning                                 |
| `decay`          | Weight decay (forgetting)                        |
| `rule_def`       | Rule definition parser                           |
| `example`        | Example parser, triggers induction               |
| `induction`      | Program synthesis engine                         |
| `equivalence`    | Equivalence classes                              |
| `number`         | Number parsing and computation                   |
| `query`          | Associative query and decoding                   |
| `stats`          | State statistics                                 |
| `html`           | HTML cleaning and tokenization helpers           |

## Theoretical Foundations

mindx is inspired by the following papers and theoretical works. They correspond to different mechanism layers of the system.

### Tabula Rasa and Developmental Frameworks

> **A Developmental Framework for Authentic Machine Intelligence: From Embodied Instinct to Emergent Personhood**
> Joshua Knoechelman, 2025
> https://zenodo.org/records/15875750
>
> Inspiration: genuine intelligence cannot be directly programmed; it must be cultivated through long-term development from a blank-slate state.

### Hebbian Learning and Continual Learning

> **Continual Learning with Hebbian Plasticity in Sparse and Predictive Coding Networks**
> 2024
> https://ar5iv.labs.arxiv.org/html/2407.17305
>
> Inspiration: learning in sparse networks is mathematically equivalent to local Hebbian mechanisms, and is naturally suited for continual learning.

### VSA (Vector-Symbolic Architecture)

> **Cross-Layer Design of Vector-Symbolic Computing**
> ACM TECS, 2026
> https://dl.acm.org/doi/10.1145/3807784
>
> Inspiration: VSA binding, bundling, and permutation operations, and the quasi-orthogonality of high-dimensional vectors.

> **Classification using hyperdimensional computing: a review**
> Springer, 2025
> https://link.springer.com/article/10.1007/s10462-025-11181-2
>
> Inspiration: information representation and encoding methods in hyperdimensional computing.

### Induction and Program Synthesis (AIXI)

> **Universal Algorithmic Intelligence: A Mathematical Top→Down Approach**
> Marcus Hutter, 2007
> https://researchportalplus.anu.edu.au/en/publications/universal-algorithmic-intelligence-a-mathematical-topdown-approac/
>
> Inspiration: a theoretically optimal agent model that assumes no prior knowledge of the environment and infers regularities from interaction history.

> **On the computability of Solomonoff induction and AIXI**
> Leike & Hutter, 2018
> https://researchportalplus.anu.edu.au/en/publications/on-the-computability-of-solomonoff-induction-and-aixi/
>
> Inspiration: quantitative analysis of AIXI's uncomputability, and its computable approximations.

### Artificial Life

> **Toward a Living AI: A Self-Evolving System Based on Conway's Game of Life**
> Zenodo
> https://zenodo.org/records/15417438
>
> Inspiration: a "living AI" built on cellular automata, with emergence, self-organization, and open-ended evolution.

## Correspondence

| Paper area                               | Corresponding mechanism in mindx                       |
| ---------------------------------------- | ------------------------------------------------------ |
| Tabula rasa and developmental frameworks | Zero pretrained knowledge, development from input      |
| Hebbian continual learning               | Sparse association, online learning, decay             |
| VSA / hyperdimensional computing         | Sparse vector representation, compositional operations |
| AIXI / Solomonoff                        | Program synthesis, rule induction from examples        |
| Artificial life                          | Continuous operation, self-maintenance, evolvability   |
