use std::io;

fn main() {
    println!("--------------");
    println!("INTCODE ONLINE");
    println!("--------------");

    let mut memory: Vec<usize> = vec![];
    let mut line = String::new();
    while 0 < io::stdin()
        .read_line(&mut line)
        .expect("Error reading STDIN")
    {
        let parts = line.trim().split(',');
        let data = parts.map(|s| s.parse().expect("Error parsing int"));
        for n in data {
            memory.push(n)
        }
    }

    memory[1] = 12;
    memory[2] = 2;

    let opcodes = [op_add, op_mult];

    let mut pc = 0;
    loop {
        //println!("pc={} [{},{},{},{}]", pc, memory[pc], memory[pc+1], memory[pc+2], memory[pc+3]);

        let opcode = memory[pc];
        println!("[{}] {}", pc, opcode);
        if opcode == 99 {
            break;
        } else {
            let opfn = opcodes[opcode - 1];
            let arg1 = memory[pc + 1];
            let arg2 = memory[pc + 2];
            let arg3 = memory[pc + 3];
            opfn(&mut memory, arg1, arg2, arg3);
        }
        pc += 4;
    }

    //println!("{:?}", memory);
    println!("POSITION ZERO => {}", memory[0]);
}

fn op_add(memory: &mut Vec<usize>, arg1: usize, arg2: usize, arg3: usize) {
    println!("  ADD {} + {} => {}", arg1, arg2, arg3);
    memory[arg3] = memory[arg1] + memory[arg2];
}

fn op_mult(memory: &mut Vec<usize>, arg1: usize, arg2: usize, arg3: usize) {
    println!("  MULT {} * {} => {}", arg1, arg2, arg3);
    memory[arg3] = memory[arg1] * memory[arg2];
}
