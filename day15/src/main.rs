use std::collections::{HashMap, HashSet};
use std::ops::Range;
use crate::PositionData::{Beacon, NotPossibleBeacon, Sensor};

#[derive(Debug, PartialEq)]
enum PositionData {
    Sensor,
    Beacon,
    NotPossibleBeacon,
    PossibleBeacon
}

struct BeaconRowData {
    row: i32,
    ranges: Vec<Range<i32>>
}

fn main() {
    let input = include_str!("../Input.txt");
    let row = 2000000;

    part1(input, row);
}

fn part1(input: &str, row: i32) {
    let mut beacon_data:Vec<BeaconRowData> = Vec::new();
    let mut row_index_mapping:HashMap<i32, i32> = HashMap::new();
    let mut beacon_sensor_positions:HashSet<(i32, i32)> = HashSet::new();

    for line in input.lines() {
        let (sensor, beacon) = get_sensor_and_beacon(line);

        // println!("Sensor: {:?}, Beacon: {:?}", sensor, beacon);
        let manhattan_distance = (sensor.0-beacon.0).abs() + (sensor.1-beacon.1).abs();
        // println!("Manhattan distance: {}", manhattan_distance);

        let mut y_index = 0;

        for y in sensor.1-manhattan_distance..sensor.1+manhattan_distance+1 {
            if row_index_mapping.contains_key(&y) {
                let index = row_index_mapping[&y];
                beacon_data[usize::try_from(index).unwrap()].ranges.push(sensor.0-y_index..sensor.0+y_index+1);
            } else {
                beacon_data.push(BeaconRowData{row: y, ranges: vec![sensor.0-y_index..sensor.0+y_index+1]});
                row_index_mapping.insert(y, i32::try_from(beacon_data.len()).unwrap()-1);
            }

            if y >= sensor.1 {
                y_index -= 1;
            } else {
                y_index += 1;
            }
        }

        beacon_sensor_positions.insert(sensor);
        beacon_sensor_positions.insert(beacon);
    }

    let mut beacon_cannot_be_present = 0;

    // println!("{:?}", beacon_data[usize::try_from(row_index_mapping[&row]).unwrap()].ranges.clone());

    let aggregated_ranges = aggregate_ranges(beacon_data[usize::try_from(row_index_mapping[&row]).unwrap()].ranges.clone(), 0);

    // println!("{:?}", aggregated_ranges);

    for range in aggregated_ranges {
        beacon_cannot_be_present += range.len();
        for invalid_positions in &beacon_sensor_positions {
            if invalid_positions.1 == row && range.contains(&invalid_positions.0) {
                beacon_cannot_be_present -= 1;
            }
        }
    }

    println!("Part1: {}", beacon_cannot_be_present);
}

fn aggregate_ranges(mut ranges: Vec<Range<i32>>, current_index: usize) -> Vec<Range<i32>> {
    if current_index+1 >= ranges.len() {
        return ranges;
    }

    let range1 = &ranges[current_index].clone();
    let range2 = &ranges[current_index+1].clone();

    if range1.contains(&range2.start) && range1.contains(&range2.end) {
        ranges.remove(current_index+1);
        return aggregate_ranges(ranges, current_index)
    } else if range1.contains(&range2.start) && !range1.contains(&range2.end) {
        ranges.remove(current_index+1);
        ranges[current_index] = range1.start..range2.end;
        let mut index = current_index;
        if current_index != 0 {
            index -= 1;
        }
        return aggregate_ranges(ranges, index)
    } else if !range1.contains(&range2.start) && range1.contains(&range2.end) {
        ranges.remove(current_index+1);
        ranges[current_index] = range2.start..range1.end;
        let mut index = current_index;
        if current_index != 0 {
            index -= 1;
        }
        return aggregate_ranges(ranges, index)
    } else if range2.contains(&range1.start) && range2.contains(&range1.end) {
        ranges.remove(current_index);
        return aggregate_ranges(ranges, current_index)
    } else if range2.contains(&range1.start) && !range2.contains(&range1.end) {
        ranges.remove(current_index+1);
        ranges[current_index] = range2.start..range1.end;
        let mut index = current_index;
        if current_index != 0 {
            index -= 1;
        }
        return aggregate_ranges(ranges, index)
    } else if !range2.contains(&range1.start) && range2.contains(&range1.end) {
        ranges.remove(current_index+1);
        ranges[current_index] = range1.start..range2.end;
        let mut index = current_index;
        if current_index != 0 {
            index -= 1;
        }
        return aggregate_ranges(ranges, index)
    }

    aggregate_ranges(ranges, current_index+1)
}

fn get_sensor_and_beacon(input_line: &str) -> ((i32, i32), (i32, i32)) {
    // return ((8, 7), (2, 10));
    let line = input_line.replace("Sensor at ", "");
    let line = line.replace(" closest beacon is at ", "");
    let split_line = line.split(":").collect::<Vec<&str>>();
    let sensor_text_x = split_line[0].split(", ").collect::<Vec<&str>>()[0].trim();
    let sensor_text_y = split_line[0].split(", ").collect::<Vec<&str>>()[1].trim();
    let beacon_text_x = split_line[1].split(", ").collect::<Vec<&str>>()[0].trim();
    let beacon_text_y = split_line[1].split(", ").collect::<Vec<&str>>()[1].trim();

    let mut sensor: (i32, i32) = (0,0);
    sensor.0 = sensor_text_x.replace("x=", "").parse::<i32>().unwrap();
    sensor.1 = sensor_text_y.replace("y=", "").parse::<i32>().unwrap();

    let mut beacon: (i32, i32) = (0,0);
    beacon.0 = beacon_text_x.replace("x=", "").parse::<i32>().unwrap();
    beacon.1 = beacon_text_y.replace("y=", "").parse::<i32>().unwrap();

    (sensor, beacon)
}
