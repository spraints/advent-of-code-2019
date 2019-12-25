use std::io;

fn main() {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Error reading stdin");
    let parts: Vec<u32> = line
        .trim()
        .split('-')
        .map(|s| s.parse().expect("Error parsing number"))
        .collect();
    let min = parts[0];
    let max = parts[1];
    println!("SEARCH IN RANGE [{}:{}]", min, max);

    let mut candidates = 0;
    for i in min..max {
        if is_candidate(i) {
            candidates += 1;
            println!("[{}] {}", candidates, i);
        }
    }
}

fn is_candidate(num: u32) -> bool {
    let d1 = num / 100_000;
    let d2 = (num / 10_000) % 10;
    let d3 = (num / 1_000) % 10;
    let d4 = (num / 100) % 10;
    let d5 = (num / 10) % 10;
    let d6 = num % 10;

    let increases_only = d1 <= d2 && d2 <= d3 && d3 <= d4 && d4 <= d5 && d5 <= d6;
    let has_double = (d1 == d2 && d2 != d3)
        || (d1 != d2 && d2 == d3 && d3 != d4)
        || (d2 != d3 && d3 == d4 && d4 != d5)
        || (d3 != d4 && d4 == d5 && d5 != d6)
        || (d4 != d5 && d5 == d6);

    increases_only && has_double
}
