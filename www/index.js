import * as wasm from "./pkg"

function run() {
    const terminal = new wasm.Terminal(window);

    // Example commands
    console.log(terminal.handle_command("hello"));
    console.log(terminal.handle_command("time"));

    wasm.message("In construction! Something amazing is in progress, please wait ...");
}

run();
