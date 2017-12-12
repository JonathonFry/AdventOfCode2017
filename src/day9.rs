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

    let has_groups = contains_groups(input);
    println!("input: {} contains groups: {}", input, has_groups);

    let groups = get_groups(input);
    // if it doesn't contain any groups and is valid

    if valid(input) {
        return value;
    } else if groups.len() > 1 {
        for group in groups {
            value += solve(level + 1, group.as_ref());
        }
    } else {
        return 0;
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

fn contains_groups(input: &str) -> bool {
    // Remove first and last character if {}
    // Copy everything from first bracket to
    /*
    {{}}
    find first closing bracket, match with last opening bracket (substring)
    */
    let mut groups: Vec<String> = Vec::new();
    let buf = String::new();
    let mut index = 0;
    for c in input.chars() {
        if  c.to_string() == "}" {

        }
        if c.to_string() == "}" && index as usize != input.len() {
            return true;
        }
        index += 1;
    }

    return false;
}
