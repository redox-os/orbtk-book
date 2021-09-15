# Einführung

<!--
> Hinweis: Diese Ausgabe des Buches ist identisch mit dem [The Orbital Widget Toolkit]
> [nsprust], das in gedruckter Form sowie als E-Book bei [No Starch Press][nsporbtk] erhältlich ist.

[nsporbtk]: https://nostarch.com/orbtk
[nsp]: https://nostarch.com/
-->

[<img src="img/orbtk.svg" width="720"/>](img/orbtk.svg)

Willkommen zu *Das Orbital Widget Toolkit*, einem Einführungsbuch über
`OrbTk`.  Die Programmiersprache Rust hilft Dir, schnellere und
zuverlässige Software zu schreiben. `OrbTk` bringt die nötigen
Komponenten mit, um moderne grafische Benutzeroberflächen zu
entwickeln. Es bietet eine kohärente Codebasis, die zu nativem
Binärcode kompiliert wird, welcher auf der gewünschten Zielplattform
ausgeführt wird.

## Merkmale

* Moderne, leichtgewichtige API
* Plattformübergreifend
* Modulare Komponenten.
* Basiert auf der Entity-Component-System-Bibliothek DCES
* Flexibles Ereignissystem
* Integrierte Widget-Bibliothek
* Benutzerdefinierte Widgets
* Benutzerdefinierte Theming-Engine
* Dynamische Themenumschaltung
* Integrierte Debugging-Werkzeuge
* Lokalisierung

## Unterstützte Plattformen

* Redox OS (nativ)
* Linux (nativ | cargo-node)
* macOS (nativ | cargo-node)
* Windows (nativ | cargo-node)
* openBSD (nicht getestet, sollte aber funktionieren)
* Web (cargo-node)
* Android (nativ geplant | cargo-node)
* iOS (nativ geplant | cargo-node geplant)
* Ubuntu Touch (nativ geplant | cargo-node geplant)

## Für wen OrbTk gedacht ist

`OrbTk` ist ideal für Programmierer, die die Vorteile der
Programmiersprache Rust nutzen wollen. Es besteht keine
Notwendigkeit, Datenstrukturen und Typen zu transformieren: OrbTk
selbst ist in Rust geschrieben. Es übernimmt somit natürlich alle
strukturellen Vorteile der Programmiersprache und stellt die
benötigten GUI Elemente bereit, um Deine grafische Anwendung zu
programmieren. Schauen wir uns ein paar der wichtigsten Gruppen an.

### Teams von Entwicklern

Rust erweist sich als produktives Werkzeug für die Zusammenarbeit in
großen Teams von Entwicklern mit unterschiedlichem Kenntnisstand in
der Systemprogrammierung. Wirf auch einen Blick in das Rust-Buch,
welches die grundlegenden Prinzipien erläutert, und dir hilft besseren
und sicheren Code zu produzieren.

`OrbTk` verwendet die Rust-Toolchain so weit wie möglich wieder.
Zeitgenössische Entwickler, die die Lernkurve durchlaufen haben, werden deren Vorteile nutzen:

* Cargo, der mitgelieferte Abhängigkeitsmanager und das Build-Tool,
  macht das Hinzufügen, Kompilieren und Verwalten von Abhängigkeiten
  mühelos und konsistent im gesamten Rust Ökosystem.
* Rustfmt sorgt für einen konsistenten Formatierungsstil unter den
  Entwicklern.
* Der Rust Language Server unterstützt die integrierte
  Entwicklungsumgebung (IDE) ihrer Wahl was Integration für
  Code-Vervollständigung und Inline-Fehlermeldungen
  betrifft. Natürlich vorausgesetzt, dass die IDE ihrer Wahl das LSP
  und die Sprache als solches unterstützt.

### Studenten

Rust ist für Studenten und alle, die sich für das Erlernen von
Systemkonzepten interessieren.  Mit Hilfe von Rust haben viele Leute
etwas über Themen wie Betriebssystementwicklung gelernt. Die Community
ist sehr einladend und beantwortet gerne Fragen von Anfängern und
Studierenden. Durch Bemühungen wie dieses Buch, will das Rust-Team
die Systemkonzepte von Rust mehr Menschen zugänglich machen.
Insbesondere solchen, die neu in der Programmierung sind.

### Unternehmen

Hunderte von Unternehmen, große und kleine, verwenden Rust produktiv
für eine Vielzahl von Aufgaben. Zu diesen Aufgaben gehören
Kommandozeilen-Tools, Web-Services, DevOps-Tooling, eingebettete
Geräte, Audio- und Videoanalyse und Transkodierung, Kryptowährungen,
Bioinformatik, Suchmaschinen, Anwendungen für das Internet der Dinge,
oder maschinelles Lernen. Sogar große Teile des Firefox-Webbrowsers
sind mittlerweile in Rust neu geschrieben worden.

### Open-Source-Entwickler

`OrbTk` ist für Leute, die mit der Programmiersprache Rust, zusammen
mit der Gemeinschaft, ihren Entwickler-Tools und Bibliotheken arbeiten
wollen. Wir würden uns freuen, wenn Du zum Ökosystem mit seinen
Komponenten und Einträgen beitragen könntest. Du bist herzlich
eingeladen.

## Für wen ist dieses Buch?

Dieses Buch setzt voraus, dass Du bereits Code in einer anderen
Programmiersprache und anderen GUI-Toolkits geschrieben hast. Es ist
nicht wesentlich, welche Sprache oder welches Toolkit dies war. Wir
haben versucht, das Material so aufzubereiten, dass Personen mit einer
Vielzahl von Entwicklungshintergründen damit arbeiten können. Im
Fokus liegt nicht ein Diskurs was Programmierung *ist* oder wie man
darüber denkt. Wenn Du völlig neu in der Programmierung bist, wäre
es besser, wenn Du zunächstein Buch zur Hand nimmst, das speziell die
Einführung in die Programmierung zum Thema hat. Auch hier gibt es von der
Rust-Gemeinschaft bereits einige Anstrengungen, wie z.B. [Rust By
Example] https://doc.rust-lang.org/stable/rust-by-example/.

## Wie man dieses Buch benutzt

Im Allgemeinen geht dieses Buch davon aus, dass Du es in der
Reihenfolge von vorne nach hinten liest. Spätere Kapitel bauen auf
Konzepten früherer Kapitel auf, und frühere Kapitel gehen
möglicherweise nicht mehr im Detail auf ein bereits besprochenes
Themas ein; Ist es wesentlich, greifen wir das Thema typischerweise in
einem späteren Kapitel wieder auf.

In diesem Buch finden Sie zwei Arten von Kapiteln: `Konzeptkapitel` und `Projektkapitel`.
In Konzeptkapiteln lernst Du einen Aspekt von `OrbTk` kennen. In
Projektkapiteln werden wir gemeinsam kleine Programme schreiben und
dabei das bisher Gelernte anwenden.

Kapitel 1 erklärt, wie man Rust und OrbTk installiert, wie man ein
minimales Programm schreibt und wie Du `cargo`, den Paketmanager und
das Build-Tool von Rust, verwendest.

Schließlich enthalten einige Anhänge noch nützliche Informationen über
Rust in einem eher referenzähnlichen Format.

* Anhang A behandelt die Schlüsselwörter von OrbTk
* Anhang B behandelt OrbTks ableitbare Merkmale (traits) und Komponenten (crates).

Es gibt keinen falschen Weg, dieses Buch zu lesen: Wenn Du vorwärts
springen willst, nur zu! Du musst vielleicht zu früheren Kapiteln
zurückspringen, wenn ein späteres Kapitel dich verwirrt.
Was immer für Dich funktioniert ist richtig.
<span id="ferris"></span>

Ein wichtiger Teil des Lernprozesses von `OrbTk` ist das Lesen der
Fehlermeldungen, die der Compiler anzeigt: Diese helfen Dir
funktionierenden Code zu erstellen oder diesen zu verbessern. Daher
werden wir auch Beispiele haben, die sich nicht kompilieren lassen.
Zusammen mit der Fehlermeldung, die der Compiler bereitstellt können
dann die Ursachen erklärt und eine funktionierende Lösung erarbeitet
werden.

Beachte bitte, dass Deine Eingaben um ein beliebiges Beispiel
auszuführen, möglicherweise nicht sofort kompiliert! Stelle dann bitte
sicher, dass Du auch den umliegenden Text zum Beispielcode mit
einbeziehst. Ferris hilft sicher auch, den Code zu erkennen, der nicht
funktionsfähig ist:

| Ferris | Bedeutung |
|-------------------------------------------------------------------------|-----------------------------------------------------------|
| <img src="img/ferris/does_not_compile.svg" class="ferris-explain"/> | Dieser Code lässt sich nicht kompilieren!                      |
| <img src="img/ferris/panics.svg" class="ferris-explain"/>               | Dieser Code ist panisch und verweigert die Zusammenarbeit!|
| <img src="img/ferris/unsafe.svg" class="ferris-explain"/> | Dieser Quellcode enthält unsicheren Code.            |
| <img src="img/ferris/not_desired_behavior.svg" class="ferris-explain"/> | Dieser Code erzeugt nicht das gewünschte Verhalten.       |

In den meisten Situationen führen wir Dich anschliessend zur korrigierten Version des Codes, der dann kompiliert werden kann.

## Quellcode

Die Quelldateien, aus denen dieses Buch generiert wurde, findest Du
auf der Homepage unter [Orbtk book (de)][orbtk_book_de].

[orbtk_book_de]: https://github.com/redox-os/orbtk-book/src/de

<!-- [orbtk_book_de]: https://www.redox-os.org/orbtk-book/book/de -->
