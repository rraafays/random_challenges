use std::{
    convert::Infallible,
    env::{self, Args},
    fs::File,
    io::Error,
    path::Path,
};

fn main() {
    let args: Result<Args, Infallible> = Args::try_from(env::args().collect());
    let defined_args = match args {
        Ok(args) => args,
        Err(_) => panic!("no file provided"),
    };

    let file: Result<File, Error> =
        File::open(Path::new(&defined_args.into_iter()[0]));
    let data: File = match file {
        Ok(file) => {
            print!("found");
            file
        }
        Err(_) => panic!("no file found"),
    };
}
