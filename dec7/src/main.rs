use std::io;

fn main() {
    println!("--------------");
    println!("INTCODE ONLINE");
    println!("--------------");

    let memory = intcode_read_program();
    //println!("INPUT {:?}", memory);
    intcode_run(&mut IntCodeComputer {
        memory: memory.clone(),
        input: intcode_debug_input(intcode_stdin_input()),
        output: intcode_debug_output(intcode_null_output()),
    });
}

//////////
// INTCODE

struct IntCodeComputer {
    memory: IntCodeMemory,
    input: Box<dyn IntCodeInput>,
    output: Box<dyn IntCodeOutput>,
}

trait IntCodeInput {
    fn read_int(&self) -> i32;
}

trait IntCodeOutput {
    fn write_int(&self, _: i32);
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

fn intcode_run(mut computer: &mut IntCodeComputer) {
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
    println!("[{}] {:?}", pc, computer.memory);
    loop {
        println!(" ... {:?} ...", computer.memory.get(pc..pc + 4));
        let op = computer.memory[pc] as usize;
        let opcode = op % 100;
        if opcode == 99 {
            break;
        } else {
            let opfn = opcodes[opcode - 1];
            pc = opfn(&mut computer, intcode_modes(op / 100), pc);
            println!("[{}] {:?}", pc, computer.memory);
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
    computer.memory[dest_addr] = computer.input.read_int();
    pc + 2
}

fn intcode_op_output(computer: &mut IntCodeComputer, modes: IntCodeModesIter, pc: usize) -> usize {
    let params = intcode_get_params(&computer.memory, modes, pc, 1);
    computer.output.write_int(params[0]);
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

fn intcode_debug_input(input: Box<dyn IntCodeInput>) -> Box<dyn IntCodeInput> {
    Box::new(IntCodeDebugInput { input })
}

struct IntCodeDebugInput {
    input: Box<dyn IntCodeInput>,
}

impl IntCodeInput for IntCodeDebugInput {
    fn read_int(&self) -> i32 {
        let res = self.input.read_int();
        println!("  (read: {})", res);
        res
    }
}

fn intcode_stdin_input() -> Box<dyn IntCodeInput> {
    Box::new(IntCodeStdinInput {})
}

struct IntCodeStdinInput {}

impl IntCodeInput for IntCodeStdinInput {
    fn read_int(&self) -> i32 {
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Error reading input from STDIN");
        line.trim().parse().expect("Error parsing int for input")
    }
}

fn intcode_debug_output(output: Box<dyn IntCodeOutput>) -> Box<dyn IntCodeOutput> {
    Box::new(IntCodeDebugOutput { output })
}

struct IntCodeDebugOutput {
    output: Box<dyn IntCodeOutput>,
}

impl IntCodeOutput for IntCodeDebugOutput {
    fn write_int(&self, i: i32) {
        println!("  (output: {})", i);
        self.output.write_int(i);
    }
}

fn intcode_null_output() -> Box<dyn IntCodeOutput> {
    Box::new(IntCodeNullOutput {})
}

struct IntCodeNullOutput {}

impl IntCodeOutput for IntCodeNullOutput {
    fn write_int(&self, _: i32) {}
}
