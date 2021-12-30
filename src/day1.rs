use std::fs::File;
use std::io::Error;

use crate::read_files;

pub fn day1a() -> Result<(), Error> {
    //let temp = read_files::read("./file_input/day1.txt");
    let vec = read_files::file_to_vec_uint(File::open("./file_input/day1.txt")?)?;

    // use `vec` for whatever
    let mut increased = 0;
    //Day 1A
    let mut i = 0;
    while i < vec.len() - 1 {
        if vec[i + 1] > vec[i] {
            increased += 1;
        }
        i += 1;
    }
    assert_eq!(increased, 1301);
    Ok(())
}

pub fn day1b() -> Result<(), Error> {
    let vec = read_files::file_to_vec_uint(File::open("./file_input/day1.txt")?)?;
    // use `vec` for whatever
    let mut increased = 0;
    //Day 1A
    let mut i = 0;
    while i < vec.len() - 3 {
        if vec[i + 3] > vec[i] {
            increased += 1;
        }
        i += 1;
    }
    assert_eq!(increased, 1346);
    Ok(())
}