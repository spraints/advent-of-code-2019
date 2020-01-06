//////////
// INTCODE

use std::collections::HashMap;
use std::io;
use std::sync::mpsc::{Receiver, Sender};

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;

    #[test]
    fn test_day9_ex1_quine() {
        let program = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let (out_tx, out_rx) = mpsc::channel();
        run("test", program.clone(), dead_receiver(), out_tx, false);
        for val in program {
            assert_eq!(Ok(Some(val)), out_rx.recv());
        }
        if let Ok(val) = out_rx.recv() {
            if let Some(val) = val {
                panic!("expected end of values, but got {:?}", val);
            }
        }
    }

    #[test]
    fn test_day9_ex2_sixteen() {
        let program = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];
        let (out_tx, out_rx) = mpsc::channel();
        run("test", program.clone(), dead_receiver(), out_tx, false);
        let n = out_rx.recv().unwrap().unwrap();
        assert!(n >= 1_000_000_000_000_000);
        assert!(n < 10_000_000_000_000_000);
    }

    fn dead_receiver() -> Receiver<Option<Item>> {
        mpsc::channel().1
    }
}

pub type Item = i64;

struct IntCodeComputer {
    name: String,
    memory: IntCodeMemory,
    extra_memory: HashMap<usize, Item>,
    inputs: Receiver<Option<Item>>,
    outputs: Sender<Option<Item>>,
    verbose: bool,
    pc: usize,
    relative_base: Item,
}

pub type IntCodeMemory = Vec<Item>;

pub fn read_program() -> IntCodeMemory {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Error reading program from STDIN");
    let parts = line.trim().split(',');
    parts
        .map(|s| s.parse().expect("Error parsing int"))
        .collect()
}

pub fn run(
    name: &str,
    memory: IntCodeMemory,
    inputs: Receiver<Option<Item>>,
    outputs: Sender<Option<Item>>,
    verbose: bool,
) -> IntCodeMemory {
    let mut computer = IntCodeComputer {
        name: name.to_string(),
        memory,
        extra_memory: HashMap::new(),
        inputs,
        outputs,
        verbose,
        pc: 0,
        relative_base: 0,
    };

    let opcodes = [
        op_zero,
        op_add,                  // 1
        op_mult,                 // 2
        op_input,                // 3
        op_output,               // 4
        op_jump_if_true,         // 5
        op_jump_if_false,        // 6
        op_lt,                   // 7
        op_eq,                   // 8
        op_relative_base_offset, // 9
    ];

    if computer.verbose {
        println!("[{}/{}]", computer.name, computer.pc);
    }
    loop {
        if computer.verbose {
            println!(
                " {}: ... {:?} ...",
                computer.name,
                computer.memory.get(computer.pc..computer.pc + 4)
            );
        }
        let op = computer.memory[computer.pc] as usize;
        let opcode = op % 100;
        if opcode == 99 {
            break;
        } else {
            let opfn = opcodes[opcode];
            opfn(&mut computer, modes(op / 100));
            if computer.verbose {
                println!("[{}/{}]", computer.name, computer.pc);
            }
        }
    }

    computer.outputs.send(None).unwrap_or(());

    computer.memory
}

fn modes(modes: usize) -> IntCodeModesIter {
    IntCodeModesIter { modes }
}

struct IntCodeModesIter {
    modes: usize,
}

enum ModeType {
    Position,
    Immediate,
    Relative,
}

impl Iterator for IntCodeModesIter {
    type Item = ModeType;
    fn next(&mut self) -> Option<Self::Item> {
        let cur = self.modes % 10;
        self.modes = self.modes / 10;
        Some(match cur {
            0 => ModeType::Position,
            1 => ModeType::Immediate,
            2 => ModeType::Relative,
            _ => panic!("Unrecognized mode {}", cur),
        })
    }
}

fn get_param(computer: &mut IntCodeComputer, modes: &mut IntCodeModesIter, off: usize) -> Item {
    let raw = get_mem(computer, computer.pc + off);
    match modes.next().unwrap() {
        ModeType::Position => get_mem(computer, raw as usize),
        ModeType::Immediate => raw,
        ModeType::Relative => get_mem(computer, (raw + computer.relative_base) as usize),
    }
}

const MAX_MEMORY_VEC: usize = 1_000_000;

fn get_mem(computer: &mut IntCodeComputer, addr: usize) -> Item {
    let val = if addr < MAX_MEMORY_VEC {
        computer.memory.get(addr)
    } else {
        computer.extra_memory.get(&addr)
    };
    if computer.verbose {
        println!(" {}: GET [{}] => {:?}", computer.name, addr, val);
    }
    *(val.unwrap_or(&0))
}

fn set_mem(computer: &mut IntCodeComputer, addr: usize, val: Item) {
    if computer.verbose {
        println!(" {}: SET [{}] = {}", computer.name, addr, val);
    }
    if addr < MAX_MEMORY_VEC {
        ensure_mem(computer, addr);
        computer.memory[addr] = val;
    } else {
        computer.extra_memory.insert(addr, val);
    }
}

fn ensure_mem(computer: &mut IntCodeComputer, addr: usize) {
    if addr >= computer.memory.len() {
        if computer.verbose {
            println!(
                "{} EXPAND MEMORY from {} to {}",
                computer.name,
                computer.memory.len(),
                addr + 1
            );
        }
        computer.memory.resize(addr + 1, 0);
    }
}

fn op_zero(_computer: &mut IntCodeComputer, _modes: IntCodeModesIter) {
    panic!("OP 0 DOES NOT EXIST");
}

fn op_add(computer: &mut IntCodeComputer, mut modes: IntCodeModesIter) {
    let arg1 = get_param(computer, &mut modes, 1);
    let arg2 = get_param(computer, &mut modes, 2);
    let dest_addr = computer.memory[computer.pc + 3] as usize;
    set_mem(computer, dest_addr, arg1 + arg2);
    computer.pc += 4;
}

fn op_mult(computer: &mut IntCodeComputer, mut modes: IntCodeModesIter) {
    let arg1 = get_param(computer, &mut modes, 1);
    let arg2 = get_param(computer, &mut modes, 2);
    let dest_addr = computer.memory[computer.pc + 3] as usize;
    set_mem(computer, dest_addr, arg1 * arg2);
    computer.pc += 4;
}

fn op_input(computer: &mut IntCodeComputer, _: IntCodeModesIter) {
    let base = if computer.memory[computer.pc] / 100 == 2 {
        computer.relative_base
    } else {
        0
    };
    let dest_addr = (base + computer.memory[computer.pc + 1]) as usize;
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
                    set_mem(computer, dest_addr, val);
                }
            };
        }
    };
    computer.pc += 2;
}

fn op_output(computer: &mut IntCodeComputer, mut modes: IntCodeModesIter) {
    let val = get_param(computer, &mut modes, 1);
    if computer.verbose {
        println!("  ({}: output: {})", computer.name, val);
    }
    if let Err(msg) = computer.outputs.send(Some(val)) {
        panic!("{}: send error: {}", computer.name, msg);
    }
    computer.pc += 2;
}

fn op_jump_if_true(computer: &mut IntCodeComputer, modes: IntCodeModesIter) {
    jump_if(true, computer, modes)
}

fn op_jump_if_false(computer: &mut IntCodeComputer, modes: IntCodeModesIter) {
    jump_if(false, computer, modes)
}

fn jump_if(cond: bool, computer: &mut IntCodeComputer, mut modes: IntCodeModesIter) {
    let arg1 = get_param(computer, &mut modes, 1);
    let arg2 = get_param(computer, &mut modes, 2);
    if (cond && arg1 != 0) || (!cond && arg1 == 0) {
        computer.pc = arg2 as usize;
    } else {
        computer.pc += 3;
    }
}

fn op_lt(computer: &mut IntCodeComputer, mut modes: IntCodeModesIter) {
    let arg1 = get_param(computer, &mut modes, 1);
    let arg2 = get_param(computer, &mut modes, 2);
    let dest_addr = computer.memory[computer.pc + 3] as usize;
    set_mem(computer, dest_addr, if arg1 < arg2 { 1 } else { 0 });
    computer.pc += 4;
}

fn op_eq(computer: &mut IntCodeComputer, mut modes: IntCodeModesIter) {
    let arg1 = get_param(computer, &mut modes, 1);
    let arg2 = get_param(computer, &mut modes, 2);
    let dest_addr = computer.memory[computer.pc + 3] as usize;
    set_mem(computer, dest_addr, if arg1 == arg2 { 1 } else { 0 });
    computer.pc += 4;
}

fn op_relative_base_offset(computer: &mut IntCodeComputer, mut modes: IntCodeModesIter) {
    let arg = get_param(computer, &mut modes, 1);
    if computer.verbose {
        println!(
            " {} relative base = {} + {}",
            computer.name, computer.relative_base, arg
        );
    }
    computer.relative_base += arg;
    computer.pc += 2;
}
