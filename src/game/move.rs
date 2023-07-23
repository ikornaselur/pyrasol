use crate::game::card::{MatchType, RawCard};
use std::cmp::Ordering;

pub type Move = (MatchType, i32, (RawCard, Option<RawCard>));

pub fn move_sort(
    (a_move_type, a_draws, (a_left_card, a_right_card)): &Move,
    (b_move_type, b_draws, (b_left_card, b_right_card)): &Move,
) -> Ordering {
    // 1. Number of draws first
    if a_draws != b_draws {
        return a_draws.cmp(b_draws);
    }
    // 2. Then move type
    if a_move_type != b_move_type {
        return a_move_type.cmp(b_move_type);
    }

    // 3. The left card, if not the same
    if a_left_card != b_left_card {
        return a_left_card.cmp(b_left_card);
    }
    // 4. less/greated depending on right card being None
    if a_right_card.is_none() && b_right_card.is_some() {
        return Ordering::Less;
    }
    if a_right_card.is_some() && b_right_card.is_none() {
        return Ordering::Greater;
    }
    // 5. The right card if both are Some
    a_right_card.cmp(b_right_card)
}
