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

    tryrun(&memory, 12, 2, 2);

    for one in 1..99 {
        for two in 1..99 {
            let res = tryrun(&memory, one, two, 0);
            if res == 19690720 {
                tryrun(&memory, one, two, 1);
            }
        }
    }
}

fn tryrun(memory: &Vec<usize>, one: usize, two: usize, verbose_level: u8) -> usize {
    let mut runmem = memory.to_vec();
    runmem[1] = one;
    runmem[2] = two;

    run(&mut runmem, verbose_level);

    match runmem.get(0..4) {
        None => {
            if verbose_level > 0 {
                println!("[{}, {}] NO FIRST FOUR", one, two);
            }
            0
        }
        Some(vals) => {
            if verbose_level > 0 {
                println!("[{}, {}] FIRST FOUR: {:?}", one, two, vals);
            }
            vals[0]
        }
    }
}

fn run(mut memory: &mut Vec<usize>, verbose_level: u8) {
    let opcodes = [op_add, op_mult];

    let mut pc = 0;
    loop {
        //println!("pc={} [{},{},{},{}]", pc, memory[pc], memory[pc+1], memory[pc+2], memory[pc+3]);

        let opcode = memory[pc];
        // println!("[{}] {}", pc, opcode);
        if opcode == 99 {
            break;
        } else {
            let opfn = opcodes[opcode - 1];
            let arg1 = memory[pc + 1];
            let arg2 = memory[pc + 2];
            let arg3 = memory[pc + 3];
            opfn(&mut memory, arg1, arg2, arg3, verbose_level);
        }
        pc += 4;
    }
}

fn op_add(memory: &mut Vec<usize>, arg1: usize, arg2: usize, arg3: usize, verbose_level: u8) {
    if verbose_level > 1 {
        println!("  ADD {} + {} => {}", arg1, arg2, arg3);
    }
    memory[arg3] = memory[arg1] + memory[arg2];
}

fn op_mult(memory: &mut Vec<usize>, arg1: usize, arg2: usize, arg3: usize, verbose_level: u8) {
    if verbose_level > 1 {
        println!("  MULT {} * {} => {}", arg1, arg2, arg3);
    }
    memory[arg3] = memory[arg1] * memory[arg2];
}
