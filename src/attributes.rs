use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub enum Eval {
    Numeric(f32),
    Checkmate(i8),
}

impl TryFrom<&str> for Eval {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.parse() {
            Ok(num) => Ok(Self::Numeric(num)),
            Err(e) => {
                if (value.len() > 1) & (value.as_bytes()[0] == b'#') {
                    match value[1..].parse() {
                        Ok(num) => Ok(Self::Checkmate(num)),
                        Err(e) => Err(format!("{}: {}", e, value)),
                    }
                } else {
                    Err(format!("{}: {}", e, value))
                }
            }
        }
    }
}

impl Display for Eval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Numeric(num) => write!(f, "{:.2}", num),
            Self::Checkmate(num) => write!(f, "#{num}"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TimeControl(Option<(u16, u8)>);

impl TimeControl {
    pub fn find_sep(time_control: &[u8]) -> Option<usize> {
        for (i, c) in time_control.iter().enumerate() {
            if *c == b'+' {
                if i+1 < time_control.len() {
                    return Some(i);
                } else {
                    return None;
                }
            }
        }
        None
    }
}

impl TryFrom<&str> for TimeControl {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value == "-" {
            return Ok(Self(None));
        }
        if let Some(sep) = Self::find_sep(value.as_bytes()) {
            match value[..sep].parse() {
                Ok(total) => {
                    match value[sep+1..].parse() {
                        Ok(increment) => {
                            Ok(Self(Some((total, increment))))
                        }
                        Err(e) => Err(format!("{}: {}", e, value)),
                    }
                },
                Err(e) => Err(format!("{}: {}", e, value)),
            }
        } else {
            Err(format!("Found invalid TimeControl: {}", value))
        }
    }
}

impl Display for TimeControl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            Some((total, increment)) => write!(f, "{total}+{increment}"),
            None => Ok(()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ResultAttr(Option<ResultInner>);

#[derive(Debug, Clone, Copy)]
#[repr(i8)]
pub enum ResultInner {
    White = 1,
    Black = -1,
    Tie = 0,
}

impl TryFrom<&str> for ResultAttr {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "1-0" => Ok(Self(Some(ResultInner::White))),
            "0-1" => Ok(Self(Some(ResultInner::Black))),
            "1/2-1/2" => Ok(Self(Some(ResultInner::Tie))),
            "*" => Ok(Self(None)),
            _ => Err(format!("Found invalid result: {}", value)),
        }
    }
}

impl Display for ResultAttr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            Some(value) => write!(f, "{}", value as i8),
            None => Ok(()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Termination {
    Normal,
    TimeForfeit,
    RulesInfraction,
    Abandoned,
    Unterminated,
}

impl TryFrom<&str> for Termination {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Normal" => Ok(Self::Normal),
            "Time forfeit" => Ok(Self::TimeForfeit),
            "Rules infraction" => Ok(Self::RulesInfraction),
            "Abandoned" => Ok(Self::Abandoned),
            "Unterminated" => Ok(Self::Unterminated),
            _ => Err(format!("Found invalid termination: {}", value)),
        }
    }
}

impl Display for Termination {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as u8)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Eco(Option<(char, u8)>);

impl TryFrom<&str> for Eco {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value == "?" {
            return Ok(Self(None));
        } else if (value.len() == 3) & (b'A'..b'[').contains(&value.as_bytes()[0]) {
            if let Ok(num) = value[1..=2].parse() {
                return Ok(Self(Some((value.chars().next().unwrap(), num))));
            }
        }
        Err(format!("Found invalid ECO: {}", value))
    }
}

impl Display for Eco {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some((letter, num)) = self.0 {
            return write!(f, "{}{}", letter, num);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Elo(Option<u16>);

impl TryFrom<&str> for Elo {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value == "?" {
            return Ok(Self(None));
        }
        match value.parse() {
            Ok(num) => {
                Ok(Self(Some(num)))
            }
            Err(e) => Err(format!("{}: {}", e, value)),
        }
    }
}

impl Display for Elo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            Some(value) => write!(f, "{}", value),
            None => Ok(()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Title {
    BOT,
    LM,
    GM,
    IM,
    FM,
    CM,
    NM,
    WGM,
    WIM,
    WFM,
    WCM,
    WNM,
    GR,
    MC,
    MN,
    M,
}

impl TryFrom<&str> for Title {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "BOT" => Ok(Self::BOT),
            "LM" => Ok(Self::LM),
            "GM" => Ok(Self::GM),
            "IM" => Ok(Self::IM),
            "FM" => Ok(Self::CM),
            "CM" => Ok(Self::CM),
            "NM" => Ok(Self::NM),
            "WGM" => Ok(Self::WGM),
            "WIM" => Ok(Self::WIM),
            "WFM" => Ok(Self::WFM),
            "WCM" => Ok(Self::WCM),
            "WNM" => Ok(Self::WNM),
            "ГР" => {println!("Found {value}"); Ok(Self::GR)},
            "MC" => {println!("Found {value}"); Ok(Self::MC)},
            "MN" => {println!("Found {value}"); Ok(Self::MN)},
            "M" => {println!("Found {value}"); Ok(Self::M)},
            _ => Err(format!("Found invalid result: {}", value)),
        }
    }
}

impl Display for Title {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as u8)
    }
}

#[derive(Debug, Clone)]
pub struct  Opening(Option<String>);

impl From<&str> for Opening {
    fn from(value: &str) -> Self {
        if value == "?" {
            return Self(None);
        } else {
            return Self(Some(value.to_owned()));
        }
    }
}

impl Display for Opening {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(opening) = &self.0 {
            write!(f, "{}", opening)
        } else {
            Ok(())
        }
    }
}

#[derive(Debug, Clone)]
pub enum RuleSet {
    GameMode(String),
    Tournament(String, Tournament, String)
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
                return Self::GameMode(split_iter.rev().collect::<Vec<&str>>().join(" "))
            } else if last_split.starts_with("https") {
                let mut peek_split = split_iter.peekable();
                let kind = match peek_split.next_if(|value| (*value == "tournament") | (*value == "swiss")) {
                    Some("tournament") => Tournament::Arena,
                    Some("swiss") => Tournament::Swiss,
                    _ => Tournament::NonSpecified,
                };
                return Self::Tournament(peek_split.rev().collect::<Vec<&str>>().join(" "), kind, last_split.split_terminator('/').rev().next().map(Into::into).unwrap_or_default());
            }
        }
        Self::Tournament(value.to_owned(), Tournament::NonSpecified, String::new())
    }
}

impl Display for RuleSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuleSet::GameMode(name) => write!(f, "{}", name),
            RuleSet::Tournament(name, kind, url_id) => write!(f, "{name}|{}|{url_id}", *kind as u8)
        }
    }
}
