use std::fs::File;
use std::io::prelude::*;
fn main(){
    question_01();
    question_02();
    question_03();
    question_04();
}

fn question_04(){
    let mut pre_gama :Vec<i32> = vec![0,0,0,0,0,0,0,0,0,0,0,0];
    let mut gama :Vec<i32> = Vec::new();
    let data: String = load_string("src/assets/question_4");
    let mut counter: usize = 0;
    
    for row in data.split('\n'){
        counter = 0;
        for col in row.chars(){
            match col {
                '0' => pre_gama[counter] -= 1,
                '1' => pre_gama[counter] += 1,
                _ => print!("Error in question 4")
            }
            counter += 1;
        }
    }

    for item in pre_gama.iter(){
        if item > &0 {
            gama.push(1)
        } else {
            gama.push(0)
        }
    }
    print!("Question4: {:?}\n", gama);
}

fn question_03(){
    // If we wanted to make this faster we could get rid of the struct and
    // just use the commands during the parse, we could eleminate an entire itteration
    // but I want this to be nicer looking then that and speed is not an issue
    // for this problem atm
    let mut horizontal: i32 = 0;
    let mut depth: i32 = 0;
    let mut aim: i32 = 0;
    let ship_commands = load_string("src/assets/question_3");
    //split on \n
    let mut commands: Vec<ShipCommand> = Vec::new();
    for command in ship_commands.split('\n'){
        let mut temp: Vec<&str> = Vec::new();
        
        // split the commands apart 
        for part in command.split(' '){
            temp.push(part);
        }
        
        let temp_command = ShipCommand{
            command: String::from(temp[0]),
            value: temp[1].parse().unwrap()
        };
        
        commands.push(temp_command);
    }
    for command in commands.iter(){
        match &command.command[..] {
            "forward" => {
                horizontal += command.value;
                depth += aim * command.value;
            },
            "up" => aim -= command.value,
            "down" => aim += command.value,
            _ => print!("Error no matching pattern")
        }
    }
    
    print!("Question3: {:?}\n",horizontal*depth)

}

fn question_02() {
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
fn load_string(file_path: &str) -> String{
    let mut file = File::open(file_path).unwrap();
    let mut contents = String::new();
    
    file.read_to_string(&mut contents).unwrap();

    contents
}
struct ShipCommand{
    command: String,
    value: i32
}