# 前言

之前作者自己写的 Rust 都是一些程序，几乎没有涉及到大型的项目管理。Rust 的模块和其它语言不太一样的，而一个大型项目的又需要多个模块组合在一起。为此作者专门查阅资料补齐了 Project Layout 这一个知识点，并记录下这篇学习笔记。

本笔记以[参考资料1](#参考资料)为主，实验环境为 Ubuntu 21.10，rustc 版本为`1.56.1`。作者为本笔记创建了一个[代码仓库](https://github.com/TomSawyer404/rust-project-layout)供读者参考学习。读者可以`git clone`后用`git reset`指令回溯之前的历史。

# 0、万物起源

最开始我们用 Cargo 新建一个项目：

```bash
cargo new project-layout
```

进入目录并编辑源代码：

```rust
// src/main.rs

#[derive(Debug)]
struct A {
    a: i32,
}

#[derive(Debug)]
struct B {
    b: i32,
}

fn main() {
    let first = A { a: 42 };
    println!("{:?}", first);
}
```

和其他新手一样，我们最开始就很简单的把所有东西（什么结构体啦、函数啦、杂七杂八的...）都放到一个文件里。

# 1、使用module

随着项目的增长，我们的`src/main.rs`开始显得有点凌乱。我们尝试把所有的结构体都塞到一个模块（module）中：

```bash
// src/main.rs

mod something {
    #[derive(Debug)]
    pub struct A {
        pub a: i32,
    }

    #[derive(Debug)]
    pub struct B {
        pub b: i32,
    }
}

use crate::something::*;

fn main() {
    let first = A { a: 42 };
    println!("{:?}", first);
}
```

我们要在`mod`里要导出的结构体和它的成员前面加上`pub`关键字。在`main()`函数前面还得加上`use crate::something::*`，这个意思是我们要使用`something`模块的命名空间，`something`的结构体`A`这个符号我可以在本地模块直接用。

`crate`似乎指的位置似乎是当前`Cargo.toml`所在的文件夹。



# 2、将 module 至于其它文件

把模块都塞到一个文件里还是太丑了，我们把刚才的模块`something`移到外部，在`src/main.rs`只留下模块的声明：

```rust
// src/main.rs

mod something;
use crate::something::*;

fn main() {
    let first = A { a: 42 };
    println!("{:?}", first);
}
```

模块的具体实现都留在`src/something.rs`中：

```rust
// src/something.rs

#[derive(Debug)]
pub struct A {
    pub a: i32,
}

#[derive(Debug)]
pub struct B {
    pub b: i32,
}
```

注意文件名必须和模块名一致，这有点像 Cpp 中声明文件是`something.h`，则你的实现得写在`something.cc`里一样。

# 3、用文件夹囊括多个 submodules

有时候我们的一个子系统里包含了许多模块。我们很自然地想到用一个文件夹代表一个子系统，里面有多个文件，作为该子系统的模块。在 Rust 中我们把上述的例子进行重构，目录树如下：

```text
src
├── something
│  ├── a.rs
│  ├── b.rs
│  └── mod.rs
└── main.rs
```

在`main()`函数中，我们的代码不变：

```rust
// src/main.rs 

mod something;
use crate::something::a::*;

fn main() {
    let first = A { a: 42 };
    println!("{:?}", first);
}
```

`src/something/mod.rs`是非常重要的，它告诉 Cargo 关于当前模块有哪些结构体是导出的。它的内容如下：

```rust
// src/something/mod.rs

pub mod a;
pub mod b;
```

最后`something`目录下的各个子模块内容如下：

```rust
// src/something/a.rs

#[derive(Debug)]
pub struct A {
    pub a: i32,
}
```

```rust
// src/something/b.rs

#[derive(Debug)]
pub struct B {
    pub b: i32,
}
```



# 4、使用 lib 和 executable file

Rust 中的 crate 有两个形式：library 和 executable。现在我们把所以和功能无关的代码都封装到 library 中，然后创建一个`bin/`目录，里面存放和 executable 有关的代码，让我们的代码组织起来更加尽然有序。现在的目录树如下：

```text
src
├── bin
│  ├── aloha.rs
│  └── framework_broke_my_heart.rs
├── something
│  ├── a.rs
│  ├── b.rs
│  └── mod.rs
└── lib.rs
```

现在我们的 crate 变成了一个 library，因为它没有入口点了。在`src/lib.rs`中我们描述了本 library 的模块导出关系：

```rust
// src/lib.rs

pub mod something;
```

然后`src/something`下的东西都不动。

新创建的是`src/bin/framework_broke_my_heart.rs`，它是本 library 的一个可执行程序入口：

```rust
// src/bin/framework_broke_my_heart.rs

use project_layout::something::a::*;

fn main() {
    let first = A { a: 42 };
    println!("{:?}", first);
}
```

另外一个是`src/bin/aloha.rs`，它是另一个可执行程序入口：

```rust
// src/bin/aloha.rs

use project_layout::something::b::*;

fn main() {
    let b = B { b: 22 };
    println!("{:?}", b);
}
```

用 Cargo 运行程序的时候，我们需要制定一个可执行程序入口，如下所示：

```bash
 # project-layout on 🌱 main [✘?] is 📦 v0.1.0 via 🦀 v1.56.1
➜  cargo r --bin framework_broke_my_heart
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/framework_broke_my_heart`
A { a: 42 }

# project-layout on 🌱 main [✘?] is 📦 v0.1.0 via 🦀 v1.56.1
➜  cargo r --bin aloha
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/aloha`
B { b: 22 }
```

# 5、Workspaces

当项目大到一定程度后，我们可以重新组织结构，让一个 crate 里包含多个 crates。现在我们的目录树如下：

```text
.
├── src
│  ├── bin
│  │  ├── aloha.rs
│  │  └── framework_broke_my_heart.rs
│  ├── something
│  │  ├── a.rs
│  │  ├── b.rs
│  │  └── mod.rs
│  └── lib.rs
├── target
|  |_ ... // We don't care about this
├── Cargo.lock
└── Cargo.toml
```

在项目根目录下，我们新建两个 crate：

```bash
cargo init --lib my_lib
cargo init my_cli
```

之后我们的目录树如下：

```text
.
├── my_cli		// this whole folder is new
│  ├── src
│  │  └── main.rs
│  └── Cargo.toml
├── my_lib		// this whole folder is new
│  ├── src
│  │  └── lib.rs
│  └── Cargo.toml
├── src
│  ├── bin
│  │  ├── aloha.rs
│  │  └── framework_broke_my_heart.rs
│  ├── something
│  │  ├── a.rs
│  │  ├── b.rs
│  │  └── mod.rs
│  └── lib.rs
├── target
|  |_ ...
├── Cargo.lock
└── Cargo.toml
```

现在我们更改项目根目录的`Cargo.toml`配置：

```toml
[workspace]
members = ["my_lib", "my_cli"]
```

现在它不包含`[package]`和`[dependencies]`，原来的 crate 变成了一个外壳。我们内部多了两个 crates （`my_lib`和`my_cli`），现在我们更改`my_cli`的`Cargo.toml`：

```toml
[package]
name = "my_cli"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
my_lib = { path = "../my_lib" }
```

这里声明一个依赖项，告诉 Cargo 它来自上一级目录的`my_lib`文件夹。

最后我们重新组织代码，得到如下的目录树：

```text
.
├── my_cli
│  ├── src
│  │  └── main.rs
│  └── Cargo.toml
├── my_lib
│  ├── src
│  │  ├── something
│  │  │  ├── a.rs
│  │  │  ├── b.rs
│  │  │  └── mod.rs
│  │  └── lib.rs
│  └── Cargo.toml
├── Cargo.lock
└── Cargo.toml
```

其中`my_lib`里的内容和前几节一样不变，而`my_cli/src/main.rs`的内容如下：

```rust
use my_lib::something::a::*;

fn main() {
    let first = A { a: 42 };
    println!("{:?}", first);
}
```

因为我们在`my_cli/Cargo.toml`中配置了`[dependencies]`，所以我们可以直接用`use my_lib::something::a::*;`来获取需要的符号。

正如你所见的，如果代码被正确地解耦，整个过程非常低调的。当然是好的设计应该从一开始就规划，但有时项目变得很复杂，我们仍然需要不断地重构代码。

# 参考资料

- [Rust: Project structure example step by step](https://dev.to/ghost/rust-project-structure-example-step-by-step-3ee)
- [Package Layout - The Cargo Book](https://doc.rust-lang.org/cargo/guide/project-layout.html)
- [Rust for OOP - Project Management](https://oribenshir.github.io/afternoon_rusting/blog/project-management#:~:text=The%20basics%20of%20Rust%20project%20layout%20are%20simple%2C,whenever%20you%20want%20to%20produce%20a%20runnable%20application.)
- [Cargo 项目管理器 - RustPrimer](https://rustcc.gitbooks.io/rustprimer/content/cargo-projects-manager/cargo-projects-manager.html)
- [Rust编程入门、实践与进阶 第10.2.8节](http://product.dangdang.com/29233731.html)
- [wezsh仓库 - 一个开源社区的实例样本](https://github.com/wez/wzsh)



---