use crate::board::{Board, Card, Move, RawCard};
use colored::{ColoredString, Colorize};
use std::collections::HashMap;

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

/// Parse cards and stack strings into vectors of raw cards
///
/// For example the string 76jkj would parse into:
///     RawCard(7)
///     RawCard(6)
///     RawCard(11)
///     RawCard(13)
///     RawCard(24)  // Second card offset by 13
///
pub fn parse_board(cards_str: String, stack_str: String) -> (Vec<RawCard>, Vec<RawCard>) {
    let mut cards: Vec<RawCard> = vec![];
    let mut stack: Vec<RawCard> = vec![];

    let mut counts: HashMap<u8, u8> = HashMap::new();

    // Go through the cards first
    for char in cards_str.chars() {
        let val = match char {
            'a' | 'A' => 1,
            'j' | 'J' => 11,
            'q' | 'Q' | 'd' | 'D' => 12, // I keep typing queen as d
            'k' | 'K' => 13,
            '0' => 10,
            '1'..='9' => char.to_digit(10).unwrap() as u8,
            _ => panic!("Unknown val: {}", char),
        } - 1;
        let count = counts.entry(val).or_insert(0);
        cards.push(RawCard(val + *count * 13));
        *count += 1;
    }

    // Then the stack
    for char in stack_str.chars() {
        let val = match char {
            'a' | 'A' => 1,
            'j' | 'J' => 11,
            'q' | 'Q' => 12,
            'k' | 'K' => 13,
            '0' => 10,
            '1'..='9' => char.to_digit(10).unwrap() as u8,
            _ => panic!("Unknown val: {}", char),
        } - 1;
        let count = counts.entry(val).or_insert(0);
        stack.push(RawCard(val + *count * 13));
        *count += 1;
    }

    (cards, stack)
}

pub fn pretty_print_board(board: &Board) {
    // The cards are stored in a single array. Print them in a pyramid shape of 7 rows, where the
    // top row is one card, followed by two cards, then three, etc.
    let mut idx = 0;
    for row in 0..7 {
        let mut line = String::new();
        for _ in 0..(6 - row) {
            line.push(' ');
        }
        print!("{}", line);
        for _ in 0..(row + 1) {
            if board.leaf_idxs.contains(&(idx as u8)) {
                print!("{} ", pretty_print_card(board.cards[idx]).purple());
            } else {
                print!("{} ", pretty_print_card(board.cards[idx]));
            };
            idx += 1;
        }
        println!();
    }

    // Then just print the stack in order
    println!();
    print!("Stack: ");
    for (idx, card) in board.stack.iter().enumerate() {
        if idx as i32 == board.stack_idx || idx as i32 == board.stack_idx - 1 {
            print!("{} ", pretty_print_card(*card).purple());
        } else {
            print!("{} ", pretty_print_card(*card));
        }
    }
    println!();

    // And the move count
    println!("Moves: {}", board.moves);
}

/// Check if two cards are a matching pair
pub fn cards_match(a: RawCard, b: RawCard) -> bool {
    if a == b {
        return false;
    }

    let a: Card = a.into();
    let b: Card = b.into();
    match_card(a) == b
}

fn pretty_print_card(card: RawCard) -> ColoredString {
    match Card::from(card) {
        Card(1) => String::from("A"),
        Card(10) => String::from("0"), // Single width values for all cards
        Card(11) => String::from("J"),
        Card(12) => String::from("Q"),
        Card(13) => String::from("K"),
        Card(x) => x.to_string(),
    }
    .green()
}

pub fn pretty_print_move(board: &Board, idx: u8, (_, draws, (left_card, right_card)): Move) {
    let draw_str = if draws > 0 {
        format!("Draw {} cards and ", draws).blue()
    } else {
        String::from("").blue()
    };

    let cards_str = match (left_card, right_card) {
        (left_card, None) => format!(
            "{}emove {} {}",
            if draw_str.is_empty() { "R" } else { "r" },
            pretty_print_card(left_card),
            get_loc(board, left_card),
        ),
        (left_card, Some(right_card)) => format!(
            "{}atch {} {} and {} {}",
            if draw_str.is_empty() { "M" } else { "m" },
            pretty_print_card(left_card),
            get_loc(board, left_card),
            pretty_print_card(right_card),
            get_loc(board, right_card),
        ),
    };

    println!("[{}] {}{}", idx, draw_str, cards_str);
}

fn card_pos(board: &Board, card: RawCard) -> String {
    let idx = board.cards.iter().position(|&x| x == card).unwrap();
    match idx {
        0 => "on board 1st row, 1st card".to_string(),
        1..=2 => format!("on board 2nd row, card {}", idx),
        3..=5 => format!("on board 3rd row, card {}", idx - 2),
        6..=9 => format!("on board 4th row, card {}", idx - 5),
        10..=14 => format!("on board 5th row, card {}", idx - 9),
        15..=20 => format!("on board 6th row, card {}", idx - 14),
        21..=27 => format!("on board 7th row, card {}", idx - 20),
        _ => "No idea".to_string(),
    }
}

pub fn get_loc(board: &Board, card: RawCard) -> ColoredString {
    if board.cards.contains(&card) {
        // Count the leaves
        let mut num_counts: HashMap<Card, usize> = HashMap::new();
        for leaf in board.leaves().iter() {
            *num_counts.entry(Card::from(*leaf)).or_insert(0) += 1;
        }

        if num_counts[&Card::from(card)] == 1 {
            "on the board".yellow()
        } else {
            card_pos(board, card).yellow()
        }
    } else {
        "on the stack".red()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_board() {
        let cards_str = "12jk".to_string();
        let stack_str = "aakq".to_string();

        let (cards, stack) = parse_board(cards_str, stack_str);

        assert_eq!(
            cards,
            vec![RawCard(0), RawCard(1), RawCard(10), RawCard(12)]
        );
        assert_eq!(
            stack,
            vec![RawCard(13), RawCard(26), RawCard(25), RawCard(11)]
        );
    }
}
