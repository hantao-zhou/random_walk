# Random Walk Simulator

This project provides a simple random walk simulator with a browser UI.  The
computation is performed entirely in JavaScript so it can be run without any
WebAssembly toolchain.

## Running locally

```bash
npm install
npm start
```

These commands install the dependencies and then serve the `web/` folder at
`http://localhost:8080` using `http-server`. Opening `index.html` directly will
not load the ES module scripts correctly.

## Features

- 2D random walk visualisation using HTML canvas
- Edge reinforced random walk simulator with configurable parameters
- Run multiple walks at once, each drawn with a unique colour
