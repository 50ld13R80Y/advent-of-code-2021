use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

//Read from buffer and convert to vector
fn read<R: Read>(io: R) -> Result<Vec<u64>, Error> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

pub fn day1a() -> Result<(), Error> {
    let vec = read(File::open("./src/day1")?)?;
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
    let vec = read(File::open("./src/day1")?)?;
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