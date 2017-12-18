use util::read;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn solution() {
    let data = read("day12".to_owned());

    let input = data.split("\n").map(|x| {
        let v = x.split(" <-> ").map(|x| x.to_string()).collect::<Vec<String>>();
        let key = v[0].to_owned();
        let value = v[1].split(", ").map(|x| x.to_string()).collect::<Vec<String>>();
        (key, value)
    }).collect::<HashMap<String, Vec<String>>>();

    let mut visited: HashSet<String> = HashSet::new();
    visited.insert("0".to_owned());

    println!("Day 12 Part 1 {:?}", visit(&input, &mut visited, input.get("0").unwrap()));

    let mut parent: HashSet<String> = HashSet::new();
    for (key, _) in &input {
        let mut visited: HashSet<String> = HashSet::new();
        visit(&input, &mut visited, &input.get(key).unwrap());
        let mut temp = visited.iter().map(|x| x.to_string()).collect::<Vec<String>>();
        temp.sort();
        parent.insert(temp.join(""));
    }

    println!("Day 12 part 2 {}", parent.len())
}

fn visit(map: &HashMap<String, Vec<String>>, visited: &mut HashSet<String>, children: &Vec<String>) -> u32 {
    for child in children {
        if map.contains_key(child) && !&visited.contains(child) {
            visited.insert(child.to_owned());
            visit(map, visited, map.get(child).unwrap());
        }
    }

    return visited.len() as u32;
}
