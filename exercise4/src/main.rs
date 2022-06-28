use std::{io, cmp::Ordering};

use rand::Rng;

fn main() {
    run();
}

fn input_message () -> io::Result<String>{
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_owned())
}

fn rand_num () -> i32 {
    let mut num = rand::thread_rng();
    num.gen_range(0..100)
}

fn run () {
    let num = rand_num();
    let mut my_bool = false;
    while my_bool == false {
        println!("Guess your number\n");
        let guess: i32 = match input_message().unwrap().trim().parse(){
            Ok(num) => num,
            Err(_) =>{
                println!("please insert a number");
                continue;
            }
        };

        match guess.cmp(&num)   {
            Ordering::Greater =>{println!("The number is less than your guess\n")},
            Ordering::Less => {println!("The number is greater than your guess\n")},
            Ordering::Equal=> {
                println!("Congrats you find de number");
                my_bool = true
            }
        }
    }
}