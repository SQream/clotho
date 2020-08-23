## SQL Variable Replacer

### Building

This tool is build against Rust 1.45.0+.

- Install Rust if needed (See below). This should provide you with the `cargo` build command. 

- Clone this repo

- `cd` to repo 

- `cargo build --release --target x86_64-unknown-linux-musl`

This builds a standalone executable in the `target\release` folder.

### Usage

Use `-v` to specify replacements, `-f` to specify the file. Example:

`clotho -v col1="'names'" -v col2=ages'' -v col3='"names2"' -v limit=14 -f check.txt`

- Install Rust if needed (See below). This should provide you with the `cargo` build command. 

- Clone this repo

- `cd` to repo 

- `cargo build --release --target x86_64-unknown-linux-musl`

This builds a standalone executable in the `target\release` folder.


### Testing against an SQL file

compile and run the file using `cargo run`, pass `--` to cargo to then pass command line arguments to the app.

Example:

- ` cargo run -- -v col1="'names'" -v col2=ages'' -v col3='"names2"' -v limit=14 -f check.txt`

### Running the executable

- Build the file (See Building)

- Copy the standalone executable from `target\release` to a desired location

- Run the executable. Example: 

 `clotho -v col1="'names'" -v col2=ages'' -v col3='"names2"' -v limit=14 -f ../..check.txt`


### Installing Rust

##### Linux

- Install Rust:

`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

- Add musl library to allow building static binaries: 

`rustup target add x86_64-unknown-linux-musl`

- Make it the default toolchain (optional):
 
`rustup default x86_64-unknown-linux-musl`

##### Windows

Download and run [rustup-init](https://static.rust-lang.org/rustup/dist/i686-pc-windows-gnu/rustup-init.exe)


### Updating Rust

When a new stable version of Rust comes out, you can update any machine where Rust was already installed via:

`rustup update`


### Editor Support

##### IntelliJ IDEA

https://plugins.jetbrains.com/plugin/8182-rust

##### VIM

https://github.com/rust-lang/rust.vim

##### Emacs

https://github.com/rust-lang/rust-mode

##### VS Code

https://marketplace.visualstudio.com/items?itemName=rust-lang.rust

##### Sublime Text

https://github.com/rust-lang/rust-enhanced
