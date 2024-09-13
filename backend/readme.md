# Backend: Fancy TODO App

This folder contains a simple rust actix backend. :)
It uses the Rust language (See below how to compile).
It listens to all incoming HTTP requests on Port 8000.
It uses the environment variable RUST_LOG (e.g. 'info' or 'debug') for logging.

### Build locally (or in a docker maybe?)

```
cargo build --release
```
creates an executable in `./target/release/project_name` (See Cargo.toml for the project name).

### Run an executable

To execute an executable named `executable`, execute the command `/executable`.