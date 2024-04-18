/*
Optimize for the following:
    1. Minimizing copying into and out of the WebAssembly linear memory
    2. Minimizing serializing and deserializing
*/

mod webgl;
mod app;
mod utils;
mod consts;