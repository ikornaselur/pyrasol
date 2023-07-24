use crate::game::card::{Card, RawCard};
use crate::game::utils::pretty_print_card;
use anyhow::{bail, Result};

pub fn validate_board(board_cards: &[RawCard], stack_cards: &[RawCard]) -> Result<()> {
    let mut card_counts: Vec<u8> = vec![0; 13];

    for card in board_cards.iter() {
        let card_value = Card::from(*card).0 - 1;
        card_counts[card_value as usize] += 1;
    }
    for card in stack_cards.iter() {
        let card_value = Card::from(*card).0 - 1;
        card_counts[card_value as usize] += 1;
    }

    if let Some(idx) =
        card_counts.iter().enumerate().find_map(
            |(idx, count)| {
                if *count != 4 {
                    Some(idx)
                } else {
                    None
                }
            },
        )
    {
        let card = RawCard(idx.try_into().unwrap());
        bail!(
            "Card {} is present {} times, but every card needs to be present 4 times across the board and stack",
            pretty_print_card(card, true),
            card_counts[idx]
        )
    }

    Ok(())
}
