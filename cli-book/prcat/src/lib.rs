use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(default_value = "-")]
    files: Vec<String>,

    /// Number lines
    #[arg(short = 'n', conflicts_with = "number_nonblank_lines")]
    number_lines: bool,

    /// Number nonblank lines
    #[arg(short = 'b')]
    number_nonblank_lines: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run() -> MyResult<()> {
    let config = get_args()?;

    for filename in config.files {
        let file = open(&filename)?;
        let mut n_lines = 0;
        for l in file.lines() {
            let line = l?;
            if (config.number_nonblank_lines && !line.is_empty()) || config.number_lines {
                n_lines += 1;
            }
            if config.number_lines || (config.number_nonblank_lines && !line.is_empty()) {
                println!("{n_lines} {line}");
            }
            else {
                println!("{line}");
            }
        }
    }

    Ok(())
}

fn get_args() -> MyResult<Args> {
    let args = Args::parse();
    Ok(args)
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}