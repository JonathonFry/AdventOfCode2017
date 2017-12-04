use std::fs::File;
use std::io::prelude::*;

pub fn read(name: String) -> String {
    let mut data = String::new();
    let mut f = File::open(format!("{}{}{}", "./files/", name, ".txt"))
        .expect("Unable to open file");
    f.read_to_string(&mut data).expect("Unable to read string");
    return data;
}
