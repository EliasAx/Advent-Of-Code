struct BeaconMatrix {
    x: i32,
    y: i32,
    thing_here: WhatToNameThis
}

enum WhatToNameThis {
    Sensor,
    Beacon,
    NotPossibleBeacon,
    PossibleBeacon
}

fn main() {
    let input = include_str!("../TestInput.txt");

    read_input(input);
}

fn read_input(input: &str) {
    for line in input.lines() {
        let (sensor, beacon) = get_sensor_and_beacon(line);

        println!("Sensor: {:?}, Beacon: {:?}", sensor, beacon);
        let manhattan_distance = (sensor.0-beacon.0).abs() + (sensor.1-beacon.1).abs();


    }
}

fn get_sensor_and_beacon(input_line: &str) -> ((i32, i32), (i32, i32)) {
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
