use std::env;

////////////////////////////////////////

#[derive(Debug)]
pub enum ACTION {
    RunUi,
    Help,
    Version,
}

////////////////////////////////////////

pub fn parse_args() -> ACTION {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return ACTION::RunUi;
    }

    match args[1].as_str() {
        "-v" | "--version" => ACTION::Version,
        _ => ACTION::Help,
    }
}
