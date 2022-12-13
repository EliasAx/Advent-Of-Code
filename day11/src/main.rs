use std::collections::VecDeque;
use crate::large_numer_handler::{convert_number_to_vector, LargeNumber};

mod large_numer_handler;

#[derive(Default, Debug, Clone)]
struct Monkey {
    starting_items: VecDeque<u64>,
    operation: Operation,
    test: Test,
    number_of_inspects: u64,
}

#[derive(Default, Debug, Clone)]
struct MonkeyPart2 {
    starting_items: VecDeque<LargeNumber>,
    operation: Operation,
    test: Test,
    number_of_inspects: u64,
}

#[derive(Default, Debug, Clone)]
struct Test {
    divisible_by: u8,
    test_success_monkey: usize,
    test_fail_monkey: usize,
}

#[derive(Default, Debug, Clone)]
struct Operation {
    val1: String,
    val2: String,
    operand: Operand
}

#[derive(Clone, Debug)]
enum Operand {
    Add,
    Subtract,
    Multiply,
    Divide
}

impl Operation {
    fn create_from_input(input: String) -> Self {
        let split = input.split("Operation: new = ").collect::<Vec<&str>>()[1].split(" ").collect::<Vec<&str>>();

        let val1 = String::from(split[0]);
        let val2 = String::from(split[2]);

        let operand = match split[1] {
            "+" => Operand::Add,
            "-" => Operand::Subtract,
            "*" => Operand::Multiply,
            "/" => Operand::Divide,
            _=> {
                println!("Operand does not exist!");
                Operand::Add
            }
        };

        Self {
            val1,
            val2,
            operand
        }
    }
    fn perform(&self, val1: u64, val2: u64) -> u64 {
        return match self.operand {
            Operand::Add => val1 + val2,
            Operand::Subtract => val1 - val2,
            Operand::Multiply => val1 * val2,
            Operand::Divide => val1 / val2,
        }
    }
    fn perform_part2(&self, val1: &LargeNumber, val2: &LargeNumber) -> LargeNumber {
        return match self.operand {
            Operand::Add => val1.add(val2),
            Operand::Subtract => val1.subtract(val2),
            Operand::Multiply => val1.multiply(val2),
            Operand::Divide => Default::default(), //Division never happens
        }
    }
}

impl Test {
    fn perform(&self, item_worry_level: &u64) -> usize {
        if item_worry_level % u64::from(self.divisible_by) == 0 {
            return self.test_success_monkey
        }

        self.test_fail_monkey
    }

    fn perform_part2(&self, item_worry_level: &LargeNumber) -> usize {
        if item_worry_level.is_divisible_by(self.divisible_by) {
            return self.test_success_monkey
        }
        self.test_fail_monkey
    }
}

impl Default for Operand {
    fn default() -> Self {
        Operand::Add
    }
}

impl Monkey {
    fn inspect(&mut self) -> bool {
        let current_worry_level = match self.starting_items.front() {
            None => return false,
            Some(item) => item
        };

        let val1;
        let val2;

        //If it's none the input was "old"
        if self.operation.val1 == "old" {
            val1 = *current_worry_level;
        } else {
            val1 = self.operation.val1.parse::<u64>().expect("NOOOOOO");
        }
        if self.operation.val2 == "old" {
            val2 = *current_worry_level;
        } else {
            val2 = self.operation.val2.parse::<u64>().expect("NOOOOOO");
        }

        let new_worry_level = self.operation.perform(val1, val2);

        self.starting_items[0] = new_worry_level;
        self.number_of_inspects += 1;

        return true
    }
    fn relief(&mut self) {
        self.starting_items[0] = self.starting_items[0]/3;
    }
    fn throw(&mut self, monkey_to: &mut Monkey) {
        monkey_to.starting_items.push_back(self.starting_items.pop_front().unwrap());
    }
}

impl MonkeyPart2 {
    fn inspect(&mut self) -> bool {
        let current_worry_level = match self.starting_items.front() {
            None => return false,
            Some(item) => item
        };

        let val1;
        let val2;

        //If it's none the input was "old"
        if self.operation.val1 == "old" {
            val1 = current_worry_level.clone();
        } else {
            let vec= self.operation.val1.split("").filter(|&item| !item.is_empty()).map(|item| item.parse::<u64>().expect("Numbers plz")).collect();
            val1 = LargeNumber{vec};
        }
        if self.operation.val2 == "old" {
            val2 = current_worry_level.clone();
        } else {
            let vec= self.operation.val2.split("").filter(|&item| !item.is_empty()).map(|item| item.parse::<u64>().expect("Numbers plz")).collect();
            val2 = LargeNumber{vec};
        }

        let new_worry_level = self.operation.perform_part2(&val1, &val2);

        self.starting_items[0] = new_worry_level;
        self.number_of_inspects += 1;

        return true
    }

    // fn relief(&mut self) {
    //     self.starting_items[0] = self.starting_items[0]/3;
    // }

    fn throw(&mut self, monkey_to: &mut MonkeyPart2) {
        monkey_to.starting_items.push_back(self.starting_items.pop_front().unwrap());
    }
}

fn main() {
    let input = include_str!("../TestInput.txt");

    let monkeys_input = input.split("Monkey ").skip(1).collect::<Vec<&str>>();
    let mut monkeys = <Vec<Monkey>>::new();

    for monkey_input in monkeys_input {
        let mut monkey = Monkey::default();
        for line in monkey_input.lines() {
            match line {
                tmp if tmp.contains("Starting items: ") => {
                    let items = tmp.split("Starting items: ").collect::<Vec<&str>>();
                    monkey.starting_items = items[1].split(", ").map(|item| item.parse::<u64>().expect("Numbers plz")).collect();
                },
                tmp if tmp.contains("Operation: ") => monkey.operation = Operation::create_from_input(String::from(tmp)),
                tmp if tmp.contains("Test: ") => {
                    monkey.test = Test::default();
                    monkey.test.divisible_by = tmp.split("Test: divisible by ").collect::<Vec<&str>>()[1].parse::<u8>().expect("Need moah numberz");
                }
                tmp if tmp.contains("If true: ") =>
                    monkey.test.test_success_monkey = tmp.split("If true: throw to monkey ").collect::<Vec<&str>>()[1].parse::<usize>().expect("Need moah numberz"),
                tmp if tmp.contains("If false: ") =>
                    monkey.test.test_fail_monkey = tmp.split("If false: throw to monkey ").collect::<Vec<&str>>()[1].parse::<usize>().expect("Need moah numberz"),
                _=> ()
            }
        }

        monkeys.push(monkey)
    }

    // part1(&mut monkeys);
    part2(&mut monkeys);
    // testing();
}

fn part1(monkeys: &mut Vec<Monkey>) {
    let rounds = 20;
    for _ in 0..rounds {
        for index in 0..monkeys.len() {
            let mut monkey = monkeys[index].clone();
            // For as long as the monkey has items, inspect them
            while monkey.inspect() {
                monkey.relief();
                let monkey_id = monkey.test.perform(monkey.starting_items.front().unwrap());
                let monkey_to = &mut monkeys[monkey_id];
                monkey.throw(monkey_to);
            }

            monkeys[index] = monkey;
        }
    }

    let mut top_monkey1 = 0;
    let mut top_monkey2 = 0;
    let mut index = 0;
    for monkey in monkeys {
        if monkey.number_of_inspects > top_monkey1 {
            top_monkey2 = top_monkey1;
            top_monkey1 = monkey.number_of_inspects;
        } else if monkey.number_of_inspects > top_monkey2 {
            top_monkey2 = monkey.number_of_inspects;
        }
        println!("Monkey {} inspected items {} times", index, monkey.number_of_inspects);
        println!("{:?}", monkey.starting_items);
        index += 1;
    }

    let monkey_business = top_monkey1*top_monkey2;
    println!("Level of monkey business: {}", monkey_business)
}

fn part2(monkeys_in: &mut Vec<Monkey>) {
    let mut monkeys = convert_to_monkey_part2(monkeys_in);
    println!("{:?}", monkeys);

    let rounds = 200;
    for _ in 0..rounds {
        for index in 0..monkeys.len() {
            let mut monkey = monkeys[index].clone();
            // For as long as the monkey has items, inspect them
            while monkey.inspect() {
                let monkey_id = monkey.test.perform_part2(monkey.starting_items.front().unwrap());
                let monkey_to = &mut monkeys[monkey_id];
                monkey.throw(monkey_to);
            }

            monkeys[index] = monkey;
        }
    }

    let mut top_monkey1 = 0;
    let mut top_monkey2 = 0;
    let mut index = 0;
    for monkey in monkeys {
        if monkey.number_of_inspects > top_monkey1 {
            top_monkey2 = top_monkey1;
            top_monkey1 = monkey.number_of_inspects;
        } else if monkey.number_of_inspects > top_monkey2 {
            top_monkey2 = monkey.number_of_inspects;
        }
        println!("Monkey {} inspected items {} times", index, monkey.number_of_inspects);
        // println!("{:?}", monkey.starting_items);
        index += 1;
    }

    let monkey_business = top_monkey1*top_monkey2;
    println!("Level of monkey business: {}", monkey_business)
}

fn convert_to_monkey_part2(monkeys: &mut Vec<Monkey>) ->  Vec<MonkeyPart2> {
    let mut monkeys_part2: Vec<MonkeyPart2> = Vec::new();
    for monkey in monkeys {
        let mut large_number_starting_items: VecDeque<LargeNumber> = VecDeque::new();
        for index in 0..monkey.starting_items.len() {
            large_number_starting_items.push_back(convert_number_to_vector(monkey.starting_items[index]));
        }
        monkeys_part2.push(
            MonkeyPart2 {
                starting_items: large_number_starting_items,
                operation: monkey.operation.clone(),
                test: monkey.test.clone(),
                number_of_inspects: 0
            }
        );
    }
    monkeys_part2
}


fn testing() {
    // let num1 = LargeNumber{vec:VecDeque::from([6, 5, 3, 5, 2, 5, 3, 5, 3, 5, 2, 5, 3, 5, 3, 5, 2, 5, 3, 5, 3, 5, 2, 5, 3, 5, 3, 5, 2, 5, 3, 5, 3, 5, 2, 5, 3, 5, 3, 5, 2, 5, 3, 5, 3, 5, 2, 5, 3, 5, 3, 5, 2, 5, 3, 5, 3, 5, 2, 5, 3])};
    // let num2 = LargeNumber{vec:VecDeque::from([6, 5, 3, 5, 2, 5, 3, 5, 3, 5, 2, 5, 3, 5, 3, 5, 2, 5, 3, 5, 3, 5, 2, 5, 3, 5, 3, 5, 2, 5, 3, 5, 3, 5, 2, 5, 3, 5, 3, 5, 2, 5, 3, 5, 3, 5, 2, 5, 3, 5, 3, 5, 2, 5, 3, 5, 3, 5, 2, 5, 3])};
    let num1 = LargeNumber{vec:VecDeque::from([7, 0])};
    let num2 = LargeNumber{vec:VecDeque::from([6, 9])};
    assert_eq!(convert_number_to_vector(432).vec, vec![4,3,2]);
    assert_eq!(num1.add(&num2).vec, vec![1, 3, 9]);
    assert_eq!(num1.subtract(&num2).vec, vec![0, 1]);
    assert_eq!(num1.multiply(&num2).vec, vec![4, 8, 3, 0]);


    let num1 = LargeNumber{vec:VecDeque::from([2, 4, 9])};
    let num2 = LargeNumber{vec:VecDeque::from([6, 5])};
    assert_eq!(num1.add(&num2).vec, vec![3, 1, 4]);
    assert_eq!(num1.subtract(&num2).vec, vec![1, 8, 4]);
    assert_eq!(num1.multiply(&num2).vec, vec![1, 6, 1, 8, 5]);


    let num1 = LargeNumber{vec:VecDeque::from([2, 4, 9])};
    let num2 = LargeNumber{vec:VecDeque::from([1, 5, 8, 7])};
    assert_eq!(num1.add(&num2).vec, vec![1, 8, 3, 6]);
    assert_eq!(num1.multiply(&num2).vec, vec![3, 9, 5, 1, 6, 3]);

    let num1 = LargeNumber{vec:VecDeque::from([2, 2, 1])};
    assert_eq!(num1.is_divisible_by(13), true);
    assert_eq!(num1.is_divisible_by(17), true);
    assert_eq!(num1.is_divisible_by(19), false);
    assert_eq!(num1.is_divisible_by(23), false);


    let num1 = LargeNumber{vec:VecDeque::from([2, 6, 6, 6, 9, 4, 9, 8])};
    assert_eq!(num1.is_divisible_by(13), false);
    assert_eq!(num1.is_divisible_by(17), true);
    assert_eq!(num1.is_divisible_by(19), false);
    assert_eq!(num1.is_divisible_by(23), false);


    let num1 = LargeNumber{vec:VecDeque::from([2, 0, 3, 4, 3, 3, 1])};
    assert_eq!(num1.is_divisible_by(13), true);
    assert_eq!(num1.is_divisible_by(17), false);
    assert_eq!(num1.is_divisible_by(19), false);
    assert_eq!(num1.is_divisible_by(23), false);
}