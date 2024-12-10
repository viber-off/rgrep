use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(value_name = "QUERY")]
    pub query: String,

    #[arg(value_name = "PATH")]
    pub path: String,

    #[arg(
        short = 'i',
        long = "ignore-case",
        default_value_t = false,
        help = "Perform case-insensitive matching."
    )]
    pub ignore_case: bool,

    #[arg(
        short = 'l',
        long = "line-numbers",
        default_value_t = false,
        help = "Display line numbers."
    )]
    pub line_numbers: bool,
}
