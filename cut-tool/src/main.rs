use clap::Parser;
use cut_tool::parser::parse_lines;

#[derive(Parser)]
struct Args {
    #[arg(short = 'f')]
    col: usize,
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let text = common::input_reader::read_string(Some("tests/sample.tsv"))?;
    let col = parse_lines(args.col, &text)?;

    for val in col.iter() {
        println!("{}", val);
    }

    Ok(())
}
