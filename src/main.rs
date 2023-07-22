use pyramid_solver::board::Board;
use pyramid_solver::{parse_board, pretty_print_board, pretty_print_move};
use std::collections::HashSet;

fn main() {
    let (cards, stack) = parse_board(
        "jj6j88a95k3ka02j4q32k0767qk7".to_string(),
        "68480a55q69a2339527q4490".to_string(),
    );
    let leaf_idxs: Vec<u8> = vec![21, 22, 23, 24, 25, 26, 27];
    let board = Board::new(cards, stack, leaf_idxs);
    pretty_print_board(&board);
    let first_top_moves = 3;
    let first_games = 5;
    let top_moves = 2;

    let solution =
        simulate_games(board.clone(), 50, top_moves, first_top_moves, first_games).unwrap();
    describe_solution(board, solution);
}

fn describe_solution(mut board: Board, solution: Vec<usize>) {
    let mut moves_made: i32 = 0;
    println!("Solution: {:?}", solution);
    for move_num in solution.iter() {
        pretty_print_board(&board);

        let moves = board.get_moves();
        if moves.is_empty() {
            panic!("No moves left?");
        }

        println!("All options ({} was picked):", move_num);
        for (idx, r#move) in moves.iter().enumerate() {
            pretty_print_move(&board, idx as u8 + 1, *r#move);
        }

        let r#move = match moves.get(*move_num - 1) {
            Some(x) => x,
            None => panic!("Move {} not found in moves: {:?}", move_num, moves),
        };

        pretty_print_move(&board, moves_made as u8, *r#move);
        
        board.play_move(*r#move);

        moves_made += 1 + r#move.1;
    }
}

fn simulate_games(
    board: Board,
    known_min_moves: usize,
    top_moves: usize,
    first_top_moves: usize,
    first_games: usize,
) -> Result<Vec<usize>, ()> {
    let mut seen_states: HashSet<String> = HashSet::new();

    // Pre-create 60 queues for different move counts
    let mut queues: Vec<Vec<(Board, Vec<usize>)>> = vec![vec![]; 60];

    // Initial board at 0 moves
    queues[0].push((board, vec![]));

    let mut queue_num = 0;
    while queue_num < 60 {
        let queue = queues.get(queue_num).unwrap().clone();
        let queue_size = queue.len();

        println!("Queue {} size: {}", queue_num, queue_size);

        for (board, moves_made) in queue {
            if board.completed {
                return Ok(moves_made);
            }

            let moves = board.get_moves();
            if moves.is_empty() {
                continue;
            }
            // Sort first by draws and then by the card being removed

            let max_moves = if board.moves as usize <= first_games {
                first_top_moves
            } else {
                top_moves
            };

            for (moves_played, r#move) in moves.iter().enumerate() {
                let (_, draws, _) = r#move;

                // Always play all no draw moves
                if *draws > 0 && moves_played > max_moves {
                    break;
                }
                if *draws + board.moves > known_min_moves as i32 {
                    break;
                }

                let mut new_board: Board = board.clone();
                new_board.play_move(*r#move);

                let board_state = new_board.get_state();
                if seen_states.contains(&board_state) {
                    continue;
                }
                seen_states.insert(board_state);

                let mut moves_made = moves_made.clone();
                moves_made.push(moves_played + 1);

                let sub_queue = queues.get_mut(new_board.moves as usize).unwrap();
                sub_queue.push((new_board, moves_made));
            }
        }

        queue_num += 1;
    }

    Ok(vec![])
}
