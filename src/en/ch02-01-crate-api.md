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
As already stated, the widget tree that makes up all widget entities
needs to be initialized.  Since every entity is handled via the `Entity
Component Manager` (DCES), the init system provides the needed
methods and structures to handle this task.

The elements of the structure `InitSystem` offers

* the `registry` object, that allows to access global elements (like services)
* the `context_provider`, used to share the dependencies of DCES entities

## post_layout_state_system

## render_system
The `RenderSystem` iterates over all visual widgets. For all entities
that make up the widget, a path of render objects is created.
The render objects are used to be drawn on the screen.

To improve rendering time and speed of interactivity, widgets do have a state.
Only widgets that have been marked as `dirty`, are taken into account
when starting rendering threads in further iterations.

# widget_base

## context

Inside the `OrbTk` world you need functions to handle known object
(entities) as well as there individual properties (components). For
`windows` and `widgets` obviously you have to create, update and delete them.
To present values of the stored properties, you need queries to select them.
You want to get information about their position inside the object tree as well
as the ability to manipulate this position (parent, child, sibling, ancestor).

When it comes to events and messages `OrbTk` will off dedicated `adapters` that
efficiently handles the intercommunication needs (e.g message_adapter, event_adapter)

To adapt the look and feel `localization` and `theming` comes in handy.

## event_adapter

## message_adapter
The `MessageAdapter` is the thread save entry point to sent and read
messages to and from widgets.

Inside the `State` implementation of a widget, define a `message` function.
This function will react on messages (type `MessageReader`) that are exchanged
with your bound registry structure (type `Registry`). You are completely free to operate
in any desired way when you are coding the intended interaction.

Just to be clear, you may have multiple widgets interested in a given
message stored in the registry. And this widgets will usually react differently on a given
massage. `OrbTk` will handle your requests (load and save) utilizing asynchronous functions.

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

This API component offers the building blocks that are used from the
context functions inside the `OrbTk` world to create and manipulate
entities inside the underlying `DCES`. The online library
documentation will briefly explain all available implementations.

As an overview, i will just grab out some essential function:

* `create_entity()` will take care to create and register a new
instance inside the `DCES` store.
* `register_*()` will offer a family of functions to register widget handlers,
layouts, properties or states of entities.
* `append_*()` will manipulate the entity tree inside the `DECS`. The
given parent will insert the named child as a valid member.
