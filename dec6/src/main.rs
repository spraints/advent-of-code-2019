use std::collections::HashMap;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Orbit {
    center: String,
    satellite: String,
}

type Orbits = Vec<Orbit>;

struct OrbObj {
    name: String,
    center: String,
    satellites: Vec<String>,
}

type OrbitGraph = HashMap<String, OrbObj>;

fn main() {
    let orbits = read_orbits();
    //println!("{:?}", orbits);
    let orbits = make_graph(orbits);

    println!("TOTAL ORBITS: {}", walk_dist(&orbits, "COM"));
}

fn walk_dist(objs: &OrbitGraph, center: &str) -> u32 {
    walk_dist2(objs, &center.to_string()).1
}

// Returns (num_descendants, orbits)
fn walk_dist2(objs: &OrbitGraph, center: &String) -> (u32, u32) {
    match objs.get(center) {
        None => {
            println!("WARNING! tried to walk to {}, but it isn't found!", center);
            (0, 0)
        }
        Some(obj) => {
            let mut res = (0, 0);
            for sat in &obj.satellites {
                let satres = walk_dist2(objs, sat);
                // Descendents accumulates all of the satellite's descendants, plus the satellite.
                res.0 = res.0 + satres.0 + 1;
                // Orbits accumulates all of the satellite's orbits, plus one more for each
                // descendant, plus one for this orbit.
                res.1 = res.1 + satres.1 + satres.0 + 1;
            }
            res
        }
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

fn make_graph(orbits: Orbits) -> OrbitGraph {
    let mut res = HashMap::new();
    for orbit in orbits {
        match res.get_mut(&orbit.satellite) {
            None => {
                res.insert(
                    orbit.satellite.to_string(),
                    OrbObj {
                        name: orbit.satellite.to_string(),
                        center: orbit.center.to_string(),
                        satellites: vec![],
                    },
                );
            }
            Some(obj) => {
                obj.center = orbit.center.to_string();
            }
        };
        match res.get_mut(&orbit.center) {
            None => {
                res.insert(
                    orbit.center.to_string(),
                    OrbObj {
                        name: orbit.center.to_string(),
                        center: "??".to_string(),
                        satellites: vec![orbit.satellite.to_string()],
                    },
                );
            }
            Some(obj) => {
                obj.satellites.push(orbit.satellite.to_string());
            }
        };
    }
    res
}
