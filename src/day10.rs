use std::ops::Range;

pub fn solution() {
    let mut data: Vec<u32> = Range { start: 0, end: 256 }.collect();

    let input = [212, 254, 178, 237, 2, 0, 1, 54, 167, 92, 117, 125, 255, 61, 159, 164];
    let mut position = 0;
    let mut skip_size = 0;
    let length = data.len();

    for i in input.iter() {
        let data_clone = &data.clone().to_owned();

        let mut temp: Vec<u32> = Vec::new();
        for j in 0..i.clone() {
            let pos = (position + j) % length as i32;
            temp.push(data_clone[pos as usize]);
        }
        temp = temp.iter().rev().cloned().collect();

        for j in 0..i.clone() {
            let pos = (position + j) % length as i32;
            data[pos as usize] = temp[j as usize].to_owned();
        }

        position += i + skip_size;
        skip_size += 1;
    }
    let part_one = data[0] * data[1];

    println!("Day 10 Part 1 {:?}", part_one);
}