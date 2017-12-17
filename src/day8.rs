use util::read;
use std::collections::HashMap;

#[derive(Debug)]
enum Type {
    Increment,
    Decrement
}

#[derive(Debug)]
enum Operation {
    Greater,
    Less,
    GreaterOrEqual,
    LessOrEqual,
    Equal,
    NotEqual
}

#[derive(Debug)]
struct Item {
    target: String,
    t: Type,
    value: i32,
    check: String,
    operation: Operation,
    check_value: i32
}

fn parse_type(value: String) -> Type {
    return match value.as_ref() {
        "inc" => Type::Increment,
        "dec" => Type::Decrement,
        _ => panic!("OMG")
    };
}

fn parse_operation(value: String) -> Operation {
    return match value.as_ref() {
        ">" => Operation::Greater,
        "<" => Operation::Less,
        ">=" => Operation::GreaterOrEqual,
        "<=" => Operation::LessOrEqual,
        "==" => Operation::Equal,
        "!=" => Operation::NotEqual,
        _ => panic!("OMG {} ", value)
    };
}

pub fn solution() {
    let mut map: HashMap<String, i32> = HashMap::new();

    let data = read("day8".to_owned());
    let input = data.split("\n").map(|x| {
        let temp: Vec<String> = x.split(" ").map(|s| s.to_string()).collect();

        Item {
            target: temp[0].to_owned(),
            t: parse_type(temp[1].to_owned()),
            value: temp[2].to_owned().parse::<i32>().unwrap(),
            check: temp[4].to_owned(),
            operation: parse_operation(temp[5].to_owned()),
            check_value: temp[6].to_owned().parse::<i32>().unwrap()
        }
    }).collect::<Vec<Item>>();

    let mut max = 0;

    for item in input {
        let current_value = get_value(item.check, &mut map);
        let value = item.value;
        let check_value = item.check_value;
        let complete = match item.operation {
            Operation::Greater => current_value > check_value,
            Operation::Less => current_value < check_value,
            Operation::GreaterOrEqual => current_value >= check_value,
            Operation::LessOrEqual => current_value <= check_value,
            Operation::Equal => current_value == check_value,
            Operation::NotEqual => current_value != check_value,
            _ => panic!("OMG")
        };


        if complete {
            let target = item.target;
            let target_value = get_value(target.to_owned(), &mut map);

            let new_value = match item.t {
                Type::Increment => target_value + value,
                Type::Decrement => target_value - value,
                _ => panic!("OMG")
            };
            if new_value > max {
                max = new_value;
            }
            map.insert(target, new_value);
        }
    }

    let max_in_map = map.values().into_iter().max();

    println!("Day 8 Part 1 {:?}", max_in_map.unwrap());
    println!("Day 8 Part 2 {:?}", max);
}

fn get_value(key: String, map: &mut HashMap<String, i32>) -> i32 {
    if map.contains_key(&key) {
        return map.get(&key).unwrap().clone();
    }
    map.insert(key, 0);
    return 0;
}