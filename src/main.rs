mod scanner;

use anyhow::{Ok, Result};
use clap::{Arg, ArgAction, Command};
use log::info;
use std::{
    fs,
    io::{self, Write},
    process::exit,
};

use crate::scanner::Scanner;

fn main() -> Result<()> {
    env_logger::init();
    info!(
        "{} {}v loading...",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );

    let arg = parse_arg()?;

    match arg {
        Operation::REPL => {
            run_repl()?;
        }
        Operation::Interpreter(s) => {
            run_file(&s)?;
        }
    }

    Ok(())
}

fn run_repl() -> Result<()> {
    let mut buf = String::new();
    loop {
        print!(">>>");
        io::stdout().flush()?;
        buf.clear();
        io::stdin().read_line(&mut buf)?;
        let line = buf.trim();
        if line.is_empty() {
            break;
        } else {
            run(&line);
        }
    }
    Ok(())
}

fn run_file(script: &str) -> Result<()> {
    run(&fs::read_to_string(script)?);
    Ok(())
}

fn run(source: &str) {
    let mut scanner = Scanner::new(source);
    let mut error_flag = false;

    scanner.scan(&mut error_flag);

    // if code contains error, just exit
    if error_flag {
        exit(1);
    }

    scanner.tokens().iter().for_each(|s| println!("{:#?}", s));
}

fn parse_arg() -> Result<Operation> {
    let app = Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg_required_else_help(true)
        .arg(
            Arg::new("script")
                .help("file path")
                .required_unless_present("repl"),
        )
        .arg(
            Arg::new("repl")
                .long("repl")
                .help("enter REPL")
                .action(ArgAction::SetTrue),
        );
    let matches = app.get_matches();

    let script = matches.get_one::<String>("script");

    if let Some(s) = script {
        return Ok(Operation::Interpreter(s.clone()));
    } else {
        return Ok(Operation::REPL);
    }
}

enum Operation {
    REPL,
    Interpreter(String),
}
