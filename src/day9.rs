use util::read;
use regex::Regex;
use std::collections::HashMap;

pub fn solution() {
    let data = read("day9".to_owned());

    let mut s = data;
    s = strip_cancels(s.as_ref());

    let (part_one, part_two) = solve(s.as_ref());
    println!("Day 9 Part 1 {}", part_one);
    println!("Day 9 Part 2 {}", part_two);
}

fn solve(input: &str) -> (u32, u32) {
    let mut level = 0;
    let mut value = 0;
    let mut last_open: HashMap<u32, usize> = HashMap::new();
    let mut ignore = false;
    let mut count = 0;

    for (i, c) in input.chars().into_iter().enumerate() {
        if ignore && c != '>' {
            count += 1;
        }
        if c == '<' {
            ignore = true;
        } else if c == '>' {
            ignore = false;
        }
        if c == '{' && !ignore {
            level += 1;
            last_open.insert(level, i);
        } else if c == '}' && !ignore {
            let mut group: String = String::new();
            if i == input.len() - 1 {
                group = input.to_owned();
            } else {
                let temp = last_open.get(&level).unwrap().to_owned();
                group = input.chars().skip(temp).take((i + 1) - temp).collect();
            }

            if valid(&group) {
                value += level;
            }
            level -= 1;
        }
    }
    return (value as u32, count as u32);
}

fn strip_cancels(input: &str) -> String {
    let re = Regex::new("(!.)").unwrap();
    let s = re.replace_all(input, "");
    return s.to_string();
}

fn valid(input: &str) -> bool {
    let re = Regex::new("<+[^>]*>|\\{}").unwrap();
    let valid = re.is_match(input);
    return valid;
}