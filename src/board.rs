use std::collections::{HashMap, HashSet};

fn card_directly_blocks(card: u8) -> Vec<u8> {
    // This is is 'blocks_cards_map' in python
    // TODO: Reduce to just value and count? vec![3, 4] is just (3, 2) (two values)
    // and vec![9] would be (9, 1) (one value)
    match card {
        0 => vec![],
        1 | 2 => vec![0],
        3 => vec![1],
        4 => vec![1, 2],
        5 => vec![2],
        6 => vec![3],
        7 => vec![3, 4],
        8 => vec![4, 5],
        9 => vec![5],
        10 => vec![6],
        11 => vec![6, 7],
        12 => vec![7, 8],
        13 => vec![8, 9],
        14 => vec![9],
        15 => vec![10],
        16 => vec![10, 11],
        17 => vec![11, 12],
        18 => vec![12, 13],
        19 => vec![13, 14],
        20 => vec![14],
        21 => vec![15],
        22 => vec![15, 16],
        23 => vec![16, 17],
        24 => vec![17, 18],
        25 => vec![18, 19],
        26 => vec![19, 20],
        27 => vec![20],
        _ => vec![],
    }
}

fn card_blocks(card: u8) -> Vec<u8> {
    // This is 'blockers_map' in python
    match card {
        0 => vec![],
        1 | 2 => vec![0],
        3 => vec![1, 0],
        4 => vec![1, 2, 0],
        5 => vec![2, 0],
        6 => vec![3, 1, 0],
        7 => vec![3, 4, 1, 2, 0],
        8 => vec![4, 5, 1, 2, 0],
        9 => vec![5, 2, 0],
        10 => vec![6, 3, 1, 0],
        11 => vec![6, 7, 3, 4, 1, 2, 0],
        12 => vec![7, 8, 3, 4, 5, 1, 2, 0],
        13 => vec![8, 9, 4, 5, 1, 2, 0],
        14 => vec![9, 5, 2, 0],
        15 => vec![10, 6, 3, 1, 0],
        16 => vec![10, 11, 6, 7, 3, 4, 1, 2, 0],
        17 => vec![11, 12, 6, 7, 8, 3, 4, 5, 1, 2, 0],
        18 => vec![12, 13, 7, 8, 9, 3, 4, 5, 1, 2, 0],
        19 => vec![13, 14, 8, 9, 4, 5, 1, 2, 0],
        20 => vec![14, 9, 5, 2, 0],
        21 => vec![15, 10, 6, 3, 1, 0],
        22 => vec![15, 16, 10, 11, 6, 7, 3, 4, 1, 2, 0],
        23 => vec![16, 17, 10, 11, 12, 6, 7, 8, 3, 4, 5, 1, 2, 0],
        24 => vec![17, 18, 11, 12, 13, 6, 7, 8, 9, 3, 4, 5, 1, 2, 0],
        25 => vec![18, 19, 12, 13, 14, 7, 8, 9, 4, 5, 1, 2, 0],
        26 => vec![19, 14, 9, 5, 2, 0],
        27 => vec![20, 15, 10, 6, 3, 1, 0],
        _ => vec![],
    }
}

fn card_blocked_by(card: u8) -> Vec<u8> {
    // this is 'blocked_by_map' in python
    match card {
        0 => (1..28).collect(),
        1 => vec![3, 4, 6, 7, 8, 10, 11, 12, 13, 15, 16, 17, 18, 19, 21, 22, 23, 24, 25, 26],
        2 => vec![4, 5, 7, 8, 9, 11, 12, 13, 14, 16, 17, 18, 19, 20, 22, 23, 24, 25, 26, 27],
        3 => vec![6, 7, 10, 11, 12, 15, 16, 17, 18, 21, 22, 23, 24, 25],
        4 => vec![7, 8, 11, 12, 13, 16, 17, 18, 19, 22, 23, 24, 25, 26],
        5 => vec![8, 9, 12, 13, 14, 17, 18, 19, 20, 23, 24, 25, 26, 27],
        6 => vec![10, 11, 15, 16, 17, 21, 22, 23, 24],
        7 => vec![11, 12, 16, 17, 18, 22, 23, 24, 25],
        8 => vec![12, 13, 17, 18, 19, 23, 24, 25, 26],
        9 => vec![13, 14, 18, 19, 20, 24, 25, 26, 27],
        10 => vec![15, 16, 21, 22, 23],
        11 => vec![16, 17, 22, 23, 24],
        12 => vec![17, 18, 23, 24, 25],
        13 => vec![18, 19, 24, 25, 26],
        14 => vec![19, 20, 25, 26, 27],
        15 => vec![21, 22],
        16 => vec![22, 23],
        17 => vec![23, 24],
        18 => vec![24, 25],
        19 => vec![25, 26],
        20 => vec![26, 27],
        _ => vec![],
    }
}

/// A raw value will be from 0 to 51
/// Aces will be 0, 13, 26 and 39 for example
/// This will turn a raw value into a card value
/// 0/13/26/39 will be "1" for Ace
fn card_from_raw(val: u8) -> u8 {
    (val % 13) + 1
}



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
    pub fn new(cards: Vec<u8>, stack: Vec<u8>, leaf_idxs: Vec<u8>) -> Board {
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

    pub fn remove_cards(&mut self, cards: Vec<u8>) {
        // Get the indexes of the cards we are going to remove
        // TODO: Calling cards.contains for each card in cards.. O(n*m)? Can we improve? Cards to
        // remove shouldn't be more than 2 at most, so I guess O(n*2) -> O(n) in practice?
        let card_idxs: Vec<u8> = self
            .cards
            .iter()
            .enumerate()
            .filter(|(_, &card)| cards.contains(&card))
            .map(|(idx, _)| idx as u8)
            .collect();

        println!("Removing cards {:?} at indexes {:?}", cards, card_idxs);

        // One by one, remove those indexes and check if we introduce a new leaf
        let mut leaf_candidates: HashSet<u8> = HashSet::new();
        for card_idx in card_idxs {
            self.leaf_idxs.remove(&card_idx);

            if card_idx == 0 {
                self.completed = true;
            }

            let blocked_cards = card_directly_blocks(card_idx);
            leaf_candidates.extend(blocked_cards.iter());
        }

        // Remove candidates that are blocked by other candidates, as if those
        // blockers are going in they are going to block the previous card (this
        // hardly makes any sense, but it should be obvious.. right?)
        let mut candidate_blockers: HashSet<u8> = HashSet::new();
        for candidate in leaf_candidates.clone() {  // TODO: Is the clone necessary?
            candidate_blockers.extend(card_blocks(candidate).iter());
        }

        // And now check the ones that aren't blocked by other candidates
        for candidate in leaf_candidates.difference(&candidate_blockers) {
            let blocked_by_set: HashSet<u8> = card_blocked_by(*candidate).into_iter().collect();
            
            if blocked_by_set.is_disjoint(&self.leaf_idxs) {
                self.leaf_idxs.insert(*candidate);
            }
        }

        // Also reduce the counts..
        for card in cards {
            match self.card_counts.get_mut(&card_from_raw(card)) {
                Some(count) => *count -= 1,
                None => panic!("Tried to remove a card that doesn't exist!"),
            }
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

    #[test]
    fn test_board_remove_cards() {
        let cards: Vec<u8> = vec![
            7, 
            6, 4,
            11, 0, 3, 
            10, 19, 13, 5,
            24, 2, 26, 37, 32,
            18, 1, 9, 17, 30, 8,
            12, 14, 22, 16, 27, 23, 15,
        ];
        let stack: Vec<u8> = vec![
            31, 20, 36, 44, 33, 25, 38, 46, 35, 50, 28, 29, 40, 49, 39, 45, 48, 21, 34, 42, 41, 51,
            47, 43,
        ];
        let leaf_idxs: Vec<u8> = vec![21, 22, 23, 24, 25, 26, 27];

        let mut board = Board::new(cards, stack, leaf_idxs);

        assert_eq!(board.leaves(), HashSet::from([12, 14, 15, 16, 22, 23, 27]));

        board.remove_cards(vec![12, 16]);

        assert_eq!(board.leaves(), HashSet::from([14, 15, 22, 23, 27]));

        board.remove_cards(vec![14, 22]);

        assert_eq!(board.leaves(), HashSet::from([18, 1, 9, 27, 23, 15]));
    }
}
