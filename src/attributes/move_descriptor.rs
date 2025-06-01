//! Represents a move made in a Lichess game. It holds Nag, suffix, and the whole characterization of the move. It is used to turn it into a u32 descriptor of a move with the following formatting:
//!
//! `player` (31st bit - 1 total) - 0 for the black player, 1 for the white player.
//! `moved_role` (30th-28th bits - 3 total) - The role of the moved piece.
//! `starting_square` (27th-22nd bits - 6 total) - The starting square of the move.
//! `captured_role` (21st-19th bits - 3 total) - The role of the captured piece. 7 for En-Passant.
//! `ending_square` (18th-13th bits - 6 total) - The ending square of the move, or the rook's square if it's a castling.
//! `promoted_role` (12th-10th bits - 3 total) - The role of the promoted piece. 6 and 7 for castling (King and Queen side, respectively).
//! `is_check` (9th bit - 1 total) - Whether it's check or not.
//! `is_checkmate` (8th bit - 1 total) - Whether it's checkmate or not.
//! `nag` (7th-0th bits - 8 total) - The value of the nag.

use pgn_reader::Nag;
use shakmaty::{
    Color,
    Move::{self, Castle, EnPassant, Normal, Put},
    Position,
    Role::{King, Pawn},
    Square,
    san::{
        SanPlus,
        Suffix::{self, Check, Checkmate},
    },
};

use crate::{attribute_err, attribute_fmt};

use super::error::{AttributeParsingError, ValuedAttributeParsingError};

/// Mask that indicates there has been a check.
const CHECK: u32 = 0b01 << 8;
/// Mask that indicates there has been a checkmate (check, and checkmate).
const CHECKMATE: u32 = 0b11 << 8;
/// Mask for indicating a pawn has moved.
const MOVED_PAWN: u32 = (Pawn as u32) << 28;
/// Mask for indicating a king has moved.
const MOVED_KING: u32 = (King as u32) << 28;
/// Mask for indicating an en-passant move.
const EN_PASSANT: u32 = 7 << 19;
/// Mask for indicating a castling move.
const CASTLING: u32 = (King as u32) << 10;

/// A move made in a Lichess game.
#[derive(Debug, Clone)]
pub struct MoveDescriptor {
    r#move: Move,
    suffix: Option<Suffix>,
    pub nag: Nag,
    pub color: Color,
}

impl Default for MoveDescriptor {
    fn default() -> Self {
        Self {
            r#move: Move::Normal {
                role: Pawn,
                from: Square::A1,
                capture: None,
                to: Square::A1,
                promotion: None,
            },
            suffix: None,
            nag: Nag(0),
            color: Color::Black,
        }
    }
}

impl MoveDescriptor {
    /// Creates a new move from the given san and the position it was played in.
    ///
    /// # Errors
    /// Will return [`ValuedAttributeParsingError`] if the given [`SanPlus`] is not valid in for this [`Position`] or if it results in a [`Move::Put`].
    pub fn from_san(
        san: &SanPlus,
        pos: &impl Position,
    ) -> Result<Self, ValuedAttributeParsingError> {
        match san.san.to_move(pos) {
            Err(_) | Ok(Put { role: _, to: _ }) => Err(
                ValuedAttributeParsingError::from_inner_utf8(ERROR, san.to_string()),
            ),
            Ok(r#move) => Ok(Self {
                r#move,
                suffix: san.suffix,
                nag: Nag(0),
                color: pos.turn(),
            }),
        }
    }

    /// Creates and plays new move from the given san and the position it was played in.
    ///
    /// # Errors
    /// Will return [`ValuedAttributeParsingError`] if the given [`SanPlus`] is not valid in for this [`Position`] or if it results in a [`Move::Put`].
    pub fn from_and_play_san(
        san: &SanPlus,
        pos: &mut impl Position,
    ) -> Result<Self, ValuedAttributeParsingError> {
        match san.san.to_move(pos) {
            Err(_) | Ok(Put { role: _, to: _ }) => Err(
                ValuedAttributeParsingError::from_inner_utf8(ERROR, san.to_string()),
            ),
            Ok(r#move) => {
                let color = pos.turn();
                pos.play_unchecked(&r#move);
                Ok(Self {
                    r#move,
                    suffix: san.suffix,
                    nag: Nag(0),
                    color,
                })
            }
        }
    }

    /// Creates a new move from the given shakmaty move and the position it was played in.
    ///
    /// # Errors
    /// Will return [`AttributeParsingError`] if the given [`Move`] is a [`Move::Put`].
    pub const fn from_move(
        r#move: Move,
        suffix: Option<Suffix>,
        color: Color,
    ) -> Result<Self, AttributeParsingError> {
        match r#move {
            Put { role: _, to: _ } => Err(ERROR),
            r#move => Ok(Self {
                r#move,
                suffix: suffix,
                nag: Nag(0),
                color,
            }),
        }
    }

    /// Generates a u32 representation of the value. It is structured in the following manner:
    /// `player` (31st bit - 1 total) - 0 for the black player, 1 for the white player.
    /// `moved_role` (30th-28th bits - 3 total) - The role of the moved piece.
    /// `starting_square` (27th-22nd bits - 6 total) - The starting square of the move.
    /// `captured_role` (21st-19th bits - 3 total) - The role of the captured piece. 7 for En-Passant.
    /// `ending_square` (18th-13th bits - 6 total) - The ending square of the move, or the rook's square if it's a castling.
    /// `promoted_role` (12th-10th bits - 3 total) - The role of the promoted piece. 6 and 7 for castling (King and Queen side, respectively).
    /// `is_check` (9th bit - 1 total) - Whether it's check or not.
    /// `is_checkmate` (8th bit - 1 total) - Whether it's checkmate or not.
    /// `nag` (7th-0th bits - 8 total) - The value of the nag.
    pub const fn to_u32(&self) -> u32 {
        let r#move = match self.r#move {
            Normal {
                role,
                from,
                capture,
                to,
                promotion,
            } => {
                let mut result =
                    ((role as u32) << 28) | ((from as u32) << 22) | ((to as u32) << 13);
                if let Some(capture) = capture {
                    result |= (capture as u32) << 19;
                }
                if let Some(promotion) = promotion {
                    result |= (promotion as u32) << 10;
                }
                result
            }
            EnPassant { from, to } => {
                MOVED_PAWN | ((from as u32) << 22) | EN_PASSANT | ((to as u32) << 13)
            }
            Castle { king, rook } => {
                MOVED_KING
                    | ((king as u32) << 22)
                    | ((rook as u32) << 13)
                    | CASTLING
                    | (rook as u32 & 0b000100) << 8
            }
            _ => 0,
        };
        let suffix = match self.suffix {
            Some(Check) => CHECK,
            Some(Checkmate) => CHECKMATE,
            None => 0,
        };
        ((self.color as u32) << 31) | r#move | suffix | (self.nag.0 as u32)
    }
}

attribute_fmt!(MoveDescriptor, "SAN notation");
attribute_err!(MoveDescriptor);

#[cfg(test)]
mod test {
    use pgn_reader::Nag;
    use pretty_assertions::assert_eq;
    use shakmaty::{Color, Move, Role, Square, san::Suffix};

    use crate::prelude::error::AttributeParsingError;

    use super::MoveDescriptor;

    /// Unwraps a move adding a nag to it, and turns it into its [`u32`] representation.
    const fn unwrap_move(r#move: Result<MoveDescriptor, AttributeParsingError>, nag: u8) -> u32 {
        if let Ok(mut r#move) = r#move {
            r#move.nag = Nag(nag);
            r#move.to_u32()
        } else {
            0
        }
    }

    /// Tests whether the Move::to_u32 actually works or not.
    #[test]
    pub fn move_to_u32_test() {
        // 0b00000000000000000000000000000000.
        const TEST_CASES: [(u32, u32); 4] = [
            (
                unwrap_move(
                    MoveDescriptor::from_move(
                        Move::Normal {
                            role: Role::Rook,
                            from: Square::B2,
                            capture: Some(Role::Knight),
                            to: Square::E5,
                            promotion: None,
                        },
                        Some(Suffix::Check),
                        Color::Black,
                    ),
                    0b00000000,
                ),
                0b01000010010101001000000100000000,
            ),
            (
                unwrap_move(
                    MoveDescriptor::from_move(
                        Move::EnPassant {
                            from: Square::C3,
                            to: Square::D4,
                        },
                        Some(Suffix::Checkmate),
                        Color::White,
                    ),
                    0b00000000,
                ),
                0b10010100101110110110001100000000,
            ),
            (
                unwrap_move(
                    MoveDescriptor::from_move(
                        Move::Castle {
                            king: Square::E1,
                            rook: Square::A1,
                        },
                        None,
                        Color::Black,
                    ),
                    0b00010111,
                ),
                0b01100001000000000001100000010111,
            ),
            (
                unwrap_move(
                    MoveDescriptor::from_move(
                        Move::Castle {
                            king: Square::E1,
                            rook: Square::H1,
                        },
                        None,
                        Color::White,
                    ),
                    0b00010111,
                ),
                0b11100001000000001111110000010111,
            ),
        ];
        for ((value, truth), i) in TEST_CASES.iter().zip(1..) {
            println!(
                "{i:02}: {value:032b}\n    {truth:032b}\n   ({:032b})",
                value ^ truth
            );
            assert_eq!(value, truth);
            assert_eq!(value ^ truth, 0);
        }
    }
}
