use std::{env, fs, io::stdin};

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
    let mut line: String = String::new();
    loop {
        match stdin().read_line(&mut line) {
            Ok(_) => {
                let line = std::mem::take(&mut line);
                run(&line)
            }
            Err(e) => Err(e)?,
        }?
    }
}

fn run_file(fname: &String) -> anyhow::Result<()> {
    run(&fs::read_to_string(fname)?)
}

fn run(program: &String) -> anyhow::Result<()> {
    todo!("take the program and run it")
}
