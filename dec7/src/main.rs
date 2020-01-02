use std::io;

fn main() {
    println!("--------------");
    println!("INTCODE ONLINE");
    println!("--------------");

    let program = intcode_read_program();
    let mut max_out = 0;
    for inputs in all_perms([0, 1, 2, 3, 4]) {
        let out = try_inputs(&program, inputs);
        if out > max_out {
            max_out = out;
        }
    }
    println!("MAX OUTPUT: {}", max_out);
}

fn try_inputs(program: &IntCodeMemory, inputs: [i32; 5]) -> i32 {
    let mut last_output = 0;
    for input in &inputs {
        let vals: [i32; 2] = [*input, last_output];
        let mut computer = IntCodeComputer {
            memory: program.clone(),
            inputs: Box::new(FFFFFFFU { vals, cursor: 0 }),
            outputs: vec![],
            verbose: false,
        };
        intcode_run(&mut computer);
        assert_eq!(1, computer.outputs.len());
        last_output = computer.outputs[0];
    }
    //println!("{:?} => {}", inputs, last_output);
    last_output
}

struct FFFFFFFU {
    vals: [i32; 2],
    cursor: usize,
}

impl Iterator for FFFFFFFU {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        if self.cursor >= 2 {
            None
        } else {
            self.cursor += 1;
            Some(self.vals[self.cursor - 1])
        }
    }
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
    memory: IntCodeMemory,
    inputs: Box<dyn Iterator<Item = i32>>,
    outputs: Vec<i32>,
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
    if computer.verbose {
        println!("[{}] {:?}", pc, computer.memory);
    }
    loop {
        if computer.verbose {
            println!(" ... {:?} ...", computer.memory.get(pc..pc + 4));
        }
        let op = computer.memory[pc] as usize;
        let opcode = op % 100;
        if opcode == 99 {
            break;
        } else {
            let opfn = opcodes[opcode - 1];
            pc = opfn(&mut computer, intcode_modes(op / 100), pc);
            if computer.verbose {
                println!("[{}] {:?}", pc, computer.memory);
            }
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
    let val = match computer.inputs.next() {
        None => {
            println!("ERROR!!! expected to find an input, but didn't!");
            0
        }
        Some(n) => {
            if computer.verbose {
                println!("  (read: {})", n);
            }
            n
        }
    };
    computer.memory[dest_addr] = val;
    pc + 2
}

fn intcode_op_output(computer: &mut IntCodeComputer, modes: IntCodeModesIter, pc: usize) -> usize {
    let params = intcode_get_params(&computer.memory, modes, pc, 1);
    let val = params[0];
    if computer.verbose {
        println!("  (output: {})", val);
    }
    computer.outputs.push(val);
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
