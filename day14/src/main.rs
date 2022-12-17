use std::collections::HashMap;
use crate::Material::{ROCK, SAND};

#[derive(Debug, PartialEq, Clone)]
enum Material {
    SAND,
    ROCK
}

fn main() {
    let input = include_str!("../Input.txt");
    let resp = input_to_cave(input);
    let cave = resp.0;
    let largest_y = resp.1;

    part1(cave.clone());
    part2(cave.clone(), largest_y);
}

fn part1(mut cave: HashMap<(i32, i32), Material>) {
    let mut finished = false;
    let mut number_of_sand = 0;

    while !finished {
        let mut sand_pos = (500, 0);
        let mut number_of_empty_falls = 0;
        loop {
            let key = &(sand_pos.0, sand_pos.1+1);
            if cave.contains_key(key) {
                // Try left then right
                if !cave.contains_key(&(key.0-1, key.1)) {
                    sand_pos.0 -= 1;
                    sand_pos.1 += 1;
                    continue;
                } else if !cave.contains_key(&(key.0+1, key.1)) {
                    sand_pos.0 += 1;
                    sand_pos.1 += 1;
                    continue;
                } else {
                    cave.insert(sand_pos, SAND);
                    number_of_sand += 1;
                    break;
                }
            } else {
                number_of_empty_falls += 1;
            }

            if number_of_empty_falls > 1000 {
                finished = true;
                break;
            }
            sand_pos.1 += 1;
        }
    }

    println!("Number of sand at rest: {}", number_of_sand);
}

fn part2(mut cave: HashMap<(i32, i32), Material>, largest_y: i32) {
    let mut finished = false;
    let mut number_of_sand = 0;

    while !finished {
        let mut sand_pos = (500, 0);
        loop {
            let key = &(sand_pos.0, sand_pos.1+1);

            if sand_pos.1+1 == largest_y+2 {
                cave.insert(sand_pos, SAND);
                number_of_sand += 1;
                break;
            }

            if cave.contains_key(key) {
                // Try left then right
                if !cave.contains_key(&(key.0-1, key.1)) {
                    sand_pos.0 -= 1;
                    sand_pos.1 += 1;
                    continue;
                } else if !cave.contains_key(&(key.0+1, key.1)) {
                    sand_pos.0 += 1;
                    sand_pos.1 += 1;
                    continue;
                } else {
                    //If sand comes at rest at source stop
                    if sand_pos == (500, 0) {
                        cave.insert(sand_pos, SAND);
                        number_of_sand += 1;
                        finished = true;
                        break;
                    }
                    cave.insert(sand_pos, SAND);
                    number_of_sand += 1;
                    break;
                }
            }

            sand_pos.1 += 1;
        }
    }

    // print_cave(&cave);
    println!("Number of sand at rest: {}", number_of_sand);
}

fn input_to_cave(input: &str) -> (HashMap<(i32, i32), Material>, i32) {
    let mut cave = HashMap::new();
    let mut largest_y = 0;

    let mut rock_positions = Vec::new();
    for line in input.lines() {
        rock_positions.push(line.split(" -> ").map(|pos| pos.split_once(",").map(|posa| (posa.0.parse::<i32>().unwrap(), posa.1.parse::<i32>().unwrap())).unwrap()).collect::<Vec<(i32, i32)>>());
    }

    for rock_pos in &rock_positions {
        for index in 0..rock_pos.len()-1 {
            if rock_pos[index].0 == rock_pos[index+1].0 {
                let mut start_index = 0;
                let mut end_index = 0;
                if rock_pos[index].1 < rock_pos[index+1].1 {
                    start_index = rock_pos[index].1;
                    end_index = rock_pos[index+1].1+1;
                } else {
                    start_index = rock_pos[index+1].1;
                    end_index = rock_pos[index].1+1;
                }
                for y_pos in start_index..end_index {
                    if y_pos > largest_y {
                        largest_y = y_pos;
                    }
                    cave.insert((rock_pos[index].0, y_pos), ROCK);
                }
            } else {
                let mut start_index = 0;
                let mut end_index = 0;
                if rock_pos[index].0 < rock_pos[index+1].0 {
                    start_index = rock_pos[index].0;
                    end_index = rock_pos[index+1].0+1;
                } else {
                    start_index = rock_pos[index+1].0;
                    end_index = rock_pos[index].0+1;
                }
                for x_pos in start_index..end_index {
                    if rock_pos[index].1 > largest_y {
                        largest_y = rock_pos[index].1;
                    }
                    cave.insert((x_pos, rock_pos[index].1), ROCK);
                }
            }
        }
    }

    (cave, largest_y)
}

fn print_cave(cave: &HashMap<(i32, i32), Material>) {
    let mut smallest_x = 500;
    let mut largest_x = 500;
    let mut smallest_y = 0;
    let mut largest_y = 0;

    for key in cave.keys() {
        if key.0 > largest_x {
            largest_x = key.0;
        }
        if key.0 < smallest_x {
            smallest_x = key.0;
        }
        if key.1 > largest_y {
            largest_y = key.1;
        }
        if key.1 < smallest_y {
            smallest_y = key.1;
        }
    }

    for row in smallest_y..largest_y+1 {
        for col in smallest_x..largest_x+1 {
            if cave.contains_key(&(col, row)) {
                match cave.get(&(col, row)).unwrap() {
                    ROCK => print!("#"),
                    SAND => print!("o")
                }
            } else if row == 0 && col == 500 {
                print!("~");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
