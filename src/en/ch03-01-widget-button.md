# Template: Button

This subsection will describe an `OrbTk` UI element called **Button**.

The complete source that demonstrates this template element is presented in [Listing 3-1][widget_button].
After a successful compile run, it should produce the attached screen-shot:

[<img src="img/examples/orbtk_widget_button.png" width="320"/>](img/examples/orbtk_widget_button.png)

We did compile for a desktop target (Linux). And if you did clone the
book source to your development system, the corresponding source-code
examples can be found inside the listings sub-directory. To compile it yourself, first change into this directory

*src/listings/ch03-01-widget-button/listing-03-01*

Next, use **cargo** to pipeline the compile and linking process. In
the end the target binary will be executed. If you like to get a
rendered output, that annotates the tree structure with respect to
their bounds, please make use of the feature **debug**. This feature
will draw blue boxes around any involved entities.

```console
$ cargo run --features debug
```

[<img src="img/examples/orbtk_widget_button_debug.png" width="320"/>](img/examples/orbtk_widget_button_debug.png)

[widget_button]: #complete-example-source

## Recap and annotation

### The anatomy of this template

Let’s review the relevant parts of the **widget_button** application.

### OrbTk code framing the app
As a first step, We put the needed OrbTk parts into scope.

```rust,ignore
{{#include ./listings/ch03-01-widget-button/listing-03-01/src/main.rs:Use}}
```

Next we do declare `"str"` constants to any involved id's. This isn't
strictly necessary, but helps to identify the entities by meaningful names.

```rust,ignore
{{#include ./listings/ch03-01-widget-button/listing-03-01/src/main.rs:Constants}}
```

The **main** function instantiates a new application, that makes use of
the **theme_default_dark** and a re-sizable **Window** as its first
children. For a deeper insight into this UI elements, please consult the
relevant part in this book.

We will now focus our interest on the next part, where we do create a
**ButtonView** as a child inside the **Window** entity.


```rust,ignore
{{#include ./listings/ch03-01-widget-button/listing-03-01/src/main.rs:Child_ButtonView}}
```

The syntax advises the compiler, to implement a *ButtonView* for the
**Template** trait. The `widget!()` macro relieves us to type out all
the boiler plate stuff and takes care to create the needed code sugar.

```rust,ignore
{{#include ./listings/ch03-01-widget-button/listing-03-01/src/main.rs:MacroCall_View}}
```

We do use a [**Container**][widget_container] widget as a first child
inside the template method. It allows us, to place a padding around
the included children. Please refer to its documentation section for
a deeper dive.

```rust,ignore
{{#include ./listings/ch03-01-widget-button/listing-03-01/src/main.rs:Child_Container_Definition}}
```

The container will have a [**Stack**][widget_stack] child, that we do
consume to attach multiple children in a horizontal
direction. [**TextBlocks**][widget_textblock] are used to render some
header text above the **Button** children. This is the part of the
code, that we are finally interested in.

[widget_container]: https://doc.redox-os.org/orbtk-book/ch03-05-widget_container.html
[widget_stack]: https://doc.redox-os.org/orbtk-book/ch03-21-widget-stack.html
[widget_textblock]: https://doc.redox-os.org/orbtk-book/ch03-24-widget-textblock.html

### OrbTk widget specific: **Button**

We are going to consume a **button** widget.

As any other template inside the widget tree of `OrbTk`, the template
is rendered with a preset of sane property values. If you choose not to
explicitly declare any property values inside the view code, the
defaults coded in the template definition will be evaluated.

The following Class-Diagram presents the **button** internal widget tree,
including its default property values:

```mermaid
classDiagram

Button --o MouseBehavior

MouseBehavior --o Container

Container --o Stack

Stack --o FontIconBlock
Stack --o TextBlock

Button : name["button"]
Button : style["button"]
Button : height[36.0]
Button : min_width[64.0]
Button : background[colors-LYNCH_COLOR]
Button : border_radius[4.0]
Button : border_width[0.0]
Button : border_brush["transparent"]
Button : padding['16.0, 0.0, 16.0, 0.0']
Button : foreground[colors-LINK_WATER_COLOR]
Button : text[]
Button : font_size[orbtk_fonts-FONT_SIZE_12]
Button : font["Roboto-Regular"]
Button : icon[]
Button : icon_font["MaterialIcons-Regular"]
Button : icon_size[orbtk_fonts-ICON_FONT_SIZE_12]
Button : icon_brush[colors-LINK_WATER_COLOR]
Button : pressed[false]
Button : spacing[8.0]
Button : container_margin[0]

MouseBehavior : pressed(id)
MouseBehavior : enabled(id)
MouseBehavior : target(id.0)

Container : background(id)
Container : border_radius(id)
Container : border_width(id)
Container : border_brush(id)
Container : padding(id)
Container : opacity(id)
Container : margin("container_margin", id)

Stack : orientation(horizontal)
Stack : spacing(id)
Stack : h_align(center)

FontIconBlock : icon(id)
FontIconBlock : icon_brush(id)
FontIconBlock : icon_font(id)
FontIconBlock : icon_size(id)
FontIconBlock : v_align("center")

TextBlock : font(id)
TextBlock : font_size(id)
TextBlock : foreground(id)
TextBlock : opacity(id)
TextBlock : text(id)
TextBlock : v_align("center")
```

<span class="caption">Workflow 3-1: Button tree</span>

The first button child hasn't choosen an icon. Thus the rendered
output will just present the text property.

```rust,ignore
{{#include ./listings/ch03-01-widget-button/listing-03-01/src/main.rs:Child_Button_TextOnly}}
```

The button hasn't declared a "text" property. Thus the rendered output will just present the icon content.

```rust,ignore
{{#include ./listings/ch03-01-widget-button/listing-03-01/src/main.rs:Child_Button_IconOnly}}
```

Using a style method. Properties assingned via a theme definition take
precedence over property definitons inside the view code.

```rust,ignore
{{#include ./listings/ch03-01-widget-button/listing-03-01/src/main.rs:Child_Button_Styled}}
```


### Complete example source

Find attached the complete source code for our **orbtk_widget_button**
example.

```rust,ignore
{{#rustdoc_include ./listings/ch03-01-widget-button/listing-03-01/src/main.rs:All}}
```

<span class="caption">Listing 3-2: orbtk_widget_button - Available button styles.</span>

### Compiling and Running Are Separate Steps

The **cargo** compiled `orbtk_widget_button` binary will be placed in the target subfolder of the project.

```console
$ cargo build --release --bin orbtk_widget_button
$ ../target/release/orbtk_widget_button
```

On Windows, you need to use `backslash` as a path delimiter:

```powershell
> cargo build --release --bin orbtk_widget_button
> ..\target\release\orbtk_widget_button.exe
```
