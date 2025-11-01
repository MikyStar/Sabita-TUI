use core::fmt;
use std::env;

////////////////////////////////////////

#[derive(Debug)]
pub enum ACTION {
    RunUi,
    Help,
    Version,
}

impl fmt::Display for ACTION {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let action = match self {
            ACTION::RunUi => "RunUi",
            ACTION::Help => "Help",
            ACTION::Version => "Version",
        };

        write!(f, "{action}")
    }
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
