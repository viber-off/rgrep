use std::process;

use clap::Parser;
use cli::Cli;
use highlight::highlight;
use regex::Regex;
use search::{has_match, SearchResult};

mod cli;
mod file;
mod highlight;
mod search;

fn main() {
    let cli = Cli::parse();

    let lines = file::read_file(&cli.path);

    let pattern = if cli.ignore_case {
        format!("(?i){}", cli.query)
    } else {
        cli.query.clone()
    };

    let re = Regex::new(&pattern);

    match re {
        Err(err) => {
            eprintln!("Invalid regex expression: {}", err);
            process::exit(1);
        }
        Ok(re) => match lines {
            Ok(lines) => {
                for (i, line) in lines.iter().enumerate() {
                    match has_match(line, &re, cli.ignore_case) {
                        SearchResult::Match(pos) => {
                            if cli.line_numbers {
                                println!("{}: {}", i + 1, highlight(line, pos));
                            } else {
                                println!("{}", highlight(line, pos));
                            }
                        }
                        SearchResult::NoMatch => {}
                    }
                }
            }
            Err(e) => {
                eprintln!("{}", e);
                process::exit(1);
            }
        },
    }
}
