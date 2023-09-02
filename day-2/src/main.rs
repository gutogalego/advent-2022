use std::fs;
use std::path::Path;
use std::collections::HashMap;

fn main() {
    solve2();
}


fn solve1() {
    let relative_path = Path::new("input1.txt");
    let lines = fs::read_to_string(relative_path).expect("Unable to read file");

    let mut mapper = HashMap::new();
    mapper.insert('A', "rock");
    mapper.insert('B', "paper");
    mapper.insert('C', "scissors");
    mapper.insert('X', "rock");
    mapper.insert('Y', "paper");
    mapper.insert('Z', "scissors");

    let mut points_mapper = HashMap::new();
    points_mapper.insert("rock", 1);
    points_mapper.insert("paper", 2);
    points_mapper.insert("scissors", 3);
    points_mapper.insert("win", 6);
    points_mapper.insert("draw", 3);
    points_mapper.insert("loss", 0);


    let mut points = 0;

    for line in lines.lines(){
        let elf_hand = line.chars().nth(0).expect("issue");
        let elf_hand_normalized = mapper.get(&elf_hand).expect("issue");

        let my_hand = line.chars().nth(2).expect("issue");
        let my_hand_normalized = mapper.get(&my_hand).expect("issue");
        let mut result:&str;
        let paper: &str = "paper";
        let rock: &str = "rock";

        if my_hand_normalized == elf_hand_normalized {
            result = "draw";
        }

        else if my_hand_normalized.eq(&paper) {
            if elf_hand_normalized == &rock {
                result = "win";
            } else {
                result = "loss";
            }
        }
        else if my_hand_normalized == &rock {
            if elf_hand_normalized == &paper {
                result = "loss";
            }
            else {
                result = "win";
            }
        }
        else {
            if elf_hand_normalized.eq(&paper) {
                result = "win";
            }
            else {
                result = "loss";
            }
        }
        points += points_mapper.get(&result).expect("") + points_mapper.get(my_hand_normalized).expect("");
    }
    println!("{}", points);
}

fn solve2() {
    let relative_path = Path::new("input1.txt");
    let lines = fs::read_to_string(relative_path).expect("Unable to read file");

    let mut mapper = HashMap::new();
    mapper.insert('A', "rock");
    mapper.insert('B', "paper");
    mapper.insert('C', "scissors");
    mapper.insert('X', "loss");
    mapper.insert('Y', "draw");
    mapper.insert('Z', "win");

    let mut points_mapper = HashMap::new();
    points_mapper.insert("rock", 1);
    points_mapper.insert("paper", 2);
    points_mapper.insert("scissors", 3);
    points_mapper.insert("win", 6);
    points_mapper.insert("draw", 3);
    points_mapper.insert("loss", 0);


    let mut points = 0;

    for line in lines.lines(){
        let elf_hand = line.chars().nth(0).expect("issue");
        let elf_hand_normalized = mapper.get(&elf_hand).expect("issue");

        let my_hand = line.chars().nth(2).expect("issue");
        let result = mapper.get(&my_hand).expect("issue");

        let mut hand_played:&str;

        
        let paper: &str = "paper";
        let rock: &str = "rock";

        if result == &"draw" {
            hand_played = elf_hand_normalized
            
            
        } else if result == &"win" {
            if elf_hand_normalized == &"rock" {
                hand_played = "paper";
            } else if elf_hand_normalized == &"paper" {
                hand_played = "scissors";
            } else {
                hand_played = "rock";
            }

        } else {
            if elf_hand_normalized == &"rock" {
                hand_played = "scissors";
            } else if elf_hand_normalized == &"paper" {
                hand_played = "rock";
            } else {
                hand_played = "paper";
            }
        }



        points += points_mapper.get(result).expect("") + points_mapper.get(&hand_played).expect("");
    }
    println!("{}", points);
}