# WebAssembly Rust Demo

A simple demo of compiling Rust code into wasm, and loading wasm in web page.

## Requirements

- rustc: 1.73.0
- target: wasm32-unknown-unknown

## Usage

- Compile rust code with: `cargo build --target wasm32-unknown-unknown --release`. A wasm file named `wasm_rust_demo.wasm` will be created under `./target/wasm32-unknown-unknown/release/` directory.
- Move `wasm_rust_demo.wasm` to root directory of this project.
- Host `index.html` page with any server that you like.
- In the browser console, try the function loaded from wasm file.

## Limitations

- WebAssembly is not yet integrated with `<script type='module'>` or `import` statements, thus there is not a path to have the browser fetch modules for you using imports. [Link](https://developer.mozilla.org/en-US/docs/WebAssembly/Loading_and_running)
- WebAssembly has no direct access to the DOM apis. To access any Web API, WebAssembly needs to call out to JavaScript API, which then makes the Web API call. [Link](https://developer.mozilla.org/en-US/docs/WebAssembly/Concepts#porting_from_cc)

## Proposals

- A proposal is made to support WebAssembly get direct access to Web APIs. [Link](https://github.com/WebAssembly/proposals/issues/16)

## References

- https://depth-first.com/articles/2020/06/29/compiling-rust-to-webassembly-a-simple-example/
- https://github.com/lordpoint/wasm-demo
