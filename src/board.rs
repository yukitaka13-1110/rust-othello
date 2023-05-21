use std::cmp::Ordering;

macro_rules! line {
    ($start:expr, $data:expr, $shift:ident, $n:expr) => {
        {
            let mut result = $data & $shift($start, $n);
            result |= $data & $shift(result, $n);
            result |= $data & $shift(result, $n);
            result |= $data & $shift(result, $n);
            result |= $data & $shift(result, $n);
            result |= $data & $shift(result, $n);
            result
        }
    }
}

#[inline]
const fn shift_l(a: u64, b: u32) -> u64 {
    a << b
}

#[inline]
const fn shift_r(a: u64, b: u32) -> u64 {
    a >> b
}

#[derive(Clone, Copy)]
enum Piece {
    Empty,
    Black,
    White,
}

impl Piece {
    fn to_char(&self) -> char {
        match self {
            Piece::Empty => '.',
            Piece::Black => '●',
            Piece::White => '○',
        }
    }
}

#[derive(PartialEq)]
pub enum Player {
    Black,
    White,
}

pub enum GameResult {
    Winner(Player),
    Draw,
}

#[allow(dead_code)]
pub struct Board {
    black: u64,
    white: u64,
    turns: usize,
}

#[allow(dead_code)]
impl Board {
    const LR_EDGE_MASK: u64 = 0x7e7e7e7e7e7e7e7e;
    const TB_EDGE_MASK: u64 = 0x00FFFFFFFFFFFF00;
    const LTRB_EDGE_MASK: u64 = 0x007e7e7e7e7e7e00;
    const MOVE_POSITION_MASK: u64 = 0x03F566ED27179461;
    const SHIFT_MASK_LIST: [(u32, u64); 4] = [
        (1, Board::LR_EDGE_MASK),
        (8, Board::TB_EDGE_MASK),
        (7, Board::LTRB_EDGE_MASK),
        (9, Board::LTRB_EDGE_MASK),
    ];
    const POSITION_TABLE: [u8; 64] = [
        0,1,59,2,60,40,54,3,61,32,49,41,55,19,35,4,
        62,52,30,33,50,12,14,42,56,16,27,20,36,23,44,5,
        63,58,39,53,31,48,18,34,51,29,11,13,15,26,22,43,
        57,38,47,17,28,10,25,21,37,46,9,24,45,8,7,6
    ];

    pub fn new() -> Self {
        let black = 0x0000000810000000;
        let white = 0x0000001008000000;
        let n_moves = 0;
        Board { black, white, turns: n_moves }
    }

    pub fn turn(&self) -> Player {
        match self.turns % 2 {
            0 => Player::Black,
            1 => Player::White,
            _ => panic!(),
        }
    }

    pub fn turns(&self) -> usize {
        self.turns
    }

    pub fn end(&self) -> bool {
        (self.white ^ self.black).count_ones() == 64
    }

    pub fn pass(&self) -> Board {
        Board { black: self.black, white: self.white, turns: self.turns+1 }
    }

    pub fn show(&self) {
        println!("  ABCDEFGH");
        for i in 0..8 {
            let mut row = String::new();
            for j in 0..8 {
                let pos = 1 << (8*(7-i)+7-j);
                let piece = if (self.white & pos) == pos {
                    Piece::White
                } else if (self.black & pos) == pos {
                    Piece::Black
                } else {
                    Piece::Empty
                };
                row.push(piece.to_char());
            }
            println!("{} {}", i+1, row);
        }
    }

    pub fn split_moves(&self, n: u64) -> Vec<u64> {
        let mut moves = Vec::<u64>::new();
        let mut memo = n;
        while memo != 0 {
            let y = memo & !memo.wrapping_sub(1);
            let index = (y.wrapping_mul(Board::MOVE_POSITION_MASK)) >> 58;
            let n_shift = Board::POSITION_TABLE[index as usize];
            moves.push(1u64 << n_shift);
            memo ^= y;
        }
        moves
    }

    pub fn legal_moves(&self) -> u64 {
        #[inline]
        const fn calc(tp: u64, ntp: u64, mask: u64, shift: u32) -> u64 {
            let l = line!(tp, ntp & mask, shift_l, shift);
            let r = line!(tp, ntp & mask, shift_r, shift);
            shift_l(l, shift) | shift_r(r, shift)
        }

        let players = [self.black, self.white];
        let tp = players[self.turns % 2];
        let ntp = players[(self.turns+1) % 2];
        let blank_board = !(tp | ntp);
        let mut possible = 0;
        for (shift, mask) in Board::SHIFT_MASK_LIST {
            possible |= calc(tp, ntp, mask, shift);
        }
        possible & blank_board
    }

    pub fn reverse(&self, position: u64) -> Self {
        #[inline]
        const fn calc(tp: u64, ntp: u64, position: u64, mask: u64, shift: u32) -> u64 {
            let mask = ntp & mask;
            let l1 = line!(position, mask, shift_l, shift);
            let r1 = line!(tp, mask, shift_r, shift);
            let r2 = line!(position, mask, shift_r, shift);
            let l2 = line!(tp, mask, shift_l, shift);
            (l1 & r1) | (r2 & l2)
        }

        let players = [self.black, self.white];
        let tp = players[self.turns % 2];
        let ntp = players[(self.turns+1) % 2];
    
        let mut target = 0u64;
        for (shift, mask) in Board::SHIFT_MASK_LIST {
            target |= calc(tp, ntp, position, mask, shift);
        }
        let new_tp = tp ^ position ^ target;
        let new_ntp = ntp ^ target;
        let new_players = [new_tp, new_ntp];
        let black = new_players[self.turns % 2];
        let white = new_players[(self.turns+1) % 2];
        Board { black, white, turns: self.turns + 1 }
    }

    pub fn judge(&self) -> GameResult {
        let white_count = self.white.count_ones();
        let black_count = self.black.count_ones();
        
        match white_count.cmp(&black_count) {
            Ordering::Equal => GameResult::Draw,
            Ordering::Greater => GameResult::Winner(Player::White),
            Ordering::Less => GameResult::Winner(Player::Black),
        }
    }

    fn debug(board: u64) {
        for i in (0..64).rev() {
            print!("{}", (board & (1u64 << i)) >> i);
            if (64 - i) % 8 == 0 {
                println!("");
            }
        }
    }
}