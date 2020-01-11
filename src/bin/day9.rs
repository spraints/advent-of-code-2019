use spraints_advent_of_code_2019::intcode::{self, IntCodeMemory};

fn main() {
    println!("--------------");
    println!("INTCODE ONLINE");
    println!("--------------");

    let program = intcode::read_program();

    part1(&program);
}

fn part1(program: &IntCodeMemory) {
    println!("PART 1");
    println!("OUTPUT:");
    for output in intcode::run_easy("part1", program.clone(), vec![1], false) {
        println!(" -> {}", output);
    }
}
