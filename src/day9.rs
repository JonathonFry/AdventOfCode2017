use util::read;
use regex::Regex;

pub fn solution() {
    let data = read("day9".to_owned());

    test();

    let mut s = data;
    s = strip_cancels(s.as_ref());
    s = strip_noise(s.as_ref());

//    println!("{}", solve(0, s.as_ref()));
}

fn test() {
    let test = ["{<>}",
        "{<random characters>}",
        "{{<<<<>}}",
        "{{},{<{!>}>}}",
        "{<!!>}",
        "{<!!!>>}",
        "\"\"{<{o\"i!a,<{i<a>,{}}\"\""
    ];
    for string in test.iter() {
        let mut s = string.to_string();
        s = strip_cancels(s.as_ref());
        s = strip_noise(s.as_ref());

        println!("{}", solve(0, s.as_ref()));
    }
}

fn solve(level: u32, input: &str) -> u32 {
    let mut new_level = level + 1;
    let mut value = new_level;

    println!("level: {}, input {:?}", new_level, input);

    let groups = get_groups(input);

    println!("input: {}, valid = {}", input, valid(input));

    if groups.len() > 0 {
        for group in &groups {
            value += solve(new_level, group.as_ref());
        }
    } else {
        let valid = valid(input);
        println!("input: {}, valid = {}", input, valid);
        if valid {
            return value;
        } else {
            return 0;
        }
    }

    return value;
}

fn strip_cancels(input: &str) -> String {
    let re = Regex::new("(!.)").unwrap();
    let s = re.replace_all(input, "");
    return s.to_string();
}

fn strip_noise(input: &str) -> String {
    let re = Regex::new("[^<>{},]").unwrap();
    let s = re.replace_all(input, "");
    return s.to_string();
}


fn valid(input: &str) -> bool {
    let re = Regex::new("<+[^>]*>|\\{}").unwrap();
    let valid = re.is_match(input);
    return valid;
}

fn get_groups(input: &str) -> Vec<String> {
    let mut data: String = input.chars().skip(1).take(input.len() - 2).collect();
    if data.starts_with("{{") && data.ends_with("}}") {
        let mut groups: Vec<String> = Vec::new();
        groups.push(data);
        return groups;
    } else if data.starts_with("<") && data.ends_with(">") {
        return Vec::new()
    }

    let mut groups = data.split(",")
        .filter(|x| x.contains("{") && x.contains("}"))
        .filter(|x| !x.is_empty())
        .map(|x| x.to_owned()).collect::<Vec<String>>();

    return groups;
}
//correct answer is 7053