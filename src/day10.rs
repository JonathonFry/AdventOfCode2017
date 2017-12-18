use std::ops::Range;

pub fn solution() {
    part_one();
    part_two();
}

fn part_one() {
    let mut data: Vec<u32> = (0..256).collect();

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

fn part_two() {
    let temp = b"212,254,178,237,2,0,1,54,167,92,117,125,255,61,159,164";
    let mut input: Vec<u32> = temp.iter().map(|x| format!("{:?}", x).parse::<u32>().unwrap()).collect();
    let extra: Vec<u32> = [17, 31, 73, 47, 23].iter().map(|x| x.to_owned() as u32).collect();
    input.extend(extra);

    let mut data: Vec<u32> = (0..256).collect();

    let mut position = 0;
    let mut skip_size = 0;
    let len = data.len() as u32;

    for _round in 0..64 {
        for length in input.iter() {
            for i in 0..(length / 2) {
                data.swap(((position + i) % len) as usize, ((position + length - i - 1) % len) as usize);
            }

            position += length + skip_size;
            skip_size += 1;
        }
    }

    let hash: String = data.chunks(16)
        .map(|chunk| chunk.iter().fold(0, |acc, &v| acc ^ v as u8))
        .map(|v| {
            let mut t = format!("{:x}", v);
            if t.len() == 1 {
                t.insert(0, '0');
            }
            t
        })
        .collect::<Vec<String>>()
        .join("");

    println!("Day 10 Part 2 {}", hash);
}
