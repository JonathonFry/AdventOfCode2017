use util::read;

pub fn solution() {
    println!("Day 2 part 1 {}", part1());
    println!("Day 2 part 2 {}", part2());
}
fn part1() -> i64 {
    let data = read("day2".to_owned());

    let spreadsheet = data.split("\n").map(|x| {
        x.split("	")
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>()
    });
    let mut sum = 0;
    for s in spreadsheet {
        let mut max = s.iter().max();
        let mut min = s.iter().min();
        sum += max.unwrap() - min.unwrap();
    }
    return sum;
}

fn part2() -> i64 {
    let data = read("day2".to_owned());

    let spreadsheet = data.split("\n").map(|x| {
        x.split("	")
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>()
    });
    let mut sum = 0;
    for s in spreadsheet {
        for i in 0..s.len() {
            for j in 0..s.len() {
                if i == j {
                    continue;
                }
                if s[i] % s[j] == 0 {
                    sum += s[i] / s[j];
                }
            }
        }
    }
    return sum;
}
