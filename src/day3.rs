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
    let data = read("day3".to_owned());
    let size = data.parse::<i64>().unwrap();

    let square = (size.to_owned() as f64).sqrt().round();
    let square_size = (square * square) as i64;
    let middle = (square as i64 - 1) / 2;
    let x = square as i64 - (square_size - size) - 1;
    let y = square as i64 - 1;

    let sum = (x - middle).abs() + (y - middle).abs();
    return sum as i32;
}

fn part2() -> i32 {
    let data = read("day3".to_owned()).parse::<i32>().unwrap();

    let mut map: HashMap<Pair, i32> = HashMap::new();
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut amount = 1;
    let mut move_x = true;
    let mut positive = true;

    let mut i = 1;
    map.insert(Pair { x, y }, i);
    'outer: loop {
        if move_x {
            for _j in 0..amount {
                if i == data {
                    break 'outer;
                }
                if positive {
                    x += 1;
                } else {
                    x -= 1;
                }
                i += 1;
                let value = get_value(x, y, &map);
                if value > data {
                    return value;
                }
                map.insert(Pair { x, y }, value);
            }
            positive = !positive;
        } else {
            for _j in 0..amount {
                if i == data {
                    break 'outer;
                }
                if positive {
                    y += 1;
                } else {
                    y -= 1;
                }

                i += 1;
                let value = get_value(x, y, &map);
                if value > data {
                    return value;
                }
                map.insert(Pair { x, y }, value);
            }
            amount += 1;
        }
        move_x = !move_x;
        if i == data {
            break 'outer;
        }
    }

    return 0;
}


fn get_value(x: i32, y: i32, map: &HashMap<Pair, i32>) -> i32 {
    let mut sum = 0;

    for i in -1..2 {
        for j in -1..2 {
            if i == x && j == y {
                continue;
            }
            sum += map.get(&Pair { x: x + i, y: y + j }).unwrap_or(&0);
        }
    }

    return sum;
}
