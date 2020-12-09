# The API Crate

WIP: all about the OrbTK API

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
Component Manager` (emc), the init system provides the needed
methods and structures to handle this task.

The elements of the structure `InitSystem` offers

* the `registry` object, that allows to access global elements (like services)
* the `context_provider`, used to shared the dependencies of emc entities

## post_layout_state_system

## render_system
The `RenderSystem` iterates over all visual widgets. For all entities
that make up the widget, a path of render objects is created.
The render objects are used to be drawn on the screen.

To improve rendering time and speed of interactivity, widgets do have a state.
Only widgets that have been marked as `dirty`, are taken into account
when starting rendering threads in further iterations.

# widget_base
