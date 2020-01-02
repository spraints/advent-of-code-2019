use std::io;

fn main() {
    println!("--------------");
    println!("INTCODE ONLINE");
    println!("--------------");

    let memory = intcode_read_program();
    //println!("INPUT {:?}", memory);
    intcode_run(&mut memory.clone());
}

//////////
// INTCODE

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

fn intcode_run(mut memory: &mut IntCodeMemory) {
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
    println!("[{}] {:?}", pc, memory);
    loop {
        println!(" ... {:?} ...", memory.get(pc..pc + 4));
        let op = memory[pc] as usize;
        let opcode = op % 100;
        if opcode == 99 {
            break;
        } else {
            let opfn = opcodes[opcode - 1];
            pc = opfn(&mut memory, intcode_modes(op / 100), pc);
            println!("[{}] {:?}", pc, memory);
        }
    }
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

fn intcode_op_add(memory: &mut IntCodeMemory, modes: IntCodeModesIter, pc: usize) -> usize {
    let params = intcode_get_params(&memory, modes, pc, 2);
    let arg1 = params[0];
    let arg2 = params[1];
    let dest_addr = memory[pc + 3] as usize;
    memory[dest_addr] = arg1 + arg2;
    pc + 4
}

fn intcode_op_mult(memory: &mut IntCodeMemory, modes: IntCodeModesIter, pc: usize) -> usize {
    let params = intcode_get_params(&memory, modes, pc, 2);
    let arg1 = params[0];
    let arg2 = params[1];
    let dest_addr = memory[pc + 3] as usize;
    memory[dest_addr] = arg1 * arg2;
    pc + 4
}

fn intcode_op_input(memory: &mut IntCodeMemory, _: IntCodeModesIter, pc: usize) -> usize {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Error reading input from STDIN");
    let dest_addr = memory[pc + 1] as usize;
    memory[dest_addr] = line.trim().parse().expect("Error parsing int for input");
    pc + 2
}

fn intcode_op_output(memory: &mut IntCodeMemory, modes: IntCodeModesIter, pc: usize) -> usize {
    let params = intcode_get_params(&memory, modes, pc, 1);
    println!(" ==> {}", params[0]);
    pc + 2
}

fn intcode_op_jump_if_true(
    memory: &mut IntCodeMemory,
    modes: IntCodeModesIter,
    pc: usize,
) -> usize {
    intcode_jump_if(true, memory, modes, pc)
}

fn intcode_op_jump_if_false(
    memory: &mut IntCodeMemory,
    modes: IntCodeModesIter,
    pc: usize,
) -> usize {
    intcode_jump_if(false, memory, modes, pc)
}

fn intcode_jump_if(
    cond: bool,
    memory: &mut IntCodeMemory,
    modes: IntCodeModesIter,
    pc: usize,
) -> usize {
    let params = intcode_get_params(&memory, modes, pc, 2);
    let arg1 = params[0];
    let arg2 = params[1];
    if (cond && arg1 != 0) || (!cond && arg1 == 0) {
        arg2 as usize
    } else {
        pc + 3
    }
}

fn intcode_op_lt(memory: &mut IntCodeMemory, modes: IntCodeModesIter, pc: usize) -> usize {
    let params = intcode_get_params(&memory, modes, pc, 2);
    let arg1 = params[0];
    let arg2 = params[1];
    let dest_addr = memory[pc + 3] as usize;
    memory[dest_addr] = if arg1 < arg2 { 1 } else { 0 };
    pc + 4
}

fn intcode_op_eq(memory: &mut IntCodeMemory, modes: IntCodeModesIter, pc: usize) -> usize {
    let params = intcode_get_params(&memory, modes, pc, 2);
    let arg1 = params[0];
    let arg2 = params[1];
    let dest_addr = memory[pc + 3] as usize;
    memory[dest_addr] = if arg1 == arg2 { 1 } else { 0 };
    pc + 4
}
