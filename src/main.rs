use anyhow::Result;
use colored::Colorize;
use pyrasol::validators::validate_board;
use pyrasol::Board;
use pyrasol::{parse_board, pretty_print_board, pretty_print_move};
use rayon::prelude::*;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};

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

    /// Clear all the cards, including the stack
    #[arg(long, short, default_value_t = false)]
    clear_all: bool,

    /// Verbose output
    #[arg(long, short, default_value_t = false)]
    verbose: bool,

    /// Max depth to simulate
    #[arg(long, short, default_value_t = 60)]
    max_depth: usize,

    /// Increased options per iteration
    ///
    /// Run the search by playing additional options each iteration. By default the simulation will
    /// only run all moves that require no card draws and then 2-3 moves that require draws.
    /// Setting this flag will increase the options tried, but slow down the overall search.
    #[arg(long, short, default_value_t = false)]
    increased_options: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let (board_cards, stack_cards) = parse_board(args.board, args.stack);
    validate_board(&board_cards, &stack_cards);
    let leaf_idxs: Vec<usize> = vec![21, 22, 23, 24, 25, 26, 27];
    let board = Board::new(board_cards, stack_cards, leaf_idxs, args.clear_all);

    pretty_print_board(&board);

    let (first_top_moves, first_games, top_moves) = if args.increased_options {
        (5, 10, 3)
    } else {
        (3, 5, 2)
    };

    let solution = simulate_games(
        board.clone(),
        args.max_depth,
        top_moves,
        first_top_moves,
        first_games,
        args.verbose,
    )?;

    describe_solution(board, solution, false);

    Ok(())
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
) -> Result<Vec<usize>> {
    let seen_states = Arc::new(Mutex::new(HashSet::new()));

    // Pre-create 60 queues for different move counts
    let queues = Arc::new(Mutex::new(vec![vec![]; max_depth]));

    // Initial board at 0 moves
    queues
        .lock()
        .unwrap()
        .get_mut(0)
        .unwrap()
        .push((board, vec![]));

    let mut queue_num = 0;
    while queue_num < max_depth {
        let mut queue = queues.lock().unwrap().get(queue_num).unwrap().clone();
        let queue_size = queue.len();

        if verbose {
            println!("Queue {} size: {}", queue_num, queue_size);
        }

        let result = queue.par_drain(..).find_map_any(|(board, moves_made)| {
            if board.completed {
                println!(
                    "{}",
                    format!("Solution found with {} moves made", board.moves).green()
                );
                return Some(moves_made.clone());
            }

            let moves = board.get_moves();
            if moves.is_empty() {
                return None;
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
                let mut seen_states = seen_states.lock().unwrap();

                if seen_states.contains(&board_state) {
                    continue;
                }
                seen_states.insert(board_state);

                let mut moves_made = moves_made.clone();
                moves_made.push(moves_played + 1);

                match queues.lock().unwrap().get_mut(new_board.moves as usize) {
                    Some(sub_queue) => sub_queue.push((new_board, moves_made)),
                    None => panic!("No queue for move count {}", new_board.moves),
                };
            }
            None
        });

        if let Some(result) = result {
            return Ok(result);
        }

        queue_num += 1;
    }

    println!(
        "{}",
        format!("No solution found with a max depth of {}", max_depth).red()
    );

    Ok(vec![])
}
