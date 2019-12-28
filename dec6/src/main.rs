use std::collections::HashMap;
use std::io::{self, BufRead};

type Orbits = HashMap<String, String>;

fn main() {
    let orbits = read_orbits();
    println!("{:?}", orbits);

    let mut total_dist = 0;
    let the_center = "COM".to_string();
    for satellite in orbits.keys() {
        println!("{}:", satellite);
        total_dist = total_dist + dist_to(&orbits, satellite, &the_center);
    }

    println!("TOTAL DISTANCE: {}", total_dist);
}

fn dist_to(orbits: &Orbits, satellite: &String, dest: &String) -> u32 {
    let center = orbits.get(satellite).unwrap();
    println!("-> {}", center);
    if center == dest {
        1
    } else {
        1 + dist_to(orbits, center, dest)
    }
}

fn read_orbits() -> Orbits {
    let mut res: Orbits = HashMap::new();
    for line in io::stdin().lock().lines() {
        let parts: Vec<String> = line.unwrap().trim().split(')').map(|s| s.to_string()).collect();
        let center = &parts[0];
        let satellite = &parts[1];
        res.insert(satellite.to_string(), center.to_string());
    }
    res
}
