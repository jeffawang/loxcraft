use std::env;

use anyhow::bail;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => run_prompt(),
        2 => run_file(&args[1]),
        _ => bail!(format!("Too many args!\nusage: {} [lox_file]", &args[0])),
    }
}

fn run_prompt() -> anyhow::Result<()> {
    todo!("Reads and interprets from stdin")
}

fn run_file(_fname: &String) -> anyhow::Result<()> {
    todo!("Reads and interprets lox from the passed in file")
}
