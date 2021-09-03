mod lib;

fn main() {
    //p1_calc_multiples_of_3_and_5_below_n(1000);
    //p2_calc_even_fib_below_n(4000000);
    //p3_calc_largest_prime_factor_of_n(775146);
    p4_calc_sum_of_primes_below_n(2000000);
}

#[allow(dead_code)]
fn p1_calc_multiples_of_3_and_5_below_n(n : i32) {
    let mut sum : i32 = 0;
    for number in 1..(n-1) {
        if number % 3 == 0 || number % 5 == 0 {
            sum = sum + number;
        }
    }
    println!("Problem 1: {}", sum);
}

#[allow(dead_code)]
fn p2_calc_even_fib_below_n(n: i32) {
    let mut sum : i32 = 0;
    let mut last : i32 = 1;
    let mut curr : i32 = 1;

    while curr < n {
        if curr % 2 == 0 {
            sum = sum + curr;
        }
        curr = last + curr;
        last = curr - last;
     }

    println!("Problem 2: {}", sum);
}

#[allow(dead_code)]
fn p3_calc_largest_prime_factor_of_n(n : u64) {
    let primes = lib::primes_less_than_n(n);
    for p in primes.iter().rev() {
        if 600851475143 % p == 0 {
            println!("Problem 3: {}",p);
            break;
        }
    }
}

#[allow(dead_code)]
fn p4_calc_sum_of_primes_below_n(n : u64) {
    let ps : Vec<u64> = lib::primes_less_than_n(n);
    let sum : u64 = ps.iter().sum::<u64>();
    println!("Problem 10: {}", sum);
}