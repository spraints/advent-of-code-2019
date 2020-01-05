//////////
// INTCODE

use std::sync::mpsc::{Receiver, Sender};
use std::io;

pub struct IntCodeComputer {
    pub name: String,
    pub memory: IntCodeMemory,
    pub inputs: Receiver<Option<i32>>,
    pub outputs: Sender<Option<i32>>,
    pub verbose: bool,
}

trait IntCodeInput {
    fn read_int(&mut self) -> i32;
}

trait IntCodeOutput {
    fn write_int(&mut self, _: i32);
}

pub type IntCodeMemory = Vec<i32>;

pub fn intcode_read_program() -> IntCodeMemory {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Error reading program from STDIN");
    let parts = line.trim().split(',');
    parts
        .map(|s| s.parse().expect("Error parsing int"))
        .collect()
}

pub fn intcode_run(mut computer: IntCodeComputer) {
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
