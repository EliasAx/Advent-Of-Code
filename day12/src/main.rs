extern crate pathfinding;

use pathfinding::num_traits::{AsPrimitive, ToPrimitive};
use pathfinding::prelude::astar;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32);

impl Pos {
    fn distance(&self, other: &Pos) -> usize {
        ((self.0 - other.0).abs() + (self.1 - other.1).abs()) as usize
    }

    fn neighbours(&self, height_map: &Vec<Vec<char>>) -> Vec<(Pos, usize)> {
        let &Pos(x, y) = self;
        let x = x.to_usize().unwrap();
        let y = y.to_usize().unwrap();
        let mut possible_paths: Vec<(Pos, usize)> = Vec::new();

        let current_height = height_map[x][y];
        let current_height = match current_height {
            'S' => 1,
            'E' => 27,
            val => val.to_digit(36).expect("Char to digit issues...")-10+1
        };

        let mut test_positions = Vec::new();
        // Up
        if x != 0 {
            let up = (x-1, y);
            test_positions.push((up, height_map[up.0][up.1]));
        }
        // Down
        if x != height_map.len()-1 {
            let down = (x+1, y);
            test_positions.push((down, height_map[down.0][down.1]));
        }
        // Right
        if y != height_map[x].len()-1 {
            let right = (x, y+1);
            test_positions.push((right, height_map[right.0][right.1]));
        }
        // Left
        if y != 0 {
            let left = (x, y-1);
            test_positions.push((left, height_map[left.0][left.1]));
        }

        for test_position in test_positions {
            // If not already been at test position consider it as possible next position
            let height = match height_map[test_position.0.0][test_position.0.1] {
                'S' => 1,
                'E' => 27,
                val => val.to_digit(36).expect("Char to digit issues...")-10+1
            };
            // println!("Test pos Height {}, Current Height {}", height, current_height);

            // Make sure it's possible to go to this as the next position
            if height <= current_height+1 {
                possible_paths.push((Pos(test_position.0.0.to_i32().unwrap(), test_position.0.1.to_i32().unwrap()), 1));
            }
        }

        // println!("possible paths: {:?}", possible_paths);
        // println!();

        possible_paths
    }
}

fn main() {
    let input = include_str!("../Input.txt");

    let mut height_map: Vec<Vec<char>> = Vec::new();
    let mut start_pos: (usize, usize) = (0, 0);
    let mut end_pos: (usize, usize) = (0, 0);

    let mut row_index = 0;
    for line in input.lines() {
        let heights = line.split("").collect::<Vec<&str>>();
        let mut col_heights = Vec::new();
        let mut col_index = 0;

        for height in heights {
            if !height.is_empty() {
                if height == "S" {
                    start_pos = (row_index, col_index);
                } else if height == "E" {
                    end_pos = (row_index, col_index);
                }
                col_heights.push(height.chars().next().expect("My char(izard)?!"));
                col_index += 1;
            }
        }
        height_map.push(col_heights);
        row_index += 1;
    }

    // for heights in &height_map {
    //     println!("{:?}", heights);
    // }

    // let weighted_height_map = get_weighted_height_map(&height_map, (end_pos.0.to_i32().unwrap(), end_pos.1.to_i32().unwrap()));

    // println!("\n\n\n NEW!\n");
    // for heights in &weighted_height_map {
    //     println!("{:?}", heights);
    // }

    let end_pos = Pos(end_pos.0.to_i32().unwrap(), end_pos.1.to_i32().unwrap());
    let pathfind = astar(&Pos(start_pos.0.to_i32().unwrap(), start_pos.1.to_i32().unwrap()), |p| p.neighbours(&height_map), |p| p.distance(&end_pos), |p| *p == end_pos).expect("Ouch");
    let mut shortest_path = pathfind.1;
    for row in 0..height_map.len() {
        for col in 0..height_map[row].len() {
            if height_map[row][col] == 'a' {
                let pathfind = match astar(&Pos(row.to_i32().unwrap(), col.to_i32().unwrap()), |p| p.neighbours(&height_map), |p| p.distance(&end_pos), |p| *p == end_pos) {
                    None => (Vec::new(), usize::MAX),
                    Some(val) => val
                };
                if pathfind.1 < shortest_path {
                    shortest_path = pathfind.1;
                }
            }
        }
    }

    println!("{:?}", shortest_path)

    // let mut path = Vec::new();
    // path.push(start_pos);
    // let steps = shortest_path(&weighted_height_map, &mut path, 0);
    // println!("Steps {}", steps);
}

static mut FINISHED_PATHS_STEPS: Vec<i32> = Vec::new();

fn shortest_path(height_map: &Vec<Vec<(char, f64)>>, path: &mut Vec<(usize, usize)>, steps_taken: i32) -> i32 {
    let curruent_position = match path.last() {
        None => &(0, 0),
        Some(pos) => pos
    };

    let possible_paths = find_possible_paths(height_map, path, curruent_position);

    if possible_paths.is_empty() {
        return i32::MAX;
    }

    for possible_path in possible_paths {
        let mut new_path = path.clone();
        new_path.push(possible_path);
        if height_map[possible_path.0][possible_path.1].0 == 'E' {
            println!("Finished path {:?}", new_path);
            return steps_taken + 1
        }
        unsafe {

            for finished_path_steps in &FINISHED_PATHS_STEPS {
                // println!("Steps taken {}", steps_taken);
                if &steps_taken > finished_path_steps {
                    println!("Not interested!!!!!!!!!!!!!");
                    return i32::MAX
                }
            }
            FINISHED_PATHS_STEPS.push(shortest_path(height_map, &mut new_path, steps_taken + 1));
        }
    }


    let mut lowest_step = i32::MAX;
    unsafe {
        // println!("steps per path: {:?}", FINISHED_PATHS_STEPS);
        for steps in &FINISHED_PATHS_STEPS {
            if steps < &lowest_step {
                lowest_step = *steps;
            }
        }
    }

    lowest_step
}

fn find_possible_paths(height_map: &Vec<Vec<(char, f64)>>, current_path: &Vec<(usize, usize)>, current_position: &(usize, usize)) -> Vec<(usize, usize)> {
    let mut possible_paths: Vec<(usize, usize)> = Vec::new();

    let current_height = &height_map[current_position.0][current_position.1].0;
    let current_height = match current_height {
        'S' => 0,
        'E' => 27,
        ' ' => 1000,
        val => val.to_digit(36).expect("Char to digit issues...")-10+1
    };
    // println!("current path: {:?}", current_path);

    let mut test_positions = Vec::new();
    // Up
    if current_position.0 != 0 {
        let up = (current_position.0-1, current_position.1);
        test_positions.push((up, height_map[up.0][up.1].1));
    }
    // Down
    if current_position.0 != height_map.len()-1 {
        let down = (current_position.0+1, current_position.1);
        test_positions.push((down, height_map[down.0][down.1].1));
    }
    // Right
    if current_position.1 != height_map[current_position.0].len()-1 {
        let right = (current_position.0, current_position.1+1);
        test_positions.push((right, height_map[right.0][right.1].1));
    }
    // Left
    if current_position.1 != 0 {
        let left = (current_position.0, current_position.1-1);
        test_positions.push((left, height_map[left.0][left.1].1));
    }

    test_positions.sort_by(|a, b|a.1.partial_cmp(&b.1).expect("Shiet"));

    for test_position in test_positions {
        // If not already been at test position consider it as possible next position
        // println!("test pos : {:?}", test_position);
        // println!("test pos compare : {:?}", current_path.contains(&test_position.0));
        if !current_path.contains(&test_position.0) {
            let height = match height_map[test_position.0.0][test_position.0.1].0 {
                'S' => 0,
                'E' => 27,
                ' ' => 1000,
                val => val.to_digit(36).expect("Char to digit issues...")-10+1
            };
            // println!("Test pos Height {}, Current Height {}", height, current_height);

            // Make sure it's possible to go to this as the next position
            if height <= current_height+1 {
                possible_paths.push((test_position.0.0, test_position.0.1));
            }
        }
    }

    // possible_paths.sort_by(|a, b|a.2.partial_cmp(&b.2).expect("Shiet"));
    // println!("possible paths: {:?}", possible_paths);
    // println!();

   possible_paths
}

fn get_weighted_height_map(height_map: &Vec<Vec<char>>, goal: (i32, i32)) -> Vec<Vec<(char, f64)>> {
    let mut weighted_height_map= Vec::new();
    for row in 0..height_map.len() {
        let mut row_map:Vec<(char, f64)> = Vec::new();
        for col in 0..height_map[row].len() {
            // println!("row {}, col {}, goal {:?}", row, col, goal);
            let row_int = row.to_i32().unwrap();
            let col_int = col.to_i32().unwrap();
            let euclidean_distance = ((row_int - goal.0).pow(2) + (col_int-goal.1).pow(2)).to_f64().unwrap().sqrt();
            let height = match height_map[row][col] {
                'S' => 0,
                'E' => 27,
                ' ' => 1000,
                val => val.to_digit(36).expect("Char to digit issues...")-10+1
            };
            let euclidean_distance = euclidean_distance+(27-height).to_f64().unwrap();
            row_map.push((height_map[row][col], euclidean_distance));
        }
        weighted_height_map.push(row_map);
    }

    weighted_height_map
}
