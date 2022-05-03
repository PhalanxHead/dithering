use dithering::run;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Err(e) = run(&args) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }

    println!("Done!");
}
