use std::fmt::Display;

use pgn_reader::{Nag, RawComment, RawHeader, SanPlus, Visitor};
use shakmaty::Outcome;

#[derive(Debug, Default)]
pub struct Stats {
    pub games: usize,
    pub headers: usize,
    pub sans: usize,
    pub nags: usize,
    pub comments: usize,
    pub variations: usize,
    pub outcomes: usize,
}

impl Display for Stats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "File stats:\n")?;
        writeln!(f, "Games: {}", self.games)?;
        writeln!(f, "Headers: {}", self.headers)?;
        writeln!(f, "SANs: {}", self.sans)?;
        writeln!(f, "NAGs: {}", self.nags)?;
        writeln!(f, "Comments: {}", self.comments)?;
        writeln!(f, "Variations: {}", self.variations)?;
        writeln!(f, "Outcomes: {}", self.outcomes)
    }
}

impl Visitor for Stats {
    type Result = ();

    fn header(&mut self, _key: &[u8], _value: RawHeader<'_>) {
        self.headers += 1;
    }

    fn san(&mut self, _san: SanPlus) {
        self.sans += 1;
    }

    fn nag(&mut self, _nag: Nag) {
        self.nags += 1;
    }

    fn comment(&mut self, _comment: RawComment<'_>) {
        self.comments += 1;
    }

    fn end_variation(&mut self) {
        self.variations += 1;
    }

    fn outcome(&mut self, _outcome: Option<Outcome>) {
        self.outcomes += 1;
    }

    fn end_game(&mut self) {
        self.games += 1;
    }
}
