use std::fs::File;
use std::io;
use std::io::{BufRead};
use std::path::Path;

fn main() {
    let lines = read_lines("Input.txt").expect("Can't you read?!");
    part1(lines);
    let lines = read_lines("Input.txt").expect("Can't you read?!");
    part2(lines);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn part1(lines: io::Lines<io::BufReader<File>>) {

    let mut cycle = 0;
    let mut x_register = 1;
    let mut signal_strength = 0;

    for line in lines {
        let line = line.expect("Where is my string?");
        if line == "noop" {
            cycle += 1;
            if cycle%40 == 20 {
                println!("Cycle: {}, X: {}", cycle, x_register);
                signal_strength += x_register*cycle;
            }
        } else if line.contains("addx") {
            let split = line.split(" ").collect::<Vec<&str>>();
            let addx = str::parse::<i32>(split[1]).expect("Not a number!");

            //Takes 2 cycles
            for _i in 0..2 {
                cycle += 1;
                if cycle%40 == 20 {
                    println!("Cycle: {}, X: {}", cycle, x_register);
                    signal_strength += x_register*cycle;
                }
            }
            x_register += addx;
        }
    }

    println!("Part1: {}", signal_strength)
}

fn part2(lines: io::Lines<io::BufReader<File>>) {
    let mut cycle = 0;
    let mut x_register = 1;
    let mut current_crt_row = String::new();

    for line in lines {
        let line = line.expect("Where is my string?");
        if line == "noop" {
            cycle += 1;
            println!("Start cycle {}: begin executing noop", cycle);
            let position = (cycle-1)%40;
            if position == 0 && cycle != 1 {
                current_crt_row.push('\n');
            }
            if (x_register-1..x_register+2).contains(&position) {
                println!("During cycle {}: CRT draws pixel in position {}", cycle, position);
                current_crt_row.push('#');
                println!("Current CRT row: {}", current_crt_row);
            } else {
                println!("During cycle {}: CRT draws pixel in position {}", cycle, position);
                current_crt_row.push('.');
                println!("Current CRT row: {}", current_crt_row);
            }

        } else if line.contains("addx") {
            let split = line.split(" ").collect::<Vec<&str>>();
            let addx = str::parse::<i32>(split[1]).expect("Not a number!");

            //Takes 2 cycles
            for i in 0..2 {
                cycle += 1;

                if i == 0 {
                    println!("Start cycle {}: begin executing addx {}", cycle, addx);
                }

                let position = (cycle-1)%40;
                if position == 0 && cycle != 1  {
                    current_crt_row.push('\n');
                }
                if (x_register-1..x_register+2).contains(&position) {
                    println!("During cycle {}: CRT draws pixel in position {}", cycle, position);
                    current_crt_row.push('#');
                    println!("Current CRT row: {}", current_crt_row);
                } else {
                    println!("During cycle {}: CRT draws pixel in position {}", cycle, position);
                    current_crt_row.push('.');
                    println!("Current CRT row: {}", current_crt_row.trim());
                }
                if i == 0 {
                    println!()
                }

            }

            x_register += addx;
            println!("End of cycle {}: finish executing addx {} (Register X is now {})", cycle, addx, x_register);
        }
        println!();
    }

    println!("\n\nFinal:\n{}", current_crt_row);
}
