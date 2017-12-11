use util::read;
use regex::Regex;

pub fn solution() {
    let data = read("day9".to_owned());

    let mut s = data;
    s = strip_cancels(s.as_ref());
    s = strip_noise(s.as_ref());

    println!("{}", solve(0, s.as_ref()));
}

fn solve(level: u32, input: &str) -> u32 {
    println!("level: {}, input: {}", level, input);
    let mut value = level + 1;

    let groups = get_groups(input);
    // if it doesn't contain any groups and is valid

    if groups.len() > 1 {
        for group in groups {
            value += solve(level + 1, group.as_ref());
        }
    } else {
        if !valid(input) {
            return 0;
        }
    }

    println!("value: {}", value);
    return value;
}

fn strip_cancels(input: &str) -> String {
    let re = Regex::new("(!.)").unwrap();
    let s = re.replace_all(input, "");
    return s.to_string();
}

fn strip_noise(input: &str) -> String {
    let re = Regex::new("[^<>{}]").unwrap();
    let s = re.replace_all(input, "");
    return s.to_string();
}

fn get_groups(input: &str) -> Vec<String> {
    let re = Regex::new("\\{[^}]*}").unwrap();

    let mut groups: Vec<String> = Vec::new();

    for cap in re.captures_iter(input) {
        groups.push(cap[0].to_string());
    }

    return groups;
}

fn valid(input: &str) -> bool {
    let re = Regex::new("<+[^>]*>|\\{}").unwrap();
    let valid = re.is_match(input);
    return valid;
}
