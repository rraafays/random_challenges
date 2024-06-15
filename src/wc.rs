use std::{
    fs::File,
    io::{
        BufRead,
        BufReader,
    },
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

    let path: &Path = Path::new(&arguments.path);
    let file: File = match File::open(path) {
        Ok(contents) => contents,
        Err(_) => {
            println!(
                "wc: {}: No such file or directory",
                path.to_str().expect("error")
            );
            exit(1);
        }
    };

    let lines = get_lines(&file);
    println!(
        "{} {} {} {}",
        lines.len(),
        count_words(&lines),
        file.metadata().unwrap().len(),
        path.to_str().expect("")
    );
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

fn count_words(lines: &Vec<String>) -> usize {
    let mut words = 0;
    for line in lines {
        words += line.split_whitespace().count();
    }
    return words;
}
