use core::fmt;
use std::env::args;

use regex::Regex;

use crate::core::difficulty::DIFFICULTY;

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

pub struct StateParam {
    pub difficulty_level: Option<DIFFICULTY>,
    pub full_screen: Option<bool>,
}

////////////////////////////////////////

pub struct Args {
    pub action: ACTION,
    pub state_param: StateParam,
}

////////////////////////////////////////

pub fn parse_args() -> Args {
    let mut cli_args: Vec<String> = args().collect();
    cli_args.remove(0);
    let cli_phrase = cli_args.join(" ");

    // Version
    let is_version = Regex::new(r"(^|\s)(--version|-v)($|\s)").unwrap();
    if is_version.is_match(&cli_phrase) {
        return Args {
            action: ACTION::Version,
            state_param: StateParam {
                difficulty_level: None,
                full_screen: None,
            },
        };
    }

    // Help
    let is_help = Regex::new(r"(^|\s)(--help|-h)($|\s)").unwrap();
    if is_help.is_match(&cli_phrase) {
        return help();
    }

    // Optional state args
    let full_screen_regex = Regex::new(r"(^|\s)(--fullscreen|-f)($|\s)").unwrap();
    let full_screen = full_screen_regex.is_match(&cli_phrase).then_some(true);

    let difficulty_level_regex = Regex::new(r"(^|\s)(?:--difficulty|-d)=([1-5])($|\s)").unwrap();
    let difficulty_level = difficulty_level_regex
        .captures(&cli_phrase)
        .and_then(|caps| {
            let level = caps[2].parse::<u8>().unwrap();

            Some(DIFFICULTY::from(level - 1))
        });

    println!("{:?}", difficulty_level_regex.captures(&cli_phrase));

    return Args {
        action: ACTION::RunUi,
        state_param: StateParam {
            difficulty_level,
            full_screen,
        },
    };
}

////////////////////

fn help() -> Args {
    Args {
        action: ACTION::Help,
        state_param: StateParam {
            difficulty_level: None,
            full_screen: None,
        },
    }
}
