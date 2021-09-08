use criterion::{black_box, criterion_group, criterion_main, Criterion};
use peulerlib;

fn primes(n: u64) -> Vec<u64> {
    peulerlib::primes_less_than_n(n)
}

fn collatz_bench(n: u64) {
    let mut memo = std::collections::HashMap::new();
    let mut largest_seen_vec : Vec<u64> = Vec::new();
    for i in (2..n) {
        let val = peulerlib::collatz(i, &memo);
        if(val.len() > largest_seen_vec.len()) {
            largest_seen_vec = val.clone();
        }
        memo.insert(i,val);
        
    }
}

fn sieve_basic_bencmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("primes_basic_group");
    group.sample_size(100);
    group.bench_function("primes 100", |b| b.iter(|| primes(black_box(100))));
    group.finish();
}

fn collatz_basic_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("collatz_basic_group");
    group.sample_size(100);
    group.bench_function("collatz 100", |b| b.iter(|| collatz_bench(black_box(100))));
    group.finish();
}

criterion_group!(benches, sieve_basic_bencmark, collatz_basic_bench);
criterion_main!(benches);