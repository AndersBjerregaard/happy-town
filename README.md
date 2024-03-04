# happy-town

## Tooling

### Rust

Install using rustup (stable):
```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Rustc follows a 6 week stable release schedule.

Checking rustc version:
```shell
rustc --version
```

#### Cargo Watch

Installing cargo-watch:
```shell
cargo install cargo-watch
```

Have `cargo-watch` monitor your source code to trigger commands every time a file changes:
```shell
cargo watch -x check
```

The command above will execute `cargo check` every time a file changes.

Commands can even be chained together:
```shell
cargo watch -x check -x test -x run
```

The command above, after a file changes, will execute `cargo check`, if that succeeds, it will run `cargo test`,
and finally if that also succeeds `cargo run`.

#### Code Coverage

`cargo tarpaulin` can compute code coverage for the application code, ignoring test functions.
Whose can results can be uploaded metrics to services like [Codecov](https://codecov.io/) or [Coveralls](https://coveralls.io/).

Install:
```shell
cargo install cargo-tarpaulin
```

Compute code coverage:
```shell
cargo tarpaulin --ignore-tests
```

#### TODO: Linting

## user_service

Build for different profiles:

```
cargo build --profile <profile>
```

At the time of writing this, there are 3 available profiles: `debug`, `staging` and `release`.

### Diesel.rs : Object Relational Mapping

https://diesel.rs is the choice of ORM. With integrated features for postgres db.

Adding to project:
```toml
[dependencies]
diesel = { version = "2.1.0", features = ["postgres"] }
dotenvy = "0.15"
```

Run `cargo build` after adding dependencies to the `cargo.toml` file, to install dependencies.

#### Installing the Diesel.rs CLI for Postgres

Make sure you have the `libpq-dev` or `libpq` package installed:
```shell
sudo apt update
sudo apt install -y libpq-dev
```

Then install the diesel cli, only with features for postgres:
```shell
cargo install diesel_cli --no-default-features --features postgres
```

## Frontend

### Runtime Environments

When previewing runtime environments in a development setting. Make use of the `vite` commmand with the `--mode` option:
```
vite --mode staging
```
In a realistic production envrionment. Compile and package the project using:
```
npm run build
```
Then bundle the files into an nginx server or something.

### Integration between Frontend & Backend
