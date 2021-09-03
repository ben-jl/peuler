mod lib;

fn main() {
    let result = lib::primes_less_than_n(20);
    println!("{:?}", result);
}
