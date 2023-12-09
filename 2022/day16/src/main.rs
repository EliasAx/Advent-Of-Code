use std::collections::HashMap;
use crate::ValveState::CLOSED;

#[derive(PartialEq, Eq, Hash)]
struct Valve {
    flow_rate: i32,
    name: String,
    state: ValveState,
    neighbours: Vec<Valve>
}

#[derive(PartialEq, Eq, Hash)]
enum ValveState {
    OPEN,
    CLOSED
}

fn main() {
    let input = include_str!("../TestInput.txt");

    read_input(input);
}

fn read_input(input: &str) {
    let mut valves = Vec::new();
    let mut valve_neighbours = HashMap::new();

    for line in input.lines() {
        let line = line.replace("Valve ", "");
        let line = line.replace(" has flow rate=", ",");
        let line = line.replace(" ; tunnels lead to valves ", ";");

        let split = line.split(";").collect::<Vec<&str>>();

        let valve_name = split[0].split(",").collect::<Vec<&str>>()[0];
        let valve_flow_rate = split[0].split(",").collect::<Vec<&str>>()[1].parse::<i32>().expect("Should be number");
        let valve_neighbours_text = split[1].split(", ").collect::<Vec<&str>>();

        let valve = Valve{flow_rate: valve_flow_rate, name: valve_name.to_string(), state: CLOSED, neighbours: Vec::new()};

        valves.push(valve);

        valve_neighbours.insert(valve, valve_neighbours_text);
    }
}
