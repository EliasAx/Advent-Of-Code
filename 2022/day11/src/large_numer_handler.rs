use std::collections::VecDeque;
use std::process::exit;

#[derive(Debug, Clone)]
pub struct LargeNumber {
    pub vec: VecDeque<u64>
}

impl Default for LargeNumber {
    fn default() -> Self {
        LargeNumber{vec: VecDeque::new()}
    }
}

impl LargeNumber {
    pub fn add(&self, val: &LargeNumber) -> Self {
        if val.vec.len() == 0 {
            return self.clone()
        } else if self.vec.len() == 0 {
            return val.clone()
        }
        let mut return_number = self.clone();

        let mut remainder = 0;

        let mut largest_number = val.vec.len();
        if self.vec.len() > largest_number {
            largest_number = self.vec.len();
        }

        for index in 0..largest_number {
            let digit1 = match (0..val.vec.len()).rev().collect::<Vec<usize>>().get(index) {
                None => 0,
                Some(indexx) => val.vec[*indexx]
            };

            let (digit2, index2, front) = match (0..self.vec.len()).rev().collect::<Vec<usize>>().get(index) {
                None => (0, 0, true),
                Some(val) => (self.vec[*val], *val, false)
            };

            // println!("Digit1: {}, Digit2: {}, Remainder: {}", digit1, digit2, remainder);
            let add = digit1+digit2+remainder;
            // println!("Add: {}", add);
            if add >= 10 {
                remainder = add / 10;
                if front {
                    return_number.vec.push_front(add % 10);
                } else {
                    return_number.vec[index2] = add % 10;
                }
            } else {
                remainder = 0;
                if front {
                    return_number.vec.push_front(add);
                } else {
                    return_number.vec[index2] = add;
                }
            }

            //If last index add remainder
            if index == largest_number-1 && remainder != 0 {
                return_number.vec.push_front(remainder);
            }
        }

        // println!("Add return: {:?}", return_number);
        return_number
    }

    pub fn subtract(&self, val: &LargeNumber) -> Self {

        if self.vec.len() < val.vec.len() {
            println!("Don't support negative subtracting");
            exit(1234)
        }
        let mut return_number = self.clone();

        let mut remainder = 0;
        // println!("Subtracting {:?} - {:?}", self.vec, val.vec);
        for index in 0..self.vec.len() {
            let digit2 = match (0..val.vec.len()).rev().collect::<Vec<usize>>().get(index) {
                None => 0,
                Some(indexx) => val.vec[*indexx]
            };
            // let digit1 = val.vec.get(index).unwrap();
            let index2 = *(0..self.vec.len()).rev().collect::<Vec<usize>>().get(index).unwrap();
            let digit1 = self.vec[index2];

            // println!("Digit1: {}, Digit2: {}, Remainder: {}", digit1, digit2, remainder);
            if digit1 >= digit2+remainder {
                let subtract = digit1 - digit2 - remainder;
                return_number.vec[index2] = u64::from(subtract);
                remainder = 0;
            } else {
                let subtract = 10 - (digit2 - digit1 + remainder);
                return_number.vec[index2] = u64::from(subtract);
                remainder = 1;
            }
        }

        return_number
    }

    pub fn multiply(&self, val: &LargeNumber) -> Self {
        let mut return_number = LargeNumber{vec: VecDeque::new()};

        let mut digits_to_add: Vec<LargeNumber> = Vec::new();

        let mut index1 = 0;
        for pow1 in (0..val.vec.len()).rev() {
            let mut index2 = 0;
            for pow2 in (0..self.vec.len()).rev() {
                let mut multiplied = val.vec[index1]*self.vec[index2];
                let total_pow = pow1 + pow2;


                digits_to_add.push(convert_number_with_power_to_vector(multiplied, total_pow));
                index2 += 1;
            }
            index1 += 1;
        }

        // println!("Digisadd: {:?}", digits_to_add);
        for num in digits_to_add {
            return_number = return_number.add(&num);
        }

        return_number
    }

    pub fn is_divisible_by(&self, divisible_by: u8) -> bool {
        match divisible_by {
            2 => self.is_divisible_by_2(),
            3 => self.is_divisible_by_3(),
            4 => self.is_divisible_by_4(),
            5 => self.is_divisible_by_5(),
            6 => self.is_divisible_by_2() && self.is_divisible_by_3(),
            7 => self.is_divisible_by_7(),
            13 => self.is_divisible_by_13(),
            17 => self.is_divisible_by_17(),
            19 => self.is_divisible_by_19(),
            23 => self.is_divisible_by_23(),
            _ => false
        }
    }

    fn is_divisible_by_2(&self) -> bool {
        self.vec.back().unwrap() % 2 == 0
    }

    fn is_divisible_by_3(&self) -> bool {
        let mut sum = 0;
        for digit in &self.vec {
            sum += digit;
        }
        sum % 3 == 0
    }

    fn is_divisible_by_4(&self) -> bool {
        let mut sum = 0;
        for index in self.vec.len()-2..self.vec.len() {
            sum += self.vec[index];
        }
        sum % 4 == 0
    }

    fn is_divisible_by_5(&self) -> bool {
        self.vec.back().unwrap() == &0 || self.vec.back().unwrap() == &5
    }

    fn is_divisible_by_7(&self) -> bool {

        let mut sections_of_three: Vec<LargeNumber> = Vec::new();
        for index in 0..self.vec.len() {
            let new_index = index/3;
            sections_of_three[new_index].vec.push_back(self.vec[index]);
        }

        let mut sum = LargeNumber{vec: sections_of_three[0].vec.clone()};
        for index in 1..sections_of_three.len() {
            if index % 2 == 1 {
                sum = sum.subtract(&sections_of_three[index]);
            } else {
                sum = sum.add(&sections_of_three[index]);
            }
        }

        sum.to_number() % 7 == 0
    }

    fn is_divisible_by_13(&self) -> bool {

        // println!("Self {:?}", self);
        let mut sections_of_three: Vec<LargeNumber> = vec![LargeNumber{vec: VecDeque::new()}; (self.vec.len()/3+1)];
        let mut number_of_adds_per_section = 0;
        let mut section_index = 0;
        for index in (0..self.vec.len()).rev() {
            if number_of_adds_per_section == 3 {
                section_index += 1;
                number_of_adds_per_section = 0;
            }
            sections_of_three[section_index].vec.push_front(self.vec[index]);
            number_of_adds_per_section += 1;
        }

        // println!("Sections of three {:?}", sections_of_three);
        let mut sum = LargeNumber{vec: sections_of_three[0].vec.clone()};
        for index in 1..sections_of_three.len() {
            if index % 2 == 1 {
                sum = sum.subtract(&sections_of_three[index]);
            } else {
                sum = sum.add(&sections_of_three[index]);
            }
        }

        sum.to_number() % 13 == 0
    }

    fn is_divisible_by_17(&self) -> bool {
        let mut current_num = self.clone();
        // When it's 5 digits or less we can handle it normally
        while current_num.vec.len() > 5 {
            let last_num = current_num.vec.pop_back().unwrap();
            current_num = current_num.subtract(&convert_number_to_vector(last_num*5));
        }
        current_num.to_number() % 17 == 0
    }

    fn is_divisible_by_19(&self) -> bool {
        let mut current_num = self.clone();
        // When it's 5 digits or less we can handle it normally
        while current_num.vec.len() > 5 {
            let last_num = current_num.vec.pop_back().unwrap();
            current_num = current_num.add(&convert_number_to_vector(last_num*2));
        }

        current_num.to_number() % 19 == 0
    }

    fn is_divisible_by_23(&self) -> bool {
        let mut current_num = self.clone();
        // When it's 5 digits or less we can handle it normally
        while current_num.vec.len() > 5 {
            let last_num = current_num.vec.pop_back().unwrap();
            current_num = current_num.add(&convert_number_to_vector(last_num*7));
        }

        current_num.to_number() % 23 == 0
    }

    fn to_number(&self) -> u64 {
        let mut sum:u64 = 0;
        let base:u64 = 10;
        let mut power = 0;
        for index in (0..self.vec.len()).rev() {
            sum += base.pow(u32::try_from(power).unwrap())*self.vec[index];
            power += 1;
        }
        sum
    }
}

pub fn convert_number_to_vector(mut number: u64) -> LargeNumber {
    let base = 10;
    let mut power= 0;

    let mut tmp_number = number;

    while tmp_number >= base {
        tmp_number = tmp_number/base;
        power += 1;
    }

    let mut number_vector: VecDeque<u64> = VecDeque::new();
    for powe in (0..power+1).rev() {
        let vec_number = number / (base.pow(powe));
        number = number % (base.pow(powe));
        number_vector.push_back(vec_number);
    }
    LargeNumber{vec: number_vector}
}

pub fn convert_number_with_power_to_vector(mut number: u64, mut pow: usize) -> LargeNumber {
    let base = 10;
    let mut power= 0;

    let mut tmp_number = number;

    while tmp_number >= base {
        tmp_number = tmp_number/base;
        power += 1;
    }

    let mut number_vector: VecDeque<u64> = VecDeque::new();
    for powe in (0..power+1).rev() {
        let vec_number = number / (base.pow(powe));
        number = number % (base.pow(powe));
        number_vector.push_back(vec_number);
    }

    for _ in 0..pow {
        number_vector.push_back(0)
    }
    LargeNumber{vec: number_vector}
}