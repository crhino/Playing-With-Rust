extern crate num;
use std::from_str;
use std::io::BufferedReader;
use std::io;
use num::bigint::BigUint;
use std::num::One;

fn main() {
    let mut reader = BufferedReader::new(io::stdin());
    

    loop {
        let input = reader.read_line().unwrap();
        let arg = input.slice_to(input.len()-1);
        println!("Input is {}.", arg);
        if arg == "quit" {
            println!("Received the 'quit' command.");
            break;
        }
        match from_str::from_str(arg) {
            Some(num) => {  let n = FromPrimitive::from_u64(num);
                            match n {
                                Some(bignum) => println!("Factorial of {} is {}.", num, 
                                                    recursive_factorial(&bignum).to_str()),
                                _ => println!("Num ({}), is not a uint.", num)
                            }
                         }
            None      => {println!("Argument must be a number.");}
        };
    }
}

fn recursive_factorial(n: &BigUint) -> ~BigUint {
    match n {
        n if n.eq(&One::one()) => ~One::one(),
        _ => ~n.mul(recursive_factorial(&n.sub(&One::one())))
    }
}
