use std::io;

fn main() {
    println!("WIRE DETANGLER");

    let mut wires = vec![];

    loop {
        let mut line = String::new();
        let n = io::stdin()
            .read_line(&mut line)
            .expect("Error reading STDIN");
        if n == 0 {
            break;
        }
        let parts: Vec<(char, u32)> = line.trim().split(',').map(|s| parse_segment(s)).collect();
        println!("{:?}", parts);
        wires.push(parts);
    }
}

fn parse_segment(s: &str) -> (char, u32) {
    let direction = s.chars().nth(0).unwrap();
    let distance = match s.get(1..) {
        Some(part) => part.parse().expect("COULD NOT PARSE"),
        None => 0,
    };
    (direction, distance)
}

//fn parse_wire(line: &str) -> Wire {
//    let parts = line.trim().split(',');
//    let segments = parts.map(|s| parse_wire_segment(s));
//    let mut wire = Wire{};
//    for segment in segments {
//        append_segment(&mut wire, segment);
//    }
//    wire
//}
//
//fn parse_wire_segment(tok: String) {
//    (tok.get(0..1), tok.get(1..))
//}
//
//fn append_segment(_: &mut Wire, _: u32) {
//    // todo
//}
//
//struct Wire {
//}
