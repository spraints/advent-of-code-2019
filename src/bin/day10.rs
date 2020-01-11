use std::io::{self, Read};

#[cfg(test)]
mod tests {
    #[test]
    fn test_1() {
        let input = ".#..#\n\
                     .....\n\
                     #####\n\
                     ....#\n\
                     ...##";
        assert_eq!(((3, 4), 8), score(input));
    }

    #[test]
    fn test_2() {
        let input = "......#.#.\n\
                     #..#.#....\n\
                     ..#######.\n\
                     .#.#.###..\n\
                     .#..#.....\n\
                     ..#....#.#\n\
                     #..#....#.\n\
                     .##.#..###\n\
                     ##...#..#.\n\
                     .#....####";
        assert_eq!(((5, 8), 33), score(input));
    }

    #[test]
    fn test_3() {
        let input = "#.#...#.#.\n\
                     .###....#.\n\
                     .#....#...\n\
                     ##.#.#.#.#\n\
                     ....#.#.#.\n\
                     .##..###.#\n\
                     ..#...##..\n\
                     ..##....##\n\
                     ......#...\n\
                     .####.###.";
        assert_eq!(((1, 2), 35), score(input));
    }

    #[test]
    fn test_4() {
        let input = ".#..#..###\n\
                     ####.###.#\n\
                     ....###.#.\n\
                     ..###.##.#\n\
                     ##.##.#.#.\n\
                     ....###..#\n\
                     ..#.#..#.#\n\
                     #..#.#.###\n\
                     .##...##.#\n\
                     .....#.#..";
        assert_eq!(((6, 3), 41), score(input));
    }

    #[test]
    fn test_5() {
        let input = ".#..##.###...#######\n\
                     ##.############..##.\n\
                     .#.######.########.#\n\
                     .###.#######.####.#.\n\
                     #####.##.#.##.###.##\n\
                     ..#####..#.#########\n\
                     ####################\n\
                     #.####....###.#.#.##\n\
                     ##.#################\n\
                     #####.##.###..####..\n\
                     ..######..##.#######\n\
                     ####.##.####...##..#\n\
                     .#####..#.######.###\n\
                     ##...#.##########...\n\
                     #.##########.#######\n\
                     .####.#.###.###.#.##\n\
                     ....##.##.###..#####\n\
                     .#.#.###########.###\n\
                     #.#.#.#####.####.###\n\
                     ###.##.####.##.#..##";
        assert_eq!(((11, 13), 210), score(input));
    }

    fn score(input: &str) -> (super::Coords, usize) {
        let map = super::parse_map(input);
        let mut res = None;
        let (width, height) = super::extents(&map);
        for x in 0..width {
            for y in 0..height {
                if map[y][x] {
                    let coords = (x, y);
                    let count = super::visible(&map, coords);
                    res = Some((coords, count));
                }
            }
        }
        res.unwrap()
    }

    #[test]
    fn test_points_between() {
        assert_between(vec![], (0, 0), (0, 0));
        assert_between(vec![], (0, 0), (1, 1));
        assert_between(vec![(1, 1)], (0, 0), (2, 2));
        assert_between(vec![(1, 1)], (2, 2), (0, 0));
        assert_between(vec![(1, 1)], (0, 2), (2, 0));
        assert_between(vec![(1, 1)], (2, 0), (0, 2));
        assert_between(vec![(1, 1), (2, 2)], (0, 0), (3, 3));
        assert_between(vec![(1, 2), (2, 1)], (3, 0), (0, 3));
        assert_between(vec![], (0, 0), (10, 1));
        assert_between(vec![], (0, 0), (1, 10));
        assert_between(vec![], (1, 10), (0, 0));
        assert_between(vec![], (10, 1), (0, 0));
        assert_between(vec![(0, 1), (0, 2)], (0, 0), (0, 3));
        assert_between(vec![(0, 1), (0, 2)], (0, 3), (0, 0));
        assert_between(vec![(1, 0), (2, 0)], (0, 0), (3, 0));
        assert_between(vec![(1, 0), (2, 0)], (3, 0), (0, 0));
        assert_between(vec![(10, 1)], (0, 0), (20, 2));
        assert_between(vec![(1, 10)], (0, 0), (2, 20));
        assert_between(vec![(10, 1)], (20, 2), (0, 0));
        assert_between(vec![(1, 10)], (2, 20), (0, 0));
        assert_between(vec![(3, 8), (5, 15)], (1, 1), (7, 22));
    }

    fn assert_between(expected: Vec<super::Coords>, a: super::Coords, b: super::Coords) {
        assert_eq!(
            expected,
            super::points_between(a, b),
            "points_between({:?}, {:?})",
            a,
            b
        );
    }
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("reading stdin");
    println!("{:?}", parse_map(&input));
}

type Map = Vec<Vec<bool>>;
type Coords = (usize, usize);

fn visible(map: &Map, coords: Coords) -> usize {
    let (width, height) = extents(map);
    let mut res = 0;
    for x in 0..width {
        for y in 0..height {
            if can_see(map, (x, y), coords) {
                res += 1;
            }
        }
    }
    res
}

fn can_see(map: &Map, from: Coords, to: Coords) -> bool {
    for (x, y) in points_between(from, to) {
        if map[y][x] {
            return false;
        }
    }
    true
}

fn points_between(a: Coords, b: Coords) -> Vec<Coords> {
    let abslope = slope(&a, &b);
    let mut res = vec![];
    let xrange = if a.0 < b.0 { a.0..=b.0 } else { b.0..=a.0 };
    let yrange = if a.1 < b.1 { a.1..=b.1 } else { b.1..=a.1 };
    for x in xrange {
        for y in yrange.clone() {
            let coord = (x, y);
            println!("{:?} / {:?} / {:?}?", a, coord, b);
            if coord != a && coord != b && slope(&a, &coord) == abslope {
                println!("  YES!");
                res.push(coord);
            }
        }
    }
    res
}

fn slope(a: &Coords, b: &Coords) -> f32 {
    let (x1, y1) = (a.0 as f32, a.1 as f32);
    let (x2, y2) = (b.0 as f32, b.1 as f32);
    let res = (y2 - y1) / (x2 - x1);
    println!("{:?} / {:?} => {}", a, b, res);
    res
}

fn extents(map: &Map) -> (usize, usize) {
    (map[0].len(), map.len())
}

fn parse_map(input: &str) -> Map {
    let mut res = vec![];
    let mut width = None;
    for line in input.lines() {
        let mut resline = vec![];
        for c in line.chars() {
            match c {
                '.' => resline.push(false),
                '#' => resline.push(true),
                _ => (),
            };
        }
        match width {
            None => width = Some(resline.len()),
            Some(x) => {
                if x != resline.len() {
                    panic!("Inconsistent lengths! {} vs {}", x, resline.len());
                }
            }
        };
        res.push(resline);
    }
    res
}
