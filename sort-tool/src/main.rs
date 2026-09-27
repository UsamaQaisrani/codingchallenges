use clap::Parser;
use sort_tool::sort;
use std::io::Write;

#[derive(clap::Parser)]
struct Args {
    file_path: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = std::io::stdout().lock();
    let args = Args::parse();
    let words = sort::sort(args.file_path.as_deref())?;
    for word in words {
        match writeln!(stdout, "{}", word) {
            Ok(()) => {}
            Err(e) if e.kind() == std::io::ErrorKind::BrokenPipe => return Ok(()),
            Err(e) => return Err(e.into()),
        }
    }
    Ok(())
}
