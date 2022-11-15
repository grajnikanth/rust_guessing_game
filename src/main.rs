use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..101);
    //println!("The secrete number is: {}", secret_number);
    //loop without a break will be an infinite loop
    loop {
        
        println!("Please input your guess.");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        //Below guess variable is shadowing the previous guess variable. 
        // shadowing is used when we converting the variable type.
        // trim - removes begining and ending strings. parse converts string to u32 
        // number type based on : u32 syntax. If it cannot parse, you get the error handled
        // by the expect() function string message we passed.
        // expect crashes the program when error occurs
        // let guess : u32 = guess.trim().parse().expect("Please enter a number");

        //Alternate way to handle Result returned by parse() function instead of
        // using expect
        let guess : u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue // _ is catchall, continue makes the program continue 
            // even if there is an error
        };


        
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                //break here breaks out the entire program
                break;
            }
        }
    }
    
}