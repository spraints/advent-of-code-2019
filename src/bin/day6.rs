use std::collections::HashMap;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Orbit {
    center: String,
    satellite: String,
}

type Orbits = Vec<Orbit>;

struct OrbObj {
    center: Option<String>,
    satellites: Vec<String>,
}

type OrbitGraph = HashMap<String, OrbObj>;

fn main() {
    let orbits = read_orbits();
    //println!("{:?}", orbits);
    let orbits = make_graph(orbits);

    println!("TOTAL ORBITS: {}", walk_dist(&orbits, "COM"));

    let path = find_path(&orbits, "YOU", "SAN").unwrap();
    println!("DISTANCE FROM 'YOU' TO 'SAN': {}", path.len() - 1);
    println!("   {:?}", path);
}

fn find_path(objs: &OrbitGraph, from: &str, to: &str) -> Result<Vec<String>, String> {
    let from = from.to_string();
    let to = to.to_string();
    match objs.get(&from) {
        None => Err("FROM not found".to_string()),
        Some(from) => match objs.get(&to) {
            None => Err("TO not found".to_string()),
            Some(to) => intersecting_paths(
                walk_to_root(objs, &from.center, vec![]),
                walk_to_root(objs, &to.center, vec![]),
            ),
        },
    }
}

fn walk_to_root(objs: &OrbitGraph, start: &Option<String>, mut accum: Vec<String>) -> Vec<String> {
    match start {
        None => accum,
        Some(next_name) => {
            accum.push(next_name.to_string());
            match objs.get(next_name) {
                None => accum,
                Some(next_obj) => walk_to_root(objs, &next_obj.center, accum),
            }
        }
    }
}

fn intersecting_paths(path1: Vec<String>, path2: Vec<String>) -> Result<Vec<String>, String> {
    println!("p1: {:?}", path1);
    println!("p2: {:?}", path2);
    let overlap = path1
        .iter()
        .rev()
        .zip(path2.iter().rev())
        .filter(|(item1, item2)| item1 == item2)
        .count();
    if overlap == 0 {
        Err("no overlap!".to_string())
    } else {
        let path1 = path1.iter().rev().skip(overlap - 1);
        let path2 = path2.iter().rev().skip(overlap);
        Ok(path1.rev().chain(path2).map(|s| s.to_string()).collect())
    }
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
                        center: Some(orbit.center.to_string()),
                        satellites: vec![],
                    },
                );
            }
            Some(obj) => {
                obj.center = Some(orbit.center.to_string());
            }
        };
        match res.get_mut(&orbit.center) {
            None => {
                res.insert(
                    orbit.center.to_string(),
                    OrbObj {
                        center: None,
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
