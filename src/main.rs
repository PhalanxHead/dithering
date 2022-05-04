use dithering::{run, Config};
use clap::Parser;
use std::process;

fn main() {
    let parsed_args = Config::parse();
    println!("{:?}", parsed_args);

    if let Err(e) = run(parsed_args) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }

    println!("Done!");
}
