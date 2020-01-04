use std::collections::HashMap;
use std::io;

type Segment = (char, u32);
type Path = Vec<Segment>;
type Coord = (i32, i32);
type WalkState = (Coord, u32);
type Location = (u8, u32, u32); // names, manhattan distance, cumulative wire length.
type Grid = HashMap<Coord, Location>;

#[cfg(test)]
mod tests {
    use super::{parse_path, plot, taxi_dist, wire_dist, HashMap};

    fn test_path(line1: &str, line2: &str, expected: (u32, u32)) {
        let mut grid = HashMap::new();
        plot(&mut grid, parse_path(line1), 0x01);
        plot(&mut grid, parse_path(line2), 0x02);

        assert_eq!(expected.0, taxi_dist(&grid));
        assert_eq!(expected.1, wire_dist(&grid));
    }

    #[test]
    fn test_ex1() {
        test_path(
            "R75,D30,R83,U83,L12,D49,R71,U7,L72",
            "U62,R66,U55,R34,D71,R55,D58,R83",
            (159, 610),
        );
    }

    #[test]
    fn test_ex2() {
        test_path(
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
            "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
            (135, 410),
        );
    }
}

fn main() {
    println!("WIRE DETANGLER");

    let mut grid = HashMap::new();

    plot(&mut grid, read_path(), 0x01);
    plot(&mut grid, read_path(), 0x02);

    println!("MIN TAXI DISTANCE: {}", taxi_dist(&grid));
    println!("MIN WIRE DISTANCE: {}", wire_dist(&grid));
}

fn taxi_dist(grid: &Grid) -> u32 {
    grid.values().fold(10000000, |res, (wires, dist, _)| {
        if *wires == 0x03 && dist < &res {
            *dist
        } else {
            res
        }
    })
}

fn wire_dist(grid: &Grid) -> u32 {
    grid.values().fold(10000000, |res, (wires, _, dist)| {
        if *wires == 0x03 && dist < &res {
            *dist
        } else {
            res
        }
    })
}

fn plot(mut grid: &mut Grid, path: Path, wire_name: u8) {
    let mut state = ((0, 0), 0);
    for segment in path {
        println!("{:?} --> {:?}", state, segment);
        state = match segment {
            ('R', n) => walk(&mut grid, state, wire_name, n, |(x, y)| (x + 1, y)),
            ('L', n) => walk(&mut grid, state, wire_name, n, |(x, y)| (x - 1, y)),
            ('U', n) => walk(&mut grid, state, wire_name, n, |(x, y)| (x, y + 1)),
            ('D', n) => walk(&mut grid, state, wire_name, n, |(x, y)| (x, y - 1)),
            other => {
                println!("ERR unrecognized segment for {}: {:?}", wire_name, other);
                state
            }
        };
    }
    println!("{:?}", state);
}

fn walk<F>(grid: &mut Grid, state: WalkState, wire_name: u8, count: u32, step: F) -> WalkState
where
    F: Fn(Coord) -> Coord,
{
    let (mut coords, mut dist) = state;
    for _ in 0..count {
        coords = step(coords);
        dist = dist + 1;
        // TODO fix mutable/immutable borrowing conflict here.
        match grid.get(&coords) {
            None => grid.insert(coords, (wire_name, calc_dist(&coords), dist)),
            Some((wire_names, taxidist, wiredist)) => {
                let mut newwiredist = *wiredist;
                if wire_names & wire_name == 0 {
                    newwiredist = newwiredist + dist;
                }
                grid.insert(coords, (wire_names | wire_name, *taxidist, newwiredist))
            }
        };
    }
    (coords, dist)
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
    parse_path(line.trim())
}

fn parse_path(s: &str) -> Path {
    s.split(',').map(|s| parse_segment(s)).collect()
}

fn parse_segment(s: &str) -> Segment {
    let direction = s.chars().nth(0).unwrap();
    let distance = match s.get(1..) {
        Some(part) => part.parse().expect("COULD NOT PARSE"),
        None => 0,
    };
    (direction, distance)
}
