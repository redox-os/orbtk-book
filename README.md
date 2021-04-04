# orbtk-book

![Alt text](https://github/redox-ox/orbtk-book/src/img/orbtk_space.png "OrbTK")
This repository contains the source of "The Orbital Widget Toolkit" book.
We will further reference to it as the `OrbTK` book.

<!--
    WIP: once it is ready to be shipped
    [The book is available in dead-tree form from No Starch Press][nostarch].

    [nostarch]: https://nostarch.com/

    You can read the book for free online. Please see the book as shipped with
    the latest [stable], or [develop] OrbTK releases. Be aware that issues
    in those versions may have been fixed in this repository already, as those
    releases are updated less frequently.

    [stable]: https://doc.orbtk.org/stable/book/
    [develop]: https://doc.orbtk.org/develop/book/

    See the [releases] to download just the code of all the code listings that appear in the book.

    [releases]: https://github.com/redox-os/orbtk/book/releases
-->

### Requirements

Building the book requires [mdBook], ideally the same version that
rust-lang/rust uses in [this file][rust-mdbook]. To get it:

[mdBook]: https://github.com/rust-lang-nursery/mdBook
[rust-mdbook]: https://github.com/rust-lang/rust/blob/master/src/tools/rustbook/Cargo.toml

```bash
$ cargo install mdbook --vers [version-num] mdbook-linkchecker
```

We do make uses of `cargo-readme`. It resolves rust `doc comments`
to generate this README.md file. To get it:

[cargo-readme]: https://github.com/livioribeiro/cargo-readme

```bash
$ cargo install cargo-readme
```

### Building

#### Building the book

To build the book, change into this directory and type:

```bash
$ mdbook build
```

The output will be in the `book` subdirectory. To check it out, open it in
your web browser.

_Firefox:_
```bash
$ firefox book/index.html                       # Linux
$ open -a "Firefox" book/index.html             # OS X
$ Start-Process "firefox.exe" .\book\index.html # Windows (PowerShell)
$ start firefox.exe .\book\index.html           # Windows (Cmd)
```

_Chrome:_
```bash
$ google-chrome book/index.html                 # Linux
$ open -a "Google Chrome" book/index.html       # OS X
$ Start-Process "chrome.exe" .\book\index.html  # Windows (PowerShell)
$ start chrome.exe .\book\index.html            # Windows (Cmd)
```

Executing `mdbook serve` will have **mdbook** act has a web service
which can be accessed opening the following URL:  http://localhost:3000.

To run the tests:

```bash
$ mdbook test
```

#### Building the README

To build the markdown README file, change into this directory and type:

```bash
$ cargo readme > README.md
```

### Code of Conduct

We are committed to providing a friendly, safe and welcoming
environment. Read more about our policy in the [code-of-conduct][coc] page.

[coc]: https://github.com/redox-os/orbtk-book/blob/main/policies/code-of-conduct.md

### Contributing

We'd love your help! Please see [CONTRIBUTING.md][contrib] to learn about the
kinds of contributions we're looking for.

[contrib]: https://github.com/redox-os/orbtk-book/blob/main/CONTRIBUTING.md

#### Translations

We'd love help translating the book! See the [Translations] label to join in
efforts that are currently in progress. Open a new issue to start working on
a new language! We're waiting on [mdbook support] for multiple languages
before we merge any in, but feel free to start! A [pull request] looks promising.

[Translations]: https://github.com/redox-os/orbtk-book/issues?q=is%3Aopen+is%3Aissue+label%3ATranslations
[mdbook support]: https://github.com/rust-lang-nursery/mdBook/issues/5
[pull request]: https://github.com/rust-lang/mdBook/pull/1306

### Spellchecking

To scan source files for spelling errors, you can use the `spellcheck.sh`
script. It needs a dictionary of valid words, which is provided in
`dictionary.txt`. If the script produces a false positive (say, you used word
`BTreeMap` which the script considers invalid), you need to add this word to
`dictionary.txt` (keep the sorted order for consistency).
