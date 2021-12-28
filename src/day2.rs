use std::fs;

pub fn read_string(file_name: &str) -> Vec<String> {
    let data = fs::read_to_string(file_name).expect("Unable to read file");
    let temp: Vec<String> = data.split_whitespace().map(|s| s.to_string()).collect();
    temp
}

pub fn day2a() {
    let temp = read_string("./src/day2.txt");

    let mut counter = 0;
    let mut horz_position = 0;
    let mut vert_position = 0;

    while counter < temp.len()
    {
        if temp[counter] == "forward" {
            horz_position += temp[counter + 1].parse::<i32>().unwrap();
        } else if temp[counter] == "up" {
            vert_position -= temp[counter + 1].parse::<i32>().unwrap();
        } else if temp[counter] == "down" {
            vert_position += temp[counter + 1].parse::<i32>().unwrap();
        } else {
            ()
        }

        counter += 1;
    }

    let final_depth = horz_position * vert_position;

    assert_eq!(final_depth, 1714950);
}






