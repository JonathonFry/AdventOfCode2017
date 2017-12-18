use util::read;

pub fn solution() {
    let input: Vec<String> = read("day11".to_owned()).split(",").map(|s| s.to_string()).collect();
    let mut x = 0;
    let mut y = 0;

    for i in input {
        let (new_x, new_y) = calc(i.as_ref());
        x += new_x;
        y += new_y;
    }
    println!("x {}, y {}", x, y);
}

fn calc(direction: &str) -> (i32, i32) {
    return match direction {
        "nw" => (-1, 1),
        "n" => (0, 1),
        "ne" => (1, 1),
        "se" => (1, -1),
        "s" => (0, -1),
        "sw" => (-1, -1),
        _ => panic!("OMG")
    };
}
