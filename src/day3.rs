use util::read;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash)]
struct Pair {
    x: i32,
    y: i32,
}

pub fn solution() {
    println!("Day 3 part 1 {}", part1());
    println!("Day 3 part 2 {}", part2());
}
fn part1() -> i32 {
    let data = read("day3".to_owned()).parse::<i64>().unwrap();
 
    let mut map: HashMap<i64, Pair> = HashMap::new();
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut amount = 1;
    let mut moveX = true;
    let mut positive = true;

    let mut i = 1;
    map.insert(i, Pair { x: x, y: y });
    'outer: loop {
        if moveX {
            for _j in 0..amount {
                if i == data { break 'outer; }
                if positive {
                    x+= 1;  
                } else{
                    x-= 1;  
                }
                i+= 1;  
                map.insert(i, Pair { x: x, y: y });
            }
            positive = !positive;
        } else {
            for _j in 0..amount {
                if i == data { break 'outer; }
                if positive{
                    y+= 1;  
                } else{
                    y-= 1;  
                }
                
                i+= 1;  
                map.insert(i, Pair { x: x, y: y });
            }
            amount += 1;
        }
        moveX = !moveX;
        if i == data { break 'outer; }
    }
    
    let vector = map.get(&data).unwrap();
    
    let mut sum = vector.x.abs() + vector.y.abs();
    return sum;
}

fn part2() -> i64 {
    return 0;
}