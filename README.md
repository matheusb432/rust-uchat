# rust-uchat

'Build a Full-Stack Twitter Clone with Rust' course code and notes

[Original Course Repository](https://github.com/jayson-lennon/ztm-project-uchat)

## Updating Dependencies

As the `Cargo.lock` file is included in this repository, you can update the dependencies by running:

```bash
cargo update
```

## Adding a New Dependency

To add a new dependency to one of the crates, run:

```bash
cargo add -p <crate_name> <dependency>
```

## Project Init

This will check for the dependencies listed above and attempt to install the Rust
dependencies. Dependencies which require manual install will provide a link to
installation instructions.

```bash
cargo run -p project-init
cargo install watchexec-cli
```

If on windows, also run:

```bash
cargo install --locked wasm-bindgen-cli
```

## Formatting

To format the code, run:

```bash
cargo fmt
```

## Linting

To lint the code, run:

```bash
cargo clippy
```

To fix the code, run:

```bash
just fix
```

## Notes in Code

- To visualize notes right next to their example implementations that are scattered throughout this repository, I'd recommend using the VS Code extension `Todo Tree`, and then just filter for any comment with a `NOTE` prefix to it.

  - e.g. `// NOTE This contains a note`

- You can also just search for `NOTE` (case sensitive and with one whitespace afterwards) in the IDE of your choice and it should show every note in the project.
