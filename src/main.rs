use clap::{Arg, ArgAction, Command};
#[allow(unused_imports)]
use rusty_queens::Board;

fn main() {
    let matches = Command::new("rusty_queens")
        .version("0.1.0")
        .author("Matthew Gianni <matt@gianni.org>")
        .about("learning Rust by example")
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .action(ArgAction::SetTrue)
                .help("whine more"),
        )
        .get_matches();

    // Get the value of the "input" argument
    let verbose = matches.get_flag("verbose");
    println!("Verbose flag: {}", verbose);
}
