use std::io;

fn main() {
    println!("--------------");
    println!("INTCODE ONLINE");
    println!("--------------");

    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Error reading program from STDIN");
    let parts = line.trim().split(',');
    let memory: Vec<usize> = parts
        .map(|s| s.parse().expect("Error parsing int"))
        .collect();

    println!("INPUT {:?}", memory);

    run(&mut memory.clone());
}

fn run(mut memory: &mut Vec<usize>) {
    let opcodes = [op_add, op_mult, op_input, op_output];

    let mut pc = 0;
    println!("[{}] {:?}", pc, memory);
    loop {
        let op = memory[pc];
        let opcode = op % 100;
        if opcode == 99 {
            break;
        } else {
            let opfn = opcodes[opcode - 1];
            pc = opfn(&mut memory, modes(op / 100), pc);
            println!("[{}] {:?}", pc, memory);
        }
    }
}

fn modes(modes: usize) -> ModesIter {
    ModesIter { modes }
}

struct ModesIter {
    modes: usize,
}

enum ModeType {
    Position,
    Immediate,
}

impl Iterator for ModesIter {
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

fn get_params(memory: &Vec<usize>, modes: ModesIter, pc: usize, count: usize) -> Vec<usize> {
    let params: Vec<usize> = memory[pc + 1..pc + 1 + count]
        .into_iter()
        .zip(modes)
        .map(|(raw, mode)| match mode {
            ModeType::Position => memory[*raw],
            ModeType::Immediate => *raw,
        })
        .collect();
    assert_eq!(count, params.len());
    params
}

fn op_add(memory: &mut Vec<usize>, modes: ModesIter, pc: usize) -> usize {
    let params = get_params(&memory, modes, pc, 2);
    let arg1 = params[0];
    let arg2 = params[1];
    let dest_addr = memory[pc + 3];
    memory[dest_addr] = arg1 + arg2;
    pc + 4
}

fn op_mult(memory: &mut Vec<usize>, modes: ModesIter, pc: usize) -> usize {
    let params = get_params(&memory, modes, pc, 2);
    let arg1 = params[0];
    let arg2 = params[1];
    let dest_addr = memory[pc + 3];
    memory[dest_addr] = arg1 * arg2;
    pc + 4
}

fn op_input(memory: &mut Vec<usize>, _: ModesIter, pc: usize) -> usize {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Error reading input from STDIN");
    let dest_addr = memory[pc + 1];
    memory[dest_addr] = line.trim().parse().expect("Error parsing int for input");
    pc + 2
}

fn op_output(memory: &mut Vec<usize>, modes: ModesIter, pc: usize) -> usize {
    let params = get_params(&memory, modes, pc, 1);
    println!(" ==> {}", params[0]);
    pc + 2
}
