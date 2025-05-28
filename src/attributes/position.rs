//! Represents a position in a Lichess game. It holds Nag, suffix, and the whole characterization of the move.

use deranged::RangedU8;
use shakmaty::{Color, Position as PositionTrait, Role};

use crate::{attribute_err, attribute_fmt};

use super::error::ValuedAttributeParsingError;

pub struct Position<P: PositionTrait>(pub P);

impl<P: PositionTrait> Position<P> {
        
}

attribute_fmt!(Position, "3 bits for each piece and color, except the pawns, which need 4 bits.");
attribute_err!(Position);
