use util::read;

pub fn solution() {
    let data = read("day5".to_owned());
    let vec = data.split("\n")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    println!("Day 5 part 1 {}", part1(vec));
//    println!("Day 5 part 2 {}", part2());
}

fn part1(vec: Vec<i32>) -> u32 {
    let mut input = vec.clone();

    let mut steps = 0;
    let mut value = 0;

    loop {
        if value >= input.len() {
            break;
        }

        let temp = input[value];
        input[value] += 1;

        let new_value = value as i32 + temp;
        value = new_value as usize;

        steps += 1;
    }

    return steps;
}

fn part2() -> u32 {
    let sum = 0;
    return sum;
}
