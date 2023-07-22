use crate::card::{Card, RawCard};

pub fn validate_board(board_cards: &[RawCard], stack_cards: &[RawCard]) {
    let mut card_counts: Vec<u8> = vec![0; 13];

    for card in board_cards.iter() {
        let card_value = Card::from(*card).0 - 1;
        card_counts[card_value as usize] += 1;
    }
    for card in stack_cards.iter() {
        let card_value = Card::from(*card).0 - 1;
        card_counts[card_value as usize] += 1;
    }

    for count in card_counts.iter() {
        if *count != 4 {
            panic!("Invalid board: card {} appears {} times", count, *count);
        }
    }
}
