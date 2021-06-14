# OrbTK workspace structure

While developing OrbTK, we try to take care of some basic principles

* modularity
* extensibility
* multiplatfom awareness

Within the Rust ecosystem, `workspaces` are a natural way to distinct code
blocks, that define a logical entity. It's quite obvious that `OrbTK` is using such a structure.

We do provide to following workspaces:

* orbtk
* orbtk_core
* orbtk_orbclient
* orbtk_tinyskia
* orbtk_widgets
* proc_macros
* utils

We will discuss their components and and relations within the code
base step by step in the following chapters.
