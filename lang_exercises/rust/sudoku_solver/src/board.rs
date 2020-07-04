use std::fmt;
use std::fs::File;
use std::io::{Error, BufReader, BufRead};

use crate::bitset::BitSet;

pub struct Board {
    table : [[u8; 9]; 9],
    validity_set : [[BitSet; 9]; 9],
}

impl Board {
    fn empty() -> Board {
        Board{ 
            table : [[0;9];9], 
            validity_set : [[BitSet::full(); 9]; 9] 
        }
    }

    pub fn check_validities (&mut self) {
        for x in 0..9 {
            for y in 0..9 {
                self.validity_set[x][y] = self.get_vset(x, y)
            }
        }
    }

    pub fn get_least_restr(&self) -> (usize, usize) {
        let mut min_pos = (9, 9);
        let mut min_possible = 9;

        for x in 0..9 {
            for y in 0..9 {
                let possible = self.validity_set[x][y].count();
                if 1 < possible && possible < min_possible {
                    min_possible = possible;
                    min_pos = (x, y);
                }
            }
        }
        min_pos
    }

    fn get_vset(&self, x : usize, y : usize) -> BitSet {
        if self.table[x][y] != 0 {
            let mut bs = BitSet::empty();
            bs.set(self.table[x][y] - 1);
            return bs;    
        }

        let mut bs = BitSet::full();
        for yc in 0..9 {
            if self.table[x][yc] != 0 {
                bs.unset(self.table[x][yc] - 1);
            }
        }
        for xc in 0..9 {
            if self.table[xc][y] != 0 {
                bs.unset(self.table[xc][y] - 1);
            }
        }
        for xc in (x/3)..(x/3+1) {
            for yc in (y/3)..(y/3+1) {
                if self.table[xc][yc] != 0 {
                    bs.unset(self.table[xc][yc] - 1)
                }
            }
        }
        bs
    }
}

macro_rules! h_delim {
    ($f:expr) => {
        {
            for _ in 0..3 { write!($f, "+-------")?; }
            write!($f, "+\t")?;
            for _ in 0..3 { write!($f, "+-------------")?; }
            writeln!($f, "+")
        }
    };
}

fn h_line(f : &mut fmt::Formatter<'_>, arr : [u8; 9], arr2 : [BitSet; 9] ) -> fmt::Result {
    write!(f, "|")?;
    for (i, x) in arr.iter().enumerate() {
        let c = if *x == 0 {String::from("_")} else {x.to_string()};
        if i % 3 == 0 && i > 0 { 
            write!(f, " | {}", c)?; 
        } else {
            write!(f, " {}", c)?;
        }
    }
    write!(f, " |\t|")?;
    for (i, x) in arr2.iter().enumerate() {
        if i % 3 == 0 && i > 0 { 
            write!(f, " | {}", format!("{:o}", x))?; 
        } else {
            write!(f, " {}", format!("{:o}", x))?;
        }
    }
    writeln!(f, " |")
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..9 {
            if i % 3 == 0 {
                h_delim!(f)?;
            }
            h_line(f, self.table[i], self.validity_set[i])?;
        }
        h_delim!(f)?;
        Ok(())
    }
}


pub fn read_sudokus(filepath : &str) -> Result<Vec<Board>, Error> {
    
    let file = File::open(filepath)?;
    let buf_reader = BufReader::new(file);
    let mut it = buf_reader.lines();
    
    let num = it.next().unwrap().unwrap().parse::<usize>().unwrap();
    let mut v : Vec<Board> = Vec::with_capacity(num);
    
    while let Some(line) = it.next() {
        let line = line.unwrap();
        let mut board = Board::empty();
        for (i, c) in line.chars().enumerate() {
            board.table[i/9][i%9] = c.to_digit(10).unwrap() as u8;
        }
        v.push(board);
    }
    Ok(v)
}
