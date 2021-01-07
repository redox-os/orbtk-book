#  Das Orbital Widget Toolkit

![Build Status](https://github.com/redox-os/orbtk-book/workflows/CI/badge.svg)

Dieses Repository enthält den Source-Code zum "The Orbital Widget Toolkit" Buch.
Wir werden dies im folgende mit `OrbTK` referenzieren.

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

## Anforderungen

Die Erstellung des Buchs erfordert das tool [mdbook], idealer Weise in
der gleichen Version, die auch rust-lang/rust in [dieser
Datei][rust-mdbook] verwendet. Ein Download steht unter folgenden URLs
bereit:

[mdBook]: https://github.com/rust-lang-nursery/mdBook
[rust-mdbook]: https://github.com/rust-lang/rust/blob/master/src/tools/rustbook/Cargo.toml

```bash
$ cargo install mdbook --vers [version-num] mdbook-linkchecker
```

## Erstellung

Um das Buch zu erzeugen, wechsel bitte in dieses
Verzeichnis. Anschließend startest Du den Render-Prozess mit:

```bash
$ mdbook build
```

Die Ausgabe erfolgt in das Unterverzeichnis `book`. Öffne anschließend einen Browser und
teste die erzeugten html Seiten.

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

Mit einem Aufruf von `mdbook serve` veranlasst Du **mdbook** einen Web-Service zu starten.
Dieser steht anschließend unter der URL http://localhost:3000 bereit.

Starte die verfügbaren Tests mit:

```bash
$ mdbook test
```

## Code of Conduct

Wir verpflichten uns, eine freundliche, sichere und einladende
Umgebung bereitzustellen. Lesen Sie mehr über unsere Richtlinien auf der
[code-of-conduct][coc] Seite.

[coc]: https://github.com/redox-os/orbtk-book/blob/main/policies/code-of-conduct.md

## Mitwirkung

Wir freuen uns sehr über Eure Mithilfe! Schaut Euch bitte die Seite
[CONTRIBUTING.md][contrib] an. Hier wird beschrieben was und wie Ihr zum Projekt beisteuern könnt.

[contrib]: https://github.com/redox-os/orbtk-book/blob/main/CONTRIBUTING.md

### Übersetzungen

Wir freuen uns sehr, wenn Ihr mithelfen wollt, dieses Buch auch in
anderen Sprachen verfügbar zu machen. Das Übersetzungs-Label
[Translations] gibt einen Überblick über aktive Prozesse. Erstellt
einfach einen neuen Eintrag in der Kategorie `Issue` und startet in
Eurer gewünschten Sprache! An mdbook wird aktiv gearbeitet und wir
warten auf die Integration des Features `Mehrsprachenfähigkeit`
[mdbook support], um Eure Arbeiten einzugliedern. Ein [pull request]
sieht schon sehr vielversprechend aus.

[Translations]: https://github.com/redox-os/orbtk-book/issues?q=is%3Aopen+is%3Aissue+label%3ATranslations
[mdbook support]: https://github.com/rust-lang-nursery/mdBook/issues/5
[pull request]: https://github.com/rust-lang/mdBook/pull/1306

## Rechtschreibprüfung

Um die source Dateien auf Rechschreibfehler zu untersuchen, verwendest
Du das Script `spellcheck.sh`. Es verwendet ein Verzeichnis mit
gültigen Wörtern, die über die Datei `dictionary.txt` eingestellt
werden. Wenn Du unübliche Worte verwendest, die das Skript als
ungültig ansieht (Beispiel: `BTreeMap`), kannst Du es in die Datei
`dictionary.txt` einfügen. Es macht es einfacher, wenn Du die Worte in
lexikalischer Reihenfolge einsortierst. Folgenden Prüfläufe
berücksichtigen nun Deinen Eintrag.
