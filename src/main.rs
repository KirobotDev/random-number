use rand::Rng;
use std::io;

fn main() {
    let mut i = 0
    while i > 3{
        let mut s = String::new();
    
        let number = rand::thread_rng().gen_range(1..101);
    
        io::stdin()
            .read_line(&mut s)
            .expect("Error");
    
        let nb: i32 = s.trim().parse().expect("Error");
    
        if nb == number {
            println!("Tu a gagné")
        }
    
        else if nb != number {
            println!("le nombre et bien différents")
        }
    
        else {
            println!("Tu a perdu")
        }
    }
}   