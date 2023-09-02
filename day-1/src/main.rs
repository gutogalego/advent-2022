use std::fs;
use std::path::Path;

fn main() {
    solve1();
    solve2();
}

fn solve1() {
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

fn solve2() {
    let relative_path = Path::new("input1.txt");

    let mut current_cals = 0;
    let mut vec_elves: Vec<i64> = Vec::new();

    for line in fs::read_to_string(relative_path)
        .expect("Unable to read file")
        .lines()
    {
        if line.is_empty() {
            vec_elves.push(current_cals);
            current_cals = 0;
        } else {
            current_cals += line.parse::<i64>().expect("Issue parsing");
        }
    }
    vec_elves.sort();
    vec_elves.reverse();
    let last_three: i64 = vec_elves.iter().take(3).sum();
    println!("Result: {:?}", last_three);

}
