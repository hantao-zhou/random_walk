# Random Walk Simulator

This project provides a WebAssembly powered random walk simulator with a simple web UI.

## Building

```bash
npm run build:wasm
```

This uses `wasm-pack` to compile the Rust crate in `wasm/` to WebAssembly and outputs into `web/pkg`.

## Running locally

```bash
npm start
```

This will rebuild the wasm and serve the `web/` folder at `http://localhost:8080`.

## Features

- 2D random walk visualisation using HTML canvas
- Edge reinforced random walk simulator with configurable parameters
- Monte Carlo estimation of expected return time
- Written in Rust and compiled to WebAssembly
