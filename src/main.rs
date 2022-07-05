use rand::Rng;
use std::fs::File;
use std::io::Read;

fn fizz_buzz(n: i8) -> () {
    // get numbers in range 1 to 100
    let numbers = 1..n;

    for number in numbers {
        if number % 3 == 0 && number % 5 == 0 {
            println!("FizzBuzz");
        } else if number % 3 == 0 {
            println!("Fizz");
        } else if number % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", number);
        }
    }
}



fn read_file() {
    let file_name = "src/main.rs";
    let mut file = File::open(file_name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    // convert to list
    let lines = contents.split("\n");

    let mut count: i8 = 0;

    for line in lines {
        if line.contains("Rose") {
            println!("rose!");
            count += 1;
        }
    }

    println!("The word 'rose' appears {} times in {}.", count, file_name);
}

fn main() {
    println!("Hello world!");
    println!("I'm inside a computer!");

    // This is a comment!

    /* This is a 
    multi-line
    comment
    */

    let cost: i32 = 99;

    let cost_in_half: f32 = (cost / 2) as f32;

    println!("Half of 100 is {}.", cost_in_half);

    if cost > 100 || cost == 100 {
        println!("Cool beans!")
    }

    let tuple_item = [
        "Rose Coffee Roasters",
        "Fortitude Coffee Roasters"
    ];

    // get random
    let random_number = rand::thread_rng().gen_range(0, tuple_item.len());

    println!("Here's a random coffee roaster: {}.", tuple_item[random_number]);

    println!("The first item in tuple_item is {}.", tuple_item[0]);

    read_file();

    fizz_buzz(5);
}