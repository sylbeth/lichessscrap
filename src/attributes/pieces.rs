//! Represents a position in a Lichess game. It holds Nag, suffix, and the whole characterization of the move.

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
pub struct PiecesLeft {
    pub black: u16,
    pub white: u16,
}

impl PiecesLeft {
    /// Finds the number of pieces left on the board for each [`Color`](shakmaty::Color).
    pub fn from_board(board: Board) -> Result<PiecesLeft, ValuedAttributeParsingError> {
        let (black_bitboard, white_bitboard) = (board.black(), board.white());
        let mut pieces_left = PiecesLeft::default();
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
                pieces_left.black |= (black_pieces as u16) << displacement;
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
                pieces_left.white |= (white_pieces as u16) << displacement;
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
        Ok(pieces_left)
    }
}

attribute_fmt!(
    PiecesLeft,
    "3 bits for each piece and color, except the pawns, which need 4 bits."
);
attribute_err!(PiecesLeft);
