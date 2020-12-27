pub fn raindrops(n: u32) -> String {
    let mut base = String::new();
    if n % 3 == 0 {
        base += "Pling"
    }
    if n % 5 == 0 {
        base += "Plang"
    }
    if n % 7 == 0 {
        base += "Plong"
    }
    if base.is_empty() {
        n.to_string()
    } else {
        base
    }
}
