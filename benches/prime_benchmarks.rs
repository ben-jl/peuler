use criterion::{black_box, criterion_group, criterion_main, Criterion};
use peulerlib;

fn primes(n: u64) -> Vec<u64> {
    peulerlib::primes_less_than_n(n)
}

fn sieve_basic_bencmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("primes_basic_group");
    group.sample_size(10000);
    group.bench_function("primes 100", |b| b.iter(|| primes(black_box(100))));
    group.finish();
}

criterion_group!(benches, sieve_basic_bencmark);
criterion_main!(benches);