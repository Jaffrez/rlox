fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        println!("Usage: rlox [script]");
    } else if args.len() == 1 {
        run_file(&args[1]);
    } else {
        run_prompt()
    }
}

fn run_file(path: &str) {
    unimplemented!()
}

fn run_prompt() {
    unimplemented!()
}
