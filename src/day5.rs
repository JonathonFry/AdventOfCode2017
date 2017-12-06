use util::read;

pub fn solution() {
    let data = read("day5".to_owned());
    let vec = data.split("\n")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    println!("Day 5 part 1 {}", solve(&vec, &part1));
    println!("Day 5 part 2 {}", solve(&vec, &part2));
}


fn solve(vec: &Vec<i32>, f: &Fn(i32) -> i32) -> u32 {
    let mut input = vec.clone();

    let mut steps = 0;
    let mut value = 0;

    loop {
        if value >= input.len() {
            break;
        }

        let temp = input[value];
        input[value] += f(temp);

        let new_value = value as i32 + temp;
        value = new_value as usize;

        steps += 1;
    }

    return steps;
}

fn part1(_value: i32) -> i32 {
    return 1;
}

fn part2(value: i32) -> i32 {
    if value >= 3 {
        return -1;
    } else {
        return 1;
    }
}
