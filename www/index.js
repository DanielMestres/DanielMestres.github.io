import * as wasm from "./pkg"

function run() {
    const canvas = document.getElementById('canvas');

    // Ensure the canvas element exists
    if (!canvas) {
        throw new Error("Canvas element not found, ensure index.html has a canvas element");
    }

    // Initialize the WebGl context
    const screen = new wasm.Screen(canvas);

    screen.draw_triangle();

    // Resize the canvas when the window is resized
    window.addEventListener('resize', () => {
        screen.resize(window);
    });
}

// Entry Point
run();
