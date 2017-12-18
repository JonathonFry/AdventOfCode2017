use util::read;

pub fn solution() {
    let input: Vec<String> = read("day11".to_owned()).split(",").map(|s| s.to_string()).collect();
    let mut x = 0;
    let mut y = 0;
    let mut z = 0;

    for i in input {
        let (new_x, new_y, new_z) = calc(i.as_ref());
        x += new_x;
        y += new_y;
        z += new_z;
    }

    let value = (x.abs() + y.abs() + z.abs()) / 2;

    println!("x {}, y {}, z {}", x, y, z);
    println!("value {}", value);
}


fn calc(direction: &str) -> (i32, i32, i32) {
    return match direction {
        "nw" => (-1, 1, 0),
        "n" => (0, 1, -1),
        "ne" => (1, 0, -1),
        "se" => (1, -1, 0),
        "s" => (0, -1, 1),
        "sw" => (-1, 0, 1),
        _ => panic!("OMG")
    };
}
