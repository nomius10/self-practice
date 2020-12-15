struct CartProd<'a> {
    slice : &'a [u128],
    i : usize,
    j : usize
}

impl<'a> CartProd<'a> {
    fn new(slice : &'a [u128]) -> CartProd {
        CartProd { slice : slice, i : 0, j : 0 }
    }
}

impl<'a> Iterator for CartProd<'a> {
    type Item = (u128, u128);

    fn next(&mut self) -> Option<Self::Item> {
        self.j += 1;
        if self.j == self.slice.len() {
            self.i += 1; self.j = self.i + 1;
        }

        if self.i == self.slice.len() - 1 {
            None
        } else {
            Some((self.slice[self.i], self.slice[self.j]))
        }
    }
}

fn main() {
    let numbers : Vec<u128> =
        std::fs::read_to_string("input").unwrap()
        .split("\n")
        .filter_map(|lres| lres.parse().ok())
        .collect();

    let fault = numbers
        .windows(26)
        .find(|w| CartProd::new(&w[0..25]).all(|(a, b)| a+b != w[25]) )
        .unwrap()[25];

    println!("part1: {}", fault);

    'yolo: for i in 0..numbers.len() {
        let mut crt_sum = 0;
        for j in i..numbers.len() {
            crt_sum += numbers[j];
            if crt_sum == fault {
                println!("part 2: {}", 
                         numbers[i..j].iter().min().unwrap() +
                         numbers[i..j].iter().max().unwrap()
                );
                break 'yolo;
            }
            if crt_sum > fault { break };
        }
    }
}
