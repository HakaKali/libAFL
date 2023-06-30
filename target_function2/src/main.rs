use std::io;
//use rand::Rng;
use rand::prelude::SliceRandom;

fn main() {
    println!("Tic Tac Toe");

    let mut board = vec!['_'; 9];
    let mut player = 'X';

    loop {
        display_board(&board);

        if player == 'X' {
            println!("Player {}, choose a position (1-9): ", player);

            let position = read_input();

            if !is_valid_position(position) {
                println!("Invalid position.");
                continue;
            }

            let index = position - 1;

            if board[index] != '_' {
                println!("Position already taken.");
                continue;
            }

            board[index] = player;

            if is_winner(&board, player) {
                display_board(&board);
                println!("Player {}", player);
                break;
            } else if is_board_full(&board) {
                display_board(&board);
                println!("It is a TIE");
                break;
            }

            player = if player == 'X' { 'O' } else { 'X' };
        } else {
            let position = get_computer_move(&board);
            println!("Computer chooses position {}.", position);
            let index = position - 1;

            if board[index] != '_' {
                continue;
            }

            board[index] = player;

            if is_winner(&board, player) {
                display_board(&board);
                println!("Computer WIN!");
                break;
            } else if is_board_full(&board) {
                display_board(&board);
                println!("It is a TIE");
                break;
            }

            player = if player == 'X' { 'O' } else { 'X' };
        }
    }
}

fn display_board(board: &[char]) {
    println!("-------------");

    for row in board.chunks(3) {
        println!("| {} | {} | {} |", row[0], row[1], row[2]);
        println!("-------------");
    }
}

fn read_input() -> usize {
    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input.");

        match input.trim().parse() {
            Ok(position) => {
                if position >= 1 && position <= 9 {
                    return position;
                } else {
                    println!("Invalid position.")
                }
            }
            Err(_) => {
                println!("Invalid input.")
            }
        }
    }
}

fn is_valid_position(position: usize) -> bool {
    position >= 1 && position <= 9
}

fn is_winner(board: &[char], player: char) -> bool {
    let winning_combinations = [
        // Rows
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
        // Columns
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8],
        // Diagonals
        [0, 4, 8],
        [2, 4, 6],
    ];

    for combination in winning_combinations.iter() {
        if combination.iter().all(|&index| board[index] == player) {
            return true;
        }
    }
    false
}

fn is_board_full(board: &[char]) -> bool {
    board.iter().all(|&cell| cell != '_')
}

fn get_computer_move(board: &[char]) -> usize {
    let mut available_positions: Vec<usize> = Vec::new();

    for (index, &cell) in board.iter().enumerate() {
        if cell == '_' {
            available_positions.push(index + 1);
        }
    }

    let mut rng = rand::thread_rng();
    *available_positions.choose(&mut rng).unwrap()
}