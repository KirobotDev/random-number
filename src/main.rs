use rand::Rng;
use std::io;

fn main() {
    let mut s = String::new();

    let number = rand::thread_rng().gen_range(1..101);

    io::stdin()
        .read_line(&mut s)
        .expect("Error");

    let nb: i32 = s.trim().parse().expect("Error");

    if nb == number {
        println!("Tu a gagné")
    }

    else {
        println!("Tu a perdu")
    }
}   