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
            pc = opfn(&mut memory, op / 100, pc);
            println!("[{}] {:?}", pc, memory);
        }
    }
}

fn get_params(memory: &Vec<usize>, modes: usize, pc: usize, count: usize) -> Vec<usize> {
    let modes: Vec<char> = modes.to_string().chars().rev().collect();
    let params = memory[pc+1..].zip(modes .resize(count, '0')).map(|(raw, mode)| if mode == '0' { memory[raw] } else { raw });
    assert_eq!(count, params.len());
    params
}

fn op_add(memory: &mut Vec<usize>, modes: usize, pc: usize) -> usize {
    let params = get_params(&memory, modes, pc, 2);
    let arg1 = params[0];
    let arg2 = params[1];
    let dest_addr = memory[pc+3];
    memory[dest_addr] = arg1 + arg2;
    pc + 4
}

fn op_mult(memory: &mut Vec<usize>, modes: usize, pc: usize) -> usize {
    let params = get_params(&memory, modes, pc, 2);
    let arg1 = params[0];
    let arg2 = params[1];
    let dest_addr = memory[pc+3];
    memory[dest_addr] = arg1 * arg2;
    pc + 4
}

fn op_input(memory: &mut Vec<usize>, modes: usize, pc: usize) -> usize {
    println!("TODO: read input");
    pc + 2
}

fn op_output(memory: &mut Vec<usize>, modes: usize, pc: usize) -> usize {
    println!("TODO: read output");
    pc + 2
}
