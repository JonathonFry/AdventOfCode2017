use util::read;
use std::collections::HashMap;

struct Matrix(i32, i32, i32, f32);

pub fn solution() {
    println!("Day 3 part 1 {}", part1());
    println!("Day 3 part 2 {}", part2());
}
fn part1() -> i64 {
    let data = read("day3".to_owned());
    let size = data.parse::<i64>().unwrap();
    let temp = size.to_owned();
    let mut count = 0;

    let square = (temp as f64).sqrt().round();
    println!("{}", square);
    let squareSize = (square * square) as i64;
    println!("{}", squareSize);
    println!("{}", size);
    let x = square as i64 - (squareSize - size) - 1;
    let y = square as i64 - 1;
    println!("x,y {},{}", x, y);

    let middle = (square as i64 - 1) / 2;

    println!("middle x,y {},{}", middle, middle);
    /*
    17  16  15  14  13
    18   5   4   3  12
    19   6   1   2  11
    20   7   8   9  10
    21  22  23  24  25
    */

    let temp1 = x - middle;
    let temp2 = y - middle;

    println!("a b {},{}", temp1, temp2);

    let mut sum = 0;
    return sum;
}

fn part2() -> i64 {
    return 0;
}
