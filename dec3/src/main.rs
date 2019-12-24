use std::collections::HashMap;
use std::io;

type Segment = (char, u32);
type Path = Vec<Segment>;
type Coord = (i32, i32);
type Location = (u8, u32);
type Grid = HashMap<Coord, Location>;

fn main() {
    println!("WIRE DETANGLER");

    let mut grid = HashMap::new();

    plot(&mut grid, read_path(), 0x01);
    plot(&mut grid, read_path(), 0x02);

    let res = grid.values().fold(10000000, |res, (wires, dist)| {
        if *wires == 0x03 && dist < &res {
            *dist
        } else {
            res
        }
    });
    println!("MIN DISTANCE: {}", res);
}

fn plot(mut grid: &mut Grid, path: Path, wire_name: u8) {
    let mut coords = (0, 0);
    for segment in path {
        println!("{:?} --> {:?}", coords, segment);
        coords = match segment {
            ('R', n) => walk(&mut grid, coords, wire_name, n, |(x, y)| (x + 1, y)),
            ('L', n) => walk(&mut grid, coords, wire_name, n, |(x, y)| (x - 1, y)),
            ('U', n) => walk(&mut grid, coords, wire_name, n, |(x, y)| (x, y + 1)),
            ('D', n) => walk(&mut grid, coords, wire_name, n, |(x, y)| (x, y - 1)),
            other => {
                println!("ERR unrecognized segment for {}: {:?}", wire_name, other);
                coords
            }
        };
    }
    println!("{:?}", coords);
}

fn walk<F>(grid: &mut Grid, coords: Coord, wire_name: u8, count: u32, step: F) -> Coord
where
    F: Fn(Coord) -> Coord,
{
    let mut res = coords;
    for _ in 0..count {
        res = step(res);
        // TODO fix mutable/immutable borrowing conflict here.
        match grid.get(&res) {
            None => grid.insert(res, (wire_name, calc_dist(&res))),
            Some((wire_names, dist)) => grid.insert(res, (wire_names | wire_name, *dist)),
        };
    }
    res
}

fn calc_dist(coords: &Coord) -> u32 {
    let (x, y) = coords;
    (x.abs() + y.abs()) as u32
}

fn read_path() -> Path {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Error reading STDIN");
    line.trim().split(',').map(|s| parse_segment(s)).collect()
}

fn parse_segment(s: &str) -> Segment {
    let direction = s.chars().nth(0).unwrap();
    let distance = match s.get(1..) {
        Some(part) => part.parse().expect("COULD NOT PARSE"),
        None => 0,
    };
    (direction, distance)
}
