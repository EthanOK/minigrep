use std::{env, process};

use minigrep::Config;

fn main() {
    // 在程序中读取传入的参数
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else({
        |err| {
            eprintln!("Problem parsing arguments: {}", err);
            process::exit(1);
        }
    });

    print!("Searching for {:?} ", config.query);
    println!("In file {:?}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    };
}
