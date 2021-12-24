#![warn( clippy::pedantic )]
#![allow(clippy::cast_sign_loss)]
use std::io::BufRead;
use adventlib::aoc;

type Board = Vec<Vec<i8>>;
fn read_board<T: Iterator<Item=String>>(lines : &mut T) -> Option<Board> {
    lines.next()?; // eat empty line

    let mut board = Vec::new();
    for _ in 0..5 {
        let raw_line = lines.next()?;
        let line : Vec<_> = raw_line.split(' ').filter(|v| !v.is_empty()).map(|s| s.parse::<i8>().unwrap() ).collect();
        board.push(line);
    }

    Some( board )
}

fn find_winning_board(numbers : &[i8], mut boards : Vec<Board>) -> Option<(i8,Board)> {
    for number in numbers {
        for board in &mut boards {
            for row in board.iter_mut() {
                for cell in row {
                    if *cell == *number {
                        *cell = -1;
                    }
                }
            }

            for i in 0..5 {
                if board[i].iter().all(|v| *v == -1) || (0..5).map(|j| board[j][i] ).all(|v| v == -1 ) {
                    return Some((*number,board.clone()));
                }
            }
        }
    }

    None
}

fn find_last_winning_board(numbers : &[i8], mut boards : Vec<Board>) -> Option<(i8,Board)> {
    let mut winner = None;
    let mut finished_boards = vec![false; boards.len()];
    
    for number in numbers {
        for board_id in 0 .. boards.len() {
            if finished_boards[board_id] {
                continue;
            }

            let board = &mut boards[board_id];
            let mut board_updated = false;
            for row in board.iter_mut() {
                for cell in row {
                    if *cell == *number {
                        board_updated = true;
                        *cell = -1;
                    }
                }
            }

            if ! board_updated {
                continue;
            }

            for i in 0..5 {
                if board[i].iter().all(|v| *v == -1) || (0..5).map(|j| board[j][i] ).all(|v| v == -1 ) {
                    winner = Some((*number,board.clone()));
                    finished_boards[board_id] = true;
                }
            }
        }
    }

    winner
}

fn main() -> aoc::Result<()> {
    let reader = aoc::file("inputs/day4")?;

    let mut lines = reader.lines().map(Result::unwrap);

    let numbers : Vec<_> = lines.next().ok_or(aoc::Error::ParseFailed)?.split(',').map(|s| s.parse::<i8>().unwrap() ).collect();

    let mut boards : Vec<Board> = Vec::new();
    while let Some(board) = read_board(&mut lines) {
        boards.push(board);
    }

    let (winning_number,winning_board) = find_winning_board(&numbers,boards.clone()).unwrap();
    let sum : u32 = winning_board.into_iter().map(|row| row.into_iter().filter(|&v| v != -1).fold(0_u32,|prev,cur| prev + (cur as u32)) ).sum();

    println!("{:?}",sum*(winning_number as u32));

    let (last_winning_number,last_winning_board) = find_last_winning_board(&numbers,boards.clone()).unwrap();
    let last_sum : u32 = last_winning_board.into_iter().map(|row| row.into_iter().filter(|&v| v != -1).fold(0_u32,|prev,cur| prev + (cur as u32)) ).sum();

    println!("{:?}",last_sum*(last_winning_number as u32));

    Ok( () )
}