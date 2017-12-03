use util::read;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash)]
struct Pair {
    x: i64,
    y: i64,
}


pub fn solution() {
    println!("Day 3 part 1 {}", part1());
    println!("Day 3 part 2 {}", part2());
}
fn part1() -> i64 {
    let data = read("day3".to_owned()).parse::<i64>().unwrap();

 /*
    move x 1
    move y 1
    move x 2
    move y 2
    move x 3 
    move y 3

    */

    /*
    x positive 1
    y negative 1
    x negative 2
    y positive 2
    x positive 3
    y negative 3
    */
 
    let mut map: HashMap<Pair, i64> = HashMap::new();
    let mut x: i64 = 0;
    let mut y: i64 = 0;
    let mut amount = 1;
   
    let mut current = Pair{x: 0, y: 0};

    for i in 0..data {
        println!("{}", i);
        map.insert(Pair { x: x, y: y }, i + 1);
        x += amount;
        y += amount;

        current = Pair{ x: current.x + amount, y: current.y + amount};
    }
    /*
    0,0
    1,0
    1,-1
    0,-1
    -1,-1
    -1,0

    */

    // for (key, value) in map {
    //     println!("{},{} | {}", key.x, key.x, value);
    // }

    let mut sum = 0;
    return sum;
}

fn part2() -> i64 {
    return 0;
}
