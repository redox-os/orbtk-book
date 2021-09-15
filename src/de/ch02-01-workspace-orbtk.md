# Workspace OrbTk

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
within `OrbTk`, to document every public accessible code module,
public functions, structure or enumeration. `Inner documentation
blocks` conventionally start with three slashes (`///`).

To render the documentation lines, a simple

```console
cargo doc
```

will generate the online documentation, corresponding to the downloaded release version.
We will timely upload negotiated versions to [Docs.rs][docs_rs].

Back to our structure. To keep the code tight and clear, Rust supports
the concept of **modules**. Like in most other higher programming
languages this allows to subdivide your code into related, condense
function blocks. This resolves to increased clarity and
readability. To put the needed modules or crates into scope, take
advantage of the **use** statement.

Both principles helps quite a bit to keep a lean structure beside a
nice developer experience. `Ease of use` is one main goal, so we
prepare `prelude` modules, that will take care to present the most
needed peaces accessible in your code. Using short and pregnant
descriptors should be enough to consume the offered `OrbTk` modules
and functions in your code.

[docs_rs]: https://docs.rs/releases/search?query=orbtk
