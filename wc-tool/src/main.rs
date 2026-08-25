use clap::Parser;
use wc_tool::wc::{count_bytes, count_chars, count_lines, count_words};

#[derive(Parser)]
struct Args {
    #[arg(short = 'c')]
    config: bool,

    #[arg(short = 'l')]
    lines: bool,

    #[arg(short = 'w')]
    words: bool,

    #[arg(short = 'm')]
    chars: bool,

    file: Option<String>,
}

fn main() {
    let args = Args::parse();

    if args.config {
        let result = count_bytes(args.file.as_deref()).unwrap();
        print_result(&[result], args.file.as_deref());
    } else if args.lines {
        let result = count_lines(args.file.as_deref()).unwrap();
        print_result(&[result], args.file.as_deref());
    } else if args.words {
        let result = count_words(args.file.as_deref()).unwrap();
        print_result(&[result], args.file.as_deref());
    } else if args.chars {
        let result = count_chars(args.file.as_deref()).unwrap();
        print_result(&[result], args.file.as_deref());
    } else {
        let total_bytes = count_bytes(args.file.as_deref()).unwrap();
        let total_lines = count_lines(args.file.as_deref()).unwrap();
        let total_words = count_words(args.file.as_deref()).unwrap();
        print_result(
            &[total_lines, total_words, total_bytes],
            args.file.as_deref(),
        );
    }
}

fn print_result(counts: &[usize], file: Option<&str>) {
    let mut total_result: String = String::new();
    for byte_count in counts.iter() {
        total_result.push_str(&format!("{:>8}", byte_count));
    }
    match file {
        Some(file_path) => println!("{} {}", total_result, file_path),
        None => println!("{}", total_result),
    }
}
