use crate::blocks::{card_blocked_by, card_blocks, card_directly_blocks};
use crate::card::{Card, MatchType, RawCard};
use crate::r#move::{move_sort, Move};
use crate::utils::{cards_match, match_card};
use std::cmp::max;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct Board {
    pub(crate) cards: Vec<RawCard>,
    card_counts: HashMap<Card, u8>,

    pub(crate) stack: Vec<RawCard>,
    pub(crate) stack_idx: i32,
    stack_counts: HashMap<Card, u8>,

    pub(crate) leaf_idxs: HashSet<u8>,

    pub moves: i32,
    pub completed: bool,
}

impl Board {
    pub fn new(cards: Vec<RawCard>, stack: Vec<RawCard>, leaf_idxs: Vec<u8>) -> Board {
        let card_counts: HashMap<Card, u8> = HashMap::from([
            (Card(1), 4),
            (Card(2), 4),
            (Card(3), 4),
            (Card(4), 4),
            (Card(5), 4),
            (Card(6), 4),
            (Card(7), 4),
            (Card(8), 4),
            (Card(9), 4),
            (Card(10), 4),
            (Card(11), 4),
            (Card(12), 4),
            (Card(13), 4),
        ]);
        let mut stack_counts: HashMap<Card, u8> = HashMap::from([
            (Card(1), 0),
            (Card(2), 0),
            (Card(3), 0),
            (Card(4), 0),
            (Card(5), 0),
            (Card(6), 0),
            (Card(7), 0),
            (Card(8), 0),
            (Card(9), 0),
            (Card(10), 0),
            (Card(11), 0),
            (Card(12), 0),
            (Card(13), 0),
        ]);
        for raw_card in &stack {
            *stack_counts.get_mut(&(*raw_card).into()).unwrap() += 1;
        }

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

    pub fn get_state(&self) -> String {
        let moves = self.moves.to_string();
        let mut card_state = self
            .leaf_idxs
            .iter()
            .map(|idx| idx.to_string())
            .collect::<Vec<String>>();
        card_state.sort();
        let card_state = card_state.join(":");

        let mut stack_state = self
            .stack
            .iter()
            .map(|card| card.0.to_string())
            .collect::<Vec<String>>();
        stack_state.sort();
        let stack_state = stack_state.join(":");
        let stack_idx = self.stack_idx.to_string();

        format!("{}|{}|{}|{}", moves, card_state, stack_state, stack_idx)
    }

    pub fn remove_cards(&mut self, (left, right): (RawCard, Option<RawCard>)) {
        // Get the indexes of the cards we are going to remove
        let mut card_idxs: Vec<u8> =
            vec![self.cards.iter().position(|&card| card == left).unwrap() as u8];
        if let Some(right) = right {
            card_idxs.push(self.cards.iter().position(|&card| card == right).unwrap() as u8);
        }

        // One by one, remove those indexes and check if we introduce a new leaf
        let mut leaf_candidates: HashSet<u8> = HashSet::new();
        for card_idx in card_idxs {
            self.leaf_idxs.remove(&card_idx);

            if card_idx == 0 {
                self.completed = true;
            }

            let (blocked_card, count) = card_directly_blocks(card_idx);
            leaf_candidates.insert(blocked_card);
            if count > 1 {
                leaf_candidates.insert(blocked_card + 1);
            }
        }

        // Remove candidates that are blocked by other candidates, as if those
        // blockers are going in they are going to block the previous card (this
        // hardly makes any sense, but it should be obvious.. right?)
        let mut candidate_blockers: HashSet<u8> = HashSet::new();
        for candidate in leaf_candidates.clone() {
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
        *self.card_counts.get_mut(&(left.into())).unwrap() -= 1;
        if let Some(right) = right {
            *self.card_counts.get_mut(&(right.into())).unwrap() -= 1;
        }
    }

    pub fn get_moves(&self) -> Vec<Move> {
        // First check for kings in the leaves
        for raw_card in self.leaves() {
            let card: Card = raw_card.into();
            if card.0 == 13 {
                return vec![(MatchType::Board, 0, (raw_card, None))];
            }
        }

        let mut moves: Vec<Move> = vec![];
        let solo_cards: Vec<Card> = self
            .cards
            .iter()
            .map(|&raw_card| raw_card.into())
            .filter(|&card| self.card_counts[&card] == 1)
            .collect();

        // Check for matches on the table itself
        let mut moves_on_table = false;
        let mut already_matched: HashSet<RawCard> = HashSet::new();

        let mut leaves: Vec<RawCard> = self.leaves().into_iter().collect();
        leaves.sort();

        for leaf in leaves {
            let potential_matches: Vec<RawCard> = self
                .leaves()
                .into_iter()
                .filter(|&card| cards_match(leaf, card))
                .collect();
            for potential_match in potential_matches {
                if already_matched.contains(&potential_match) {
                    continue;
                }
                already_matched.insert(potential_match);
                already_matched.insert(leaf);
                if solo_cards.contains(&leaf.into()) {
                    // Last pair match, only logical move
                    return vec![(MatchType::Board, 0, (leaf, Some(potential_match)))];
                }
                moves.push((MatchType::Board, 0, (leaf, Some(potential_match))));
                moves_on_table = true;
            }
        }

        // Check for stack matches
        for leaf in self.leaves() {
            let leaf_val: Card = leaf.into();
            let leaf_match = match_card(leaf_val);
            if self.stack_counts[&leaf_match] == 0 {
                // Match is not in the satck
                continue;
            }

            // We have some potential matches to make in the stack.. let's find them
            let draws = self.get_stack_draws(leaf_match);
            for draw in draws {
                let mut stack_card_idx = self.stack_idx + draw;
                if stack_card_idx > self.stack.len() as i32 {
                    // Need to wrap the stack!
                    stack_card_idx -= self.stack.len() as i32 + 1 // One for flippage
                }
                let stack_card = self.stack[stack_card_idx as usize];
                if solo_cards.contains(&leaf_val) && draw <= 0 {
                    // We should get rif og it ASAP
                    return vec![(
                        MatchType::BoardStack,
                        max(draw, 0),
                        (leaf, Some(stack_card)),
                    )];
                }

                // Left side of visible stack card is -1, no need to draw, hence the max
                moves.push((
                    MatchType::BoardStack,
                    max(draw, 0),
                    (leaf, Some(stack_card)),
                ));
            }
        }

        // Check for any moves that match in the stack
        let stack_moves = self.get_stack_moves();
        if !moves_on_table {
            for (move_type, draws, cards) in stack_moves.clone().into_iter() {
                if draws != 0 {
                    continue;
                }
                let card_val: Card = cards.0.into();
                if card_val.0 == 13 || solo_cards.contains(&card_val) {
                    // If this stack move is a solo move, but we should only return it IF there are
                    // no 0 draw moves already available
                    return vec![(move_type, draws, cards)];
                }
            }
        }
        moves.extend(stack_moves);

        // Sort the moves by:
        moves.sort_by(move_sort);

        moves
    }

    fn get_stack_draws(&self, card: Card) -> Vec<i32> {
        let mut draws = vec![];

        let stack_len = self.stack.len();

        if self.stack_idx > 0 {
            // We have to account for a left card begin visible
            let stack_card: &RawCard = &self.stack[(self.stack_idx as usize) - 1];
            let stack_card: Card = (*stack_card).into();

            if stack_card == card {
                draws.push(-1) // "Draw -1" means it's the previous visible card
            }
        }
        for (idx, raw_card) in self.stack.iter().skip(self.stack_idx as usize).enumerate() {
            let stack_card: Card = (*raw_card).into();
            if stack_card == card {
                draws.push(idx as i32);
            }
        }
        for (idx, raw_card) in self
            .stack
            .iter()
            .take(max((self.stack_idx) - 1, 0) as usize)
            .enumerate()
        {
            let stack_card: Card = (*raw_card).into();
            if stack_card == card {
                draws.push(idx as i32 + stack_len as i32 - self.stack_idx + 1);
            }
        }

        draws
    }

    fn get_stack_moves(&self) -> HashSet<Move> {
        let mut moves = HashSet::new();

        if self.stack_idx > 0 {
            // Check if the two visible cards match
            let left = self.stack[(self.stack_idx as usize) - 1];
            if self.stack_idx < self.stack.len() as i32 {
                let right = self.stack[self.stack_idx as usize];
                if cards_match(left, right) {
                    moves.insert((MatchType::Stack, 0, (left, Some(right))));
                }
            }
            // Also check if there is a king visible on the left side
            if Card::from(left).0 == 13 {
                moves.insert((MatchType::Stack, -1, (left, None)));
            }
        }

        // Lets check the right side solo as well for a king
        if self.stack_idx < self.stack.len() as i32 {
            let right = self.stack[self.stack_idx as usize];
            if Card::from(right).0 == 13 {
                moves.insert((MatchType::Stack, 0, (right, None)));
            }
        }

        // Check if any pairs match going further up the stack
        let upper_stack = self.stack.iter().skip(self.stack_idx as usize);
        for (draw, (left, right)) in upper_stack
            .zip(self.stack.iter().skip(self.stack_idx as usize + 1))
            .enumerate()
        {
            if Card::from(*right).0 == 13 {
                // Get rid of that king!
                moves.insert((MatchType::Stack, draw as i32 + 1, (*right, None)));
            } else if cards_match(*left, *right) {
                moves.insert((MatchType::Stack, draw as i32 + 1, (*left, Some(*right))));
            }
        }

        // Check if any pairs after resetting the stack
        let lower_stack = self.stack.iter().take(self.stack_idx as usize);
        let stack_len = self.stack.len();
        for (draw, (left, right)) in lower_stack
            .zip(self.stack.iter().take(self.stack_idx as usize + 1))
            .enumerate()
        {
            if cards_match(*left, *right) {
                moves.insert((
                    MatchType::Stack,
                    draw as i32 + stack_len as i32 - self.stack_idx + 1,
                    (*left, Some(*right)),
                ));
            }
        }

        moves
    }

    fn remove_stack_cards(&mut self, (left, right): (RawCard, Option<RawCard>)) {
        if self.stack_idx > 0 && self.stack[self.stack_idx as usize - 1] == left {
            // Need to shift back since the right card will have moved up
            self.stack_idx -= 1;
        }
        let card_idx = self.stack.iter().position(|&c| c == left).unwrap();
        self.stack.remove(card_idx);
        *self.stack_counts.get_mut(&(left.into())).unwrap() -= 1;

        if let Some(right) = right {
            if self.stack_idx > 0 && self.stack[self.stack_idx as usize - 1] == right {
                // Need to shift back since the right card will have moved up
                self.stack_idx -= 1;
            }
            //let stack_card_idx = self.stack.iter().position(|&c| c == right).unwrap();
            let stack_card_idx = match self.stack.iter().position(|&c| c == right) {
                Some(idx) => idx,
                None => {
                    println!("Just removed card {:?} from stack", left);
                    println!("Couldn't find card {:?} in stack", right);
                    println!("Stack: {:?}", self.stack);
                    panic!();
                }
            };

            self.stack.remove(stack_card_idx);
            *self.stack_counts.get_mut(&(right.into())).unwrap() -= 1;
        }
    }

    fn stack_draw(&mut self, draws: i32) {
        if draws == 0 {
            return;
        }
        self.stack_idx += draws;
        if self.stack_idx > self.stack.len() as i32 {
            self.stack_idx -= self.stack.len() as i32 + 1; // Extra one for the stack reset
        }
        self.moves += draws;
    }

    pub fn leaves(&self) -> HashSet<RawCard> {
        HashSet::from_iter(self.leaf_idxs.iter().map(|idx| self.cards[*idx as usize]))
    }

    pub fn play_move(&mut self, r#move: Move) {
        let (move_type, draws, cards) = r#move;
        self.stack_draw(draws);

        match (move_type, cards) {
            (MatchType::Board, cards) => self.remove_cards(cards),
            (MatchType::BoardStack, (board_card, Some(stack_card))) => {
                // Should raise value error if flipped, not wasting cycles on error checking,
                // shouldn't be mixed up in the first place!
                self.remove_stack_cards((stack_card, None));
                *self.card_counts.get_mut(&(stack_card.into())).unwrap() -= 1;
                self.remove_cards((board_card, None));
            }
            (MatchType::Stack, (left, right)) => {
                if let Some(right) = right {
                    if left == right {
                        println!("Stack: {:?}", self.stack);
                        println!("Move: {:?}", r#move);
                        panic!("Illegal move");
                    }
                }

                self.remove_stack_cards((left, right));
                *self.card_counts.get_mut(&(left.into())).unwrap() -= 1;
                if let Some(right) = right {
                    *self.card_counts.get_mut(&(right.into())).unwrap() -= 1;
                }
            }
            _ => panic!("Illegal move"),
        };

        self.moves += 1;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_base_board() -> Board {
        let cards: Vec<RawCard> = vec![
            RawCard(7),
            RawCard(6),
            RawCard(4),
            RawCard(11),
            RawCard(0),
            RawCard(3),
            RawCard(10),
            RawCard(19),
            RawCard(13),
            RawCard(5),
            RawCard(24),
            RawCard(2),
            RawCard(26),
            RawCard(37),
            RawCard(32),
            RawCard(18),
            RawCard(1),
            RawCard(9),
            RawCard(17),
            RawCard(30),
            RawCard(8),
            RawCard(12),
            RawCard(14),
            RawCard(22),
            RawCard(16),
            RawCard(27),
            RawCard(23),
            RawCard(15),
        ];
        let stack: Vec<RawCard> = vec![
            RawCard(31),
            RawCard(20),
            RawCard(36),
            RawCard(44),
            RawCard(33),
            RawCard(25),
            RawCard(38),
            RawCard(46),
            RawCard(35),
            RawCard(50),
            RawCard(28),
            RawCard(29),
            RawCard(40),
            RawCard(49),
            RawCard(39),
            RawCard(45),
            RawCard(48),
            RawCard(21),
            RawCard(34),
            RawCard(42),
            RawCard(41),
            RawCard(51),
            RawCard(47),
            RawCard(43),
        ];
        let leaf_idxs: Vec<u8> = vec![21, 22, 23, 24, 25, 26, 27];

        Board::new(cards, stack, leaf_idxs)
    }

    #[test]
    fn test_board_new() {
        let board = get_base_board();

        assert_eq!(
            board.leaves(),
            HashSet::from([
                RawCard(12),
                RawCard(14),
                RawCard(15),
                RawCard(16),
                RawCard(22),
                RawCard(23),
                RawCard(27)
            ])
        );
    }

    #[test]
    fn test_board_remove_cards() {
        let mut board = get_base_board();

        assert_eq!(
            board.leaves(),
            HashSet::from([
                RawCard(12),
                RawCard(14),
                RawCard(15),
                RawCard(16),
                RawCard(22),
                RawCard(23),
                RawCard(27)
            ])
        );

        board.remove_cards((RawCard(12), Some(RawCard(16))));

        assert_eq!(
            board.leaves(),
            HashSet::from([
                RawCard(14),
                RawCard(15),
                RawCard(22),
                RawCard(23),
                RawCard(27)
            ])
        );

        board.remove_cards((RawCard(14), Some(RawCard(22))));

        assert_eq!(
            board.leaves(),
            HashSet::from([
                RawCard(18),
                RawCard(1),
                RawCard(9),
                RawCard(27),
                RawCard(23),
                RawCard(15)
            ])
        );
    }

    #[test]
    fn test_board_get_moves() {
        let mut board = get_base_board();

        let moves = board.get_moves();

        assert_eq!(moves.len(), 1);

        let (move_type, draws, cards) = &moves[0];

        // Only one move makes sense, the king (12) in the leaves
        assert_eq!(*move_type, MatchType::Board);
        assert_eq!(*draws, 0);
        assert_eq!(*cards, (RawCard(12), None));

        // Remove the king
        board.remove_cards((RawCard(12), None));

        // Get the next possible moves
        let moves = board.get_moves();

        // There should be exactly 3 Board matches
        let board_matches = moves
            .iter()
            .filter(|r#move| r#move.0 == MatchType::Board)
            .count();
        assert_eq!(board_matches, 3);

        // There should be exactly 12 BoardStack matches
        let board_stack_matches = moves
            .iter()
            .filter(|r#move| r#move.0 == MatchType::BoardStack)
            .count();
        assert_eq!(board_stack_matches, 12);

        // And finally there should be exactly 5 Stack matches
        let stack_matches = moves
            .iter()
            .filter(|r#move| r#move.0 == MatchType::Stack)
            .count();
        assert_eq!(stack_matches, 5);

        // For a total of 20 moves
        assert_eq!(moves.len(), 20);
    }

    #[test]
    fn test_board_play_move() {
        let mut board = get_base_board();

        let moves = board.get_moves();

        assert_eq!(moves.len(), 1);
        let r#move: Move = moves.into_iter().next().unwrap();

        board.play_move(r#move);

        let moves = board.get_moves();

        assert_eq!(moves.len(), 20);
    }

    #[test]
    fn test_board_get_moves_has_a_stable_order() {
        let mut board_a = get_base_board();
        let mut board_b = get_base_board();

        // Play moves to get to 20 moves, as the first list of moves is just one move
        let r#move: Move = board_a.get_moves().into_iter().next().unwrap();
        board_a.play_move(r#move);
        let r#move: Move = board_b.get_moves().into_iter().next().unwrap();
        board_b.play_move(r#move);

        let moves_a = board_a.get_moves();
        let moves_b = board_b.get_moves();

        println!("{:?}", moves_a);
        println!("{:?}", moves_b);

        assert_eq!(moves_a.len(), 20);
        assert_eq!(moves_b.len(), 20);

        assert_eq!(moves_a, moves_b);

        let board_c = board_a.clone();
        let moves_c = board_c.get_moves();

        assert_eq!(moves_a, moves_c);
    }

    #[test]
    fn test_stack_draws_king_in_stack() {
        let cards = vec![
            RawCard(1),
            RawCard(6),
            RawCard(0),
            RawCard(7),
            RawCard(13),
            RawCard(12),
            RawCard(9),
            RawCard(14),
            RawCard(22),
            RawCard(10),
            RawCard(11),
            RawCard(20),
            RawCard(35),
            RawCard(5),
            RawCard(23),
            RawCard(25),
            RawCard(26),
            RawCard(8),
            RawCard(2),
            RawCard(33),
            RawCard(18),
            RawCard(38),
            RawCard(19),
            RawCard(31),
            RawCard(4),
            RawCard(36),
            RawCard(39),
            RawCard(24),
        ];
        let stack = vec![
            RawCard(3),
            RawCard(37),
            RawCard(17),
            RawCard(16),
            RawCard(29),
            RawCard(44),
            RawCard(50),
            RawCard(49),
            RawCard(21),
            RawCard(34),
            RawCard(15),
            RawCard(47),
            RawCard(28),
            RawCard(30),
            RawCard(51),
            RawCard(48),
            RawCard(46),
            RawCard(41),
            RawCard(42),
            RawCard(43),
        ];
        let leaf_idxs = vec![18, 17, 22];

        let mut board = Board::new(cards, stack, leaf_idxs);
        board.stack_idx = 11;
        board.moves = 18;

        let stack_moves: Vec<Move> = board.get_stack_moves().into_iter().collect::<Vec<_>>();
        assert_eq!(
            stack_moves,
            vec![(MatchType::Stack, 3, (RawCard(51), None))]
        );

        let moves = board.get_moves();
        assert_eq!(
            moves,
            vec![
                (MatchType::Stack, 3, (RawCard(51), None)),
                (MatchType::BoardStack, 4, (RawCard(2), Some(RawCard(48)))),
                (MatchType::BoardStack, 7, (RawCard(8), Some(RawCard(42)))),
                (MatchType::BoardStack, 10, (RawCard(8), Some(RawCard(3)))),
                (MatchType::BoardStack, 13, (RawCard(8), Some(RawCard(16)))),
                (MatchType::BoardStack, 14, (RawCard(8), Some(RawCard(29)))),
                (MatchType::BoardStack, 15, (RawCard(19), Some(RawCard(44))))
            ]
        );
    }
}
