use std::fmt::Display;

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
