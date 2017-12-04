use util::read;

pub fn solution() {
    println!("Day 4 part 1 {}", part1());
    println!("Day 4 part 2 {}", part2());
}

fn part1() -> u32 {
    let data = read("day4".to_owned());

    let phrases = data.split("\n").map(|x| {
        x.split(" ")
            .collect::<Vec<&str>>()
    });

    let mut duplicates : Vec<Vec<&str>> = Vec::new();
    let mut size = 0;
    for s in phrases {
        size+=1;
        let mut temp = s;
        temp.sort();
        
        'phrase: for i in 1..temp.len() {
            if valid(temp[i-1], temp[i]) {
                duplicates.push(temp);
                break 'phrase;
            }
        }
    }

    fn valid(first: &str, second: &str) -> bool {
        return first == second;
    }

    let mut sum = size - duplicates.len() as u32;
    return sum;
}

fn part2() -> u32 {
    let data = read("day4".to_owned());

    let phrases = data.split("\n").map(|x| {
        x.split(" ")
        .map(|y| {
            let mut temp = y.chars().collect::<Vec<char>>();
            temp.sort();
            temp.into_iter().collect::<String>()
        })
            .collect::<Vec<String>>()
    });

    let mut duplicates : Vec<Vec<String>> = Vec::new();
    let mut size = 0;
    for s in phrases {
        size+=1;
        let mut temp = s;
        temp.sort();
        
        'phrase: for i in 1..temp.len() {
            if valid(temp[i-1].as_str(), temp[i].as_str()) {
                duplicates.push(temp);
                break 'phrase;
            }
        }
    }

    fn valid(first: &str, second: &str) -> bool {
        return first == second;
    }

    let mut sum = size - duplicates.len() as u32;
    return sum;
}


