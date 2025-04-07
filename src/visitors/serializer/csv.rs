use std::fs::File;

use csv::Writer;

use super::LichessSerializer;

use super::{DataSerializer, GAMES_CSV, MOVES_CSV};
use lichess::data::{game::Game, r#move::Move};

#[derive(Debug)]
pub struct CSVSerializer {
    games: Writer<File>,
    moves: Writer<File>,
}

impl LichessSerializer for CSVSerializer {
    fn new() -> Self {
        let mut csv = Self {
            games: Writer::from_path(GAMES_CSV).expect("The creation of the games csv failed."),
            moves: Writer::from_path(MOVES_CSV).expect("The creation of the moves csv failed."),
        };
        csv.games
            .write_record(&[
                "GameId",
                "Site",
                "TimeControl",
                "Result",
                "Termination",
                "Date",
                "UTCDate",
                "UTCTime",
                "Opening",
                "ECO",
                "Event",
                "Round",
                "White",
                "WhiteElo",
                "WhiteTitle",
                "WhiteRatingDiff",
                "Black",
                "BlackElo",
                "BlackTitle",
                "BlackRatingDiff",
            ])
            .expect("The writing of the header of the games csv failed.");
        csv.moves
            .write_record(&["GameId", "Num", "San", "Nag", "Eval", "Clk"])
            .expect("The writing of the header of the moves csv failed.");
        csv
    }
}

impl DataSerializer for CSVSerializer {
    fn write_game(&mut self, game: &Game) {
        self.games
            .write_record(&[
                &format!("{}", game.game_id),
                &game.site,
                &game.time_control,
                &game.result,
                &game.termination,
                &game.date,
                &game.utc_date,
                &game.utc_time,
                &game.opening,
                &game.eco,
                &game.event,
                &game.round,
                &game.white,
                &game.white_elo,
                &game.white_rating_diff,
                &game.white_title,
                &game.black,
                &game.black_elo,
                &game.black_rating_diff,
                &game.black_title,
            ])
            .unwrap();
    }

    fn write_move(&mut self, r#move: &Move) {
        self.moves
            .write_record(&[
                &format!("{}", r#move.game_id),
                &format!("{}", r#move.num),
                &r#move.san,
                &r#move.nag.map(|nag| format!("{}", nag)).unwrap_or_default(),
                &r#move.eval,
                &r#move.clk,
            ])
            .expect("The writing of the moves csv failed.");
    }
}
