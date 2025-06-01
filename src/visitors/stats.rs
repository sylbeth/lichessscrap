//! Stats visitor for a PGN reader. Used for analyzing how much data there is in a given PGN file.

use log::info;
use pgn_reader::{Nag, RawComment, RawHeader, SanPlus, Visitor};
use shakmaty::Outcome;

#[derive(Debug, Default)]
/// Stats visitor for a PGN reader.
pub struct Stats {
    /// Number of games in the PGN file.
    pub games: usize,
    /// Number of headers in the PGN file.
    pub headers: usize,
    /// Number of SANs in the PGN file.
    pub sans: usize,
    /// Number of NAGs in the PGN file.
    pub nags: usize,
    /// Number of comments in the PGN file.
    pub comments: usize,
    /// Number of variations in the PGN file.
    pub variations: usize,
    /// Number of outcomes in the PGN file.
    pub outcomes: usize,
}

impl Stats {
    /// Logs the PGN files stats.
    #[allow(dead_code)]
    pub fn log(&self) {
        info!("File stats");
        info!("Games: {}", self.games);
        info!("Headers: {}", self.headers);
        info!("SANs: {}", self.sans);
        info!("NAGs: {}", self.nags);
        info!("Comments: {}", self.comments);
        info!("Variations: {}", self.variations);
        info!("Outcomes: {}", self.outcomes);
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
        if self.games % 1000000 == 0 {
            info!("Registered stats of {} games.", self.games);
        }
    }
}
