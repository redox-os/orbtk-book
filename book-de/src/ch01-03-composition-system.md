# Die Zusammensetzung

WIP: es fehlt eine graphische Repräsentation der Struktur!

`OrbTK`

`OrbTK` stellt eine [functional reactive-like][functional_reative]
API bereit. Es hängt dabei elementar vom Rust crate [`DCES`][dces] ab, welches
ein Entity Component System bereitstellt. Die Interaction mit `DCES` wird vom
`Entity Component Manager`(ECM) übernommen. Einem Wrapper API, das
`OrbTK` widgets transparent in `ECM` Enititäten und  `OrbTK` Attribute (properties) in
`ECM` Komponenten (components) übersetzt und verwaltet.
`DCES` ist wie `OrbTK` selbst nativ in Rust geschrieben.

[dces]: https://docs.rs/dces
[functional_reative]: https://en.wikipedia.org/wiki/Functional_reactive_programming

## GUI Elements

* Layouts
* Events
* Behaviors
* Messages

### Layouts

Auf der GUI Element Ebene, existiert ein Basiskonzept welches
übergreifend das Layout aller beteiligten Entitäten verarbeitet.
Dieses Konzept basiert auf einem zwei Phasen Modell. Ein Layout wird daher auch in
zwei Arbeitsschritten verarbeitet:

  * `Measuring`
  * `Arranging`

Das `Measuring` berechnet, welche Größenanforderungen die jeweilige
Komponete innerhalb der GUI umsetzen kann. Die Plazierung
(`Arranging`) erfolgt in einem weiteren separaten Schritt.

Stellen wir uns z.B. den Fall vor, dass eine hierarchisch
übergeordnete Entität (`parent`) zunächst einen Prozess starten muss,
der rekursiv alle angebundenen abhängigen Entitäten (`childs`)
verarbeitet, um deren Größenanforderungen zu berechnen. Erst mit
dieser Kenntnis ist eine Berechnung der eigenen Größenanforderung
unter Berücksichtigung der gegebenen Abhängigkeiten (`components`) möglich.

Diese Überlegung führt uns zu einer wesentlichen Konzept Entscheidung
innerhalt des Toolkits:

* Größe des Inhalts

Was bedeutet das? Nun, betrachten wir ein eingängiges Beispiel, das in
jeder moderen Applikation gegenwärtig ist. Die GUI soll in
unterschiedlichen Sprachvarianten umgesetzt werden. Und der wechsel
der gewählten Sprachvariante soll zu Laufzeit unterstützt werden.
Damit ergibt sich das Problem, das die Bezeichner für Felder und die
Darstellung von Beschreibungen in den jeweiligen Sprachen
unterschiedliche Glyphen, und damit unterschiedliche
Größenanforderungen für deren Höhe und Breite haben. Natürlich ist
ebenso die gewählte Schriftart bei der Berechnung der Größe zu berücksitigen.

Es würde daher keinen Sinn machen, beispielsweise die Größe eines
Button statisch festzulegen. Vielmehr sollte die richtige aktivierbare
Button-Größe innerhalb definierbarer Minimal- und Maximalausdehnungen
(`constraints`) dynamisch berechnet werden. Diese Berechnung sollte
sich am zu plazierenden Beschreibungstext und oder gewählten Icon orientieren.

Alle `controls` innerhalb von `OrbTk` unterstützen die Möglichkeit,
die Größe einer Entität anhand der natürlichen Dimensionen des Inhalts
zu dimensionieren. Damit ist es im Toolkit möglich den gesammten
Entitätenbaum im Layout dynamisch anzupassen. Ändert sich die
Applikationslogik und damit die Notwendigkeit einzelne Entitäten
hinzuzufügen, zu verändern oder auszublenden wird dies für den
gesamten Baum in einem dynamischen Layout Prozess umgesetzt.

Die Plazierung der im Layout berechnenten Entitäten erfolgt erst im Anschluß (`Arrange` run).


### Events

* bottom-up

Ein Ereignis wandert bei der Verarbeitung vom Auftreten am Blatt des Enitätenbaums (`leaf entity`) zum
Stamm (`root entity`). Also von Unten nach Oben - oder von Aussen nach Innen.

* top-down

Ein Ereignis wandert bei der Verarbeitung vom Auftreten am Stamm
(`root entity`) zu den Blätter des Enitätenbaums (`leaf entity`). Also
von Oben nach Unten - oder von Innen nach Aussen.

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
