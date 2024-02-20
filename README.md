# playground-jco

This is an example project using jco + cargo-component. This was authored in
February 2024. Please refer to [the post on the bytecode alliance
blog](bytecodealliance.org/articles/jco-1.0) for documentation of this project.

## Building this project

```bash
# install the global dependencies
npm install -g jco
cargo install cargo-component

# install and compile the Rust code
cargo component build

# install and build the JS code
npm install
jco transpile target/wasm32-wasi/debug/playground_jco.wasm -o target/jco

# run the JS code
node index.js
```

## License

MIT
