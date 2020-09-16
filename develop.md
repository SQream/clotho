##

### Purpose

This tool was built to perform variable replacement in text files, to allow a client to have SQL templates that they could then replace.

The spec required a tool that can be run standalone - the file itself on Linux would be all that is necessary. no `.so` files, not even `libc.so` etc.

### Background on Rust

Rust is a compiled non garbage collected language, akin to C/C++. It started in 2010, and has more modern faculties both in the language itself, and in the tooling.
Specifically, it's build Tool - `cargo`, which comes with Rust, can be used to build projects, run tests, download, resolve and compile dependencies etc.

#### Basics



