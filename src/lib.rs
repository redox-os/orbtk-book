#![crate_name = "orbtk_book"]
#![crate_type = "lib"]

//! <img title="Welcome to OrbTk planet" alt="OrbTK planet" src="/src/img/orbtk_planet.png">
//!
//! [![OrbTk-Book](https://github.com/redox-os/orbtk-book/workflows/CI/badge.svg)](https://github.com/redox-os/orbtk-book/actions)
//! [![License: CC BY 4.0](https://img.shields.io/badge/License-CC_BY_4.0-lightgrey.svg)](https://creativecommons.org/licenses/by/4.0/)
//! [![Latest Release](https://github.com/redox-os/orbtk-book/-/badges/release.svg)](https://github.com/redox-os/orbtk-book/orbtk-book/-/releases)
//! <!-- [![CI-Tests](https://gitlab.com/redox-os/dces-guide/workflows/CI/badge.svg)](https://gitlab.com/redox-os/dces-guide/actions) -->
//!
//! This repository contains the text source for "The Orbital Widget Toolkit" book.
//! We will further reference to it as the `OrbTk-Book`.
//!
//! <!--
//!     WIP: once it is ready to be shipped
//!     [The book is available in dead-tree form from No Starch Press][nostarch].
//!
//!     [nostarch]: https://nostarch.com/
//!
//!     You can read the book for free online. Please see the book as shipped with
//!     the latest [stable], or [develop] OrbTk releases. Be aware that issues
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
//! ## Online Book
//!
//! The following link will provide an online accessible version of
//! the [english variant][book_en] of the guide:
//!
//! `https://redox-os.github.io/orbtk-book/en/index.html`
//!
//! The CI/CD workflow will render an updated version, as soon as enhanced
//! content is merged into the maaster branch.
//! Translations to other lanuages will be uploaded, as soon as they are completed.
//! You may find working links via the [wiki page][wiki_page].
//!
//! [book_en]: https://redox-os.github.io/orbtk-book/en/index.html
//! [wiki_page]: https://github.com/redox-os/orbtk-book/wiki
//!
//! ## Offline Book
//!
//! ### mdBook
//!
//! Building the book requires [mdBook] and its helper tools. The used
//! version should be ideally the same that rust-lang/rust uses in
//! [this file][rust_mdbook].
//!
//! This command will grep the latest mdbook version from [crates.io] in
//! combination with the add-on tools mdbook-linkchecker and
//! mdbook-mermaid. With the linkchecker we are able to asure, that
//! the used links inside the markdown source can resolve to valid
//! targets. mkbook-mermaid is a preprocessor for mdbook to add
//! mermaid.js support. We do use it to create graphs that visiulize
//! some process flows.
//!
//! [crates.io]: https://crates.io/crates/cargo-readme
//!
//! ### Multilingual version of mdBook
//!
//! The OrbTk book aims to make translations as flawless as
//! possible. We are using v0.4.15 that will do the job. There is a
//! patch available that adds the needed salt to organize a book as a
//! multilingual structure: All sources stored in a single hirachical
//! code tree. This work isn't finished yet, but good enough to make
//! use of this branch for our productive needs. Thank you [Nutomic
//! and Ruin0x11][mdbook_localization].
//!
//! Go ahead and install that mdBook version like this:
//!
//! ```
//! TMPDIR=<your_temporary_directory>
//! mkdir -p $TMPDIR; cd $TMPDIR
//! git clone https://github.com/Ruin0x11/mdBook.git
//! cd mdBook
//! git checkout localization
//! cargo update
//! cargo install --path .
//! ```
//!
//! We do make use of process visualization, that will need [mermaid][mdbook_mermaid]. To
//! download and compile it from source, please use the following
//! commands:
//!
//! ```
//! cargo install mdbook-mermaid
//! mermaid install
//! ```
//!
//! [mdBook]: https://github.com/rust-lang-nursery/mdBook
//! [mdBook_localization]: https://github.com/Ruin0x11/mdBook/tree/localization
//! [mdBook_mermaid]: https://github.com/badboy/mdbook-mermaid
//! [rust_mdbook]: https://github.com/rust-lang/rust/blob/master/src/tools/rustbook/Cargo.toml
//!
//! ### Building the book
//!
//! To build the book with the default language (here: 'en'), change
//! into OrbTk-books root directory and type:
//!
//! ```console
//! $ mdbook build --language en --dest-dir orbtk-book/en
//! ```
//!
//! The rendered HTML output will be placed underneath the
//! `orbtk-book/en` subdirectory. To check it out, open it in your web
//! browser.
//!
//! _Firefox:_
//! ```console
//! $ firefox orbtk-book/en/html/index.html                       # Linux
//! $ open -a "Firefox" orbtk-book/en/html/index.html             # OS X
//! $ Start-Process "firefox.exe" .\orbtk-book\en\html\index.html # Windows (PowerShell)
//! $ start firefox.exe .\orbtk-book\en\html\index.html           # Windows (Cmd)
//! ```
//!
//! _Chrome:_
//! ```console
//! $ google-chrome orbtk-book/en/html/index.html                 # Linux
//! $ open -a "Google Chrome" orbtk-book/en/html/index.html       # OS X
//! $ Start-Process "chrome.exe" .\orbtk-book\en\html\index.html  # Windows (PowerShell)
//! $ start chrome.exe .\orbtk-book\en\html\index.html            # Windows (Cmd)
//! ```
//!
//! Executing `mdbook serve` will have **mdbook** act has a web service
//! which can be accessed opening the following URL:  http://localhost:3000.
//!
//! ### Test and validation of the book
//!
//! To run all available tests please call:
//!
//! ```console
//! $ mdbook test
//! ```
//!
//! Translated version of the book will be placed inside the code tree
//! in the subdirectory `src/<language id`.
//!
//! E.g. if you like to render the german version (language id: 'de'), change
//! into OrbTk-book root directory and type:
//!
//! ```console
//! $ mdbook build --language de --dest-dir orbtk-book/de --open
//! ```
//!
//! The rendered HTML output will be placed underneath the
//! `orbtk-book/de` subdirectory. Since we appended the `--open`
//! parameter, your default browser should be fired up and ... tada!
//!
//! ### Cargo handled README
//!
//! We do make uses of the crate [cargo-readme]. This enables us, to
//! use the rust source code as the single root for any generated
//! documentation. The README.md is one of the possible output. The
//! `doc comments` in our lib.rs is parsed to generate the README.md
//! file you are reading now.
//!
//! Install the create with the following command if you want to
//! update or regenerate the README.md yourself.
//!
//! [cargo-readme]: https://github.com/livioribeiro/cargo-readme
//!
//! ```console
//! $ cargo install cargo-readme
//! ```
//!
//! Once the cargo-readme binary is available, you can render the
//! README.md. Change into the document-root and type:
//!
//! ```console
//! $ cargo readme > README.md
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
//! We'd love help translating the book! See the [Translations] label
//! to join in efforts that are currently in progress. Open a new
//! issue to start working on a new language! We're waiting on [mdbook
//! support] for multiple languages to be finalized, but feel free to
//! start! A [pull request] looks promising. The mainline version (we
//! do depend on v0.4.12) is capable to render the existing versions
//! where sources are installed in the intended final structure.
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
//! © 2021-2022 Ralf Zerres
