# 任务3.3

***(代码均为本人所写无任何抄袭)***

其中 `answer.md` 和 `answer` 为本文档和文档用到的图片

`src` 目录中含rust源码，`result_pic` 目录中含有不同测试点文法生成的 `Trie` 树

## 实验目的

## 实验内容

使用 Rust 编程语言，实现

## 设计思路

first集的计算使用了记忆化搜索

本算法主要分为两大块内容：

1. 

2. 

### 目前支持的功能

- ⽀持: 

- 暂不支持:

## 结果展示


## 收获与挑战

### 挑战

- 发现 `bath` 和 `bat` 只有 `bath` 才能被接受，`bat` 直接无了，发现是在生成 `Trie` 树的时候，没有考虑节点的权值，导致 `bath` 的生成路径覆盖了 `bat` 的生成路径，导致 `bat` 无法被接受
- 在考虑如何获得前后缀的时候遇到了困难，前缀可以通过 `Trie` 树遍历点，直到有多个叶子结点说明走完了前缀；后缀怎么办呢，原本想的是直接遍历所以候选式，然后看能匹配到哪个前缀，再得到他们的后缀，但这样复杂度太高，于是想到在得到前缀的 DFS 中，顺便就继续 DFS 下去把后缀也给获得，并且绑定在它们共同的前缀上，这给编程带来了很大的便利

### 收获

- 算法编程能力得到了提高
- 通过这次实验，对正规表达式的计算有了更深的理解，对DFA的构造也有了更深的理解
- 更熟练的掌握了数据结构的运用
