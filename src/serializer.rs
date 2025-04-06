use std::{fs::File, io::Write};

use crate::{game::Game, r#move::Move};

#[derive(Debug)]
pub struct Serializer {
    pub games: File,
    pub moves: File,
}

impl Serializer {
    pub fn write_game(&mut self, game: &Game) -> Result<(), std::io::Error> {
        self.games.write(game.id.to_string().as_bytes())?;
        self.games.write(b",")?;
        self.games.write(game.eco.to_string().as_bytes())?;
        self.games.write(b",")?;
        self.games.write(game.event.to_string().as_bytes())?;
        self.games.write(b",")?;
        self.games.write(game.opening.to_string().as_bytes())?;
        self.games.write(b",")?;
        self.games.write(game.result.to_string().as_bytes())?;
        self.games.write(b",")?;
        self.games.write(game.site.to_string().as_bytes())?;
        self.games.write(b",")?;
        self.games.write(game.termination.to_string().as_bytes())?;
        self.games.write(b",")?;
        self.games.write(game.time_control.to_string().as_bytes())?;
        self.games.write(b",")?;
        self.games.write(game.utc_date.to_string().as_bytes())?;
        self.games.write(b",")?;
        self.games.write(game.utc_time.to_string().as_bytes())?;
        self.games.write(b",")?;
        self.games.write(game.white.to_string().as_bytes())?;
        self.games.write(b",")?;
        self.games.write(game.white_elo.to_string().as_bytes())?;
        self.games.write(b",")?;
        self.games.write(game.white_rating_diff.to_string().as_bytes())?;
        self.games.write(b",")?;
        self.games.write(game.white_title.to_string().as_bytes())?;
        self.games.write(b",")?;
        self.games.write(game.black.to_string().as_bytes())?;
        self.games.write(b",")?;
        self.games.write(game.black_elo.to_string().as_bytes())?;
        self.games.write(b",")?;
        self.games.write(game.black_rating_diff.to_string().as_bytes())?;
        self.games.write(b",")?;
        self.games.write(game.black_title.to_string().as_bytes())?;
        self.games.write(b"\n")?;
        Ok(())
    }

    pub fn write_move(&mut self, r#move: &Move) -> Result<(), std::io::Error> {
        self.moves.write(r#move.game_id.to_string().as_bytes())?;
        self.moves.write(b",")?;
        self.moves.write(r#move.num.to_string().as_bytes())?;
        self.moves.write(b",")?;
        self.moves.write(r#move.san.as_bytes())?;
        self.moves.write(b",")?;
        if let Some(nag) = r#move.nag {
            self.moves.write(nag.to_string().as_bytes())?;
        }
        self.moves.write(b",")?;
        self.moves.write(r#move.clk.as_bytes())?;
        self.moves.write(b",")?;
        self.moves.write(r#move.eval.as_bytes())?;
        self.moves.write(b"\n")?;
        Ok(())
    }
}
