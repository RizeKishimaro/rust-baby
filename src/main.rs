use std::io;
use std::cmp::Ordering;
use rand::Rng;
fn main(){
    println!("Hi It's Gussing game");
    
    loop{
        println!("Please Enter Your Number");

    let secret_number = rand::thread_rng().gen_range(1..=100);


    let mut guess = String::new();
    

    io::stdin().read_line(&mut guess).expect("Failed To Read Line!!");
    
    let guess: u32 = match guess.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            println!("Only accept Numbers Not String");
            continue;
        },
    };
    println!("You Guessed: {guess}");

    //cmp = compare 
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too Big"),
        Ordering::Equal =>{
            println!("You Win!");
            break;
        }
    }
    }
}
