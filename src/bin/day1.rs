use std::io;

fn main() {
    println!("Starting Fuel Counter-Upper!");

    let mut fuel = 0;

    loop {
        let mut line = String::new();
        let bytes = io::stdin()
            .read_line(&mut line)
            .expect("Error reading from stdin");
        if bytes == 0 {
            break;
        }
        match line.trim().parse() {
            Err(error) => println!("error parsing {}: {}", line.trim(), error),
            Ok(weight) => fuel += calc_fuel(weight),
        }
    }

    println!("got it: {}", fuel);
}

fn calc_fuel(weight: i32) -> i32 {
    let fuel = weight / 3 - 2;
    if fuel <= 0 {
        0
    } else {
        fuel + calc_fuel(fuel)
    }
}
