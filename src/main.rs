mod argument;
mod primality_test;

fn main() {
    let max = argument::fetch_max_value();
    let prime_numbers = primality_test::sieve_of_eratosthenes(max);
    println!("{:?}", prime_numbers);
}
