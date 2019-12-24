use std::io;
use std::collections::HashMap;

fn main() {
    println!("WIRE DETANGLER");

    let mut grid = HashMap::new();

    plot(&mut grid, read_path(), 0x01);
    plot(&mut grid, read_path(), 0x02);

    let res = grid.values().fold(10000000, |res, (wires, dist)| if *wires == 0x03 && dist < &res { *dist } else { res });
    println!("MIN DISTANCE: {}", res);
}

fn plot(mut grid: &mut HashMap<(i32, i32), (u8, u32)>, path: Vec<(char, u32)>, wire_name: u8) {
    let mut coords = (0,0);
    for segment in path {
        println!("{:?} --> {:?}", coords, segment);
        coords = match segment {
            ('R', n) => walk(&mut grid, coords, wire_name, n),//, |(x,y)| (x + 1, y)),
            ('L', n) => walk(&mut grid, coords, wire_name, n),//, |(x,y)| (x - 1, y)),
            ('U', n) => walk(&mut grid, coords, wire_name, n),//, |(x,y)| (x, y+1)),
            ('D', n) => walk(&mut grid, coords, wire_name, n),//, |(x,y)| (x, y-1)),
            other => {println!("ERR unrecognized segment for {}: {:?}", wire_name, other); coords},
        };
    }
    println!("{:?}", coords);
}

fn walk(mut grid: &mut HashMap<(i32, i32), (u8, u32)>, coords: (i32, i32), wire_name: u8, count: u32) -> (i32, i32) {//, step: (i32, i32) -> (i32, i32)) -> (i32, i32) {
    let mut res = coords;
    for _ in 1..count {
        let (x,y) = res;
        res = (x+1, y+1); // TODO
        grid.insert(res, (wire_name, calc_dist(&res)));
    }
    res
}

fn calc_dist(coords: &(i32, i32)) -> u32 {
    // TODO
    0
}

fn read_path() -> Vec<(char, u32)> {
  let mut line = String::new();
  io::stdin().read_line(&mut line).expect("Error reading STDIN");
  line.trim().split(',').map(|s| parse_segment(s)).collect()
}

fn parse_segment(s: &str) -> (char, u32) {
    let direction = s.chars().nth(0).unwrap();
    let distance = match s.get(1..) {
        Some(part) => part.parse().expect("COULD NOT PARSE"),
        None => 0,
    };
    (direction, distance)
}

// enum Segment {
//     R(u32),
//     L(u32),
//     U(u32),
//     D(u32),
// }

// fn read_path() -> Vec<Segment> {
//   let mut line = String::new();
//   io::stdin().read_line(&mut line).expect("Error reading STDIN");
//   line.trim().split(',').map(|s| parse_segment(s)).collect()
// }

// fn parse_segment(s: &str) -> Result<Segment, str> {
//     let direction = s.chars().nth(0).unwrap();
//     let distance = match s.get(1..) {
//         Some(part) => part.parse().expect("COULD NOT PARSE"),
//         None => 0,
//     };
//     match direction {
//         'R' => Ok(Segment::R(distance)),
//         'L' => Ok(Segment::L(distance)),
//         'U' => Ok(Segment::U(distance)),
//         'D' => Ok(Segment::D(distance)),
//         _ => Err(s),
//     }
// }
