mod bitset;
mod board;

use board::{Board, read_sudokus};


fn main() {
    let mut sudokus = read_sudokus("data/all_17_clue_sudokus.txt").unwrap();
    let mut s1 = &mut sudokus[0];
    s1.check_validities();
    println!("{}", s1);
    println!("{:?}", s1.get_least_restr());
}
