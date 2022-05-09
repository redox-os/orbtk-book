# Das Orbital Widget Toolkit

[<img src="img/orbtk_planet.svg" width="720"/>](img/orbtk_planet.svg)

*von Florian Blasius, mit der Unterstützung der Rust Gemeinschaft* <br>
*Kommentierung und Dokumentation durch Ralf Zerres und alle Unterstützer*

Diese Version des Textes geht davon aus, dass Du OrbTk v0.3.1 oder
nachfolgend in Verbindung mit einer Rust Toolchain v1.41 oder
nachfolgend verwendest. *Cargo.toml* sollte in den Metadaten
`edition="2018"` definieren. Dies ermöglicht die Nutzung von Rust 2018
Edition spezifischen Konstrukten in allen abgeleiteten Projekten.

Vgl. [“Installations” Abschnitt in Kapitel 1][install]
um OrbTk zu installieren oder zu aktualisieren.

The 2020 Edition diese Buchs ist das erste erstelle Release. Es wird
zusammen mit der OrbTk version 0.3.1 veröffentlicht.

- Appendix A, “Keywords,” erläutert neu eingeführte Bezeichner.
- Appendix D ist ein stetig fortschreitender Arbeitsprogress. Neue
  Freigaben diese Buches erfolgen nach deren Fertigstellung. Ebenso
  wie deren Übersetzung in unterstützte Sprachvarianten.

Um dieses Buch online zu lesen wird eine HTML gerenderte Version unter
[OrbTk-Book (de)][orbtk_book_de] veröffentlicht. Alternativ kann es
auch für die Offline-Nutzung auf lokal installiert werden. Entweder
wird dazu eine gerenderte `pdf` oder `ebook` Version heruntergeladen.

Oder das Buch wird aus dem Quellcode erzeugt. Das Rendern erfolgt mit
dem Aufruf von

```console
mdbook build --dest-dir doc/book_de
```
<!---
Dieser Text wird in [gedrucketer Form und als ebook bei No Starch Press][nsprust] veröffentlicht.
-->

[install]:  https://doc.redox-os.org/orbtk-book/de/ch01-01-installation.html
[nsprust]: https://nostarch.com/orbtk
[orbtk_book_de]: https://www.redox-os.org/orbtk-book/book-de/doc/book-de
