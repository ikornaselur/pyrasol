use pyramid_solver::board::{Board, Move, RawCard};
use pyramid_solver::pretty_print_move;

fn main() {
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

    let mut board = Board::new(cards, stack, leaf_idxs);

    for r#move in board.get_moves() {
        pretty_print_move(&board, 0, r#move);
    }

    let r#move: Move = board.get_moves().into_iter().next().unwrap();

    board.play_move(r#move);

    // Print all the moves, ordered by number of draws
    let mut moves: Vec<Move> = board.get_moves().into_iter().collect();
    moves.sort_by_key(|(_, draws, _)| *draws);
    for (idx, r#move) in moves.into_iter().enumerate() {
        pretty_print_move(&board, idx as u8, r#move);
    }
}
