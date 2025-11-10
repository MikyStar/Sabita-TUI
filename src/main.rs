use sabita_tui::{
    core::{
        cli::{parse_args, Args, StateParam, ACTION},
        constants::{PKG_NAME, PKG_VERSION},
        tui::run_tui,
    },
    view::instructions::{
        APP, CHANGE_DIFFICULTY, CHANGE_VALUE, CLEAR_VALUE, CYCLE, ESCAPE, FILLING, FULLSCREEN,
        MOVE, MOVING, NEW, RESET, SOLVE,
    },
};

////////////////////////////////////////

fn main() {
    let Args {
        action,
        state_param: StateParam {
            difficulty_level,
            full_screen,
        },
    } = parse_args();

    match action {
        ACTION::RunUi => run_tui(difficulty_level, full_screen.unwrap_or(false)).unwrap(),
        ACTION::Version => version(),
        ACTION::Help => {
            version();
            println!();
            help()
        }
    }
}

////////////////////

fn version() {
    println!("{PKG_NAME} v{PKG_VERSION}")
}

fn help() {
    println!("Prototype:");
    println!("           {PKG_NAME} [--difficulty=<1-5>] [--fullscreen] [--version] [--help]");
    println!();
    println!("Optional args:");
    println!("           --difficulty=<1-5> or -d=<1-5>     # Start with a specific difficulty (from 1 to 5)");
    println!("           -f or --fullscreen                 # Enter fullscreen from the launch (no keybings nor timer shown)");
    println!("           -v or --version                    # Show package version");
    println!("           -h or--help                        # Prints this help");
    println!();
    println!("Example:");
    println!("           {PKG_NAME}         # Just runs the app");
    println!("           {PKG_NAME} -d=3 -f # Runs the app with a starting difficulty of 3 and in fullscreen");
    println!();
    println!("In app bindings:");
    println!("           {FILLING}:");
    println!("              {CHANGE_VALUE}");
    println!("              {CLEAR_VALUE}");
    println!("           {MOVING}:");
    println!("              {MOVE}");
    println!("              {CYCLE}");
    println!("           {APP}:");
    println!("              {NEW}");
    println!("              {RESET}");
    println!("              {SOLVE}");
    println!("              {CHANGE_DIFFICULTY}");
    println!("              {FULLSCREEN}");
    println!("              {ESCAPE}");
}
