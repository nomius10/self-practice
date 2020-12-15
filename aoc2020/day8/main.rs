type Program = Vec<Instruction>;

#[derive(Copy, Clone)]
enum Instruction {
    Nop(isize),
    Acc(isize),
    Jmp(isize),
    End
}


fn parse_instr(s : &str) -> Option<Instruction> {
    let mut it = s.split(" ");
    let (i_name, i_off) = (it.next()?, it.next()?.parse().ok()?);
    
    match i_name {
        "nop" => Some(Instruction::Nop(i_off)),
        "acc" => Some(Instruction::Acc(i_off)),
        "jmp" => Some(Instruction::Jmp(i_off)),
        _ => None?
    }
}

fn run_program(prog : &Program) -> (isize, isize) {
    let mut visited : Vec<bool> = std::iter::repeat(false).take(prog.len()).collect();
    let (mut ip, mut acc) = (0isize, 0isize);

    loop {
        if visited[ip as usize] == true {
            return (ip, acc);
        } else {
            visited[ip as usize] = true;
        }
        match prog[ip as usize] {
            Instruction::Nop(_)      => { ip += 1; },
            Instruction::Acc(count)  => { ip += 1; acc += count },
            Instruction::Jmp(offset) => { ip += offset },
            Instruction::End         => return (ip, acc)
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    let mut prog = input
        .split("\n")
        .filter_map(parse_instr)
        .collect::<Program>();
    prog.push(Instruction::End);

    let (ip, acc) = run_program(&prog);
    println!("part1: program stopped at ip {} with acc at {}", ip, acc);

    for i in 0..prog.len() {
        if !matches!(prog[i], Instruction::Nop(..) | Instruction::Jmp(..)) {
            continue;
        }
        let old = prog[i];
        let new = match prog[i] {
            Instruction::Nop(x) => Instruction::Jmp(x),
            Instruction::Jmp(x) => Instruction::Nop(x),
            _ => panic!()
        };

        prog[i] = new;
        let (ip, acc) = run_program(&prog);
        if matches!(prog[ip as usize], Instruction::End) {
            println!("part2: flipped instruction at {} and acc at {}", i, acc);
            break;
        }
        prog[i] = old;
    }
}

