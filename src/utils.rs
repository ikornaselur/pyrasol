use crate::board::{Card, RawCard};
/// A raw value will be from 0 to 51
/// Aces will be 0, 13, 26 and 39 for example
/// This will turn a raw value into a card value
/// 0/13/26/39 will be "1" for Ace
pub fn card_from_raw(val: u8) -> u8 {
    (val % 13) + 1
}

/// Get the card value that would match with this card
///
/// 1 will return 12 (Ace matches with Jack)
/// 13 will return 13 (King matches with itself)
pub fn match_card(card: Card) -> Card {
    match card {
        Card(13) => Card(13),
        Card(x) => Card(13 - x),
    }
}

/// Check if two cards are a matching pair
pub fn cards_match(a: RawCard, b: RawCard) -> bool {
    let a: Card = a.into();
    let b: Card = b.into();
    match_card(a) == b
}
