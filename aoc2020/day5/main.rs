trait Bisectable {
    fn upper(&self) -> Self;
    fn lower(&self) -> Self;
}

impl Bisectable for std::ops::Range<usize> {
    fn upper(&self) -> std::ops::Range<usize> {
        (self.start+self.end+1)/2..self.end
    }

    fn lower(&self) -> std::ops::Range<usize> {
        self.start..(self.start+self.end)/2
    }
}

fn parse_lines(lines : Vec<&str>) -> Vec<(usize,usize)> {
    lines
        .into_iter()
        .map(|line| {
            line.chars().fold(
                ((0..127), (0..7)),
                |(row_range, col_range), chr| {
                    match chr {
                        'F' => (row_range.lower(), col_range),
                        'B' => (row_range.upper(), col_range),
                        'R' => (row_range, col_range.upper()),
                        'L' => (row_range, col_range.lower()),
                        _  => panic!("you don goofed")
                    }
                }
            )
        })
        .map(|(row_range, col_range)| {
            assert!(row_range.start == row_range.end, "s:{} e:{}", row_range.start, row_range.end);
            assert!(col_range.start == col_range.end, "s:{} e:{}", col_range.start, col_range.end);
            (row_range.start, col_range.start)
        })
        .collect::<Vec<(usize,usize)>>()
}

fn main() -> () {
    let bulk_text = std::fs::read_to_string("input").unwrap();
    let lines = bulk_text.lines().collect::<Vec<&str>>();
    let seats = parse_lines(lines);

    println!("solution to part1: {}", (&seats).into_iter().map(|(r, c)| r*8 + c).max().unwrap());

    let mut ids = seats.into_iter().map(|(r, c)| r*8 + c).collect::<Vec<usize>>();
    ids.sort();
    for i in 0..ids.len()-1 {
        if ids[i+1] - ids[i] == 2 {
            println!("solution to part2: {} ", ids[i]+1);
        }
    }

    ()
}
