use std::{
    env::{self},
    process,
};

use minigrep::Config;

// cargo run -- searchstring example-filename.txt

fn main() {
    // Reading arguments from the command line
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    ///////////////////////////////////////////////////

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
