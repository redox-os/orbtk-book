# The Widgets Crates

As an UI developer consuming OrbTK, you most probably will get in
touch with the `widget` crate. If you get comfortable with the
terminology of `views` and their `states`, it's quite easy to
implement even complex structures. The GUI components are declarative
and you will code them inside the `view` blocks. All callbacks that
will handle the functional processing are coded inside the `state`
blocks. User input (e.g. mouse events, keyboard input) as well as
event handler generated feedback is handled and processed from methods
of the associated `state` blocks.

The `behavior modules` are separated to handle specialized cases. If
an event is emitted that belongs to a behavior class, the associated
action is handled by a behavior method. In particular you will
recognize modules for the following behaviors:

* focus
* mouse
* selection
* text

## Views

When you create a `view` block inside an OrbTK application, it is
required to insert definitions that declare what elements are going to
be present inside the user interface.

### What is a View

If you take the Rust code that makes a `view` in a structural way, it
will answers to the following questions:

* Which entities are used?
* What is the entities tree formed?
* What attributes are coupled with the given entity?
* What handlers should take care when a given event is emitted?

### What is the code structure of a View

First, the inside the source code that takes your `view` needs to call
the `widget!` macro. This macro automatically implements the `Widget`
trait. When instantiated, it will inherits all default properties from
a base widget, which gets you started with consistent preset values.

The syntax of this macro call will require you, to select

* the desired `view-name` (e.g: "NavigationView")
* optional: the name of the associated `state-structure` (e.g:
  "<NavigationState>")

If you like to assign property names inside the view, go ahead and
introduce an extensible list of the property-variables. Each variable
will take a name and define its associated type.

In a next step you enhance the `Template` trait with an implementation
of your new widget. You are required to code a function called
`template`.  The syntax of this function will take the following
arguments

* `self`, the implementation of your view-name
* the `Id` of the entity
* the `Context`, as a mutual reference to the BuildContext

All the widget structures you are going to use inside of `template`
will be coded as child's of `self`.

## States

When you create a `state` block inside an OrbTK application, it is
required to define the structures you want to work on in the `State`
implementation.

### What is a State

The Rust code that makes a `state` is associated to the `view` block
of your widget. Go and ask yourself:

* What actions should be processed on a given event?
* How should we handled user input?
* What happens, if an entity attribute is changed and gets dirty?

### What is the structure of a State

First, inside the source code that takes your `state`, you will go and
declare its structure name. This name corresponds to the parameter
value you did choose inside the `widget!` macro call of your widgets
view (e.g "NavigationState").

In a next step you enhance the `State` trait with an implementation of
your state structure. Most probable, you create and adapt the
following functions:

#### The `init` function

This function is called to initialize the widget state. You can preset
attributes **before** the view is activated and presented to the user.

#### The `message` function

The `message subsystem` is offering methods to chain events, that can
be interchanged and accessed from any defined `state`. You will code a
`message` function to take advantage of this functionality.

The syntax of this function will take the following arguments

* `self`, the implementation of your message function
* the mutable `messages` variable, referencing the MessageReader
* the `Context`, as a mutual reference to the BuildContext

As already explained, you should define an action enumeration, (e.g
"NavigationAction"), that will code the values that are possible or
desired (e.g "SaveSettings", "LoadSettings"). Inside the `message`
function you will loop through the `messages` and match the action
values you are interested in.

#### The `update` function

Whenever the attribute of an entity is changed, OrbTK will render it
dirty. The `update` function is taking care to react on any triggered
dirty state. You will probably define an `Action` enumeration that
will name and list all action states you are interested in. Now, if
you match an `action` in the `update` function, you can react on this
with all the Rust syntax flexibility.

#### The `update_post_layout` function

OrbTK will run this function **after** the rendering crate has
processed the new layout for your view.
