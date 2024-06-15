use cfg_if::cfg_if;
use clap::{Arg, ArgAction, Command};
#[allow(unused_imports)]
use rusty_queens::Board;

cfg_if! {
    if #[cfg(feature = "build")] {
        const VERSION: &str = env!("CARGO_PKG_VERSION");
        const GIT_COMMIT_HASH: &str = env!("VERGEN_GIT_SHA");
        const BUILD_TIMESTAMP: &str = env!("VERGEN_BUILD_TIMESTAMP");
    } else {
        const VERSION: &str = "unknown";
        const GIT_COMMIT_HASH: &str = "unknown";
        const BUILD_TIMESTAMP: &str = "unknown";
    }
}

fn main() {
    let version = format!("{}-{}-{}", VERSION, GIT_COMMIT_HASH, BUILD_TIMESTAMP);
    let matches = Command::new("rusty_queens")
        .version(version)
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
    if verbose {
        println!("Verbose flag: {}", verbose);
    }
}
