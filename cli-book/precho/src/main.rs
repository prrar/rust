use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short)]
    no_new_line: bool,

    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    text: Vec<String>,
}

fn main() {
    let args = Args::parse();
    echo(&args);
    println!("{:?}", args.text);
}

fn echo(args: &Args) {
    print!("{}{}", args.text.join(" "), if args.no_new_line { "" } else { "\n" });
}