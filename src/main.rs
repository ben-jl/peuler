mod lib;

fn main() {
    let primes = lib::primes_less_than_n(775146);
    for p in primes.iter().rev() {
        if 600851475143 % p == 0 {
            println!("{}",p);
            break;
        }
    }
}
