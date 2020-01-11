use spraints_advent_of_code_2019::intcode::{self, IntCodeMemory, Item};

fn main() {
    println!("--------------");
    println!("INTCODE ONLINE");
    println!("--------------");

    let program = intcode::read_program();

    run_part("PART 1", &program, vec![1]);
    run_part("PART 2", &program, vec![2]);
}

fn run_part(name: &str, program: &IntCodeMemory, inputs: Vec<Item>) {
    println!("{}", name);
    println!("OUTPUT:");
    for output in intcode::run_easy(name, program.clone(), inputs, false) {
        println!(" -> {}", output);
    }
}
