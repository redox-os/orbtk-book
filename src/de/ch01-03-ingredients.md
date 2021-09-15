# Die Bestandteile

> WIP: es fehlt eine graphische Repräsentation der Struktur!

<!-- toc -->

`OrbTk` stellt eine [interactive functional reactive][functional_reative]
API bereit. Es hängt dabei elementar vom Rust crate [`DCES`][dces] ab, welches
ein Entity Component System bereitstellt. Die Interaction mit `DCES` wird vom
`Entity Component Manager`(ECM) übernommen. Einem Wrapper API, das
`OrbTk` widgets transparent in `ECM` Enititäten und  `OrbTk` Attribute (properties) in
`ECM` Komponenten (components) übersetzt und verwaltet.
`DCES` ist wie `OrbTk` selbst nativ in Rust geschrieben.

[dces]: https://docs.rs/dces
[functional_reative]: https://en.wikipedia.org/wiki/Functional_reactive_programming

## GUI Elements

* Layouts
* Events
* Behaviors
* Messages

### Layouts

#### Warum brauchen wir Layouts?

Nun, betrachten wir ein eingängiges Beispiel, das in jeder modernen
Applikation umgesetzt werden muss: Mehrere Sprachvarianten sind
erforderlich! Und der Wechsel der gewählten Sprachvariante soll zur
Laufzeit erfolgen. Wir können sicher davon ausgehen, das sich jeder
verwendete Bezeichner für Felder und Beschreibungen in den jeweiligen
Sprachen unterscheidet. Wortlängen und Glyphenbreiten in den Schriften
sind anders. Natürlich ist ebenso die gewählte Schriftart bei der
Berechnung der Größe zu berücksichtigen. Was würde passieren, wenn Du
beispielsweise die Größe einer Entität statisch festlegst? Wir würden
z.B. einen Button mit einer festen Größe kodieren. Wie reagierst Du
nun auf Kontext-Veränderungen von untergeordneten Entitäten (childs)?
Wie gehst Du damit um, dass sich z.B ein Button-Bezeichner, den der Anwender
wahrscheinlich zentriert im Button Rahmen erwartet verändert?

Puh, Du als der Programmierer müsstest an alle möglichen GUI
Darstellungen denken, programmatisch auf jede denkbare
Spracheveränderung reagieren. Ein Alptraum! Nein, wir brauchen einen
tragfähigeren Ansatz.

#### Unsere Lösung

`OrbTk` verwendet ein `layout` System. Dieses System unterstützen die
Möglichkeit, die Größe einer Entität anhand der natürlichen
Dimensionen des Inhalts aufzubereiten. Damit ist es im Toolkit möglich
den gesammten Entitätenbaum im Layout dynamisch anzupassen. Ändert
sich die Applikationslogik und damit die Notwendigkeit einzelne
Entitäten hinzuzufügen, zu verändern oder auszublenden wird dies für
den gesamten Baum in einem dynamischen Layout Prozess umgesetzt. Dabei
werden die individuellen Vorgaben der einzelnen Entitäten
berücksichtigt (`constaints`).

Die individuellen Vorgaben der Entitäten werden über Eigenschaften
(`properties`) als Komponenten im `DCES` gespeichert (`components`).
Das Konzept folgt einem zwei Phasen Modell. Ein Layout wird daher auch in
zwei Arbeitsschritten verarbeitet:

  * `Measuring` Phase
  * `Arrangement` Phase

#### Measuring

Die `Measuring` Phase erlaubt uns, die **gewünschte Größe** einer
`boxed` Entität zu berechnet (**desired size**). Die gewünschte Größe
ist eine Struktur, die die maximalen Werte für Breite und Höhe einer
Entität angibt. Diese Werte werden innerhalb des `DCES` persistent
gespeichert. Wenn die Verarbeitung eine Wertänderung der gewünschten
Größe feststellt (die gespeicherte und die aktuelle Größe
unterscheiden sich), wird die Kennzeichnung `dirty` in der Struktur aktualisiert.

#### Arrangement

Die Plazierung (`Arranging`) erfolgt in einem weiteren separaten
Schritt.  Der Vorgang arbeitet den Baum der angesprochenen Elemente in
einer Schleife ab. Dabei verwendet er die **bounds** es jeweiligen
Elements. Ein **bound** beschreibt die finalisierte Position der
Ausrichtung des Elements (Höhe, Breite) und speichert diese im `DECS`.
Ein Verarbeitungs-Prozess wird nur dann initiiert, wenn ein Element
innerhalb des Baums eine neue Anordnung erzwingt. Alle Elemente werden
nur dann mit den neuen Positionen im Ausgabe-Puffer (`render buffer`) neu
angeordnet, wenn ihr aktiver Status die als `dirty` gekennzeichnet ist.

#### Layout Methods

`OrbTk` unterstützt unterschiedliche Layout Methoden. Dies sind darauf
optimiert, spezifische Anforderungen der unterschiedlichen
Widget-Typen zu berücksichtigen:

* Absolute
* Fixed size
* Grid
* Padding
* Popup
* Stack

Du findest deren Quellcode im Workspace `orbtk_core` im Unterverzeichnis `layout`.
Weitere Informationen zu diesen Methoden werden im [Kapitel: Orbtk Core](ch02-02-workspace-orbtk-core.md) besprochen.

### Events

* bottom-up

Ein Ereignis wandert bei der Verarbeitung vom Auftreten am Blatt des Enitätenbaums (`leaf entity`) zum
Stamm (`root entity`). Also von Unten nach Oben - oder von Aussen nach Innen.

* top-down

Ein Ereignis wandert bei der Verarbeitung vom Auftreten am Stamm
(`root entity`) zu den Blätter des Enitätenbaums (`leaf entity`). Also
von Oben nach Unten - oder von Innen nach Außen.

### Behaviours

Es existieren diffenzierte Methoden für die Bearbeitung logisch gruppierter Ereignisse.
Hierzu zählen derzeit die Ereignisklassen

* Mouse Behaviors
* Selection Behaviors
* Text Behaviors

### Messages

Über das Konzept von MessageAdaptern können Nachrichten zwischen beliebigen
Sender- und Empfänger-Widget gesendet und ausgelesen werden. Die
verwendeten Methoden sind `thread save`.

Jedes Widget kann im State code eine `message` Methode
definieren. Hier ist der Entwickler frei, welche MessageAdapter
berücksichtigt werden sollen. Die Verarbeitungslogik für ausgelesende
Nachrichten ist somit wahlfrei.

## Framework Elements

Alle Elemente des `ObtTk` Frameworks sind als Sub-Module innerhalb des API organisiert.
