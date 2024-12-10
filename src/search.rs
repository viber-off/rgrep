use regex::Regex;

pub fn has_match(line: &str, query: &Regex, ignore_case: bool) -> SearchResult {
    if ignore_case {
        match query.find(line.to_lowercase().as_str()) {
            Some(m) => SearchResult::Match(Position {
                start: m.start(),
                end: m.end(),
            }),
            None => SearchResult::NoMatch,
        }
    } else {
        match query.find(line) {
            Some(m) => SearchResult::Match(Position {
                start: m.start(),
                end: m.end(),
            }),
            None => SearchResult::NoMatch,
        }
    }
}

pub enum SearchResult {
    Match(Position),
    NoMatch,
}

#[derive(Debug)]
pub struct Position {
    pub start: usize,
    pub end: usize,
}
