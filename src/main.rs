use clap::{value_parser, Arg, ArgAction, Command};
use env_logger;
use log::debug;
use rusty_queens::{Board, Config};

fn parse_args() -> Config {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    const GIT_COMMIT_HASH: &str = env!("VERGEN_GIT_DESCRIBE");
    const GIT_BRANCH: &str = env!("VERGEN_GIT_BRANCH");
    const RUSTC_HOST_TRIPLE: &str = env!("VERGEN_RUSTC_HOST_TRIPLE");
    const VERGEN_BUILD_TIMESTAMP: &str = env!("VERGEN_BUILD_TIMESTAMP");

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
            Arg::new("quiet")
                .short('q')
                .long("quiet")
                .action(ArgAction::SetTrue)
                .help("whine less"),
        )
        .arg(
            Arg::new("num")
                .short('n')
                .long("num")
                .help("size of the board")
                .value_parser(value_parser!(usize))
                .default_value("8"),
        )
        .get_matches();

    let config = Config {
        quiet: matches.get_flag("quiet"),
        num: *matches.get_one("num").unwrap(),
    };

    config
}

fn main() {
    env_logger::init();
    debug!("launching");

    debug!("processing command line arguments");
    let config = parse_args();

    debug!("creating new board");
    let mut board = Board::new(config);

    debug!("calling print_all_solutions()");
    board.print_all_solutions();
}
