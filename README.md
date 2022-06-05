# LearnRust
This repository contains all the materials/files which I referred and created while learning the Assembly Programming language Rust.

Documentation Reference : https://doc.rust-lang.org/book.

Packages: Cargo Crates - https://crates.io/crates.

Ownership Rules
First, let’s take a look at the ownership rules. Keep these rules in mind as we work through the examples that illustrate them:

Each value in Rust has a variable that’s called its owner.
There can only be one owner at a time.
When the owner goes out of scope, the value will be dropped.
