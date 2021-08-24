extern crate rand; // indica ao Rust que estamos usando uma dependência externa.

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let version = 1; // imutável
    let numero_secreto = rand::thread_rng().gen_range(1, 101);

    println!("Jogo de adivinhação v.{}", version);
    println!("Adivinhe o numero!");

    loop {
        println!("Digite o seu palpite.");

        let mut palpite = String::new(); // mutável

        io::stdin()
            .read_line(&mut palpite) // O símbolo & indica que o argumento é uma referência,
            .expect("Falha ao ler entrada");

        // Rust nos permite sombrear variaveis
        let palpite: u32 = match palpite.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Você disse: {}", palpite);

        match palpite.cmp(&numero_secreto) {
            Ordering::Less => println!("Muito Baixo!"),
            Ordering::Greater => println!("Muito Alto!"),
            Ordering::Equal => {
                println!("Muito Acertou!");
                break;
            }
        }
    }
}
