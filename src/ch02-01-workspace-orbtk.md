# Workspace OrbTK

This workspace is the entry point into the framework code.
If you are familiar with Rust code, we are following best practice.

Lets have a quick look at the `src` sub-directory. As usual you will
find a `lib.rs` source file.

Obviously here the code starts to define the crates type "lib". The
next lines define an `outer documentation block`, which serves as a
short introduction. `Outer documentation` lines are encoded with two
slashes followed by an exclamation mark (`//!`).

A very strong feature of the Rust toolchain is the availability of an
inline documentation subsystem. We do use this feature extensively
within `OrbTK`, to document every public accessible code module,
public functions, structure or enumeration. `Inner documentation
blocks` conventionally start with three slahes (`///`).

To render the documentation lines, a simple

```console
cargo doc
```

will generate the online documentation, corresponding to the downloaded release version.
We will timely upload negotiated versions to [Docs.rs][docs_rs].

Back to our structure. To keep the code tight and clear, Rust offers like
most other higher programming languages support the concept of modules.
To put modules and other crates into scope, we need to use the `use` statement.

Both principles helps quite a bit to keep a lean structure beside a
nice developer experience. `Ease of use` is one main goal, so we
prepare `prelude` modules, that will take care to prepare most needed
peaces in a logical way. To consume the offered modules and functions
of `OrbTK`, short and pregnant descriptors should be enough.

[docs_rs]: https://docs.rs/releases/search?query=orbtk
