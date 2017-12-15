pub fn solution() {
    let data = ["0", "1", "2", "3", "4"];
    let input = 3;
    let position = 0;

    let reversed: Vec<&str> = data.iter().rev().cloned().collect();
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