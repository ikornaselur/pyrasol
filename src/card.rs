use crate::utils::card_from_raw;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, PartialOrd, Ord)]
pub enum MatchType {
    Board,
    BoardStack,
    Stack,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
pub struct RawCard(pub u8);
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
pub struct Card(pub u8);

impl From<RawCard> for Card {
    fn from(raw_card: RawCard) -> Self {
        Card(card_from_raw(raw_card.0))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_move_type_order() {
        let mut move_types = vec![MatchType::BoardStack, MatchType::Stack, MatchType::Board];

        move_types.sort();

        assert_eq!(
            move_types,
            vec![MatchType::Board, MatchType::BoardStack, MatchType::Stack]
        );
    }

    #[test]
    fn test_raw_card_order() {
        let mut cards = vec![
            RawCard(10),
            RawCard(21),
            RawCard(3),
            RawCard(7),
            RawCard(1),
            RawCard(2),
            RawCard(14),
        ];

        cards.sort();

        assert_eq!(
            cards,
            vec![
                RawCard(1),
                RawCard(2),
                RawCard(3),
                RawCard(7),
                RawCard(10),
                RawCard(14),
                RawCard(21)
            ]
        );
    }
}
