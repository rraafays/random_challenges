use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
    path::Path,
    process::exit,
};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Arguments {
    #[arg(index(1))]
    path: String,
}

fn main() {
    let arguments: Arguments = Arguments::parse();

    let file: Result<File, Error> = File::open(Path::new(&arguments.path));
    match file {
        Ok(contents) => {
            let count: usize = get_lines(&contents).len();
            println!("lines: {}", count)
        }
        Err(_) => {
            eprintln!("file not found!");
            exit(1);
        }
    };
}

fn get_lines(file: &File) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    let data: BufReader<&File> = BufReader::new(file);
    for line in data.lines() {
        match line {
            Ok(line) => lines.push(line),
            Err(_) => todo!(),
        }
    }
    lines
}
