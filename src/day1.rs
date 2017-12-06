use util::read;

pub fn solution() {
    let data = read("day1".to_owned());
    let vec = data.chars()
        .map(|x| x.to_digit(10).unwrap())
        .collect::<Vec<u32>>();

    println!("Day 1 part 1 {}", part1(&vec));
    println!("Day 1 part 2 {}", part2(&vec));
}

fn part1(vec: &Vec<u32>) -> u32 {
    let mut last = vec.last().unwrap();
    let mut sum = 0;
    for i in 0..vec.len() {
        let current = &vec[i];
        if current == last {
            sum += current;
        }
        last = &vec[i];
    }
    return sum;
}

fn part2(vec: &Vec<u32>) -> u32 {
    let mut sum = 0;
    let len = vec.len();
    let step = len / 2;

    for i in 0..len {
        let current = vec[i];
        let next = step + i;
        let lookahead;
        if next >= len {
            lookahead = vec[next - len];
        } else {
            lookahead = vec[next];
        }

        if current == lookahead {
            sum += current;
        }
    }
    return sum;
}
