use clap::Parser;
use cut_tool::parser::parse_lines;

#[derive(Parser)]
struct Args {
    #[arg(short = 'f')]
    cols: String,

    #[arg(short = 'd')]
    delimiter: Option<String>,

    file_path: String,
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let text = common::input_reader::read_string(Some(&args.file_path))?;
    let delimiter = args.delimiter.unwrap_or("\t".to_string());
    let cols: Vec<usize> = args
        .cols
        .split(|c: char| c == ',' || c.is_whitespace())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<usize>())
        .collect::<Result<_, _>>()?;
    let cols = parse_lines(&cols, &text, &delimiter)?;
    display(cols, &delimiter);
    Ok(())
}

fn display(cols: Vec<Vec<&str>>, delimiter: &str) {
    for fields in cols.iter() {
        let line = fields.join(delimiter);
        println!("{}", line);
    }
}
