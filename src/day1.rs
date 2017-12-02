pub fn part1(vec: &Vec<u32>) -> u32 {
    let mut last = vec.last().unwrap();
    let mut sum = 0;
    for i in 0..vec.len() {
        let current = vec[i];
        if current == *last {
            sum += current;
        }
        last = &vec[i];
    }
    return sum;
}

pub fn part2(vec: &Vec<u32>) -> u32 {
    let mut sum = 0;
    let len = vec.len();
    let step = len / 2;

    for i in 0..len {
        let current = vec[i];
        let next = step + i;
        let mut lookahead = 0;
        if next >= len {
            lookahead = vec[next - len];
        } else {
            lookahead = vec[next];
        }

        if current == lookahead {
            sum += current;
        }
    }
    return sum;
}
