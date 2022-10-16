use std::fs::File;
use std::io::prelude::*;
fn main(){
    question_01();
    question_02();
}

fn question_02() {
    // 199  A      
    // 200  A B    
    // 208  A B C  
    // 210    B C D
    // 200  E   C D
    // 207  E F   D
    // 240  E F G  
    // 269    F G H
    // 260      G H
    // 263        H
    let numbers: Vec<i32> = load_csv("src/assets/question_2.csv");
    let mut previous: i32 = i32::MAX;
    let mut counter: i32 = 0;
    for num1 in numbers.windows(3) {
        let temp:i32 = num1.iter().sum();
        if temp > previous {
            counter = counter + 1;
        }
        previous = temp;
    }
    print!("Question2: {:?}\n", counter)
}

fn question_01(){ 
    let numbers: Vec<i32> = load_csv("src/assets/question_1.csv");
    
    let mut previous: i32 = i32::MAX;
    let mut counter: i32 = 0;
    for value in numbers.iter(){
        if *value > previous {
            counter = counter + 1;
        }
        previous = *value;
    }
    print!("Question1: {:?}\n", counter)
}

fn load_csv(file_path: &str) -> Vec<i32>{
    let mut numbers: Vec<i32> = Vec::new();
    let mut file = File::open(file_path).unwrap();
    let mut contents = String::new();
    
    file.read_to_string(&mut contents).unwrap();
    
    for value in contents.split(','){
        let temp:i32 = value.parse().unwrap();
        numbers.push(temp);
    }
    numbers
}