mod core;
mod utils;

use std::{
    env, 
    process
};

use crate::core::engine::Engine;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Incorrect usage. Example: shadowjs <input.js> <output.js>");
        process::exit(1);
    }

    let input_file = &args[1];
    let output_file = &args[2];

    let _ = Engine::new(input_file, output_file).run();
}
