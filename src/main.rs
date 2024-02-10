use num_bigint::{BigInt, ToBigInt, RandBigInt};
use std::io;
use num_traits::sign::Signed;
use std::io::prelude::*;
use colored::*;

// This macro is inspired from : https://docs.rs/miller_rabin/latest/src/miller_rabin/lib.rs.html#1-218
macro_rules! bigint {
    ($e:expr) => {
        ($e).to_bigint().unwrap()
    };
}

fn main()
{
    println!("Welcome to this Miller-Rabin implementation in Rust!");
    let mut user_input = String::new();

    while user_input != "q"
    {
        // Prompt
        print!("{}", "> ".blue());
        io::stdout().flush().unwrap();
        
        // Reading from stdin
        user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line from stdin");
        
        
        // Parsing and checking for errors
        // let n: BigInt = BigInt::parse_bytes(user_input.trim().as_bytes(), 10).unwrap();
        let n = BigInt::parse_bytes(user_input.trim().as_bytes(), 10);
        let n:BigInt = match n {
            None => {
                println!("Your input was not an integer :(");
                continue;
            },
            Some(i) => i
        };

        // Checking and sending result in stdout
        match is_prime(&(n.abs()))
        {
            true => println!("This is a prime number!"),
            false => println!("This is a composite number!")
        };
    }


}

// This function run with different values of `a` the `is_a_witness` function
fn is_prime(n: &BigInt) -> bool
{
    // Handling special case
    if n == &bigint!(3) || n == &bigint!(2)
    {
        return true;
    }
    if n == &bigint!(1)
    {
        return false;
    }
    if n % bigint!(2) == bigint!(0)
    {
        return false;
    }

    let precision: u16 = 20;    // You can tweak this value but 20 is okay, accuracy of (1/4)^{20}
    let mut rng = rand::thread_rng();

    for _ in 0..precision
    {
        let a = rng.gen_bigint_range(&bigint!(2), &(n - 1));

        if is_a_witness(bigint!(a), &n)
        {
            return false;
        }
    }

    true
}

/*
    According to the miller-rabin test we have to write the candidate `n` in the form :
        n = 1 + q*2^t
    The following function return `q` and `t` in a tuple
    get_q_and_t(mut n: BigInt) -> (q: BigInt, t: BigInt)
*/
fn get_q_and_t(mut n: BigInt) -> (BigInt, BigInt)
{
    n = &n - 1u8;
    let mut t: BigInt = bigint!(0);

    while &n % bigint!(2) == bigint!(0)
    {
        t += 1;
        n /= 2;
    }

    (n, t)
}

// The goal of this function is to check if a number `a` is a miller rabin witness for `n`
fn is_a_witness(mut a: BigInt, n: &BigInt) -> bool
{
    // Retrieve `q` and `t` from `n`
    let (q, t) = get_q_and_t(n.clone());
    let mut t = t;
    let n_minus_one: BigInt = n - 1u8;

    a = a.modpow(&q, n);

    if a == bigint!(1)
    {
        return false;
    }

    while t > bigint!(0)
    {
        if a == n_minus_one
        {
            return false;
        }

        a = a.modpow(&bigint!(2), n);
        t = &t - 1u8;
    }

    true
}