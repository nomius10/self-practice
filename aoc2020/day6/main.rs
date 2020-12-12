use std::collections::HashSet;

fn part1(group : &str) -> u128 {
    group
        .chars()
        .filter(|c| ('a'..='z').contains(c))
        .collect::<HashSet<char>>()
        .len() as u128
}

fn part2(group : &str) -> u128 {
    group
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect::<HashSet<char>>())
        .fold(
            ('a'..='z').collect::<HashSet<char>>(), 
            |acc , x| acc.intersection(&x).map(|x| *x).collect()
            )
        .len() as u128
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    let (res1, res2) = (&input)
        .split("\n\n")
        .map(|x| (part1(x), part2(x)))
        .fold((0,0), |acc, x| (acc.0 + x.0, acc.1 + x.1));

    println!("solution to part1 is: {}", res1);
    println!("solution to part2 is: {}", res2);
}
