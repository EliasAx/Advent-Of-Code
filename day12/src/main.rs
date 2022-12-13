fn main() {
    let input = include_str!("../TestInput.txt");

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

    // for heights in height_map {
    //     println!("{:?}", heights);
    // }


    let mut path = Vec::new();
    path.push(start_pos);
    let steps = shortest_path(&height_map, &mut path, 0);
    println!("Steps {}", steps);
}

fn shortest_path(height_map: &Vec<Vec<char>>, path: &mut Vec<(usize, usize)>, steps_taken: i32) -> i32 {
    let curruent_position = match path.last() {
        None => &(0, 0),
        Some(pos) => pos
    };

    let possible_paths = find_possible_paths(height_map, path, curruent_position);

    if possible_paths.is_empty() {
        return i32::MAX;
    }

    let mut steps_taken_per_path = Vec::new();
    for possible_path in possible_paths {
        let mut new_path = path.clone();
        new_path.push(possible_path);
        if height_map[possible_path.0][possible_path.1] == 'E' {
            println!("Finished path {:?}", new_path);
            return steps_taken + 1
        }

        steps_taken_per_path.push(shortest_path(height_map, &mut new_path, steps_taken+1));
    }

    // println!("steps per path: {:?}", steps_taken_per_path);
    let mut lowest_step = i32::MAX;
    for steps in steps_taken_per_path {
        if steps < lowest_step {
            lowest_step = steps;
        }
    }

    lowest_step
}

fn find_possible_paths(height_map: &Vec<Vec<char>>, current_path: &Vec<(usize, usize)>, current_position: &(usize, usize)) -> Vec<(usize, usize)> {
    let mut possible_paths: Vec<(usize, usize)> = Vec::new();

    let current_height = &height_map[current_position.0][current_position.1];
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
        test_positions.push(up);
    }
    // Down
    if current_position.0 != height_map.len()-1 {
        let down = (current_position.0+1, current_position.1);
        test_positions.push(down);
    }
    // Right
    if current_position.1 != height_map[current_position.0].len()-1 {
        let right = (current_position.0, current_position.1+1);
        test_positions.push(right);
    }
    // Left
    if current_position.1 != 0 {
        let left = (current_position.0, current_position.1-1);
        test_positions.push(left);
    }

    for test_position in test_positions {
        // If not already been at test position consider it as possible next position
        // println!("test pos : {:?}", test_position);
        // println!("test pos compare : {:?}", current_path.contains(&test_position));
        if !current_path.contains(&test_position) {
            let height = match height_map[test_position.0][test_position.1] {
                'S' => 0,
                'E' => 27,
                ' ' => 1000,
                val => val.to_digit(36).expect("Char to digit issues...")-10+1
            };
            // println!("Test pos Height {}, Current Height {}", height, current_height);

            // Make sure it's possible to go to this as the next position
            if height == current_height+1 || height == current_height {
                possible_paths.push(test_position);
            }
        }
    }
    // println!("possible paths: {:?}", possible_paths);
    // println!();

   possible_paths
}

// fn compress_height_map(height_map: Vec<Vec<char>>) {
//     for row in 1..height_map.len()-1 {
//         for col in 1..height_map.len()-1 {
//             if
//         }
//     }
// }
