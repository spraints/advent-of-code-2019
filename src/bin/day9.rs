use spraints_advent_of_code_2019::intcode::{self, IntCodeMemory};
use std::sync::mpsc::{self, Sender};

fn main() {
    println!("--------------");
    println!("INTCODE ONLINE");
    println!("--------------");

    let program = intcode::read_program();

    part1(&program);
}

fn part1(program: &IntCodeMemory) {
    println!("PART 1");
    let (in_tx, in_rx) = mpsc::channel();
    let (out_tx, out_rx) = mpsc::channel();
    send_all(in_tx, vec![1]);
    intcode::run("part1", program.clone(), in_rx, out_tx, true);
    for x in out_rx {
        println!("OUTPUT: {:?}", x);
    }
}

fn send_all(tx: Sender<Option<intcode::Item>>, vals: Vec<intcode::Item>) {
    for x in vals {
        tx.send(Some(x)).expect("send should be ok");
    }
}
