use std::fs::read_to_string;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "gf",
    about = "A markdown-to-groff transpiler for typesetting efficiently"
)]
struct Args {
    /// Input file
    #[structopt(parse(from_os_str))]
    input: PathBuf,

    /// Output file, stdout if not present
    #[structopt(parse(from_os_str))]
    output: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    color_backtrace::install();
    let args = Args::from_args();
    let input = read_to_string(args.input)?;
    let output = parser::parse(&input);
    match args.output {
        Some(path) => {
            let mut file = File::create(path)?;
            file.write_fmt(format_args!("{:#?}", output))?;
        }
        None => println!("{:#?}", output),
    }
    Ok(())
}
