## Hello OrbTK!

Now that you’ve installed the needed building blocks, let’s write your first
OrbTK program. It’s traditional when learning a new language to write a little
program that outputs the text `Hello, world!`. So we’ll do the same here. We
create a minimal app, that creates a window, position this window at the given
coordinate of your screen. The text will be placed  in the center of this
widget.

> Note: This book assumes basic familiarity with the command line. Rust makes
> no specific demands about your editing or tooling or where your code lives, so
> if you prefer to use an integrated development environment (IDE) instead of
> the command line, feel free to use your favorite IDE. Many IDEs now have some
> degree of Rust support; check the IDE’s documentation for details. Recently,
> the Rust team has been focusing on enabling great IDE support, and progress
> has been made rapidly on that front!

### Creating a Project Directory

You’ll start by making a directory to store your OrbTk code. It doesn’t matter
to Rust and OrbTK where your code lives, but for the exercises and projects in
this book, we suggest making a *projects* directory in your home directory and
keeping all your projects there.

Open a terminal and enter the following commands to make a *projects* directory
and a directory for the “Hey OrbTK!” project within the *projects* directory.

For Linux, bsd, macOS, and PowerShell on Windows, enter this:

```console
$ mkdir ~/orbtk
$ cd ~/orbtk
$ mkdir examples
$ cd examples
```

For Windows CMD, enter this:

```cmd
> mkdir "%USERPROFILE%\orbtk"
> cd /d "%USERPROFILE%\orbtk"
> mkdir examples
> cd examples
```

### Writing and Running a OrbTK Application

Next, make a new source file and call it *hello_orbtk.rs*. Rust files always end
with the *.rs* extension. If you’re using more than one word to name your source
file, as we have choosen here, it is a good rust habit to separate its  words
with an underscore.

Now open the *hello_orbtk.rs* file you just created and enter the code in
Listing 1-1.

<span class="filename">Filename: hello_orbtk.rs</span>

```rust
use orbtk::prelude::*;

fn main() {
	// use this only if you want to run it as web application.
	orbtk::initialize();

	Application::new()
		.window(|ctx| {
			Window::new()
				.title("OrbTk - Hello OrbTK example")
				.position((100.0, 100.0))
				.size(420.0, 240.0)
				.child(
					TextBlock::new()
						.text("Hey OrbTk!")
						.h_align("center")
						.v_align("center")
						.build(ctx)
					)
				.build(ctx)
		})
		.run();
}
```

<span class="caption">Listing 1-1: An OrbTK app that prints `Hey OrbTK!`</span>

In a second step, we need to define the dependencies that are needed to compile our application.
Within the Rust toolchain we will use Cargo. Cargo is a tool that allows Rust packages to
declare their various dependencies and ensure that you’ll always get a repeatable build.

Go back to your terminal window and change back to your *projects* directory.

```console
$ cd ~/orbtk
```

Create the *Cargo.toml* file in this directory and enter the code in Listing 1-2.

<span class="filename">Filename: Cargo.toml</span>

```cargo
[package]
name = "orbtk-projects"
version = "0.3.1-alpha4"
authors = [
	"Florian Blasius <flovanpt@posteo.de>",
	"Ralf Zerres <ralf.zerres.de@gmail.com>",
]
description = "The Orbital Widget Toolkit - Training projects"
documentation = "https://docs.rs/orbtk"
repository = "https://github.com/redox-os/orbtk"
readme = "README.md"
license = "MIT"
keywords = [
	"orbital",
	"widget",
	"ui",
]
edition = "2018"

[profile.dev]
opt-level = 1

[dependencies]
#orbtk = { version = "~0.3.1-alpha4" }
orbtk = { git = "https://github.com/redox-os/orbtk.git", branch = "develop" }
```

Save the file and go back to your terminal window. On Linux or macOS, enter
the following commands to compile and run the file:

```console
$ cargo run --release  --examples hello_orbtk.rs
```

On Windows, enter the command `.\main.exe` instead of `./main`:

```powershell
> cargo run --release  --examples hello_orbtk.rs
```

Regardless of your operating system, a window should be placed on the screen
that prints the string `Hey OrbTK!` in its center. If something is preventing
to successfull position the window, refer back to the [“Troubleshooting”][troubleshooting]
<!-- ignore --> part of the Installation section for ways to get help.

If your can enjoy the rendered output of your `Hey OrbTK!` app,
congratulations! You’ve officially written an OrbTK application.
That makes you a OrbTK programmer — welcome!

### Anatomy of an OrbTK Application

Let’s review in detail what just happened in your “Hey OrbTK!” application.
Here’s the first piece of the puzzle:

```rust
use orbtk::prelude::*;

fn main() {

}
```

The first line is introducing a *use* declaration. A *use* declaration is used
to shorten the path required to refer to rust module items. The *prelude* is a
convinient way to a list of things, that rust will automatically import to you
program. Here, we bind the path *orbtk::prelude*. All default items defined in
this path (referenced with *::*;) are now accessible in your source using their
shorthand name. No need to type in their common prefix (*orbtk::prelude::*)

the third line define a function in Rust. The `main` function is special: it is
always the first code that runs in every executable Rust program. The first
line declares a function named `main` that has no parameters and returns
nothing. If there were parameters, they would go inside the parentheses, `()`.

Also, note that the function body is wrapped in curly brackets, `{}`. Rust
requires these around all function bodies. It’s good style to place the opening
curly bracket on the same line as the function declaration, adding one space in
between.

An automatic formatter tool called `rustfmt` will help you to stick to a
standard style across Rust projects. OrbTK is following this guidance.
`rustfmt` will format your code in a particular style. Depending on the version
of your rust toolchain, it is probably already installed on your computer!
Check the online documentation for more details.

Inside the `main` function is the following code:

```rust
	orbtk::initialize();
	Application::new()
```

There are some important details to notice here.
* First, Rust style is to indent with four spaces, not a tab.
* Second, the method `orbkt::initialize` does all the hard work to initialize
  the orbtk environment.
* Third, the method `Application::new` creates a new entity in the entity
  comonent system (DECS). DECS an OrbTK dependency that will create and
  organize all OrbTK entities. If OrbTK methods change attibutes to the widget
  elements, the corresponding DECS object will store this attibutes as
  components to the given entity.

We’ll discuss OrbTK macros and methods in more detail in Chapter <WIP: chapter>.
For now, you just need to know that using a `::new()` means that you’re calling
the creation method of a given widget (here: `Application`).

Let's explain the other lines:

```rust
		.window(|ctx| {
			Window::new()
				.title("OrbTk - minimal example")
				.position((100.0, 100.0))
				.size(420.0, 240.0)
				.child(TextBlock::new()
					.text("OrbTk")
					.h_align("center")
					.v_align("center")
					.build(ctx)
				)
				.build(ctx)
		})
		.run();
   ```

Inside the `Application` method, we pipe in further instructions. Please notice
the important details:

* First, Rust style is to indent with another four spaces, not a tab.
* Second, The piping is encoded using a `dot` followed by a new method name
  (using `window` and `run`).
* Third, the `windows` method takes a Rust closure as its argument.

If you are not familiar with the concept of [closures](https://doc.rust-lang.org/rust-by-example/fn/closures.html), go ahead
and consult the Rust book reference for a deep dive. For now, you just need to
know that a closure can be used as a language shotcut for a function.
When the closure `|ctx| {}` is executed, the result will be
captured inside a return variable (`ctx`). The curly braces define the body,
with the code that is executed inside the closure.

Let's examine this body code of our closure:

* First, We call a method to create a new window entity
  (`Windows::new`).
* Second, We define attributes attached to this entity (`title`,
  `position`, `size`).
* Third, Inside the defined windows, we create a new child entity
  (`child`).
* Fourth, The child method takes arguments. We do create a new textblock
  entity (`Textblock::new`). The textblock is extended with the attributes
  (`text`, `h_align`, `v_align`).
  The text attribute takes the desired string. It's positioning is
  controlled with the attriburtion of the horizontal and vertical
  alignment. By choosing "center", we do advise the renderer to place
  the entity centered within its parent entity, which is the window.

OrbTK is as lazy as possible. We need to call the build method (`build(ctx)`),
that will instatiate the our methods and let the renderer do its work.
With the last statement, we finally call the method that will activate the
Application and drow the Widget on our screen(`run`).

Most lines of Rust code end with a semicolon (`;`), to indicates that this
expression is over and the next one is ready to begin.

### Compiling and Running Are Separate Steps

Before running an OrbTK application, you must compile its source code. A typical
OrbTK project will generate the executable binary code using cargo and place the
result in the target subfolder of the project.

Profiles may be used to configure compiler options such as optimization levels
and debug settings. By default the `dev` or `test` profiles are used. If the
`--release` flag is given, then the release or bench profiles are used.

```console
$ cargo build --release --bin hello_orbtk.rs
$ ../target/release/hello_orbtk
```

On Windows, you need to use `backslash` as a path delimeter:

```powershell
> cargo build --release --bin hello_orbtk.rs
> ..\target\release\hello_orbtk.exe
```

If you like to get debug feedback you can call the build process like this

```console
$ cargo build --features debug --bin hello_orbtk.rs
```
[troubleshooting]: ch01-01-installation.html#troubleshooting
