use std::fs;
use std::path::Path;

fn main() {
    println!("Hello, world!");
    let relative_path = Path::new("input1.txt");

    let mut max_cal: i64 = 0;
    let mut current_cals = 0;

    for line in fs::read_to_string(relative_path)
        .expect("Unable to read file")
        .lines()
    {
        if line.is_empty() {
            if current_cals > max_cal {
                max_cal = current_cals;
                println!("{}", max_cal);
            }
            current_cals = 0;
        } else {
            current_cals += line.parse::<i64>().expect("Issue parsing");
        }
    }
}
