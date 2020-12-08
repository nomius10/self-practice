use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() -> () {
    let mut vec : Vec<String> = vec![];

    let br = BufReader::new(File::open("input").unwrap());
    for line in br.lines() {
        vec.push(line.unwrap());
    }

    let mut total : usize = 0;
    for (i, line) in vec.iter().enumerate() {
        total += (line.chars().nth(i*3 % line.len()) == Some('#')) as usize;
    }
    println!("solution to part1: {}", total);

    total = 1;
    for (right, down) in [(1,1), (3,1), (5,1), (7,1), (1,2)].iter() {
        let mut subtotal : usize = 0;
        for (i, line) in vec.iter().enumerate() {
            if i % down != 0 { continue; }
            subtotal += (line.chars().nth((i/down)*right % line.len()) == Some('#')) as usize;
        }
        println!("{}", subtotal);
        total *= subtotal;
    }
    println!("solution to part2: {}", total);

    ()
}
