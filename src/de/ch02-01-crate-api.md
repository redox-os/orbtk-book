# The API Crate

WIP: all about the OrbTk API

# application
# event
# layout
# properties

# render_object

# services

# system
## cleanup_system
## event_state_system

## init_system
Wie bereits erwähnt muss der `widget` Baum initialisiert werden. Damit werden
alle definierten Objekt-Instanzen (**entities**) im `Entity Component Manager`
 (DCES) erzeugt und für die weitere Verarbeitung bereitgestellt.
Das init System stellt die hierzu notwendigen Methoden und Strukturen.

Folgende Elemente der `InitSystem` Struktur werden bereitgestellt:

* das `registry` Objekt, über das auf globale Elemente zugegriffen werden kann (vgl. dies mit `services`)
* der `context_provider`, ermöglicht das Teilen von Abhängigkeiten der DCES Objekt-Instanzen

## post_layout_state_system

## render_system

Das `RenderSystem` arbeitet alle zu visualisierenden widgets
ab. Hierzu wird für alle **widgets** ein Pfad von Renderobjekten erstellt.
Diese Render-Objekte werden für die Abbildung auf dem Bildschirm verwendet.

Um die Zeit für den Renderprozess zu beschleunigen und gleichzeitig
die Reaktionszeit für interaktive Änderungen so kurz wie möglich zu
gestalten wird für alle widgets ein `state` verwaltet. Nur solche
widgets, deren Objekt-Eigenschaft als verändert markiert werden
(`dirty`), werden in den Renderläufen (`rendering threads`)
berücksichtigt und verarbeitet. Das `dirty` flag wird anschließend
zurückgesetzt.

# widget_base

## context

Innerhalb der `OrbTk` Welt werden Funktionen benötigt, die sowohl die
bekannten Objekte selbst (`entities`), als auch deren spezifische
Eigenschaften (`properties`) verwalten. Für `windows` und `widgets`
muss es selbstverständlich möglich sein, diese Objekte zu erstellen,
sie zu aktualisieren und zu löschen. Um die aktuellen Werte der
Objekt-Eigenschaften darzustellen, sind Abfragen notwendig
(**queries**). Darüber hinaus ist es erforderlich, die Position der
Objekte im Objekt-Baum abzufragen, anzupassen und zu
verändern. Baum-Operationen ermöglichen deshalb die gezielte Adressierung
der gewünschten Objekte anhand ihrer Position (vgl. **parent**, **child**, **sibling**,
**ancestor**).

Wenn wir über Ereignisse (**events**) oder Nachrichten-Austausch
zwischen Objekten (**messages**) sprechen, stellt `OrbTk` hierzu
speziell definierte `adapter` zur Verfügung. Deren Aufgabe ist die
effiziente Bearbeitung von Inter-Kommunikations Anfragen. So exitieren
beispeilsweise die Module `message_adapter` und `event_adapter`.

Um die Benutzeroberfläche dynamisch anpassen zu können verfügt `OrbTk`
über eie Localisierungs- (`localization`) und ein Layout-Subsystem
(`theming`).

## event_adapter

## message_adapter
Der `MessageAdapter` ist ein Einstiegspunkt, der den Austausch von
Nachrichten zwischen **widgets** in parallelisierbaren Strängen
(**threads**) sicher implementiert.

Definiere hierzu im `State` der beteiligten widgets eine `massage`
Funktion. Diese Funktion wird ausgewertet, wenn Nachrichten (Typ
`MessageReader`) innerhalb gebundener Registrierungsstrukturen
ausgetauscht werden (Typ `Registry`). Dabei bist Du bei der
Implementierung der auszuführenden Verarbeitungsschritte als Reaktion
auf eine empfangenen Nachricht im empfangenden widget komplett frei.

Betrachten wir einen oft vorgefundenen Fall, bei dem mehrere
voneinander unabhängige **widgets** auf einen strukturierten
Nachrichtentyp reagieren sollten. Üblicherweise werden die Empfänger
unterschiedliche Verarbeitungsschritte ausführen, wenn der
entsprechende Nachrichtentyp empfangen wird. `OrbTk` implementiert
diese Anfragen (**load**, **save**) als Asynchrone Funktionen.

Der nachfolgend gelistete Quellcode dient als einfache Referenz:

```rust
use orbtk::prelude::*;

use serde::{Deserialize, Serialize};

use crate::widgets::configuration::configuration_view::ConfigurationView;

/// Valid `actions` that are handled as state changes in the `Configuration` widget.
#[derive(Clone, Debug)]
pub enum ConfigurationAction {
	SaveConfiguration,
	LoadConfiguration,
}

/// Define valid configuration data.
/// This structure is serialized and saved inside the OS dependent settings file.
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
struct ConfigurationData(
	pub String,
	pub String
);

/// Valid `structures` that are handled inside the state of the `Configuration` widget.
#[derive(Debug, Default, AsAny)]
pub struct ConfigurationState {}

impl State for ConfigurationState {
	fn message {
		&mut self,
		mut messages: MessageReader,
		registry: &mut Registry,
		ctx: &mut Context<'_>,
	) {
		for message in messages.read::<ConfigurationAction>() {
			match message {
				ConfigurationAction::LoadConfiguration => {
					registry
						.get::<Settings>("settings")
						.load_async::<ConfigurationData>("configuration_data".to_string(),
							ctx.entity()
						);
				}
				ConfigurationAction::SaveConfiguration => {
					let configuration_file: String = ConfigurationView::configuration_file_clone(&ctx.widget());
					let language_id: String = ConfigurationView::language_id_clone(&ctx.widget());
					registry
						.get::<Settings>("settings")
						.save_async("configuration_data".to_string(),
							ConfigurationData(
								configuration_file,
								language_id
							),
							ctx.entity(),
						);
				}
			}
		}

		// save the result
		for message in messages.read::<SettingsResult<()>>() {
			println!("Result {:?}", message);
		}

		// load result
		for message in messages.read::<SettingsResult<ConfigurationData>>() {
			if let Ok(data) = message {
				ConfigurationView::configuration_file_set(&mut ctx.widget(), data.0);
				ConfigurationView::language_id_set(&mut ctx.widget(), data.1);
			}
		}
	}
}
```

## build_context

Der `build_context` ist eine API Komponente, um die Handhabung von
Objekten zwischen der `OrbTk` Welt und dem darunterliegenden `DCES` zu
synchronisieren. Die Details zu den definierten Funktionen dieses
Modul finden sich in der Dokumentation der Bibliothek `orbtk_core`

Zum Zwecke eines kurzen Überblicks greife ich hier wesentlichen
Funktionen heraus:

* `create_entity()` erzeugt und registriert eine neue Objektinstanz im `DCES` Speicher.
* `register_*()` gruppierte Funktionen, die gewünschte Widget Objekte
  registrieren (**handler**, **layouts**, **properties**, **states**).
* `append_*()` verändert den Objektbaum innerhalb von `DCES`. Zum
  jeweiligen Eltern-Objekt (**parent**) wird das angegebene Objekt
  als zulässiges Kind-Objekt (**child**) eingefügt.
