## Hallo OrbTk!

[<img src="img/orbtk.svg" width="720"/>](img/orbtk.svg)

Nachdem du nun die erforderlichen Bausteine installiert hast, lass uns
dein erstes OrbTk Programm schreiben.  Es Tradition mit der
Einarbeitung in eine neue Programmiersprache ein kleines Programm zu
schreiben, das die Worte `Hello, world!` auf den Bildschirm ausgibt.
Also los. Wir erstellen eine minimale App, die ein Fenster erzeugt und
dieses Fenster an den gegebenen Koordinaten auf dem Bildschirm positioniert.
Das Widget wird unsern Text zentrieren.

> Anmerkung: Diese Buch geht davon aus, dass du Basis-Kenntnisse bei
> der Bedienung der Kommandozeile besitzt. Rust selbst hat keine
> speziellen Anforderungen, welche Werkzeuge du für das editieren von
> Quellcode verwendest und wo du diesen abspeicherst. Wenn du also
> bereits mit einer integrierten Entwicklungsumgebung arbeitest (IDE),
> nur zu, es spricht nichts dagegen diese auch für OrbTk zu nutzen.
> Viele IDEs besitzen mittlerweile ein gewisses Maß an Unterstützung
> für die Sprache Rust. Prüfe einfach die vorhandene Dokumentation.
> In letzter Zeit hat das Rust Team ein besonderes Augenmerk auf die
> Integration von IDE Unterstützung gelegt. Und es wurden große
> Fortschritte in dieser Richtung erzielt!

### Ein Projekt-Verzeichnis erstellen

Zunächst wird eine Verzeichnis erstellt, in dem wir unseren OrbTk
Quellcode speichern wollen. Es spielt für rust und OrbTk keine große
Rolle, wo sich dieser befindet. Aber für die Beispiele und Übungen in
diesem Buch solltest Du einen Unterordner *projects* in deinem Home-Verzeichnis
erzeugen. Wir werden im Folgenden immer auf diesen referenzieren.

Öffne ein Terminal und tippe die folgenden Kommandos ein um die gewünschte
Unterordner Struktur *projects* zu erzeugen:

Für Linux, BSD, macOS und  Power-Shell unter Windows:

```console
$ mkdir -p ~/orbtk-book/projects
$ cd ~/orbtk/projects
```

In der Windows Shell:

```cmd
> mkdir "%USERPROFILE%\orbtk-book"
> cd /d "%USERPROFILE%\orbtk-book"
> mkdir projects
> cd projects
```

### Erstellen und starten der OrbTk Applikation

Im nächsten Schritt erzeugen wir ein neues Projekt und verwenden
hierzu *Cargo*. Mit einer *.toml* Datei beschreiben wir die für den
Rust Code erforderlichen Abhängigkeiten und Metadaten. Das stellt
sicher, das auch bei Folgeaufrufen der Kompilier-Prozesses (build) eine
konsistentes Ergebnis erzeugen kann.

Tippe einfach ein:

```console
$ cargo new orbtk_hello
$ cd orbtk_hello
```

Das erste Kommando, `cargo new`, verwendet als erstes Argument den Projektnamen.
("`orbtk_hello`"). Das zwiete commando wechselt in das neu erstellte Projekt Unterverzeichnis.

Schauen wir uns das erzeugte *Cargo.toml* mal an:

<span class="filename">Filename: Cargo.toml</span>

```toml
{{#include ./listings/ch01-02-orbtk-hello/no-listing-01-02-cargo-new/Cargo.toml}}
```

<span class="caption">Listing 1-1: Default Metadaten "orbtk_hello"</span>

Mit `cargo new`, wurde die Projekt Struktur automatisch
erstellt. Vielleicht wurden auch schon die Angaben für Autor und Email angepasst,
wenn *Cargo* diese Metadaten aus deinen Umgebungsvariablen auslesen konnte.
*Cargo* hat auch bereits den Quellcode für "Hello, world!" erzeugt.
Lass uns die in der Quelldatei *src/main.rs* prüfen:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ./listings/ch01-02-orbtk-hello/no-listing-01-02-cargo-new/src/main.rs}}
```

<span class="caption">Listing 1-2: Default source file "main.rs"</span>

Es gibt keinen Grund, diesen Stand unseres Programmes mit `cargo run` zu kompilieren,
da wir zunächst noch ein paar Projekt Metadaten zusammen mit ein paar Code Zeilen ergänzen wollen.

#### Aktualisierung von Cargo.toml

Zuerst öffne bitte die *Cargo.toml* Datei und gib die Code-Zeilen aus dem Listing 1-1 ein:

<span class="filename">Filename: Cargo.toml</span>

```toml,ignore
{{#include ./listings/ch01-02-orbtk-hello/listing-01-02/Cargo.toml:All}}
```

<span class="caption">Listing 1-1: Project metadata "orbtk_hello"</span>

Vielleicht wundert es Dich, warum die Eigenschaft *name* in der *Cargo.toml* Datei
als `hello_orbtk` formatiert wurde.

```toml,ignore
{{#include ./listings/ch01-02-orbtk-hello/listing-01-02/Cargo.toml:Name}}
```

Es ist eine sinnvolle und empfehlenswerte Gewohnheit, den Rust
Namenkonventionen zu folgen. Ich möchte dich ermutigen, in Rust Code
sogenannte [snake_case][naming] Namen zu nutzen. Wenn wir unsere *OrbTk*
Beispiele erweitern, werden wir den Gruppierungsprefix `orbtk` weiter
verwenden. Aus diesem Grund verwenden wir für unser erstes kleines
Programm den Namen `orbtk_hello`.

#### Aktualisierung von  main.rs

Der gesamte *OrbTk* spezifische Quellcode der für die Übersetzung des
ersten Beispeilprogramms "Hello OrbTk!" notwendig ist wird in Listing
1-2 angezeigt. Diesen bitte in die Datei *src/main.rs* übertragen.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ./listings/ch01-02-orbtk-hello/listing-01-02/src/main.rs:All}}
```

<span class="caption">Listing 1-2: Quellcode zum erzeugen des Fensters und Ausgabe
von "Hey OrbTk!"</span>

Speicher die Datei und gehe zurück in dein Terminal Fenster. Gebe die
folgenden Kommandos ein um das Programm zu Kompilieren und zu starten:

```console
$ cargo run --release orbtk_hello
```

> ***Anmerkung***: Eventuell ist die Installation der Entwicklungsversion von
SDL2 über den Paketmanager der Distribution erforderlich (Ubuntu: libsdl2-dev).

Gleichgültig welches Betriebssystem du gerade verwendest, ein Fenster
sollte sich auf dem Bildschirm öffnen, das dein Text `Hey OrbTk!`
zentriert in diesem Fenster ausgibt.

[<img src="img/examples/orbtk_hello.png" height="150"/>](img/examples/orbtk_hello.png)

<span class="caption">Image 1-2: Applikations-Fenster mit `Hey OrbTk`</span>

Wenn etwas die Fensterausgabe verhindert, schau bitte im Abschnitt
[“Troubleshooting”][troubleshooting] der Installationsbeschreibung nach,
um Hilfestellungen zu erhalten.

Wenn die die gerenderte Ausgaben von `Hey OrbTk!` deiner App bewundern kannst,
Glückwunsch! Du hast erfolgreich deiner erste OrbTk Anwendung geschrieben.
Das macht Dich zum OrbTk Programmierer — willkommen!

### Anatomie einer OrbTk Anwendung

Lass uns die Details ansehen, was gerade mit dem Aufwurf der “Hey
OrbTk!” Anwendung passiert ist. Hier kommt das erste Puzzel-Teilchen:

Für den Moment sollte es ausreichen, die ersten Puzzleteile zu entzaubern.
Wenn Du einen generischen Blick auf die Struktur werfen willst, in
[Abschnitt Workspace][workspace] besprechen wir weitere Details.

```rust,ignore
{{#rustdoc_include ./listings/ch01-02-orbtk-hello/listing-01-02/src/main.rs:Use}}
```

Die erste Zeile fügt die *use* Anweisung ein. Eine *use* Anweisung
wird verwendet, um den Pfadname abzukürzen der notwendig ist, um in
Rust einen *Modul* zu referenzieren. Die Anweisung *prelude* ist ein
bequemer Weg eine Liste von Dingen zusammenzufassen, die Rust
automatisch in dein Programm importiert.  In unserem Fall haben wir
den Pfad *orbtk::prelude* eingebunden. Alle Elemente die über diesen
Pfad addressiert werden können (in der Notation mit *::* beschrieben)
können jetzt als Kurzform über ihren Namen angesprochen werden. Es ist
nicht mehr nötig hierzu den expliziten Pfadname mit zu erfassen (*orbtk::prelude::*)

```rust,ignore
{{#rustdoc_include ./listings/ch01-02-orbtk-hello/listing-01-02/src/main.rs:Main}}
```

Die dritte Zeile definiert eine Rust Funktion. Der Funktionsname
`main` ist insoweit besonders, als das immer die Stelle in einem Rust
Programm angibt, mit der die Code-Ausführung beginnt.  In unserem Fall
hat `main` eine Parameter und liefert auch am Ende der Funktions
nichts zurück. Gäbe es Parameter, sie stünden innerhalb der Klammern, `()`.

Bitte beachte ebenso, dass die Funktion-Körper (body) in gescheiften
Klammern eingebettet ist, `{}`. Die Rust Syntax erwartet dies für alle
Funktionsdefinitionen. Im Rust Code-Style ist es üblich, die
Geschweifte Klammer auf der gleichen Zeile wie die
Funktions-Deklaration zu plazieren und dazwischen ein Leerzeichen einzgeben.

Rust bedient sich eines Tools für die automatische Formatierung von
Codezeilen: `rustfmt`. Es hilt Dir, am Rust Code-Style innerhalb
deiner Projekte konsistent zu bleiben. OrbTk folgt dieser Anleitung.
Abhängig von der Versionsnummer deiner installierten Rust Toolchain
ist die Programmversion von `rustfmt` vermutlich schon auf deinem System
installiert. Andernfalls prüfe bitte die Online-Dokumentation.

Innerhalb der `main` Funktion findest die die folgenden Anweisungen:

```rust,ignore
{{#rustdoc_include ./listings/ch01-02-orbtk-hello/listing-01-02/src/main.rs:Initialize}}
```

Hier gibt es einige wichtige Details herauszustellen.
* Erstens, Rust code wird standardmäßig mit vier Leerzeichen eingerückt, keine Tabulatoren!
* Zweitens, die Methode `orbkt::initialize` vollzieht alle notwendigen
Schritte, um das OrbTk Umgebung zu initialisieren.

```rust,ignore
{{#rustdoc_include ./listings/ch01-02-orbtk-hello/listing-01-02/src/main.rs:Application}}
```

* Drittens, die Methode `Application::new` erstellt eine neue Entität
  im verwendeten Entity-Component-System (DECS). DECS ist eine OrbTk
  Abhängigkeit die die Erstellung und die Organisation aller innerhalb
  von OrbTk verwendeten Entitäten verwaltet. die OrbTk Methoden
  verändern die Attribute der Widget Elemente, die entsprechenden DECS
  Objekte speichern diese Attribute als Compenenten der gegebenen Entity.

Wir werden die OrbTk Makros und Methoden detaillierter in Kapitel
<WIp: chapter> besprechen. Im Moment genügt das Wissen, dass mit dem
Aufruf von `::new()` die Methode zur Erstellung eines neuen Widgets
angesprochen wird (hier: `Application`)

Nun zur den nächsten Zeilen:

```rust,ignore
{{#rustdoc_include ./listings/ch01-02-orbtk-hello/listing-01-02/src/main.rs:Window}}
```

Innerhalb der `Application` Methode, starten wir weitere
Anweisungen. Das Augenmerk liegt auf folgenden Details:

* Erstens, das  Rust Stylingsystem rückt den Code um weitere vier Leerzeichen ein. Keine Tabulatoren!
* Zweitens, das `Pipelining` von Code wird über einen Punkt (`dot`)
  eingeleitet, der um den neuen Methodennamen ergänzt wird (Hier:
  `window`).
* Drittens, die `windows` Methode verwendet eine Rust `closure` als Argument.

Wenn du bis jetzt noch nicht mit dem Konzept von `closures` vertraut
bist, dieser Link ist den Freund:
[closures](https://doc.rust-lang.org/rust-by-example/fn/closures.html).
Diese Referenz bietet ein vertiefendes Verständnis. Im Moment genügt
das Wissen, dass eine closure als effiziente Sprachkomponente an
Stelle einer Funktion genutzt werden kann, Wenn eine closure `|ctx|
{}` ausgeführt wird, wird deren Ergebnis innerhalb der
Rückgabevariable gespeichert (hier: `ctx`). Die Geschweifte Klammer
definiert den closure Korpus, mit dem Quellcode der innerhalb der closure ausgeführt wird.

Lass und den closure Korpus mal prüfen:

* Erstens, wir rufen eine Methode auf, um ein neues Fenster als Entität zu erzeugen
  (`Windows::new`).
* Zweitens, wir definieren Attribute, die wir dieser Entität anfügen (`title`,
  `position`, `size`).
* Drittens, innerhalb des neu definierten Fensters erzeugen wir eine neue, hierarchisch untergeordnete Entität
  (`child`).

```rust,ignore
{{#rustdoc_include ./listings/ch01-02-orbtk-hello/listing-01-02/src/main.rs:Child}}
```

* Viertens, dieses `child` Methode erhält ihrerseits Argumente. Wir
erzeugen eine neue Entität und beschreiben den Widget-Typ
(`Textblock::new`). Der Textblock wird mit Attributen ergänzt (`text`,
`h_align`, `v_align`). Das Attribut `text` erhält den gewünschten
Zeichenwert (string). Seine Position wird über Attribute gesteuert,
die für die horizontale und vertikale Ausrichtung zuständig sind
(`alignment`). Wir wählen `center` und weisen den später aufzurufenden
Render-Prozess damit an, den Text innerhalb der hierarchisch
übergeordneten Entität (`parent`) zentriert zu plazieren. In unserem
Fall ist das das Fenster selbst.

```rust,ignore
{{#rustdoc_include ./listings/ch01-02-orbtk-hello/listing-01-02/src/main.rs:Build}}
```

OrbTk versucht von sich aus, die gegebenen Anweisung zeitlich so weit
wie möglich aufzuschieben (lazy handling). Daher rufen wir die
eigentliche Methode für die Erzeugung der Struktur erst am Ende mit
(`build(ctx)`) auf. Die Entitäten werden instantiert. Der Renderer
wird für die veränderten Komponenten aktiv, berechnet diese neu und
gibt das Ergebnis in den Bildschirmpuffer aus.

```rust,ignore
{{#rustdoc_include ./listings/ch01-02-orbtk-hello/listing-01-02/src/main.rs:Run}}
```

Mit der letzten Anweisung aktivieren wir die Methode, die den Event
Mechanismus kontrolliert.  Die definierte Applikation wird gestartet,
die beschriebenen Widgets auf den Bildschirm ausgegeben (`run`).

Rust Codezeilen werden in der Regel mit einem Simikolon abgeschlossen
(`;`). Dies weist den Kompiler an, das eine gegebene Anweisung
abgeschlossen ist, mit der nächsten fortgefahren werden kann.

### Kompilierung und ausführung sind separate Schritte

Bevor eine OrbTk Application auf der Hardware ablauffähig ist, muss
deren Quellcode über den Kompiler in Maschinencode übersetzt
werden. Ein typisches OrbTk Projekt wird ein ausführbares Programm
(binary) über das Tool `cargo` erzeugen. `cargo` legt die erstellte
Datei in den definierten Projekt-Unterordner.

In den Projekt-Metadaten der Toml-Datei können sogenannte Profile
genutze werden, die Kompiler-Optionen für die gewünschte
Ablaufumgebung einstellen (z.B. Optimierungen, Debugging).
Als Unterlassungswerte (defaults) unterstützt *cargo* die `dev` und `test` Profile.
Wird der Aufruf von *cargo* mit dem `--release` Argument ergänzt,
kommt das sogenannte release or bench Profil zur Anwendung.

```console
$ cargo build --release --bin orbtk_hello.rs
$ ../target/release/hello_orbtk
```

Für Windows muss der `backslash` als Pfad-Trennung verwendet werden:

```powershell
> cargo build --release --bin orbtk-hello.rs
> ..\target\release\orbtk_hello.exe
```

OrbTk unterstützt Entwickler mit zusätzlichen Informationen zur
Kompile-Umgebung. Hierzu kann der Kompile-Lauf um `feature` Argumente
ergänzt werden (derzeit: debug, log).

* debug: die Widgets werden mit Umrandungen gerendert. Dies
  erleichtert die Kontrolle der Einhaltung von constraints.
* log: Bei Aufruf wird beispeilsweise die Hierarchie der verwendeten
  Widgets visualisiert und auf der Kommandozeile ausgegeben.

```console
$ cargo build --features debug,log --bin hello_orbtk.rs
```
[naming]: https://rust-lang.github.io/api-guidelines/naming.html
[troubleshooting]: https://doc.redox-os.org/orbtk-book/de/ch01-01-installation.html#troubleshooting
[workspace]: https://doc.redox-os.org/orbtk-book/ch02-05-workspace-orbtk-widgets.html
