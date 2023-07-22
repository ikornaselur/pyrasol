use colored::Colorize;
use pyramid_solver::validators::validate_board;
use pyramid_solver::Board;
use pyramid_solver::{parse_board, pretty_print_board, pretty_print_move};
use std::collections::HashSet;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// The board to solve
    ///
    /// The board to solve as a string of characters, where each character represents a card.
    /// Cards from 2-9 are just represented as their number, while 10, Jack, Queen, King, and Ace
    /// are represented as 0, j, q, k, and a respectively. Cards are read from left to right, top
    /// to bottom.
    /// Note: Ace can be either 1 or a
    ///
    /// Example:
    ///     jj6j88a95k3ka02j4q32k0767qk7
    board: String,

    /// The stack
    ///
    /// The stack as a string of characters, where each character represents a card. Cards from
    /// 2-9 are just represented as their number, while 10, Jack, Queen, King, and Ace are
    /// represented as 0, j, q, k, and a respectively. Cards are read from left to right.
    /// Note: Ace can be either 1 or a
    ///
    /// Example:
    ///     68480a55q69a2339527q4490
    stack: String,

    /// Verbose output
    #[arg(long, short, default_value_t = false)]
    verbose: bool,

    // Max depth to simulate
    #[arg(long, short, default_value_t = 60)]
    max_depth: usize,
}

fn main() {
    let args = Args::parse();

    let (board_cards, stack_cards) = parse_board(args.board, args.stack);
    validate_board(&board_cards, &stack_cards);
    let leaf_idxs: Vec<u8> = vec![21, 22, 23, 24, 25, 26, 27];
    let board = Board::new(board_cards, stack_cards, leaf_idxs);

    pretty_print_board(&board);

    let first_top_moves = 3;
    let first_games = 5;
    let top_moves = 2;

    let solution = simulate_games(
        board.clone(),
        args.max_depth,
        top_moves,
        first_top_moves,
        first_games,
        args.verbose,
    )
    .unwrap();

    describe_solution(board, solution, false);
}

fn describe_solution(mut board: Board, solution: Vec<usize>, verbose: bool) {
    let mut moves_made: i32 = 0;
    for move_num in solution.iter() {
        if verbose {
            pretty_print_board(&board);
        }

        let moves = board.get_moves();
        if moves.is_empty() {
            panic!("No moves left?");
        }

        if verbose {
            println!("Available moves:");
            for (idx, r#move) in moves.iter().enumerate() {
                if idx + 1 == *move_num {
                    print!("> ");
                }
                pretty_print_move(&board, idx as u8 + 1, *r#move, true);
            }
        }

        let r#move = match moves.get(*move_num - 1) {
            Some(x) => x,
            None => panic!("Move {} not found in moves: {:?}", move_num, moves),
        };

        if !verbose {
            pretty_print_move(&board, moves_made as u8, *r#move, true);
        }

        board.play_move(*r#move);

        moves_made += 1 + r#move.1;
    }
    println!("[{}] {}", board.moves, "All done!".green());
}

fn simulate_games(
    board: Board,
    max_depth: usize,
    top_moves: usize,
    first_top_moves: usize,
    first_games: usize,
    verbose: bool,
) -> Result<Vec<usize>, ()> {
    let mut seen_states: HashSet<String> = HashSet::new();

    // Pre-create 60 queues for different move counts
    let mut queues: Vec<Vec<(Board, Vec<usize>)>> = vec![vec![]; max_depth];

    // Initial board at 0 moves
    queues[0].push((board, vec![]));

    let mut queue_num = 0;
    while queue_num < max_depth {
        let queue = queues.get(queue_num).unwrap().clone();
        let queue_size = queue.len();

        if verbose {
            println!("Queue {} size: {}", queue_num, queue_size);
        }

        for (board, moves_made) in queue {
            if board.completed {
                println!(
                    "{}",
                    format!("Solution found with {} moves made", board.moves).green()
                );
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
                if *draws + board.moves + 1 >= max_depth as i32 {
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

                let sub_queue = queues.get_mut(new_board.moves as usize);
                match sub_queue {
                    Some(sub_queue) => sub_queue.push((new_board, moves_made)),
                    None => panic!("No queue for move count {}", new_board.moves),
                };
            }
        }

        queue_num += 1;
    }

    println!(
        "{}",
        format!("No solution found with a max depth of {}", max_depth).red()
    );

    Ok(vec![])
}
