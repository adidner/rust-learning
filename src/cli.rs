// cli - 

use std::env;

pub fn run(){
    // first param is always the path of the executible, the rest are all the positional args
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    let name = "Brad";
    let status = "100%";


    println!("Args: {:?}", args); 
    println!("command: {:?}", command); 

    if command == "hello" {
        println!("Hi {}, how are you?", name)
    } else if command == "status" {
        println!("Status is {}", status)
    } else {
        println!("that is not a valid command")
    }
}