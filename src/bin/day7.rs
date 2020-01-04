use std::io;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex1() {
        let program = vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];
        assert_eq!(43210, part1(&program, false));
    }

    #[test]
    fn test_ex2() {
        let program = vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ];
        assert_eq!(54321, part1(&program, false));
    }

    #[test]
    fn test_ex3() {
        let program = vec![
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ];
        assert_eq!(65210, part1(&program, false));
    }

    #[test]
    fn test_ex4() {
        let program = vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ];
        assert_eq!(139629729, part2(&program, false));
    }

    #[test]
    fn test_ex5() {
        let program = vec![
            3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
            -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
            53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
        ];
        assert_eq!(18216, part2(&program, false));
    }
}

fn main() {
    println!("--------------");
    println!("INTCODE ONLINE");
    println!("--------------");

    let program = intcode_read_program();

    part1(&program, false);

    part2(&program, false);
}

fn part2(program: &IntCodeMemory, strict: bool) -> i32 {
    println!("PART 2");
    println!("------");
    let mut max_out = 0;
    for inputs in all_perms([5, 6, 7, 8, 9]) {
        let out = try_inputs2(&program, inputs, strict);
        if out > max_out {
            max_out = out;
        }
    }
    println!("MAX OUTPUT: {}", max_out);
    max_out
}

fn try_inputs2(program: &IntCodeMemory, inputs: [i32; 5], strict: bool) -> i32 {
    if strict {
        println!("TRY {:?}", inputs);
    }

    let (first_tx, mut last_rx) = mpsc::channel();
    first_tx.send(Some(0)).unwrap();

    let mut threads = vec![];
    for input in &inputs {
        let (in_tx, in_rx) = mpsc::channel();
        let (out_tx, out_rx) = mpsc::channel();

        let input = *input;

        let computer = IntCodeComputer {
            name: format!("[{}]", input),
            memory: program.clone(),
            inputs: in_rx,
            outputs: out_tx,
            verbose: false,
        };

        threads.push(thread::spawn(move || {
            intcode_send(&in_tx, input);
            copy_chan(&format!("{} input", input), last_rx, in_tx);
        }));

        threads.push(thread::spawn(move || {
            //let name = computer.name.clone();
            intcode_run(computer);
            //println!("{} finished!", name);
        }));

        last_rx = out_rx;
    }

    let (final_tx, final_rx) = mpsc::channel();
    threads.push(thread::spawn(move || {
        for val in last_rx {
            match val {
                None => {
                    first_tx.send(val).unwrap_or(());
                    break;
                }
                Some(_) => {
                    final_tx
                        .send(val)
                        .expect("expect to be able to send to the final thing");
                    if let Err(msg) = first_tx.send(val) {
                        println!("Couldn't feed value back to the start: {}", msg);
                        break;
                    }
                }
            }
        }
    }));

    for t in threads {
        if strict {
            t.join().unwrap();
        } else {
            t.join().unwrap_or(());
        }
    }

    let mut last_val = None;
    for val in final_rx {
        last_val = val;
    }
    if strict {
        last_val.expect("expect at least one output value")
    } else {
        last_val.unwrap_or(0)
    }
}

fn part1(program: &IntCodeMemory, strict: bool) -> i32 {
    println!("PART 1");
    println!("------");
    let mut max_out = 0;
    for inputs in all_perms([0, 1, 2, 3, 4]) {
        let out = try_inputs(&program, inputs, strict);
        if out > max_out {
            max_out = out;
        }
    }
    println!("MAX OUTPUT: {}", max_out);
    max_out
}

fn try_inputs(program: &IntCodeMemory, inputs: [i32; 5], strict: bool) -> i32 {
    if strict {
        println!("TRY {:?}", inputs);
    }

    let (last_tx, mut last_rx) = mpsc::channel();
    intcode_send(&last_tx, 0);

    let mut threads = vec![];

    for input in &inputs {
        let (in_tx, in_rx) = mpsc::channel();
        let (out_tx, out_rx) = mpsc::channel();

        let input = *input;

        let computer = IntCodeComputer {
            name: format!("[{}]", input),
            memory: program.clone(),
            inputs: in_rx,
            outputs: out_tx,
            verbose: false,
        };

        threads.push(thread::spawn(move || {
            intcode_send(&in_tx, input);
            intcode_send(&in_tx, last_rx.recv().unwrap().unwrap());
        }));

        threads.push(thread::spawn(move || {
            intcode_run(computer);
        }));

        last_rx = out_rx;
    }

    for t in threads {
        if strict {
            t.join().unwrap();
        } else {
            t.join().unwrap_or(());
        }
    }

    if strict {
        last_rx
            .recv()
            .expect("last program should output one value")
            .unwrap()
    } else {
        last_rx.recv().unwrap_or(None).unwrap_or(0)
    }
}

fn intcode_send(chan: &Sender<Option<i32>>, val: i32) {
    chan.send(Some(val))
        .expect(&format!("should be able to send {} to {:?}", val, chan));
}

const COPY_CHAN_VERBOSE: bool = false;
fn copy_chan(label: &str, rx: Receiver<Option<i32>>, tx: Sender<Option<i32>>) {
    for val in rx {
        match val {
            None => {
                // Try to send it, but don't care if it fails.
                tx.send(val).unwrap_or(());
                break;
            }
            Some(_) => {
                // Try to send it, and complain if it doesn't work.
                if let Err(msg) = tx.send(val) {
                    if COPY_CHAN_VERBOSE {
                        println!("[{}] ERROR SENDING {:?}! {}", label, val, msg);
                    }
                    break;
                }
            }
        }
    }
    //println!("{} finished", label);
}

fn all_perms(vals: [i32; 5]) -> Vec<[i32; 5]> {
    let mut res = vec![];
    let mut vals = vals.clone();
    heap_permutation(&mut res, &mut vals, 5);
    res
}

fn heap_permutation(res: &mut Vec<[i32; 5]>, vals: &mut [i32; 5], size: usize) {
    if size == 1 {
        res.push(vals.clone());
        return;
    }

    for i in 0..size {
        heap_permutation(res, vals, size - 1);

        let x = vals[size - 1];
        if size % 2 == 1 {
            vals[size - 1] = vals[0];
            vals[0] = x;
        } else {
            vals[size - 1] = vals[i];
            vals[i] = x;
        }
    }
}

//////////
// INTCODE

struct IntCodeComputer {
    name: String,
    memory: IntCodeMemory,
    inputs: Receiver<Option<i32>>,
    outputs: Sender<Option<i32>>,
    verbose: bool,
}

trait IntCodeInput {
    fn read_int(&mut self) -> i32;
}

trait IntCodeOutput {
    fn write_int(&mut self, _: i32);
}

type IntCodeMemory = Vec<i32>;

fn intcode_read_program() -> IntCodeMemory {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Error reading program from STDIN");
    let parts = line.trim().split(',');
    parts
        .map(|s| s.parse().expect("Error parsing int"))
        .collect()
}

fn intcode_run(mut computer: IntCodeComputer) {
    let opcodes = [
        intcode_op_add,
        intcode_op_mult,
        intcode_op_input,
        intcode_op_output,
        intcode_op_jump_if_true,
        intcode_op_jump_if_false,
        intcode_op_lt,
        intcode_op_eq,
    ];

    let mut pc = 0;
    if computer.verbose {
        println!("[{}/{}] {:?}", computer.name, pc, computer.memory);
    }
    loop {
        if computer.verbose {
            println!(
                " {}: ... {:?} ...",
                computer.name,
                computer.memory.get(pc..pc + 4)
            );
        }
        let op = computer.memory[pc] as usize;
        let opcode = op % 100;
        if opcode == 99 {
            break;
        } else {
            let opfn = opcodes[opcode - 1];
            pc = opfn(&mut computer, intcode_modes(op / 100), pc);
            if computer.verbose {
                println!("[{}/{}] {:?}", computer.name, pc, computer.memory);
            }
        }
    }

    computer.outputs.send(None).unwrap_or(());
}

fn intcode_modes(modes: usize) -> IntCodeModesIter {
    IntCodeModesIter { modes }
}

struct IntCodeModesIter {
    modes: usize,
}

enum ModeType {
    Position,
    Immediate,
}

impl Iterator for IntCodeModesIter {
    type Item = ModeType;
    fn next(&mut self) -> Option<Self::Item> {
        let cur = self.modes % 10;
        self.modes = self.modes / 10;
        Some(if cur == 0 {
            ModeType::Position
        } else {
            ModeType::Immediate
        })
    }
}

fn intcode_get_params(
    memory: &IntCodeMemory,
    modes: IntCodeModesIter,
    pc: usize,
    count: usize,
) -> IntCodeMemory {
    let params: IntCodeMemory = memory[pc + 1..pc + 1 + count]
        .into_iter()
        .zip(modes)
        .map(|(raw, mode)| match mode {
            ModeType::Position => memory[*raw as usize],
            ModeType::Immediate => *raw,
        })
        .collect();
    assert_eq!(count, params.len());
    params
}

fn intcode_op_add(computer: &mut IntCodeComputer, modes: IntCodeModesIter, pc: usize) -> usize {
    let params = intcode_get_params(&computer.memory, modes, pc, 2);
    let arg1 = params[0];
    let arg2 = params[1];
    let dest_addr = computer.memory[pc + 3] as usize;
    computer.memory[dest_addr] = arg1 + arg2;
    pc + 4
}

fn intcode_op_mult(computer: &mut IntCodeComputer, modes: IntCodeModesIter, pc: usize) -> usize {
    let params = intcode_get_params(&computer.memory, modes, pc, 2);
    let arg1 = params[0];
    let arg2 = params[1];
    let dest_addr = computer.memory[pc + 3] as usize;
    computer.memory[dest_addr] = arg1 * arg2;
    pc + 4
}

fn intcode_op_input(computer: &mut IntCodeComputer, _: IntCodeModesIter, pc: usize) -> usize {
    let dest_addr = computer.memory[pc + 1] as usize;
    match computer.inputs.recv() {
        Err(msg) => panic!("{}: receive error: {}", computer.name, msg),
        Ok(optval) => {
            match optval {
                None => panic!(
                    "{}: expected a value to be available, but found None!",
                    computer.name
                ),
                Some(val) => {
                    if computer.verbose {
                        println!("  ({}: read: {})", computer.name, val);
                    }
                    computer.memory[dest_addr] = val;
                }
            };
        }
    };
    pc + 2
}

fn intcode_op_output(computer: &mut IntCodeComputer, modes: IntCodeModesIter, pc: usize) -> usize {
    let params = intcode_get_params(&computer.memory, modes, pc, 1);
    let val = params[0];
    if computer.verbose {
        println!("  ({}: output: {})", computer.name, val);
    }
    if let Err(msg) = computer.outputs.send(Some(val)) {
        panic!("{}: send error: {}", computer.name, msg);
    }
    pc + 2
}

fn intcode_op_jump_if_true(
    computer: &mut IntCodeComputer,
    modes: IntCodeModesIter,
    pc: usize,
) -> usize {
    intcode_jump_if(true, computer, modes, pc)
}

fn intcode_op_jump_if_false(
    computer: &mut IntCodeComputer,
    modes: IntCodeModesIter,
    pc: usize,
) -> usize {
    intcode_jump_if(false, computer, modes, pc)
}

fn intcode_jump_if(
    cond: bool,
    computer: &mut IntCodeComputer,
    modes: IntCodeModesIter,
    pc: usize,
) -> usize {
    let params = intcode_get_params(&computer.memory, modes, pc, 2);
    let arg1 = params[0];
    let arg2 = params[1];
    if (cond && arg1 != 0) || (!cond && arg1 == 0) {
        arg2 as usize
    } else {
        pc + 3
    }
}

fn intcode_op_lt(computer: &mut IntCodeComputer, modes: IntCodeModesIter, pc: usize) -> usize {
    let params = intcode_get_params(&computer.memory, modes, pc, 2);
    let arg1 = params[0];
    let arg2 = params[1];
    let dest_addr = computer.memory[pc + 3] as usize;
    computer.memory[dest_addr] = if arg1 < arg2 { 1 } else { 0 };
    pc + 4
}

fn intcode_op_eq(computer: &mut IntCodeComputer, modes: IntCodeModesIter, pc: usize) -> usize {
    let params = intcode_get_params(&computer.memory, modes, pc, 2);
    let arg1 = params[0];
    let arg2 = params[1];
    let dest_addr = computer.memory[pc + 3] as usize;
    computer.memory[dest_addr] = if arg1 == arg2 { 1 } else { 0 };
    pc + 4
}

////////
// I/O

//fn intcode_stdin_input() -> Box<dyn IntCodeInput> {
//    Box::new(IntCodeStdinInput {})
//}
//
//struct IntCodeStdinInput {}
//
//impl IntCodeInput for IntCodeStdinInput {
//    fn read_int(&mut self) -> i32 {
//        let mut line = String::new();
//        io::stdin()
//            .read_line(&mut line)
//            .expect("Error reading input from STDIN");
//        line.trim().parse().expect("Error parsing int for input")
//    }
//}
