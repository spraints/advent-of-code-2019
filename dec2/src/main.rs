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
        let parts = line.trim().  split(',');
        let data = parts.map(|s| s.parse().expect("Error parsing int"));
        for n in data {
            memory.push(n)
        }
    }

    //memory[0] = 12;
    //memory[1] = 2;

    let mut pc = 0;
    loop {
        //println!("pc={} [{},{},{},{}]", pc, memory[pc], memory[pc+1], memory[pc+2], memory[pc+3]);

        let opcode = memory[pc];
        match opcode {
            1 => {
                let loc1 = memory[pc+1];
                let loc2 = memory[pc+2];
                let destloc = memory[pc+3];
                memory[destloc] = memory[loc1] + memory[loc2];
            },
            //1 => memory[memory[pc+3]] = memory[memory[pc+1]] + memory[memory[pc+2]],
            //1 => set(memory, pc+3, get(memory, pc+1) + get(memory, pc + 2)),
            //1 => memory[memory[pc+3]] = memory[memory[pc+1]] + memory[memory[pc+2]],
            //2 => memory[memory[pc+3]] = memory[memory[pc+1]] * memory[memory[pc+2]],
            99 => break,
            _ => panic!("unrecognized opcode {} at pc={}", opcode, pc),
        }
        pc+=4;
    }

    println!("ALL => {:?}", memory);
    println!("POSITION ZERO => {}", memory[0]);
}

//fn set(memory: &mut Vec<usize>, off: usize, val: usize) {
//    println!("SET {} <- {}", off, val);
//}
//
//fn get(memory: &mut Vec<i64>, off: usize) -> i64 {
//    let loc = memory[off] as usize;
//    memory[loc]
//}
