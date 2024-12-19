# Gud

A mini project that's kind of a joke that my friend suggested, I built this to learn how a simple version control system works under the hood.
If you've used `git` before, you'll definitely find `gud` intuitive. If not, it's time to `git gud` lmao.

### Supported Commands

- `gud init` - Initialise a new repository (duh).
- `gud add file.txt` - Add a specific file to staging (duh).
- `gud add .` - Add all files in current directory to staging (duh).
- `gud commit -m "<Message>"` - Commit all files in staging with the associated message (duh).
- `gud status` - Get the current status of the repository (duh).

### Build

There are three ways to try `gud`, the first two within the project directory, and the last on your system (I strongly discourage this, if something happens it's not on me thanks).

Option 1: Using `cargo`

- From within the project directory, run the following commands:
```bash
cargo run -- init
cargo run -- add .
cargo run -- commit -m "Message"
cargo run -- status
```

The `--` after `cargo run` separates `cargo`'s arguments from `gud`'s arguments.

Option 2: Using build binary

- From within the project directory, first build the project, then run using the built binary:
```bash
cargo build
./target/debug/gud init
./target/debug/gud add .
./target/debug/gud commit -m "Message"
./target/debug/gut status
```

Option 3: Global

- Run the following to install onto your system:
```bash
cargo build --release
cargo install --path .
gud init
gud add .
gud commit -m "Message"
gud status
```
