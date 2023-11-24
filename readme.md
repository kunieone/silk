# Silk

## Overview

Silk is a compiled programming language that is built on the concept of composite programming. It is designed to be expressive, concise, and easy to learn. The name "Silk" was chosen for this language because it represents the smooth and flowing nature of the language, just like how silk is a luxurious and elegant fabric. Silk is a versatile language that focuses on function, composition, and inheritance, and has features of modern engineering programming languages, it has a smooth syntax that helps to write engineering-style projects with high standards and maintainable efficiently. However, learning Silk can be challenging due to its many keywords. The Silk language borrows elements of grammar from various contemporary languages.

Silk 是一种编译型语言，基于复合编程概念。它旨在具有表达性，简洁性和易于学习。“Silk”这个名称代表了语言的顺畅和流动性，就像丝绸是一种奢华和优雅的面料一样。Silk 是一种通用语言，重点是函数式+组合+继承，具有现代工程编程语言的特性和流畅的语法，有助于我们编写高标准、可维护的工程类项目。然而，由于 Silk 语言的语法具有许多关键字，因此学习 Silk 语言可能具有挑战性。这种语言在语法功能上借鉴了各种当代语言。

## Caution

> 📕 The language is in the very early stages of development, it's a personal project, but I'd like your help. If you have some ideas, please bring them to me. I will try to finish! Or join me as we develop Silk.

## Get Started

> silk 是一个面向函数式+组合式+继承的多功能语言。具有现代工程编程语言的特性，并且语法丝滑丰富，有助于我们高效率地编写工程式的高标准可维护的项目。Silk 语言的入门将会有一定的难度，因为它的语法配备了许多的关键字。在语法功能上，Silk 语言借鉴了当代各种的语言。
> Silk is a versatile language oriented towards function + composition + inheritance. Features of a modern engineering programming language and a silky syntax that helps us write engineering-style projects of high standards and maintainable efficiently. Introduction to the Silk language can be challenging because its syntax is equipped with many keywords. It is fair to say that in terms of grammatical functions, the Silk language borrows from various contemporary languages.

## Goal

Silk is a growing project, and I want Silk to have the following features:

1. Clean Enough Code, less code, more problems to solve
2. Strong grammatical flexibility, with a lot of syntactic sugar
3. There are type specifications
4. Rich abstract interfaces
5. Object-based
6. Suitable for combinatorial programming architectures

Silk 是一个正在孕育中的项目，我希望 Silk 有以下的特点：

1. 足够简洁的代码，更少的代码，解决更多的问题
2. 很强语法上的灵活性，有着很多的语法糖
3. 有类型的规范
4. 丰富的抽象接口
5. 基于对象
6. 适合组合式编程架构

## Example

```jsx
add "fs"
file_path :str = "example.txt"
contents = "Hello, world!"

fs.open(file_path)
  .write_append(contents)
  .close()
```

```jsx

f deduplicate(raw_str str):str -> Set(raw_str).to_a().join('')

deduplicate("Mississippi") // Misp

```
