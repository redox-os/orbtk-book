# Introduction

<!--
> WIP: to be uncommented, once it is done
> Note: This edition of the book is the same as [The Orbital Widget Toolkit]
> [nsprust] available in print and ebook format from [No Starch Press][nsporbtk].

[nsporbtk]: https://nostarch.com/orbtk
[nsp]: https://nostarch.com/
-->

[<img src="img/orbtk.svg" width="720"/>](img/orbtk.svg)

Welcome to *The Orbital Widget Toolkit*, an introductory book about `OrbTk`.
The Rust programming language helps you write faster and reliable software.
`OrbTk` contribute the needed crates, to develop modern graphical user interfaces.
It offers a single code base that compiles to native binary code that is executed
on your target platform.

<div class="warning">

Warning: This book is incomplete.  Documenting everything and
rewriting outdated parts take a while.
See the [issue tracker] to check what's missing or outdated. If there
are any mistakes or ideas that haven't been reported, feel free to
open a new issue there.

</div>

[issue tracker]: https://github.com/redox-os/orbtk-book/issues

## Features

* Modern lightweight API
* Cross platform
* Modular crates
* Based on Entity Component System library DCES
* Flexible event system
* Integrated widget library
* Custom widgets
* Custom theming engine
* Dynamic theme switching
* Integrated debugging tools
* Localization

## Supported Platforms

* Redox OS (native)
* Linux (native | cargo-node)
* macOS (native | cargo-node)
* Windows (native | cargo-node)
* openBSD (not tested, but should work)
* Web (cargo-node)
* Android (native planned | cargo-node)
* iOS (native planned | cargo-node planned)
* Ubuntu Touch (native planned | cargo-node planned)

## Who OrbTk Is For

`OrbTk` is ideal for programmers that like to take advantage of the Rust programming
language. No need to transform data structures and types: OrbTk itself is coded in
Rust. It naturally adopts all the structural advantages and provides the needed GUI
elements to code your application. Let’s look at a few of
the most important groups.

### Teams of Developers

Rust is proving to be a productive tool for collaborating among large teams of
developers with varying levels of systems programming knowledge. Have a look at
the Rust book that elaborates the fundamental principles that enables you to
produce more secure code.

`OrbTk` reuses the Rust toolchain as much as possible. Contemporary developer
that have passed the learning curve will take advantage of:

* Cargo, the included dependency manager and build tool, makes adding,
  compiling, and managing dependencies painless and consistent across the Rust
  ecosystem.
* Rustfmt ensures a consistent coding style across developers.
* The Rust Language Server powers Integrated Development Environment (IDE)
  integration for code completion and inline error messages.

### Students

Rust is for students and those who are interested in learning about systems
concepts. Using Rust, many people have learned about topics like operating
systems development. The community is very welcoming and happy to answer
student questions. Through efforts such as this book, the Rust teams want to
make systems concepts more accessible to more people, especially those new to
programming.

### Companies

Hundreds of companies, large and small, use Rust in production for a variety of
tasks. Those tasks include command line tools, web services, DevOps tooling,
embedded devices, audio and video analysis and transcoding, cryptocurrencies,
bioinformatics, search engines, Internet of Things applications, machine
learning, and even major parts of the Firefox web browser.

### Open Source Developers

`OrbTk` is for people who want to build with the Rust programming language,
its community, its developer tools, and libraries. We’d love to have you
contribute to its crates and entities.

## Who This Book Is For

This book assumes that you’ve written code in another programming language
and other GUI toolkits. We do not make any assumptions about which specific
one. We’ve tried to make the material broadly accessible to those from a
wide variety of development backgrounds. We don’t spend a lot of time
talking about what programming *is* or how to think about it.
If you’re entirely new to programming, you would be better served by
reading a book that specifically provides an introduction to programming.

## How to Use This Book

In general, this book assumes that you’re reading it in sequence from front to
back. Later chapters build on concepts in earlier chapters, and earlier
chapters might not delve into details on a topic; we typically revisit the
topic in a later chapter.

You’ll find two kinds of chapters in this book: concept chapters and project
chapters. In concept chapters, you’ll learn about an aspect of `OrbTk`. In project
chapters, we’ll build small programs together, applying what you’ve learned so
far.

Chapter 1 explains how to install Rust and OrbTk, how to write a minimal program,
and how to use Cargo, Rust’s package manager and build tool.


Finally, some appendixes contain useful information about the  in a
more reference-like format. Appendix A covers OrbTk’s keywords, Appendix B
covers OrbTk’s derivable traits and crates.

There is no wrong way to read this book: if you want to skip ahead, go for it!
You might have to jump back to earlier chapters if you experience any
confusion. But do whatever works for you.

<span id="ferris"></span>

An important part of the process of learning `OrbTk` is learning how to read the
error messages the compiler displays: these will guide you toward working code.
As such, we’ll provide many examples that don’t compile along with the error
message the compiler will show you in each situation. Know that if you enter
and run a random example, it may not compile! Make sure you read the
surrounding text to see whether the example you’re trying to run is meant to
error. Ferris will also help you distinguish code that isn’t meant to work:

| Ferris                                                                 | Meaning                                          |
|------------------------------------------------------------------------|--------------------------------------------------|
| <img src="img/ferris/does_not_compile.svg" class="ferris-explain"/>    | This code does not compile!                      |
| <img src="img/ferris/panics.svg" class="ferris-explain"/>              | This code panics!                                |
| <img src="img/ferris/unsafe.svg" class="ferris-explain"/>              | This code block contains unsafe code.            |
| <img src="img/ferris/not_desired_behavior.svg" class="ferris-explain"/>| This code does not produce the desired behavior. |

In most situations, we’ll lead you to the correct version of any code that
doesn’t compile.

## Source Code

The source files from which this book is generated can be found on
the Homepage at [Orbtk book (en)][orbtk_book_en].

[orbtk_book_en]: https://github.com/redox-os/orbtk-book/tree/main/src/en

<!---
[book]: https://github.com/redox-os/orbtk-book
[book]: https://github.com/redox-os/orbtk/book/tree/master/src
-->
