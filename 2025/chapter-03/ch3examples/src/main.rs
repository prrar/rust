/*
    fibonacci.rs
    2025-07-07
    prrar83@gmail.com
 */

use std::io;

fn main() {
    println!("N-fibonacci: ");
    let mut n_fibonacci: String = String::new();

    io::stdin().read_line(&mut n_fibonacci).expect("Failed to read line");
    let n_fibonacci: u32 = n_fibonacci.trim().parse().expect("Not a number");
    
    let mut result: u32 = 0;

    if n_fibonacci == 0 {
        result = 0;
    } else if n_fibonacci == 1 {
        result = 1;
    } else{
        let mut n_minus_2 = 0;
        let mut n_minus_1 = 1;
        for _ in 2..(n_fibonacci + 1) {
            result = n_minus_2 + n_minus_1;
            n_minus_2 = n_minus_1;
            n_minus_1 = result;
        }
    }

    println!("F[{n_fibonacci}] = {result}");
}
