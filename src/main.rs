mod ui;
mod core;
mod utils;
mod constants;

use std::{
    env, 
    process
};

use crate::{
    ui::ui_base::UI,
    core::engine::Engine,
};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Incorrect usage. Example: shadowjs <input.js> <output.js>");
        process::exit(1);
    }

    let input_file = &args[1];
    let output_file = &args[2];

    UI::header();
    UI::section_header("Obfuscating JavaScript", "info");

    let _ = Engine::new(input_file, output_file).run();
}
