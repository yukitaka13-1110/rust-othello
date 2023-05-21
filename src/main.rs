pub mod board;

use std::{io, time::SystemTime};

use crate::board::{Board, Player, GameResult};

fn get_input(moves: usize) -> usize {
    loop {
        let mut input = String::new();
        println!("指手の数字を入力してください↓");
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                if let Ok(n) = input.trim().parse::<usize>() {
                    if n < 1 || n > moves {
                        println!("1~{}の数字を入力してください", moves);
                        continue;
                    } else {
                        return n;
                    }
                }
            }
            Err(_) => {
                println!("1~{}の数字を入力してください", moves);
            }
        }
    }
}

fn generate_input(n: usize) -> usize {
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Failed to obtain timestamp")
        .as_nanos();
    (timestamp as usize % n) + 1
}

fn output_winner(winner: GameResult, w: &str, b: &str) {
    match winner {
        GameResult::Winner(player) => {
            match player {
                Player::Black => println!("{} の勝ちです。ゲームを終了します", b),
                Player::White => println!("{} の勝ちです。ゲームを終了します", w),
            }
        }
        GameResult::Draw => println!("引き分けです。ゲームを終了します"),
    }
}

fn main() {
    let row_label = ["H","G","F","E","D","C","B","A"];
    const BLACK_LABEL: &str = "黒●";
    const WHITE_LABEL: &str = "白○";
    let mut board = Board::new();
    let mut pass = false;

    loop {
        board.show();
        let moves_value = board.legal_moves();
        let moves = board.split_moves(moves_value);
        if moves.len() != 0 {
            let p_label = if board.turn() == Player::Black { BLACK_LABEL } else { WHITE_LABEL };
            println!("{}手目 - {} の手番です", board.turns() + 1, p_label);

            println!("Moves");
            for (i, m) in moves.iter().enumerate() {
                let n_shift = format!("{:b}", m).len()-1;
                println!("{}: {}{}", i+1, row_label[n_shift%8], 8-n_shift/8);
            }
            let index = if board.turn() == Player::Black {
                get_input(moves.len())
            } else {
                generate_input(moves.len())
            };
            board = board.reverse(moves[index-1]);
            pass = false;
            if board.end() {
                board.show();
                output_winner(board.judge(), WHITE_LABEL, BLACK_LABEL);
                return;
            }
            
        } else {
            println!("{} の手番ですが指す手がないためパスします", if board.turn() == Player::Black {BLACK_LABEL} else {WHITE_LABEL});
            board = board.pass();
            if pass {
                output_winner(board.judge(), WHITE_LABEL, BLACK_LABEL);
                return;
            }
            pass = true;
        }
    }
}
