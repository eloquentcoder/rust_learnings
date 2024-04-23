use std::collections::HashMap;

#[derive(Debug)]
pub enum MyError {
    Error1,
}

fn _divide(dividend: i32, divisor: i32) -> Result<i32, MyError> {
    if dividend % divisor != 0 {
        Err(MyError::Error1)
    } else {
        Ok(dividend / divisor)
    }
}

fn _hash_map_implementations() {
    let mut map: HashMap<i8, String> = HashMap::new();
    map.insert(0, String::from("Man"));
    map.insert(1, String::from("Woman"));

    println!("Map: {:?}", map);

    let oth_index: &str = match map.get(&0) {
        Some(index) => index,
        None => "Not Found",
    };
    println!("Oth index: {}", oth_index);
}

pub struct Bird {
    name: String,
    attack_power: u64,
}

impl Bird {
    pub fn new(name: String, attack_power: u64) -> Self {
        Self { name, attack_power }
    }

    pub fn print_name(&self) {
        println!("Bird name is {}", self.name);
    }
}

fn _match_statement() -> () {
    let i: i32 = 2;

    match i {
        0 => println!("Number is 9"),
        1..=3 => println!("Number is between 1 and 3"),
        4 => println!("Number is 4"),
        _ => println!("Number is not in range"),
    }
}

fn _string_information() -> () {
    let _str: &str = "Hello world!";
    let mut string: String = String::from("Hello world!");
    let slice: &str = &string[..6];

    println!("String slice: {:?}", slice);
    string.push('s');
    string.push_str(" store");

    string = string.replace("Hello", "Bye");

    println!("String: {:?}", string);
}

fn _array_data() -> () {
    let arr: [i8; 4] = [0, 1, 2, 3];
    let slice: &[i8] = &arr[1..3];

    _borrowing_slice(arr, slice);

    println!("Array in main {:?}", arr);
}

fn _borrowing_slice(arr: [i8; 4], slice: &[i8]) -> () {
    println!("Array {:?}", arr);
    println!("Slice {:?}", slice);
    println!("Slice length {:?}", slice.len());
}

pub fn interatable_funcs() {
    let _arr: std::ops::RangeInclusive<i32> = 1..=9;
    _arr.for_each(|item: i32| {
        println!("Current item: {:?}", item);
    });
}

pub fn variable_funcs() {
    const _ONE: i32 = 1;
    let _names: [i8; 3] = [1; 3];

    let first_tuple: (u8, i8) = (0, 0);
    let (_a, _b) = first_tuple;
}

pub fn is_even(num: i8) -> bool {
    num % 2 == 0
}
