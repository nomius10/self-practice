use std::collections::{HashMap, HashSet};
use std::iter::{FromIterator, Iterator};

/////////////////////////////////
////////////// BAG //////////////

#[derive(Eq, Hash, Default)]
struct Bag<'a> {
    color : &'a str,
    count : u128
}

impl std::fmt::Display for Bag<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}[{}]", self.count, self.color)
    }
}

impl<'d> std::fmt::Display for Graph<Bag<'d>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (root, childs) in &self.table {
            write!(f, "{} -> ", root)?;
            for child in childs {
                write!(f, "{} ", child)?;
            }
            writeln!(f, "")?;
        }
        write!(f, "total items: {}", self.table.len())
    }
}

impl PartialEq for Bag<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.count == other.count && self.color == other.color
    }
}

impl<'b> Bag<'b> {
    fn from_str(s : &'b str) -> Option<Self> {
        if s == "no other bags" {
            return None;
        }

        let mut fsp = s.find(' ').unwrap();
        let count : u128 = match s[0..fsp].parse::<u128>() {
            Ok(c)  => { fsp += 1; c }, 
            Err(_) => { fsp  = 0; 1 }
        };

        let lsp = s.find(" bag").unwrap();
        let color : &str = &s[fsp..lsp];
        
        Some(Bag { count : count, color : color })
    }
}

/////////////////////////////////////
//////////////// GRAPH //////////////

// I have to create my own struct as impl foreign traits on foreign structs is forbid
type LLGraph<T> = HashMap<T, Vec<T>>;

struct Graph<T> {
    table : LLGraph<T> 
}

impl<'c> FromIterator<Vec<&'c str>> for Graph<Bag<'c>> {
    fn from_iter<I>(iter : I) -> Self where
        I : IntoIterator<Item = Vec<&'c str>>
    {
        let mut table : LLGraph<Bag> = LLGraph::default();

        for vect in iter {
            let key  = Bag::from_str(vect[0]).unwrap();
            let vals = vect[1..]
                .iter()
                .filter_map(|x| Bag::from_str(x)) // filters None values, also unwraps
                .collect::<Vec<Bag<'c>>>();
            table.insert(key, vals);
        }

        Graph::<Bag> { table : table } 
    }
}

impl<'a> Graph<Bag<'a>> {
    fn part1<'b : 'a>(&self, node : &Bag<'b>, set : &mut HashSet<&'a str>) {
        let key = Bag { count : 1, color : node.color };
        set.insert(key.color);

        if let Some(childs) = self.table.get(&key) {
            for child in childs {
                self.part1(child, set)
            }
        }
    }

    fn part2(&self, node : &Bag, mut crt_prod : u128) -> u128 {
        let key = Bag { count : 1, color : node.color };
        crt_prod *= node.count;
        
        if let Some(childs) = self.table.get(&key) {
            crt_prod + childs.iter().map(|bag| self.part2(bag, crt_prod)).sum::<u128>()
        } else {
            crt_prod
        }
    }

    fn reverse(&self) -> Graph<Bag> {
        let mut table = LLGraph::default();

        for (key, vals) in self.table.iter() {
            for val in vals.iter() {
                let new_key = Bag { count : 1, color : val.color };
                let new_val = Bag { count : val.count, color : key.color };

                if let Some(vec) = table.get_mut(&new_key) {
                    vec.push(new_val);
                } else {
                    table.insert(new_key, vec![new_val]);
                }
            }
        }

        Graph { table : table }
    }
}

///////////////////////////////////////
////////////// the rest ///////////////

fn parse_line(line : &str) -> Vec<&str> {
    let mut it = line.split(" contain ");
    let mut v : Vec<&str> = vec![it.next().unwrap()];
    v.extend(it.next().unwrap().split(", "));
    v
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    let graph = input
        .split(".\n")
        .filter(|line| !line.is_empty()) // TODO: make parse_line return option
        .map(parse_line)
        .collect::<Graph<Bag>>();
    let rev_graph = graph.reverse();
//    println!("{}\n", graph);
//    println!("{}\n", rev_graph);

    let target = Bag::from_str("shiny gold bag").unwrap();

    let set = &mut HashSet::new();
    rev_graph.part1(&target, set);
    println!("solution to part1: {}", set.len() - 1);

    println!("solution to part2: {}", graph.part2(&target, 1) - 1);
}
