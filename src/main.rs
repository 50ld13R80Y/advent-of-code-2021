mod day1;
mod day2;
mod day3;
mod read_files;

fn main() {
    day1::day1a().unwrap();
    day1::day1b().unwrap();
    day2::day2a();
    day2::day2b();
    day3::day3a_test();
    day3::day3a();
}