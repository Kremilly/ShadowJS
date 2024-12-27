mod core;
mod utils;

use crate::core::engine::Engine;

fn main() {
    let input_file = "input.js";
    let output_file = "output.min.js";

    let _ = Engine::new(input_file, output_file).run();
}
