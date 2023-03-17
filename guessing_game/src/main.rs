use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome gussing game");
    //define a string variable to hold read value from terminal

    loop {
        let mut number: String = String::new();
        let secret_number = rand::thread_rng().gen_range(1..100);
        println!("generated number {} ", secret_number);

        println!("please enter a  number ");
        // read the nuber from the termnal to number
        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read line");
        println!("enter number is {} ", number);
        // user enter number trim and parse to uint
        let p: i32 = match number.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        match secret_number.cmp(&p) {
            Ordering::Less => println!(" Less "),
            Ordering::Equal => {
                println!(" Equals ");
                break;
            }
            Ordering::Greater => println!(" Greater "),
        }
    }
}
