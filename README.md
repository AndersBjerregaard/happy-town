# happy-town

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
