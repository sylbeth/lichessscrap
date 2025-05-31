//! Represents a board configuration in a Lichess game. It holds the number of pieces left for each color and the board position.

use mysql::{Params, params};
use shakmaty::{Board, Role};

use crate::{attribute_err, attribute_fmt};

use super::error::ValuedAttributeParsingError;

/// Data used for each role for codifing the number of pieces left.
const ROLE_DATA: [(u16, usize, Role); 5] = [
    (12, 8, Role::Pawn),
    (9, 7, Role::Knight),
    (6, 7, Role::Bishop),
    (3, 7, Role::Rook),
    (0, 7, Role::Queen),
];

/// Codified number of pieces left on the board.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BoardConfiguration {
    pub black_left: u16,
    pub white_left: u16,
    pub pawns: u64,
    pub knights: u64,
    pub bishops: u64,
    pub rooks: u64,
    pub queens: u64,
    pub kings: u64,
    pub whites: u64,
    pub blacks: u64,
}

impl BoardConfiguration {
    /// Constructs a [`BoardConfiguration`] based on a [`Board`] representation.
    pub fn from_board(board: &Board) -> Result<BoardConfiguration, ValuedAttributeParsingError> {
        let (black_bitboard, white_bitboard) = (board.black(), board.white());
        let mut pieces_left = BoardConfiguration::default();
        for (pieces, (displacement, max, role)) in [
            board.pawns(),
            board.knights(),
            board.bishops(),
            board.rooks(),
            board.queens(),
        ]
        .into_iter()
        .zip(ROLE_DATA)
        {
            let black_pieces = pieces.intersect(black_bitboard).count();
            if black_pieces <= max {
                pieces_left.black_left |= (black_pieces as u16) << displacement;
            } else {
                return Err(ValuedAttributeParsingError::from_inner_utf8(
                    ERROR,
                    format!(
                        "The number of black {}s is {black_pieces}, when it should be at most {max}.",
                        role.upper_char()
                    ),
                ));
            }
            let white_pieces = pieces.intersect(white_bitboard).count();
            if white_pieces <= max {
                pieces_left.white_left |= (white_pieces as u16) << displacement;
            } else {
                return Err(ValuedAttributeParsingError::from_inner_utf8(
                    ERROR,
                    format!(
                        "The number of white {}s is {white_pieces}, when it should be at most {max}.",
                        role.upper_char()
                    ),
                ));
            }
        }
        pieces_left.pawns = board.pawns().0;
        pieces_left.knights = board.knights().0;
        pieces_left.bishops = board.bishops().0;
        pieces_left.rooks = board.rooks().0;
        pieces_left.queens = board.queens().0;
        pieces_left.kings = board.kings().0;
        pieces_left.blacks = board.black().0;
        pieces_left.whites = board.white().0;
        Ok(pieces_left)
    }

    /// Prepares the parameters for MySQL insertion and selection of this data.
    pub fn as_params(&self) -> Params {
        params! {
            "black_pieces" => self.black_left,
            "white_pieces" => self.white_left,
            "pawns" => self.pawns,
            "knights" => self.knights,
            "bishops" => self.bishops,
            "rooks" => self.rooks,
            "queens" => self.queens,
            "kings" => self.kings,
            "black" => self.blacks,
            "white" => self.whites,
        }
    }
}

attribute_fmt!(
    BoardConfiguration,
    "left pieces (3 bits for each piece and color, except the pawns, which need 4 bits) rows (8 rows, one of 32 bits, four bits for each square, the first bit being whether it's a white or a black piece and the following three coinciding with the value of shakmaty's Role enum)"
);
attribute_err!(BoardConfiguration);
