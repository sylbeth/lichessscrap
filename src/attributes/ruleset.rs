use std::fmt::Display;

use super::AttributeFormat;

pub const FORMAT: AttributeFormat = AttributeFormat::Str("{str} <game|tournament|swiss>[ {url}]");

#[derive(Debug, Clone)]
pub enum RuleSet {
    GameMode(String),
    Tournament(String, Tournament, String),
}

#[derive(Debug, Default, Clone, Copy)]
#[repr(u8)]
pub enum Tournament {
    #[default]
    NonSpecified,
    Arena,
    Swiss,
}

impl From<&str> for RuleSet {
    fn from(value: &str) -> Self {
        let mut split_iter = value.split_terminator(' ').rev();
        if let Some(last_split) = split_iter.next() {
            if last_split == "game" {
                return Self::GameMode(split_iter.rev().collect::<Vec<&str>>().join(" "));
            } else if last_split.starts_with("https") {
                let mut peek_split = split_iter.peekable();
                let kind = match peek_split
                    .next_if(|value| (*value == "tournament") | (*value == "swiss"))
                {
                    Some("tournament") => Tournament::Arena,
                    Some("swiss") => Tournament::Swiss,
                    _ => Tournament::NonSpecified,
                };
                return Self::Tournament(
                    peek_split.rev().collect::<Vec<&str>>().join(" "),
                    kind,
                    last_split
                        .split_terminator('/')
                        .rev()
                        .next()
                        .map(Into::into)
                        .unwrap_or_default(),
                );
            }
        }
        Self::Tournament(value.to_owned(), Tournament::NonSpecified, String::new())
    }
}

impl Display for RuleSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuleSet::GameMode(name) => name.fmt(f),
            RuleSet::Tournament(name, kind, url_id) => write!(f, "{name}|{}|{url_id}", *kind as u8),
        }
    }
}
