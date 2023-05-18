#[allow(dead_code)]
struct Board {
    white: u64,
    black: u64,
    n_moves: usize,
}

#[allow(dead_code)]
impl Board {
    fn new() -> Self {
        let white = (1u64 << 36)  + (1u64 << 27);
        let black = (1u64 << 35) + (1u64 << 28);
        let n_moves = 0;
        Board { white, black, n_moves }
    }

    fn show(&self) {
        let mut count = 0;
        print!("  12345678\n1 ");
        for i in (0..64).rev() {
            let pos = 1 << i;
            count += 1;
            if (self.white & pos) == pos {
                print!("○");
            } else if (self.black & pos) == pos {
                print!("●");
            } else {
                print!(".");
            }
            if count % 8 == 0 {
                print!("\n");
                if count / 8 < 8 {
                    print!("{} ", count / 8 + 1);
                }
            }
        }
    }
}

fn main() {
    let board = Board::new();
    board.show();
}
