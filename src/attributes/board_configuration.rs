//! Represents a board configuration in a Lichess game. It holds the number of pieces left for each color as u16s and the board position as u32s for each row with the following formatting:
//!
//! # `pieces_left`
//! - `pawns` (15th-12th bits - 4 total) - Number of pawns left as a u4.
//! - `knights` (11th-9th bits - 3 total) - Number of knights left as a u3, it can fail if there are more than 7 knights for one color.
//! - `bishops` (8th-6th bits - 3 total) - Number of bishops left as a u3, it can fail if there are more than 7 bishops for one color.
//! - `rooks` (5th-3rd bits - 3 total) - Number of rooks left as a u3, it can fail if there are more than 7 rooks for one color.
//! - `queens` (2nd-0th bits - 3 total) - Number of queens left as a u3, it can fail if there are more than 7 queens for one color.
//!
//! # `row`
//! - `square` ((0..32).step_by(4) bits - 4 total, 8 squares) - Codifies the piece in each square, in the following manner:
//!   - `color` (3rd bit - 1 total) - 0 for a black piece, 1 for a white piece.
//!   - `piece` (2nd-0th bits - 3 total) - The role of the piece, following the same numbering as [`shakmaty::Role`], 0 meaning no piece and 7 being invalid.

use std::ops::BitXor;

use shakmaty::{Board, ByColor, ByRole, Role};

use crate::{attribute_err, attribute_fmt};

use super::error::ValuedAttributeParsingError;

#[cfg(any(feature = "time-mysql", feature = "chrono-mysql"))]
use mysql::{Params, params};

/// Data used for each role for codifing the number of pieces left.
const ROLE_DATA: [(u16, usize, Role); 5] = [
    (12, 8, Role::Pawn),
    (9, 7, Role::Knight),
    (6, 7, Role::Bishop),
    (3, 7, Role::Rook),
    (0, 7, Role::Queen),
];

/// Codified number of pieces left on the board and their positions. It is structured in the following manner:
///
/// # `pieces_left`
/// - `pawns` (15th-12th bits - 4 total) - Number of pawns left as a u4.
/// - `knights` (11th-9th bits - 3 total) - Number of knights left as a u3, it can fail if there are more than 7 knights for one color.
/// - `bishops` (8th-6th bits - 3 total) - Number of bishops left as a u3, it can fail if there are more than 7 bishops for one color.
/// - `rooks` (5th-3rd bits - 3 total) - Number of rooks left as a u3, it can fail if there are more than 7 rooks for one color.
/// - `queens` (2nd-0th bits - 3 total) - Number of queens left as a u3, it can fail if there are more than 7 queens for one color.
///
/// # `row`
/// - `square` ((0..32).step_by(4) bits - 4 total, 8 squares (0..3 being the 1st square, 28..31 being the 8th square)) - Codifies the piece in each square, in the following manner:
///   - `color` (3rd bit - 1 total) - 0 for a black piece, 1 for a white piece.
///   - `piece` (2nd-0th bits - 3 total) - The role of the piece, following the same numbering as [`shakmaty::Role`], 0 meaning no piece and 7 being invalid.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BoardConfiguration {
    pub black_left: u16,
    pub white_left: u16,
    pub rows: [u32; 8],
}

impl BoardConfiguration {
    /// Constructs a [`BoardConfiguration`] based on a [`Board`] representation.
    ///
    /// # Errors
    /// Will return a [`ValuedAttributeParsingError`] if the number of pieces of any of the pieces is bigger than the expected maximum. It will also return the parsed [`BoardConfiguration`] with those said `pieces_left` set to all ones.
    pub fn from_board(
        board: &Board,
    ) -> Result<BoardConfiguration, (BoardConfiguration, ValuedAttributeParsingError)> {
        let (
            ByRole {
                pawn,
                knight,
                bishop,
                rook,
                queen,
                king,
            },
            ByColor { white, black },
        ) = board.clone().into_bitboards();
        let mut configuration = BoardConfiguration::default();
        let (mut black_err, mut white_err) = (None, None);

        for (pieces, (displacement, max, role)) in [pawn, knight, bishop, rook, queen]
            .into_iter()
            .zip(ROLE_DATA)
        {
            let black_pieces = pieces.intersect(black).count();
            if black_pieces <= max {
                configuration.black_left |= (black_pieces as u16) << displacement;
            } else {
                black_err = Some(format!(
                    "The number of black {}s is {black_pieces}, when it should be at most {max}.",
                    role.upper_char()
                ));
            }
            let white_pieces = pieces.intersect(white).count();
            if white_pieces <= max {
                configuration.white_left |= (white_pieces as u16) << displacement;
            } else {
                white_err = Some(format!(
                    "The number of white {}s is {white_pieces}, when it should be at most {max}.",
                    role.upper_char()
                ));
            }
        }

        for (row, i) in configuration.rows.iter_mut().zip((0..64).step_by(8)) {
            let (pawn, knight, bishop, rook, queen, king, white) = (
                (pawn.0 >> i) & 0xFF,
                (knight.0 >> i) & 0xFF,
                (bishop.0 >> i) & 0xFF,
                (rook.0 >> i) & 0xFF,
                (queen.0 >> i) & 0xFF,
                (king.0 >> i) & 0xFF,
                (white.0 >> i) & 0xFF,
            );
            for j in 0..8 {
                *row |= (((((white >> j) & 1) << 3)
                    | ((pawn >> j) & 1)
                    | (((knight >> j) & 1) << 1)
                    | (((bishop >> j) & 1) * 3)
                    | (((rook >> j) & 1) << 2)
                    | (((queen >> j) & 1) * 5)
                    | (((king >> j) & 1) * 6))
                    << (j << 2)) as u32
            }
        }

        match (black_err, white_err) {
            (Some(mut black_err), Some(white_err)) => {
                configuration.black_left = u16::MAX;
                configuration.white_left = u16::MAX;
                black_err.push(' ');
                black_err.push_str(&white_err);
                Err((
                    configuration,
                    ValuedAttributeParsingError::from_inner_utf8(ERROR, black_err),
                ))
            }
            (Some(black_err), None) => {
                configuration.black_left = u16::MAX;
                Err((
                    configuration,
                    ValuedAttributeParsingError::from_inner_utf8(ERROR, black_err),
                ))
            }
            (None, Some(white_err)) => {
                configuration.white_left = u16::MAX;
                Err((
                    configuration,
                    ValuedAttributeParsingError::from_inner_utf8(ERROR, white_err),
                ))
            }
            (None, None) => Ok(configuration),
        }
    }

    #[cfg(any(feature = "time-mysql", feature = "chrono-mysql"))]
    /// Prepares the parameters for MySQL insertion and selection of this data.
    pub fn as_params(&self) -> Params {
        params! {
            "black_pieces" => self.black_left,
            "white_pieces" => self.white_left,
            "a_row" => self.rows[0],
            "b_row" => self.rows[1],
            "c_row" => self.rows[2],
            "d_row" => self.rows[3],
            "e_row" => self.rows[4],
            "f_row" => self.rows[5],
            "g_row" => self.rows[6],
            "h_row" => self.rows[7],
        }
    }
}

impl BitXor for BoardConfiguration {
    type Output = BoardConfiguration;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self {
            black_left: self.black_left ^ rhs.black_left,
            white_left: self.white_left ^ rhs.white_left,
            rows: [
                self.rows[0] ^ rhs.rows[0],
                self.rows[1] ^ rhs.rows[1],
                self.rows[2] ^ rhs.rows[2],
                self.rows[3] ^ rhs.rows[3],
                self.rows[4] ^ rhs.rows[4],
                self.rows[5] ^ rhs.rows[5],
                self.rows[6] ^ rhs.rows[6],
                self.rows[7] ^ rhs.rows[7],
            ],
        }
    }
}

attribute_fmt!(
    BoardConfiguration,
    "left pieces (3 bits for each piece and color, except the pawns, which need 4 bits) rows (8 rows, one of 32 bits, four bits for each square, the first bit being whether it's a white or a black piece and the following three coinciding with the value of shakmaty's Role enum)"
);
attribute_err!(BoardConfiguration);

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;
    use shakmaty::{Bitboard, Board, ByColor, ByRole};

    use super::BoardConfiguration;

    /// Tests whether the Board::from_board actually works or not.
    #[test]
    pub fn from_board_test() {
        // 0b0000000000000000.
        // 0x00000000.
        // 0x0000000000000000.
        let test_cases = [
            (
                if let Ok(board) = BoardConfiguration::from_board(&Board::from_bitboards(
                    ByRole {
                        pawn: Bitboard(0x0000000000000000),
                        knight: Bitboard(0x0000000000000000),
                        bishop: Bitboard(0x0000000000000000),
                        rook: Bitboard(0x0000000000000000),
                        queen: Bitboard(0x0000000000000000),
                        king: Bitboard(0x0000000000000000),
                    },
                    ByColor {
                        black: Bitboard(0x0000000000000000),
                        white: Bitboard(0x0000000000000000),
                    },
                )) {
                    board
                } else {
                    unreachable!();
                },
                BoardConfiguration {
                    black_left: 0b0000000000000000,
                    white_left: 0b0000000000000000,
                    rows: [
                        0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
                        0x00000000, 0x00000000,
                    ],
                },
            ),
            (
                if let Ok(board) = BoardConfiguration::from_board(&Board::from_bitboards(
                    ByRole {
                        pawn: Bitboard(0x0000400000000001),
                        knight: Bitboard(0x0080000000000200),
                        bishop: Bitboard(0x1100000000040000),
                        rook: Bitboard(0x0000000000080000),
                        queen: Bitboard(0x0000000010000000),
                        king: Bitboard(0x0000002000000000),
                    },
                    ByColor {
                        black: Bitboard(0x1180400000040001),
                        white: Bitboard(0x0000002010080200),
                    },
                )) {
                    board
                } else {
                    unreachable!();
                },
                BoardConfiguration {
                    black_left: 0b0010001011000000,
                    white_left: 0b0000001000001001,
                    rows: [
                        0x00000001, 0x000000A0, 0x0000C300, 0x000D0000, 0x00E00000, 0x01000000,
                        0x20000000, 0x00030003,
                    ],
                },
            ),
            (
                if let Ok(board) = BoardConfiguration::from_board(&Board::new()) {
                    board
                } else {
                    unreachable!();
                },
                BoardConfiguration {
                    black_left: 0b1000010010010001,
                    white_left: 0b1000010010010001,
                    rows: [
                        0xCABEDBAC, 0x99999999, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
                        0x11111111, 0x42365324,
                    ],
                },
            ),
            (
                if let Ok(board) = BoardConfiguration::from_board(&Board::from_bitboards(
                    ByRole {
                        pawn: Bitboard(0x00B14A0084112200),
                        knight: Bitboard(0x0000200200200000),
                        bishop: Bitboard(0x0040000000000000),
                        rook: Bitboard(0x0500004000000080),
                        queen: Bitboard(0x0000800000000400),
                        king: Bitboard(0x4000000000000004),
                    },
                    ByColor {
                        black: Bitboard(0x45F1EA0000000000),
                        white: Bitboard(0x0000004284312684),
                    },
                )) {
                    board
                } else {
                    unreachable!();
                },
                BoardConfiguration {
                    black_left: 0b0111001001010001,
                    white_left: 0b0110010000010001,
                    rows: [
                        0xC0000E00, 0x00900D90, 0x00A90009, 0x90000900, 0x0C0000A0, 0x51201010,
                        0x13110001, 0x06000404,
                    ],
                },
            ),
            (
                if let Ok(board) = BoardConfiguration::from_board(&Board::from_bitboards(
                    ByRole {
                        pawn: Bitboard(0x0000000000000000),
                        knight: Bitboard(0x0000000000000000),
                        bishop: Bitboard(0x0000000000000200),
                        rook: Bitboard(0x0000000000000000),
                        queen: Bitboard(0x0000004000000000),
                        king: Bitboard(0x000000A000000000),
                    },
                    ByColor {
                        black: Bitboard(0x0000006000000200),
                        white: Bitboard(0x0000008000000000),
                    },
                )) {
                    board
                } else {
                    unreachable!();
                },
                BoardConfiguration {
                    black_left: 0b0000000001000001,
                    white_left: 0b0000000000000000,
                    rows: [
                        0x00000000, 0x00000030, 0x00000000, 0x00000000, 0xE5600000, 0x00000000,
                        0x00000000, 0x00000000,
                    ],
                },
            ),
        ];
        for ((value, truth), i) in test_cases.into_iter().zip(1..) {
            println!(
                "{i:02}: white_left\n    {:016b}\n    {:016b}\n   ({:016b})",
                value.white_left,
                truth.white_left,
                value.white_left ^ truth.white_left
            );
            println!(
                "{i:02}: black_left\n    {:016b}\n    {:016b}\n   ({:016b})",
                value.black_left,
                truth.black_left,
                value.black_left ^ truth.black_left
            );
            println!(
                "{i:02}: rows[0]\n    {:08x}\n    {:08x}\n   ({:08x})",
                value.rows[0],
                truth.rows[0],
                value.rows[0] ^ truth.rows[0]
            );
            println!(
                "{i:02}: rows[1]\n    {:08x}\n    {:08x}\n   ({:08x})",
                value.rows[1],
                truth.rows[1],
                value.rows[1] ^ truth.rows[1]
            );
            println!(
                "{i:02}: rows[2]\n    {:08x}\n    {:08x}\n   ({:08x})",
                value.rows[2],
                truth.rows[2],
                value.rows[2] ^ truth.rows[2]
            );
            println!(
                "{i:02}: rows[3]\n    {:08x}\n    {:08x}\n   ({:08x})",
                value.rows[3],
                truth.rows[3],
                value.rows[3] ^ truth.rows[3]
            );
            println!(
                "{i:02}: rows[4]\n    {:08x}\n    {:08x}\n   ({:08x})",
                value.rows[4],
                truth.rows[4],
                value.rows[4] ^ truth.rows[4]
            );
            println!(
                "{i:02}: rows[5]\n    {:08x}\n    {:08x}\n   ({:08x})",
                value.rows[5],
                truth.rows[5],
                value.rows[5] ^ truth.rows[5]
            );
            println!(
                "{i:02}: rows[6]\n    {:08x}\n    {:08x}\n   ({:08x})",
                value.rows[6],
                truth.rows[6],
                value.rows[6] ^ truth.rows[6]
            );
            println!(
                "{i:02}: rows[7]\n    {:08x}\n    {:08x}\n   ({:08x})",
                value.rows[7],
                truth.rows[7],
                value.rows[7] ^ truth.rows[7]
            );
            assert_eq!(
                value ^ truth,
                BoardConfiguration {
                    black_left: 0,
                    white_left: 0,
                    rows: [0; 8]
                }
            );
            assert_eq!(value, truth);
        }
        /*
        println!("{:#018X}", 0);
        println!("{:#018X}", 0);
        println!("{:#018X}", 512u64);
        println!("{:#018X}", 0);
        println!("{:#018X}", 274877906944u64);
        println!("{:#018X}", 687194767360u64);
        println!("{:#018X}", 412316860928u64);
        println!("{:#018X}", 549755813888u64);
        */
    }
}
