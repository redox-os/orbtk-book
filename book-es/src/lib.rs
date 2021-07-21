#![crate_name = "orbtk_book-en"]
#![crate_type = "lib"]

//! ![Welcome to the OrbTK planet.][orbtk_planet]
//!
//! This repository contains the text source for "The Orbital Widget Toolkit" book.
//! We will further reference to it as the `OrbTK-Book`.
//!
//! Translation this book to the spanish language is work in progress.
//! Please be pacient. It will be updated as soon as it is done.
//!
//! <!--
//!     WIP: once it is ready to be shipped
//!     [The book is available in dead-tree form from No Starch Press][nostarch].
//!
//!     [nostarch]: https://nostarch.com/
//!
//!     You can read the book for free online. Please see the book as shipped with
//!     the latest [stable], or [develop] OrbTK releases. Be aware that issues
//!     in those versions may have been fixed in this repository already, as those
//!     releases are updated less frequently.
//!
//!     [stable]: https://doc.orbtk.org/stable/book/
//!     [develop]: https://doc.orbtk.org/develop/book/
//!
//!     See the [releases] to download just the code of all the code listings that appear in the book.
//!
//!     [releases]: https://github.com/redox-os/orbtk/book/releases
//! -->
//!
//! ## Requirements
//!
//! Building the book requires [mdBook] and its helper tools. The used
//! version should be ideally the same that rust-lang/rust uses in
//! [this file][rust-mdbook]. Install this tools with:
//!
//! ```console
//! $ cargo install mdbook --vers [version-num] mdbook-linkchecker
//! ```
//!
//! We do make uses of the crate `cargo-readme`. It resolves rust `doc
//! comments` to generate this README.md file. You can install it with
//! the following command:
//!
//! [cargo-readme]: https://github.com/livioribeiro/cargo-readme
//! [orbtk_planet]: https://github.com/redox-os/orbtk-book/blob/main/src/img/orbtk_planet.svg
//!
//! ```console
//! $ cargo install cargo-readme
//! ```
//!
//! Now generate the markdown README file. Change into the document-root and type:
//!
//! ```console
//! $ cargo readme > README.md
//! ```
//!
//! [mdBook]: https://github.com/rust-lang-nursery/mdBook
//! [rust-mdbook]: https://github.com/rust-lang/rust/blob/master/src/tools/rustbook/Cargo.toml
//!
//! ## Building
//!
//! ### Building the book
//!
//! To build the book, change into this directory and type:
//!
//! ```console
//! $ mdbook build --dest-dir ../doc/book_en
//! ```
//!
//! The rendered HTML output will be placed underneath the
//! `doc/book_en` subdirectory. To check it out, open it in your web
//! browser.
//!
//! _Firefox:_
//! ```console
//! $ firefox doc/book_en/html/index.html                       # Linux
//! $ open -a "Firefox" doc/book_en/html/index.html             # OS X
//! $ Start-Process "firefox.exe" .\doc\book_en\html\index.html # Windows (PowerShell)
//! $ start firefox.exe .\doc\book_en\html\index.html           # Windows (Cmd)
//! ```
//!
//! _Chrome:_
//! ```console
//! $ google-chrome doc/book_en/html/index.html                 # Linux
//! $ open -a "Google Chrome" doc/book_en/html/index.html       # OS X
//! $ Start-Process "chrome.exe" .\doc\book_en\html\index.html  # Windows (PowerShell)
//! $ start chrome.exe .\doc\book_en\html\index.html            # Windows (Cmd)
//! ```
//!
//! Executing `mdbook serve` will have **mdbook** act has a web service
//! which can be accessed opening the following URL:  http://localhost:3000.
//!
//! To run the tests:
//!
//! ```console
//! $ mdbook test
//! ```
//!
//! ## Code of Conduct
//!
//! We are committed to providing a friendly, safe and welcoming
//! environment. Read more about our policy in the [code-of-conduct][coc] page.
//!
//! [coc]: https://github.com/redox-os/orbtk-book/blob/main/policies/code-of-conduct.md
//!
//! ## Contributing
//!
//! We'd love your help! Please see [CONTRIBUTING.md][contrib] to learn about the
//! kinds of contributions we're looking for.
//!
//! [contrib]: https://github.com/redox-os/orbtk-book/blob/main/CONTRIBUTING.md
//!
//! ### Translations
//!
//! We'd love help translating the book! See the [Translations] label to join in
//! efforts that are currently in progress. Open a new issue to start working on
//! a new language! We're waiting on [mdbook support] for multiple languages
//! before we merge any in, but feel free to start! A [pull request] looks promising.
//!
//! [Translations]: https://github.com/redox-os/orbtk-book/issues?q=is%3Aopen+is%3Aissue+label%3ATranslations
//! [mdbook support]: https://github.com/rust-lang-nursery/mdBook/issues/5
//! [pull request]: https://github.com/rust-lang/mdBook/pull/1306
//!
//! ## Spellchecking
//!
//! To scan source files for spelling errors, you can use the `spellcheck.sh`
//! script. It needs a dictionary of valid words, which is provided in
//! `dictionary.txt`. If the script produces a false positive (say, you used word
//! `BTreeMap` which the script considers invalid), you need to add this word to
//! `dictionary.txt` (keep the sorted order for consistency).
//!
//! ## License
//!
//! <!-- License source -->
//! [Logo-CC_BY]: https://i.creativecommons.org/l/by/4.0/88x31.png "Creative Common Logo"
//! [License-CC_BY]: https://creativecommons.org/licenses/by/4.0/legalcode "Creative Common License"
//!
//! This work is licensed under a [Creative Common License 4.0][License-CC_BY]
//!
//! ![Creative Common Logo][Logo-CC_BY]
//!
//! © 2021 Ralf Zerres
