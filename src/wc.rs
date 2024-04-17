use std::{fs::File, io::Error, path::Path, process::exit};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Arguments {
    #[arg(index(1))]
    path: String,
}

fn main() {
    let arguments = Arguments::parse();

    let file: Result<File, Error> = File::open(Path::new(&arguments.path));
    let _data: File = match file {
        Ok(file) => {
            print!("found");
            file
        }
        Err(_) => {
            eprintln!("file not found!");
            exit(1);
        }
    };
}
