# Die OrbTK Workspace Struktur

Der Entwicklungsprozess von OrbTK berücksichtigt folgende definierten
Basis-Prinzipien:

* Modularität
* Erweiterbarkeit
* Nativen Multiplattform Support
* Minimieurng von Abhängigkeiten

Innerhalb des Rust Ecosystems existiert die Funktionalität von
`workspaces`. Sie sind hilfreiches Instument, ein anwachsendes crate
in sinnvolle kleinere logische Code-Einheiten aufzubrechen.  Neben dem
Ordnungsfaktor helfen `workspaces` ebenso sich wiederholende zu
reduzieren. Dies gelingt dadurch, dass nur veränderten Code-Blöcken
neu übersetzt werden müssen.

Es ist daher nachvollziehbar, das `OrbTk` sich dieser Sturktur bedient.
Das Toolkit ist in folgende workspaces unterteilt:

* orbtk
* orbtk_core
* orbtk_orbclient
* orbtk_tinyskia
* orbtk_widgets
* proc_macros
* utils

Diese Komponenten und ihre Relationen zueinander im Toolkit werden in
den folgenden Kapiteln Schritt für Schritt erläutert.
