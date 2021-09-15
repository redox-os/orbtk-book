# OrbTk workspace structure

While developing OrbTk, we try to take care of some basic principles

* modularity
* extensibility
* multiplatfom awareness

Within the Rust ecosystem, `workspaces` are a natural way to distinct
code blocks, that define a logical entity. That allow you to split one
big crate into multiple smaller ones. Beside the ordering factor this
code-splitting is great for avoiding repetitive compilation of the
code, because only crates with changes have to be recompiled. The
result may reduce the compile time by an order of magnitude. Obviously
`OrbTk` is using such a structure.

We do provide to following workspaces:

* orbtk
* orbtk_core
* orbtk_orbclient
* orbtk_tinyskia
* orbtk_widgets
* proc_macros
* utils

The components and relations within the code base will be discussed
step by step in the following chapters.
