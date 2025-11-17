use std::io;
use std::cmp::Ordering;

fn main() {
    loop{
        const BENCHMARK: i32 = 100;

        //Define an empty string
        let mut val = String::new();

        //Allow users to input a number
        println!("Please input a number");
        io::stdin().read_line(&mut val)
            .expect("Failed to read line");

        //Format and Check if val is a valid number
        let val: i32 = match val.trim().parse() {
            Ok(val) => val,
            Err(_) => continue,
        };
        //Compare with the benchmark value
        match val.cmp(&BENCHMARK){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too large!"),
            Ordering::Equal => {
                println!("Correct!");
                break;
            }
        }
    }
}


