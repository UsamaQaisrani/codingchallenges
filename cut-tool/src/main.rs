use clap::Parser;
use cut_tool::parser::parse_lines;

#[derive(Parser)]
struct Args {
    #[arg(short = 'f')]
    cols: String,

    #[arg(short = 'd')]
    delimiter: Option<String>,

    file_path: Option<String>,
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let file_path = args.file_path.filter(|p| p != "-");
    let text = common::input_reader::read_string(file_path.as_deref())?;
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
