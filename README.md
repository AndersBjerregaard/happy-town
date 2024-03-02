# happy-town

## user_service

Build for different profiles:

```
cargo build --profile <profile>
```

At the time of writing this, there are 3 available profiles: `debug`, `staging` and `release`.

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
