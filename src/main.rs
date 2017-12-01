use std::fs::File;
use std::io::prelude::*;


fn main() {
    let name = "day1";
    let data = read(name.to_owned());
    let vec = data.chars()
        .map(|x| x.to_digit(10).unwrap())
        .collect::<Vec<u32>>();

    println!("Part one {}", part1(&vec));
    println!("Part two {}", part2(&vec));
}


fn part1(vec: &Vec<u32>) -> u32 {
    let mut last = vec.last().unwrap();
    let mut sum = 0;
    for i in 0..vec.len() {
        let current = vec[i];
        if current == *last {
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
        let mut lookahead = 0;
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

fn read(name: String) -> String {
    let mut data = String::new();
    let mut f = File::open(format!("{}{}{}", "./files/", name, ".txt"))
        .expect("Unable to open file");
    f.read_to_string(&mut data).expect("Unable to read string");
    return data;
}
