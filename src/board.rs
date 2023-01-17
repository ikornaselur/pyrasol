use std::collections::{HashMap, HashSet};

pub struct Board {
    cards: Vec<u8>,
    card_counts: HashMap<u8, u8>,

    stack: Vec<u8>,
    stack_idx: u8,
    stack_counts: HashMap<u8, u8>,

    leaf_idxs: HashSet<u8>,

    pub moves: u8,
    pub completed: bool,
}

impl Board {
    pub fn new(cards: Vec<u8>, stack: Vec<u8>, leaf_idxs: Vec<u8>) -> Self {
        let card_counts: HashMap<u8, u8> = HashMap::from([
            (1, 4),
            (2, 4),
            (3, 4),
            (4, 4),
            (5, 4),
            (6, 4),
            (7, 4),
            (8, 4),
            (9, 4),
            (10, 4),
            (11, 4),
            (12, 4),
            (13, 4),
        ]);
        let stack_counts: HashMap<u8, u8> = HashMap::from([
            (1, 1),
            (2, 1),
            (3, 2),
            (4, 2),
            (5, 1),
            (6, 2),
            (7, 1),
            (8, 3),
            (9, 3),
            (10, 2),
            (11, 2),
            (12, 1),
            (13, 3),
        ]);
        let leaf_idxs: HashSet<u8> = HashSet::from_iter(leaf_idxs);

        Board {
            cards,
            card_counts,
            stack,
            stack_idx: 0,
            stack_counts,
            leaf_idxs,
            moves: 0,
            completed: false,
        }
    }

    pub fn leaves(&self) -> HashSet<u8> {
        HashSet::from_iter(self.leaf_idxs.iter().map(|idx| self.cards[*idx as usize]))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_board_new() {
        let cards: Vec<u8> = vec![
            7, 6, 4, 11, 0, 3, 10, 19, 13, 5, 24, 2, 26, 37, 32, 18, 1, 9, 17, 30, 8, 12, 14, 22,
            16, 27, 23, 15,
        ];
        let stack: Vec<u8> = vec![
            31, 20, 36, 44, 33, 25, 38, 46, 35, 50, 28, 29, 40, 49, 39, 45, 48, 21, 34, 42, 41, 51,
            47, 43,
        ];
        let leaf_idxs: Vec<u8> = vec![21, 22, 23, 24, 25, 26, 27];

        let board = Board::new(cards, stack, leaf_idxs);

        assert_eq!(board.leaves(), HashSet::from([12, 14, 15, 16, 22, 23, 27]));
    }
}
