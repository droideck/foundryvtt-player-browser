# FoundryVTT Player Browser #

### Wasm Building ###

1. Install wasm-pack - https://rustwasm.github.io/wasm-pack/installer/
2. Run the following commands to build the wasm package:

```
    ❯ cd ./wasm-foundryvtt-pb
    ❯ wasm-pack build --out-dir ../wasm-build
```

### React Building ###

1. Install npm tool in the system of your choise (i.e. dnf install npm);
2. Run the following commands to build the npm package:

```
    ❯ cd ./react-client
    ❯ npm run build
```

### Serving The Web Application ###

1. You can quickly check if it works with the next command:

```
    ❯ npm start
```

2. If you are happy with the content, you can serve the application and use it:

```
    ❯ npm install -g serve
    ❯ serve -s build
```

