use std::collections::HashSet;



fn main() {
    //solve1();
    solve2();
}


fn solve1() {
    let file = std::fs::read_to_string("input1.txt").unwrap();
    let mut total:i32 = 0;
    file.lines().for_each(|line| {
        let length = line.len();
        let (c1, c2) = line.split_at(length/2);

        let a: HashSet<char> = c1.chars().collect();
        let b: HashSet<char> = c2.chars().collect();

        let common_chars = a.intersection(&b);

        let normalized_ascii = common_chars.fold(0, |_, el| {
            let ascii = (*el) as i32;
            if ascii < 97 {
                return ascii - 38;
            } else {
                return ascii - 96;
            }
        });
        total += normalized_ascii as i32;
    });

    println!("{:?}", total);

}

fn solve2() {
    let file = std::fs::read_to_string("input0.txt").unwrap();
    let mut total:i32 = 0;
    let mut group: Vec<&str> = Vec::new();



    file.lines().for_each(|line| {
        group.push(&line);
        if group.len() == 3 {
            let sets: Vec<Vec<char>> = group.iter().map(|string| string.chars().collect()).collect();

            
            print!("{:?}",sets)
            //let badge = sets.iter().reduce(|acc, set| acc.intersection(set));

        }
    });

    println!("{:?}", total);
}
