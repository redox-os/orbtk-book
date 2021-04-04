# The Composition System

WIP: introduce a graphical representation of the structure

`OrbTK`
Everything is built on top of DECS, the underlying entity compontent
system.


## GUI Elements

* Layouts
* Events
* Behaviors
* Messages

### Layouts

At the GUI element level, we have a basic concept for implementing
the layout of all involved entities. This concept is following a two
phase model and will run in two passes:

  * `Measuring`
  * `Arranging`

`Measuring` allows a component to determine how much size it can take
inside GUI. The `Arranging` is following in a separate run. Imagine
the request of a parent entity that will ask a child to measure
several times before it can determine the optimal position and the
concrete size of this entity.

This leads us to one of the major concept decisions inside the
toolkit:

* Size the content

What does that mean? Well, lets take an obvious example that will be
visible in every modern application.  You want to support
localization, where the idioms will differ for all labels within the
GUI.  What would happen, if we would size the entities statically (e.g
a button). If we wouldn't take into account the content of a child
entity (e.g. the label centered inside the button frame) that will be
placed in the center of its parent? You as the programmer would need
to adapt the GUI views for every supported language and adapt the
sizes as needed.  That is nonsense. We have to define and render the
stuff the other way arround!

All controls in `OrbTK` support the ability to size to the natural
size of their content. This allows the toolkit to dynamically layout
the entities. No matter if things resize, or application logic will
need to add or subdivide other entities in the tree.

The `Arrange` phase allows a parent to position and determine the final
size of each child.


### Events

* bottom-up

if the events traverse from a leaf entity up to the root entity.

* top-down

if the events traverse from the root entity down to the leaf entities.

### Behaviours

Specialized event handling based on logical grouped methods
(e.g. mouse, keyboard, focus, text).

### Messages

An intelligent messaging infrastucture that instatiates subs. The
concept enables the toolkit to send and receive messages between the
linked entities (m senders -> n receivers).

## Framework Elements

They are organised as crates inside the API subtree.
