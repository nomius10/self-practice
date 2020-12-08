use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;
use std::process::exit;

fn is_valid_part1(hmap : &HashMap<String, String>) -> bool {
    for nk in ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"].iter() {
        if hmap.get(&nk.to_string()) == None {
            return false;
        }
    }
    return true;
}

fn is_valid_part2(hmap : &HashMap<String, String>) -> bool {
    return
        match hmap.get("byr") {
            Some(byr) => (1920..2003).contains(&byr.parse::<usize>().unwrap()),
            None => false
        } &
        match hmap.get("iyr") {
            Some(iyr) => (2010..2021).contains(&iyr.parse::<usize>().unwrap()),
            None => false
        } &
        match hmap.get("eyr") {
            Some(eyr) => (2020..2031).contains(&eyr.parse::<usize>().unwrap()),
            None => false
        } &
        match hmap.get("hgt") {
            Some(hgt) => {
                if hgt.contains("cm") {
                    (150..194).contains(&hgt.strip_suffix("cm").unwrap().parse::<usize>().unwrap())
                } else if hgt.contains("in") { 
                    (59..77).contains(&hgt.strip_suffix("in").unwrap().parse::<usize>().unwrap())
                } else {
                    false
                }
            },
            None => false
        } &
        match hmap.get("hcl") {
            Some(hcl) =>
                (hcl.len() == 7) &
                (hcl.chars().nth(0) == Some('#')) &
                hcl[1..].chars().fold(true, |acc, c| {
                    acc & (('0'..':').contains(&c as &char) | ('a'..'g').contains(&c as &char))
                }),
            None => false
        } &
        match hmap.get("ecl") {
            Some(ecl) => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&&ecl[..]),
            None => false
        } & 
        match hmap.get("pid") {
            Some(pid) => 
                (pid.len() == 9) &
                pid.chars().fold(true, |acc, c| {
                    acc & ('0'..':').contains(&c as &char)
                }),
            None => false
        }
}

fn main() -> () {
    let br = BufReader::new(File::open("input").unwrap());

    let mut crt_tags = HashMap::new();
    let mut part1_count = 0;
    let mut part2_count = 0;
    for line_res in br.lines() {
        if let Ok(line) = line_res {
            if line == "" {
                if is_valid_part1(&crt_tags) {
                    part1_count += 1;
                }
                if is_valid_part2(&crt_tags) {
                    part2_count += 1;
                }
                crt_tags.clear();
                continue;
            }

            for item in line.split(" ") {
                let mut it = item.split(":");
                let tag = it.next().unwrap();
                let val = it.next().unwrap();
                crt_tags.insert(tag.to_string(), val.to_string());
            }
        } else {
            exit(-2);
        }
    }

    println!("part1 solution: {}", part1_count);
    println!("part2 solution: {}", part2_count);
}
