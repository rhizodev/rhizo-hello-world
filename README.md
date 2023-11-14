# rhizo-hello-world
Hello world API route on Rhizo. This repo includes:
</br>
</br>
api.toml - An example route configuration.
</br>
src/main.rs - Hello, world! On Rhizo.
</br>
src/onchain_read.rs - A template for a route that will return the contents of on-chain bytes as the HTTP response.
</br>
</br>
</br>
Compile to WASIX WASM with:
</br>
`cargo wasix build --release`
</br></br>
Install rhizo-cli</br>
`cargo install rhizo-cli`</br></br>
Ensure sufficient SOL balance to deploy:</br>
`solana airdrop 2`</br></br>
Deploy:</br>
`rhizo-cli deploy target/wasm32-wasmer-wasi/release/helloworld-rhizo.wasm api.toml`
</br></br>
See the official documentation for installation instructions and documentation on rhizo-cli:</br>

