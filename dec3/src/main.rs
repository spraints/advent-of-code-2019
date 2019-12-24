use std::io;

fn main() {
    println!("WIRE DETANGLER");

    //let mut wires = vec![];
    let mut line = String::new();

    while 0 < io::stdin()
        .read_line(&mut line)
        .expect("Error reading STDIN")
    {
        println!("LINE {}", line);
        let parts = line.trim().split(',').map(|s| (s.get(0..1), s.get(1..)));
        println!("{:?}", parts);
        for p in parts {
            println!("{:?}", p);
        }
        line.clear();
        //wires.push(parse_wire(line));
    }
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
