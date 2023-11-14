# rhizo-hello-world
Hello world API route on Rhizo. This repo includes:
##
##
api.toml - An example route configuration.
##
src/main.rs - Hello, world! On Rhizo.
##
src/onchain_read.rs - A template for a route that will return the contents of on-chain bytes as the HTTP response.
##
##
##
Compile to WASIX WASM with:
##
`cargo wasix build --release`
##
##
For rhizo-cli related documentation see:
##
https://rhizodev.github.io/rhizo-docs/documentation.html
