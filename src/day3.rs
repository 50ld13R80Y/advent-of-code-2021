// gamma rate - find the most common bit in each position of the binary numbers
// epsilon rate - find the least common bit in each position of the binary numbers
// read binary data into a 2D int vector
// read as a string and split into vector
use crate::read_files;

//Convert a binary vector of i32 int to integer
fn bin_vector_to_decimal(input_vector: &Vec<i32>) -> i32 {
    let mut bin = 0;
    let mut counter = 0;
    while counter < input_vector.len() {

        bin *= 2;
        bin = bin + input_vector[counter];
        counter += 1;

    }
    bin
}

//Read from file into 2D vector
pub fn day3a_test() {
    //Read raw input into string
    let raw_string = read_files::file_to_raw_string("./file_input/test3.txt");
    let parsed_matrix = read_files::string_to_char_matrix(&raw_string);

    //traverse through each bit position
    const BIT_CONSTANT: usize = 5;

    let mut bit_counter = 0;

    let mut gamma_rate_vector: Vec<i32> = Vec::new();
    let mut epsilon_rate_vector: Vec<i32> = Vec::new();

    while bit_counter < BIT_CONSTANT {
        //loop from 0 to size of matrix
        let mut counter = 0;
        let mut one_count = 0;
        let mut zero_count = 0;
        while counter < parsed_matrix.len() {
            if parsed_matrix[counter][bit_counter] == '1' {
                one_count += 1;
            }
            else {
                zero_count += 1;
            }
            counter += 1;
        }
        //If there are more ones
        if one_count > zero_count {
            gamma_rate_vector.push(1);
            epsilon_rate_vector.push(0);
        }
        else {
            gamma_rate_vector.push(0);
            epsilon_rate_vector.push(1);
        }
        bit_counter += 1;
    }

    let gamma_rate = bin_vector_to_decimal(&gamma_rate_vector);
    let epsilon_rate = bin_vector_to_decimal(&epsilon_rate_vector);
    let power_rate = gamma_rate * epsilon_rate;

    assert_eq!(power_rate, 198);
}

pub fn day3a() {
    //Read raw input into string
    let raw_string = read_files::file_to_raw_string("./file_input/day3.txt");
    let parsed_matrix = read_files::string_to_char_matrix(&raw_string);

    //traverse through each bit position
    const BIT_CONSTANT:usize = 12;
    let mut bit_counter = 0;

    let mut gamma_rate_vector: Vec<i32> = Vec::new();
    let mut epsilon_rate_vector: Vec<i32> = Vec::new();

    while bit_counter < BIT_CONSTANT {
        //loop from 0 to size of matrix
        let mut counter = 0;
        let mut one_count = 0;
        let mut zero_count = 0;
        while counter < parsed_matrix.len() {
            if parsed_matrix[counter][bit_counter] == '1' {
                one_count += 1;
            }
            else {
                zero_count += 1;
            }
            counter += 1;
        }
        //If there are more ones
        if one_count > zero_count {
            gamma_rate_vector.push(1);
            epsilon_rate_vector.push(0);
        }
        else {
            gamma_rate_vector.push(0);
            epsilon_rate_vector.push(1);
        }
        bit_counter += 1;
    }

    let gamma_rate = bin_vector_to_decimal(&gamma_rate_vector);
    let epsilon_rate = bin_vector_to_decimal(&epsilon_rate_vector);
    let power_rate = gamma_rate * epsilon_rate;

    assert_eq!(power_rate, 852500);
}


