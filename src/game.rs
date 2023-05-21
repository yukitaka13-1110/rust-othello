use std::{time::SystemTime, io};

use crate::board::{Board, Player, GameResult};

#[derive(Clone, Copy)]
#[derive(PartialEq)]
pub enum PlayerType {
    Human,
    Cpu,
}

#[derive(Clone, Copy)]
#[allow(dead_code)]
pub enum OthelloGameState {
    BeforeMatch,
    InMatch,
    MatchFinished,
}

pub struct OthelloGame {
    state: OthelloGameState,
    black_player: PlayerType,
    board: Board,
}

impl OthelloGame {
    const BLACK_LABEL: &'static str = &"黒[●]";
    const WHITE_LABEL: &'static str = &"白[○]";
    const HUMAN_LABEL: &'static str = &"あなた";
    const CPU_LABEL: &'static str = &"CPU";

    pub fn new() -> Self {
        OthelloGame {
            state: OthelloGameState::BeforeMatch,
            black_player: PlayerType::Human,
            board: Board::new(),
        }
    }

    pub fn configure(&mut self) {
        let black_l = OthelloGame::BLACK_LABEL;
        let white_l = OthelloGame::WHITE_LABEL;
        let human_l = OthelloGame::HUMAN_LABEL;
        let cpu_l = OthelloGame::CPU_LABEL;
        loop {
            let mut input = String::new();
            println!("現在の設定は、{}: {} , {}: {}", black_l, human_l, white_l, cpu_l);
            println!("黒番・白番を入れ替えますか？ 1: 入れ替える, 2: そのまま");
            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    if let Ok(n) = input.trim().parse::<usize>() {
                        if n <= 0 && n > 2 {
                            println!("1 or 2 を入力してください");
                            continue;
                        } else {
                            if n == 1 {
                                self.black_player = PlayerType::Cpu;
                            }
                            return;
                        }
                    }
                }
                Err(_) => {
                    println!("1 or 2 を入力してください");
                }
            }
        }
    }

    pub fn start(&mut self) {
        let mut pass = false;
        let row_label = ["H","G","F","E","D","C","B","A"];
        loop {
            self.board.show();
            let moves_value = self.board.legal_moves();
            let moves = self.board.split_moves(moves_value);
            let black_turn = self.board.turn() == Player::Black;
            let p_label = if black_turn {"黒[●]"} else {"白[○]"};
            
            if moves.len() != 0 {
                println!("{}手目 - {} の手番です", self.board.turns() + 1, p_label);
                println!("Moves");
                for (i, m) in moves.iter().enumerate() {
                    let n_shift = format!("{:b}", m).len()-1;
                    println!("{}: {}{}", i+1, row_label[n_shift%8], 8-n_shift/8);
                }
                let input = if black_turn {
                    let human = self.black_player == PlayerType::Human;
                    if human {OthelloGame::human_input} else {OthelloGame::cpu_input}
                } else {
                    let human = self.black_player == PlayerType::Cpu;
                    if human {OthelloGame::human_input} else {OthelloGame::cpu_input}
                };
                let index = input(moves.len());
                self.board = self.board.reverse(moves[index-1]);
                pass = false;
                if self.board.end() {
                    self.state = OthelloGameState::MatchFinished;
                    return;
                }
            } else {
                println!("{} の手番ですが指す手がないためパスします", p_label);
                self.board = self.board.pass();
                if pass {
                    self.state = OthelloGameState::MatchFinished;
                    return;
                }
                pass = true;
            }
        }
    }

    pub fn results(&self) {
        let winner = self.board.judge();
        match winner {
            GameResult::Winner(player) => {
                match player {
                    Player::Black => println!("黒[●] の勝ちです。"),
                    Player::White => println!("白[○] の勝ちです。"),
                }
            }
            GameResult::Draw => println!("引き分けです。"),
        }
    }

    pub fn continue_or_not(&self) -> bool {
        loop {
            let mut input = String::new();
            println!("続ける or 終了する？ 1: 続ける, 2: 終了する");
            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    if let Ok(n) = input.trim().parse::<usize>() {
                        if n <= 0 && n > 2 {
                            println!("1 or 2 を入力してください");
                            continue;
                        } else {
                            return n == 1;
                        }
                    }
                }
                Err(_) => {
                    println!("1 or 2 を入力してください");
                }
            }
        }
    }

    fn cpu_input(n: usize) -> usize {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Failed to obtain timestamp")
            .as_nanos();
        (timestamp as usize % n) + 1
    }

    fn human_input(moves: usize) -> usize {
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
}