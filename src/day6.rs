use util::read;

pub fn solution() {
    let data = read("day6".to_owned());
    let vec = data.split(" ")
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    println!("Day 6 part 1 {}", part1(&vec));
    println!("Day 6 part 2 {}", part2(&vec));
}


fn part1(vec: &Vec<u32>) -> u32 {
    let mut sum = 0;
    let mut input = vec.clone();
    let mut history: Vec<String> = Vec::new();
    history.push(get_history(&input));

    let len = input.len() as u32;

    'searching: loop {
        let (mut block_size, mut index) = get_max(&input);
        input[index as usize] = 0;
        sum += 1;
        while block_size > 0 {
            index += 1;
            input[(index % len) as usize] += 1;
            block_size -= 1;
        }
        let item: String = get_history(&input);
        if history.contains(&item) {
            break 'searching;
        }
        history.push(item);
    }

    return sum;
}

fn part2(vec: &Vec<u32>) -> u32 {
    let mut input = vec.clone();
    let mut history: Vec<String> = Vec::new();
    history.push(get_history(&input));

    let len = input.len() as u32;

    'searching: loop {
        let (mut block_size, mut index) = get_max(&input);
        input[index as usize] = 0;
        while block_size > 0 {
            index += 1;
            input[(index % len) as usize] += 1;
            block_size -= 1;
        }
        let item: String = get_history(&input);
        if history.contains(&item) {
            let item_index = history.iter().position(|r| r == &item).unwrap();
            return  (history.len() - item_index) as u32;
        }
        history.push(item);
    }
}


fn get_max(input: &Vec<u32>) -> (u32, u32) {
    let mut max = 0;
    let mut index = 0;
    for i in 0..input.len() {
        let value = input[i];
        if value > max {
            max = value;
            index = i;
        }
    }
    return (max, index as u32);
}

fn get_history(input: &Vec<u32>) -> String {
    input.iter().map(|x| x.to_string()).collect()
}
