import * as wasm from "./pkg"

// Entry Point
function run() {
    const terminal = new wasm.Terminal();

    window.addEventListener('resize', () => {
        terminal.resize(window.innerWidth, window.innerHeight);
    });
}

run();
