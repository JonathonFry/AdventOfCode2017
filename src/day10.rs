use std::ops::Range;

pub fn solution() {
    let mut data: Vec<u32> = Range { start: 0, end: 5 }.collect();

    let input = [3, 4, 1, 5];
    let mut position = 0;
    let length = data.len();

    for i in input.iter() {
        let current = position % length;

        let temp: Vec<&u32> = data.iter().skip(position % length).take(*i).collect();
        println!("{:?}", temp);
        position += i;
    }

    let reversed: Vec<u32> = data.iter().rev().cloned().collect();
    println!("{:?}", reversed);

    /*
    initialise list with 0:255 values
    for each input
    start at current position
    reverse the range given in input
    current position += input + skip size
    skip size +=1

    Notes
    Circular array = position % length for access
    */
}