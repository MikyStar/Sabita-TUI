use sabita_tui::core::{
    cli::{parse_args, ACTION},
    constants::{PKG_NAME, PKG_VERSION},
};

////////////////////////////////////////

fn main() {
    let action = parse_args();

    match action {
        ACTION::RunUi => {
            println!("todo");
        }
        ACTION::Version => {
            version();
        }
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
    // TODO
}
