# The Orbital Widget Toolkit

[<img src="img/orbtk_planet.svg" width="720"/>](img/orbtk_planet.svg)

*by Florian Blasius, with contributions from the Rust Community*

*annotated and documented by Ralf Zerres and all contributers*

This version of the text assumes you’re using OrbTk v0.3.1 or later in
conjuction with Rust v1.41.0 or later. *Cargo.toml* should define
`edition="2018"`. That enables and uses Rust 2018 Edition idioms in
all derived projects.

See the [“Installation” section of Chapter 1][install]
to install or update OrbTk.

The 2020 Edition of this book is the initial release. It will be
released with the OrbTk version 0.3.1.

- Appendix A “Keywords”, explains the new raw identifiers.
- Appendix D “Translations”, is work in progress. We will release
  instances of this book in the target language once they are translated.

For online reading, a HTML rendered version is available at [Orbtk
book_en][orbtk_book_en]. Alternatively you might want to have it handy
for offline usage. Either you downlaod a rendered `pdf` or
`ebook`version or go ahead and download the source. Then kick on
mdbook (the definition of the target location is optional).

```console
mdbook build --dest-dir doc/book_en --open
```

<!---
This text is available in [paperback and ebook format from No Starch Press][nsprust].
-->

[install]: https://doc.redox-os.org/orbtk-book/ch01-01-installation.html
[nsprust]: https://nostarch.com/orbtk
[orbtk_book_en]: https://www.redox-os.org/orbtk-book/book-en/doc/book-en

<!--
[orbtk_book_en]: https://github.com/redox-os/orbtk-book
[orbtk_book_en_stable]: https://doc.orbtk.org/stable/book_en/html/print.html

-->
