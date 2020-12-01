use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

fn read<R: Read>(io : R) -> Result<Vec<u64>, Error> {
    let br = BufReader::new(io);
    br.lines()
        .filter(|lres| match lres.as_ref() {
            Ok(lstr) => lstr.chars().count() > 0,
            Err(_)   => false
        })
        .map(|lres| lres.and_then(
                |lstr| lstr.parse::<u64>().map_err(
                    |e| Error::new(ErrorKind::InvalidData, e))
                )
            )
        .collect()
}

fn part1(vec : &Vec<u64>, target : u64) -> Result<u64, Error> {
    let (mut tail, mut head) : (usize, usize) = (0, vec.len() - 1);

    while tail < head {
        let sum = vec[tail] + vec[head];
        
        if sum > target { head -= 1; continue; }
        if sum < target { tail += 1; continue; }

        return Ok(vec[tail] * vec[head]);
    }

    Err(Error::new(ErrorKind::NotFound, "Solution not found"))
}

fn part2(vec : &Vec<u64>, target : u64) -> Result<u64, Error> {
    for val in vec {
        let want = target - val;
        if let Ok(sub_res) = part1(&vec, want) {
            return Ok(val * sub_res);
        }
    }

    Err(Error::new(ErrorKind::NotFound, "Solution not found"))
}

fn main() -> Result<(), Error> {
    let mut vec : Vec<u64> = read(File::open("input")?)?;
    vec.sort();

    println!("solution to 1st part: {}", part1(&vec, 2020)?);
    println!("solution to 2nd part: {}", part2(&vec, 2020)?);

    Ok(())
}
