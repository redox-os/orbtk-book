## Installation

The first step is to install Rust. This is described in depth following
[Rust book Chapter 1](https://github.com/rust-lang/book/blob/master/src/ch01-01-installation.md)

When creating a OrbTk application, we define the needed dependencies to the
OrbTk crates in the Cargo.toml file of our project. The complile process
will resolve the references and download the source as needed.

> ### Command Line Notation
>
> In this chapter and throughout the book, we’ll show some commands used in the
> terminal. Lines that you should enter in a terminal all start with `$`. You
> don’t need to type in the `$` character; it indicates the start of each
> command. Lines that don’t start with `$` typically show the output of the
> previous command. Additionally, PowerShell-specific examples will use `>`
> rather than `$`.

### Troubleshooting

> ***WIP***:  What are the most common culprits? Can we provide some general, basic solutions

### Local Documentation

OrbTk offers the option to install its documentation locally, so you can read it
offline.

Any time a type, a function, a method or a crate is reference by the toolkit
and you’re not sure what it does or how to use it, have a look at its application
programming interface [API documentation] to find out!

<!-- [API documentation]: https://www.redox-os.org/orbtk/doc/en -->
<!-- [API documentation]: https://github.com/redox-os/orbtk -->
[API documentation]: https://docs.rs/orbtk

### Install Rust on Linux or macOS

If you are using Linux or macOS open up an terminal and copy and paste the text below and hit the enter key on your keyboard:

```bash
curl https://sh.rustup.rs -sSf | sh
```

### Install Rust on Windows

Download and run the Rust windows installer from https://www.rust-lang.org/tools/install.

### Install Redoxer (Redox OS)

If you want build and run your Rust application on a [KVM](https://en.wikipedia.org/wiki/Kernel-based_Virtual_Machine) capable OS for Redox you can use [redoxer](https://gitlab.redox-os.org/redox-os/redoxer).

To install Redoxer you have to first install the rust toolchain. After that open up an terminal and copy and paste the text below and hit the enter key on your keyboard:

```bash
cargo +nightly install redoxer
```

To compile and run your application on Redox OS you should check the Redox OS Book.

### Editor and IDE integration

A wide range of editors and IDE's are providing support for Rust code like

* like syntax-highlighting
* auto-completion
* linting
* lsp support

#### VS Code

There is a big community that rely on the visualstudio implementation
to handle their code base. Following are the steps needed to expand
your installation to support `VS Code for Rust` development:

1. Download VS Code [from](https://code.visualstudio.com/download).
2. Install [Rust Language Server plugin](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust) (the
   Rust Language Server).

#### Alternative Editors and IDEs

If you perefer other solution, you will find in depth help inside the
context of this inclomplete links:

* [Atom](https://atom.io/packages/language-rust)
* [Intellij IDEA](https://intellij-rust.github.io)
* [Vim](https://github.com/rust-lang/rust.vim)
* [Emacs](https://github.com/rust-lang/rust-mode)
* [Eclipse](https://github.com/eclipse/corrosion)
