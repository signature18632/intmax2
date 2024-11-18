# Js-test

Scripts to test Wasm library in nodejs environment.

## Pre-requisites
Latest (v0.13.1) version of `wasm-pack` is required to build the wasm library.

```bash
cargo install wasm-pack
```

## Build

```bash
wasm-pack build --target nodejs --out-dir js-test/pkg
```

## Install

```bash
npm install
```

## Run

```bash
export BASE_URL=<URL>
npm run main
```
