use std::ops::Range;
use std::ascii::AsciiExt;


pub fn solution() {
    let temp = b"1,2,3";

    //    let input = [212, 254, 178, 237, 2, 0, 1, 54, 167, 92, 117, 125, 255, 61, 159, 164];
    let mut input: Vec<u32> = temp.iter().map(|x| format!("{:?}", x).parse::<u32>().unwrap()).collect();
    let extra: Vec<u32> = [17, 31, 73, 47, 23].iter().map(|x| x.to_owned() as u32).collect();
    input.extend(extra);

    let mut data: Vec<u32> = Range { start: 0, end: 256 }.collect();

    let mut position = 0;
    let mut skip_size = 0;
    let length = data.len();

    for i in input.iter() {
        for x in 0..64 {
            let data_clone = &data.clone().to_owned();

            let mut temp: Vec<u32> = Vec::new();
            for j in 0..i.clone() {
                let pos = (position + j) % length as u32;
                temp.push(data_clone[pos as usize]);
            }
            temp = temp.iter().rev().cloned().collect();

            for j in 0..i.clone() {
                let pos = (position + j) % length as u32;
                data[pos as usize] = temp[j as usize].to_owned();
            }
        }

        println!("data {:?}", data);
        position += i + skip_size;
        skip_size += 1;
    }

    let mut answer: Vec<u32> = Vec::new();
    let mut it = 0;
    while it < length {
        let arr: Vec<&u32> = data.iter().skip(it).take(16).collect();
        let mut value = 1;
        for x in arr.iter().skip(1) {
            value ^= x.to_owned();
        }
        answer.push(value);
        it += 16;
    }

    let axc: String = answer.iter().map(|x| format!("{:x}", x)).collect();

    println!("{}", axc);

    println!("data {:?}", data);
}