use std::fs::File;
use std::io::prelude::*;
fn main(){
    question_01();
}

fn question_01(){ 
    let mut numbers: Vec<i32> = Vec::new();
    let mut file = File::open("src/assets/question_1.csv").unwrap();
    let mut contents = String::new();
    
    file.read_to_string(&mut contents).unwrap();
    
    for value in contents.split(','){
        let temp:i32 = value.parse().unwrap();
        numbers.push(temp);
    }
    let mut previous: i32 = i32::MAX;
    let mut counter: i32 = 0;
    for value in numbers.iter(){
        if *value > previous {
            counter = counter + 1;
        }
        previous = *value;
    }
    print!("{:?}", counter)
}
