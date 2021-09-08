pub mod lib;

fn main() {
    //p1_calc_multiples_of_3_and_5_below_n(1000);
    //p2_calc_even_fib_below_n(4000000);
    //p3_calc_largest_prime_factor_of_n(775146);
    //p10_calc_sum_of_primes_below_n(2000000);

    //p14_get_longest_collatz_sequence_less_than_1000000();

    p5_smallest_multiple_evenly_divided_by_1_through_20();
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
fn p10_calc_sum_of_primes_below_n(n : u64) {
    let ps : Vec<u64> = lib::primes_less_than_n(n);
    let sum : u64 = ps.iter().sum::<u64>();
    println!("Problem 10: {}", sum);
}

fn p4_largest_palindrome_product(d: u8) {
    let lower = (10 as u32).pow(d as u32 - 1);
    let upper = (10 as u32).pow(d  as u32);

    let mut v : Vec<String> = Vec::new();
    let mut max_seen: u32 = 0;
    for i in lower..upper {
        for j in (i+1)..upper {
            if i * j > max_seen {
                let prod = format!("{}", i*j);
                if prod == prod.chars().rev().collect::<String>() {
                
                    v.push(format!("{:4} * {:4} = {:4}", i, j, i*j));
                    max_seen = i * j;
                }
                
            }            
            
        }
    }

    println!("{}", v[v.len() - 1]);
}

pub fn p14_get_longest_collatz_sequence_less_than_1000000() {
    let mut memo = std::collections::HashMap::new();
    let mut largest_seen_vec : Vec<u64> = Vec::new();
    for i in (2..1000000) {
        let val = lib::collatz(i, &memo);
        if(val.len() > largest_seen_vec.len()) {
            largest_seen_vec = val.clone();
        }
        memo.insert(i,val);
        
    }

    println!("\nlongest seq ({}) {:?}",largest_seen_vec.len(), largest_seen_vec);
}

pub fn p5_smallest_multiple_evenly_divided_by_1_through_20() {
    for c in 1.. {
        for i in 1..20+1 {
            if c % i != 0 {
                break;
            }

            if i == 20 {
                println!("{}", c);
                return;
            }
        }
    }
}