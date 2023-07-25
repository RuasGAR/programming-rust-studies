use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    //this thread_rng relates to the scope of execution of the program, from whom it will grab
    //its seeds
    //standard integer type is i32
    let secret_number = rand::thread_rng().gen_range(1,101);

    //println!("The secret number is: {}", secret_number);

    println!("Guess the number");
 
    loop {

    
        println!("Please input your guess.");

        let mut guess = String::new();
        
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        
        // type casting pra number
        // esse recurso (de termos duas variáveis com o mesmo nome) é chamado de SHADOW
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}",guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            } 
            
        }

    }
}
