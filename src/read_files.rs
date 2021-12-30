use std;
use std::fs;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

pub fn file_to_raw_string(file_name: &str) -> String {
    let raw_string = fs::read_to_string(file_name)
        .expect("Something went wrong reading the file");

    raw_string
}

//Read from file into vector of Strings
pub fn file_to_vec_string(file_name: &str) -> Vec<String> {
    let data = fs::read_to_string(file_name).expect("Unable to read file");
    let temp: Vec<String> = data.split_whitespace().map(|s| s.to_string()).collect();
    temp
}

//Read from file into vector of u64 integers
pub fn file_to_vec_uint<R: Read>(io: R) -> Result<Vec<u64>, Error> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

//Read from file into a 2D Char Array
pub fn string_to_char_matrix(raw_string: &str) -> Vec<Vec<char>> {
    let parsed_matrix =
        //split string by newlines
        raw_string.lines()
            // filter out all empty characters
            .filter(|s| !s.is_empty())
            // map the string into a vector of chars
            .map(|c| c.chars().collect::<Vec<char>>())
            // collect all the elements of the iterator into a large variety of different types
            // store the vector of strings into an additional vector
            .collect::<Vec<Vec<char>>>();

    parsed_matrix
}



