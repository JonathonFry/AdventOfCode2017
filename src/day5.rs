use util::read;

pub fn solution() {
    let data = read("day1".to_owned());
    let vec = data.split("\n")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    println!("Day 5 part 1 {}", part1(&vec));
//    println!("Day 5 part 2 {}", part2());
}

fn part1(vec: &Vec<i32>) -> u32 {
    let mut input = vec;
    let mut steps = 0;
    let mut value = 0;
    loop {
        if value >= input.len() {
            break;
        }

        let temp = &mut input[value];
        temp = temp + 1;


        steps += 1;
        if steps == 1000 { break; }
    }

    return steps;
}

fn part2() -> u32 {
    let mut sum = 0;
    return sum;
}
