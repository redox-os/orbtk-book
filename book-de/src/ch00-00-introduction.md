# Einführung

> Hinweis: Diese Ausgabe des Buches ist identisch mit dem [The Orbital Widget Toolkit]
> [nsprust], das in gedruckter Form sowie als E-Book bei [No Starch Press][nsporbtk] erhältlich ist.

[nsporbtk]: https://nostarch.com/orbtk
[nsp]: https://nostarch.com/
-->

Willkommen zu *Das Orbital Widget Toolkit*, einem Einführungsbuch über `OrbTK`.
Die Programmiersprache Rust hilft Ihnen, schnellere und zuverlässige Software zu schreiben.
`OrbTK` bringt die nötigen Komponenten mit, um moderne grafische Benutzeroberflächen zu entwickeln.
Es bietet eine kohärente Codebasis, die zu nativem Binärcode kompiliert wird, welcher auf der gewünschten Zielplattform ausgeführt wird.

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

## Für wen OrbTK gedacht ist

`OrbTK` ist ideal für Programmierer, die die Vorteile der Programmiersprache Rust nutzen wollen.
Es besteht keine Notwendigkeit, Datenstrukturen und Typen zu transformieren: OrbTK selbst ist in Rust geschrieben.
Es übernimmt somit natürlich alle strukturellen Vorteile der Programmiersprache und stellt die benötigten GUI Elemente bereit, um Ihre grafische Anwendung zu programmieren.
Schauen wir uns ein paar der wichtigsten Gruppen an.

### Teams von Entwicklern

Rust erweist sich als produktives Werkzeug für die Zusammenarbeit in großen Teams von Entwicklern mit unterschiedlichem Kenntnisstand in der Systemprogrammierung.
Werfen Sie auch einen Blick in das Rust-Buch, welches die grundlegenden Prinzipien erläutert, mit denen Sie sicheren Code produzieren.

`OrbTK` verwendet die Rust-Toolchain so weit wie möglich wieder.
Zeitgenössische Entwickler, die die Lernkurve durchlaufen haben, werden deren Vorteile nutzen:

* Cargo, der mitgelieferte Abhängigkeitsmanager und das Build-Tool, macht das Hinzufügen, Kompilieren und Verwalten von Abhängigkeiten mühelos und konsistent im gesamten Rust Ökosystem.
* Rustfmt sorgt für einen konsistenten Formatierungsstil unter den Entwicklern.
* Der Rust Language Server unterstützt die integrierte Entwicklungsumgebung (IDE) ihrer Wahl was Integration für Code-Vervollständigung und Inline-Fehlermeldungen betrifft. Natürlich vorausgesetzt, dass die IDE ihrer Wahl das LSP und die Sprache als solches unterstützt.

### Studenten

Rust ist für Studenten und alle, die sich für das Erlernen von Systemkonzepten interessieren.
Mit Hilfe von Rust haben viele Leute etwas über Themen wie Betriebssystementwicklung gelernt.
Die Community ist sehr einladend und beantwortet gerne Fragen von Anfängern und Studierenden.
Durch Bemühungen wie dieses Buch, will das Rust-Team die Systemkonzepte Rusts mehr Menschen zugänglich machen.
Insbesondere solchen, die neu in der Programmierung sind.

### Unternehmen

Hunderte von Unternehmen, große und kleine, verwenden Rust produktiv für eine Vielzahl von Aufgaben.
Zu diesen Aufgaben gehören Kommandozeilen-Tools, Web-Services, DevOps-Tooling, eingebettete Geräte, Audio- und Videoanalyse und Transkodierung, Kryptowährungen, Bioinformatik, Suchmaschinen, Anwendungen für das Internet der Dinge, oder maschinelles Lernen.
Sogar große Teile des Firefox-Webbrowsers sind mittlerweile in Rust neu geschrieben worden.

### Open-Source-Entwickler

`OrbTK` ist für Leute, die mit der Programmiersprache Rust, zusammen mit der Gemeinschaft, ihren Entwickler-Tools und Bibliotheken arbeiten wollen.
Wir würden uns freuen, wenn Sie zum Ökosystem mit seinen Komponenten und Einträgen beitragen würden.

## Für wen ist dieses Buch?

Dieses Buch setzt voraus, dass Sie bereits Code in einer anderen Programmiersprache und anderen GUI-Toolkits geschrieben haben.
Wir machen keine Annahmen darüber, welche Sprache oder welches Toolkit dies war.
Wir haben versucht, das Material so zugänglich zu machen, damit Personen mit einer Vielzahl von Entwicklungshintergründen damit zu arbeiten.
Wir verbringen nicht viel Zeit damit, darüber zu sprechen, was Programmierung *ist* oder wie man darüber denkt.
Wenn Sie völlig neu in der Programmierung sind, wäre es besser für Sie, wenn Sie ein Buch zu lesen, das speziell eine Einführung in die Programmierung bietet.
Auch hier gibt es von der Rust-Gemeinschaft bereits einige Anstrengungen, wie z.B. [Rust By Example] https://doc.rust-lang.org/stable/rust-by-example/.

## Wie man dieses Buch benutzt

Im Allgemeinen geht dieses Buch davon aus, dass Sie es in der Reihenfolge von vorne nach hinten lesen.
Spätere Kapitel bauen auf Konzepten früherer Kapitel auf, und frühere Kapitel gehen möglicherweise nicht mehr in die Details eines Themas ein; wir greifen das Thema typischerweise in einem späteren Kapitel wieder auf.

In diesem Buch finden Sie zwei Arten von Kapiteln: Konzeptkapitel und Projektkapitel.
In Konzeptkapiteln lernen Sie einen Aspekt von `OrbTK` kennen.
In Projektkapiteln werden wir gemeinsam kleine Programme schreiben und dabei das bisher Gelernte anwenden.

Kapitel 1 erklärt, wie man Rust und OrbTK installiert, wie man ein minimales Programm schreibt und wie Sie Cargo, den Paketmanager und das Build-Tool von Rust, verwenden.

Schließlich enthalten einige Anhänge noch nützliche Informationen über Rust in einem mehr referenzähnlichen Format.
* Anhang A behandelt die Schlüsselwörter von OrbTK
* Anhang B behandelt OrbTKs ableitbare Merkmale (traits) und Komponenten (crates).

Es gibt keinen falschen Weg, dieses Buch zu lesen: Wenn Sie vorwärts springen wollen, nur zu!
Sie müssen vielleicht zu früheren Kapiteln zurückspringen, wenn Sie etwas Verwirrung beim Lesen eines späteren Kapitels verspüren.
Aber tun Sie, was immer für Sie funktioniert.
<span id="ferris"></span>

Ein wichtiger Teil des Lernprozesses von `OrbTK` ist das Lesen der
Fehlermeldungen zu lesen, die der Compiler anzeigt: Diese werden Sie zu funktionierendem Code führen.
Daher werden wir viele Beispiele, die sich nicht kompilieren lassen, zusammen mit der Fehlermeldung
Fehlermeldung, die der Compiler Ihnen in jeder Situation anzeigt. Beachten Sie, dass Sie, wenn Sie
eingeben und ein beliebiges Beispiel ausführen, wird es möglicherweise nicht kompiliert! Stellen Sie sicher, dass Sie den
umliegenden Text, um zu sehen, ob das Beispiel, das Sie ausführen wollen, einen
Fehler. Ferris hilft Ihnen auch, Code zu erkennen, der nicht funktionieren soll:

| Ferris | Bedeutung |
|------------------------------------------------------------------------|--------------------------------------------------|
| <img src="img/ferris/does_not_compile.svg" class="ferris-explain"/> | Dieser Code lässt sich nicht kompilieren!                      |
| <img src="img/ferris/panics.svg" class="ferris-explain"/> | Dieser Code ist panisch und verweigert die Zusammenarbeit!                                |
| <img src="img/ferris/unsafe.svg" class="ferris-explain"/> | Dieser Quellcode enthält unsicheren Code.            |
| <img src="img/ferris/not_desired_behavior.svg" class="ferris-explain"/>| Dieser Code erzeugt nicht das gewünschte Verhalten. |

In den meisten Situationen führen wir Sie zu der korrekten Version vom Code, welcher nicht kompiliert werden kann.

## Quellcode

Die Quelldateien, aus denen dieses Buch generiert wurde, finden Sie auf [GitHub][book].

[book]: https://github.com/redox-os/orbtk.
<!---
[book]: https://github.com/redox-os/orbtk/book/tree/master/src
-->
