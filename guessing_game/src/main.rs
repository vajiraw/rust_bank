use core::num;
use std::io;

fn main() {
    println!("Welcome gussing game");
    //define a string variable to hold read value from terminal
    let mut number : String = String::new();
    println!("please enter a  number ");

    // read the nuber from the termnal to number
    io::stdin().read_line(&mut number).expect("Failed to read line");
    println!("number is {} ",number);    

}
