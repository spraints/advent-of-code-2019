use std::collections::HashMap;
use std::io::{self, BufRead};

struct Orbit {
    center: String,
    satellite: String,
}

type Orbits = Vec<Orbit>;
type OrbitTree = HashMap<String, String>;

fn main() {
    let orbits = make_tree(read_orbits());
    println!("{:?}", orbits);

    let mut total_dist = 0;
    let the_center = "COM".to_string();
    for satellite in orbits.keys() {
        println!("{}:", satellite);
        total_dist = total_dist + dist_to(&orbits, satellite, &the_center);
    }

    println!("TOTAL DISTANCE: {}", total_dist);
}

fn dist_to(orbits: &OrbitTree, satellite: &String, dest: &String) -> u32 {
    let center = orbits.get(satellite).unwrap();
    println!("-> {}", center);
    if center == dest {
        1
    } else {
        1 + dist_to(orbits, center, dest)
    }
}

fn read_orbits() -> Orbits {
    let mut res = vec![];
    for line in io::stdin().lock().lines() {
        let parts: Vec<String> = line
            .unwrap()
            .trim()
            .split(')')
            .map(|s| s.to_string())
            .collect();
        res.push(Orbit {
            center: parts[0].to_string(),
            satellite: parts[1].to_string(),
        });
    }
    res
}

fn make_tree(orbits: Orbits) -> OrbitTree {
    let mut res = HashMap::new();
    for orbit in orbits {
        res.insert(orbit.satellite, orbit.center);
    }
    res
}
