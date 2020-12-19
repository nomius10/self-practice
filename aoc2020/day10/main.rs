use std::collections::HashMap;

/*
    Subproblem:

    Given a number of N bits, how many possible arrangements can we have such that there are no
    more than 2 consecutive 1 bits?
    
    Examples:
        0001 - valid
        1100 - valid
        1011 - valid
        1110 - invalid

    色々なもの：

    p - consecutive 1's
    r - remaining length

       す(_,r-1)
        v
    001_______
       ^
      す(1,r)

    す(p, r) = す(p+1, r-1) + す(p, r-1)
    す(3, _) = 0
    す(p, 0) = 1
*/
fn phi(prev : usize, rest : usize) -> usize {
    if prev == 3 { return 0 }
    if rest == 0 { return 1 }
    phi(prev+1, rest-1) + phi(prev, rest-1)
}

fn main() {

    let mut input = std::fs::read_to_string("input")
        .unwrap()
        .split("\n")
        .filter_map(|line| line.parse::<u64>().ok())
        .collect::<Vec<u64>>();

    input.sort();
    input.insert(0, 0);
    input.push(input.last().unwrap() + 3);
    print!("the input:\n{:?}\n", input);

    let deltas = input.iter()
        .zip(input[1..].iter())
        .map(|(a,b)| b-a)
        .collect::<Vec<u64>>();
    print!("the deltas:\n{:?}\n\n", deltas);

    /* Part 1 */
    let counts = deltas.iter()
        .fold(HashMap::new(), |mut acc, x| { *acc.entry(x).or_insert(0) += 1; acc });
    print!("counting occurences:\n{:#?}\n", counts);
    print!("part1 reult: {}\n\n", counts[&1] * counts[&3]);


    /* Part2 */
    let phi_test = (0..8)
        .map(|x| (x, phi(0, x)))
        .collect::<Vec<(usize,usize)>>();
    print!("testing the dynamic programing function:\n{:?}\n", phi_test);
    
    let leeway = deltas.iter()
        .scan(0, |state, &x| {
            let prev = *state;
            match x {
                3 => { *state  = 0; Some(prev) },
                1 => { *state += 1; Some(0) },
                _ => panic!()
            }
        })
        .filter(|x| *x > 1)
        .collect::<Vec<usize>>();
    print!("consecutive-1 intervals and their length:\n{:?}\n", leeway);

    let part2 : usize = leeway.iter()
        .map(|x| phi(0, x - 1))
        .product();
    println!("part2 result: {}", part2);

}
