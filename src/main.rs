mod scanner;
mod token;

use std::{
    env, fs,
    io::{stdin, stdout, Write},
};

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
        print!("> ");
        stdout().flush()?;
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
    let mut sc = scanner::Scanner::new(program);
    for t in sc.scan_tokens()?.iter() {
        println!("{}", t);
    }
    Ok(())
}
