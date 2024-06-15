// use cfg_if::cfg_if;
use clap::{Arg, ArgAction, Command};
#[allow(unused_imports)]
use rusty_queens::Board;

// cfg_if! {
//     if #[cfg(feature = "build")] {
//         const VERSION: &str = env!("CARGO_PKG_VERSION");
//         const GIT_COMMIT_HASH: &str = env!("VERGEN_GIT_SHA");
//         const BUILD_TIMESTAMP: &str = env!("VERGEN_BUILD_TIMESTAMP");
//     } else {
//         const VERSION: &str = "unknown";
//         const GIT_COMMIT_HASH: &str = "unknown";
//         const BUILD_TIMESTAMP: &str = "unknown";
//     }
// }

const VERSION: &str = env!("CARGO_PKG_VERSION");
const GIT_COMMIT_HASH: &str = env!("VERGEN_GIT_DESCRIBE");
const GIT_BRANCH: &str = env!("VERGEN_GIT_BRANCH");
const RUSTC_HOST_TRIPLE: &str = env!("VERGEN_RUSTC_HOST_TRIPLE");
const VERGEN_BUILD_TIMESTAMP: &str = env!("VERGEN_BUILD_TIMESTAMP");

fn main() {
    let version = format!(
        "v{}-{}-{} (build {} {})",
        VERSION, GIT_BRANCH, GIT_COMMIT_HASH, RUSTC_HOST_TRIPLE, VERGEN_BUILD_TIMESTAMP
    );
    let version_str: &'static str = Box::leak(version.into_boxed_str());

    let matches = Command::new("rusty_queens")
        .version(version_str)
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
