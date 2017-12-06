use util::read;
use std::collections::HashMap;

pub fn solution() {
    let data = read("day6".to_owned());
    let vec = data.split(" ")
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    println!("Day 6 part 1 {}", solve(&vec, false));
    println!("Day 6 part 2 {}", solve(&vec, true));
}

fn solve(vec: &Vec<u32>, cycles: bool) -> u32 {
    let mut sum = 0;
    let mut input = vec.clone();
    let mut history: HashMap<String, u32> = HashMap::new();

    history.insert(get_history(&input), 0);

    let len = input.len() as u32;

    'searching: loop {
        let (mut block_size, mut index) = get_max(&input);
        input[index] = 0;
        sum += 1;
        while block_size > 0 {
            index += 1;
            input[(index % len as usize)] += 1;
            block_size -= 1;
        }
        let item: String = get_history(&input);
        if history.contains_key(&item) {
            if cycles {
                return history.len() as u32 - history[&item];
            }
            break 'searching;
        }
        history.insert(item, sum);
    }

    return sum;
}

fn get_max(input: &Vec<u32>) -> (u32, usize) {
    let mut max = 0;
    let mut index = 0;
    for i in 0..input.len() {
        let value = input[i];
        if value > max {
            max = value;
            index = i;
        }
    }
    return (max, index);
}

fn get_history(input: &Vec<u32>) -> String {
    input.iter().map(|x| x.to_string()).collect()
}
