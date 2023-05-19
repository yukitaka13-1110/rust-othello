macro_rules! line {
    ($r:ident, $p:expr, $m:expr, $s:ident, $n:expr) => {
        $r = $m & $s($p, $n);
        $r |= $m & $s($r, $n);
        $r |= $m & $s($r, $n);
        $r |= $m & $s($r, $n);
        $r |= $m & $s($r, $n);
        $r |= $m & $s($r, $n);
    }
}

macro_rules! calc {
    ($r:ident, $b:ident, $p:expr, $o:expr, $m:expr, $n:expr) => {
        let mask = $o & $m;
        line!($b, $p, mask, shift_l, $n);
        $r |= shift_l($b, $n);
        line!($b, $p, mask, shift_r, $n);
        $r |= shift_r($b, $n);
    };
}

#[inline]
const fn shift_l(a: u64, b: u32) -> u64 {
    a << b
}

#[inline]
const fn shift_r(a: u64, b: u32) -> u64 {
    a >> b
}

#[allow(dead_code)]
struct Board {
    black: u64,
    white: u64,
    n_moves: usize,
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
    const POSITION_TABLE: [u64; 64] = [
        0,1,59,2,60,40,54,3,61,32,49,41,55,19,35,4,
        62,52,30,33,50,12,14,42,56,16,27,20,36,23,44,5,
        63,58,39,53,31,48,18,34,51,29,11,13,15,26,22,43,
        57,38,47,17,28,10,25,21,37,46,9,24,45,8,7,6
    ];

    pub fn new() -> Self {
        let black = 0x0000000810000000;
        let white = 0x0000001008000000;
        let n_moves = 0;
        Board { black, white, n_moves }
    }

    pub fn show(&self) {
        let mut v = vec!["."; 64];
        println!("  ABCDEFGH");
        for i in (0usize..64).rev() {
            let pos = 1 << i;
            let white = (self.white & pos) == pos;
            let black = (self.black & pos) == pos;
            if white || black {
                v[i] = if white {"○"} else {"●"};
            }
            if i%8 == 0 {
                println!("{} {}", 8-i/8, v[i..i+8].join(""))
            }
        }
    }

    pub fn split_moves(&self, n: u64) -> Vec<u64> {
        let mut moves = Vec::<u64>::new();
        let mut memo = n;
        while memo != 0 {
            let y = ((memo as i128) & -(memo as i128)) as u64;
            let index = ((y.wrapping_mul(Board::MOVE_POSITION_MASK)) >> 58) as u64;
            let n_shift = Board::POSITION_TABLE[index as usize];
            moves.push(1u64 << n_shift);
            memo ^= y;
        }
        moves
    }

    pub fn legal_moves(&self) -> u64 {
        let players = [self.black, self.white];
        let tp = players[self.n_moves % 2];
        let ntp = players[(self.n_moves+1) % 2];
        let blank_board = !(tp | ntp);
        let mut possible = 0u64;
        let mut _memo = 0u64;
        for (shift, mask) in Board::SHIFT_MASK_LIST {
            calc!(possible, _memo, tp, ntp, mask, shift);
        }
        possible & blank_board
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

fn main() {
    let row_label = ["H","G","F","E","D","C","B","A"];
    let board = Board::new();
    board.show();
    let moves_value = board.legal_moves();
    let moves = board.split_moves(moves_value);
    println!("Moves");
    for (i, m) in moves.iter().enumerate() {
        let n_shift = format!("{:b}", m).len()-1;
        println!("{}: {}{}", i+1, row_label[n_shift%8], 8-n_shift/8);
    }
}
