# Random Walk Simulator

This project provides a WebAssembly powered random walk simulator with a simple web UI.

## Building

```bash
npm run build:cpp
```

This uses the Emscripten toolchain (`em++`) to compile the C++ sources under `cpp/` to WebAssembly. The generated files are output into the `web/` directory.

Noted you should install emsdk and initialise it in advance

```bash
git clone https://github.com/emscripten-core/emsdk.git
cd emsdk
./emsdk install latest
./emsdk activate latest
source "emsdk_env.sh"
```

## Running locally

```bash
npm start
```

This will rebuild the wasm using Emscripten and serve the `web/` folder at `http://localhost:8080`.

## Features

- 2D random walk visualisation using HTML canvas
- Edge reinforced random walk simulator with configurable parameters
- Monte Carlo estimation of expected return time
- Written in C++ and compiled to WebAssembly via Emscripten
