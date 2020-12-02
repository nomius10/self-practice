use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn parse_line<'a>(val : &'a str) -> (usize, usize, char, &'a str) {
    let mut split = val.split(": ");
    let p1   = split.next().unwrap();
    let pass = split.next().unwrap();
    assert!(split.next() == None);

    split = p1.split(" ");
    let iv  = split.next().unwrap();
    let chr = split.next().unwrap().chars().next().unwrap();
    assert!(split.next() == None);

    split = iv.split("-");
    let min = split.next().unwrap().parse::<usize>().unwrap();
    let max = split.next().unwrap().parse::<usize>().unwrap();
    assert!(split.next() == None);

    (min, max, chr, pass)
}

fn part1(val : &str) -> bool {
    let (min, max, chr, pass) = parse_line(val);
    let count : usize = pass.chars().map(|c| if c == chr {1} else {0}).sum();
    min <= count && count <= max
}

fn part2(val : &str) -> bool {
    let (pos1, pos2, chr, pass) = parse_line(val);
    (pass.chars().nth(pos1-1).unwrap() == chr) ^ (pass.chars().nth(pos2-1).unwrap() == chr)
}

fn main() -> Result<(), Error>{
    let br = BufReader::new(File::open("input")?);
    let total = br.lines()
        .map(|lres| part1(&lres.unwrap()) as usize)
        .sum::<usize>();
    println!("part1 solution: {}", total);

    let br = BufReader::new(File::open("input")?);
    let total = br.lines()
        .map(|lres| part2(&lres.unwrap()) as usize)
        .sum::<usize>();
    println!("part2 solution: {}", total);

    Ok(())
}
