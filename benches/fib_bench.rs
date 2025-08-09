use std::hint::black_box;

use algs::algorithms::fibo::{fibo_1, fibo_2, fibo_3, fibo_3_large, fibo_4};
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use num_bigint::BigUint;

fn bench_fibo(c: &mut Criterion) {
    let mut group = c.benchmark_group("Fibonacci Comparison");
    group.sample_size(10);

    // test small, medium, and large inputs
    let small_inputs = vec![10u64, 20, 30, 40];
    let medium_inputs = vec![50u64, 60, 70, 80, 90];
    let large_inputs = vec![100u64, 200, 400, 600, 800, 1000];
    let huge_inputs = vec![10_000u64, 1_000_000];


    // fibo_1 recursive -- only on small inputs
    for &n in &small_inputs {
        group.bench_with_input(BenchmarkId::new("fibo_1", n), &n, |b, &n| {
            b.iter(|| fibo_1(black_box(n as u32)));
        });
    }

    // fibo_2 (memoized)
    for &n in &small_inputs {
        group.bench_with_input(BenchmarkId::new("fibo_2", n), &n, |b, &n| {
            b.iter(|| {
                let mut cache = vec![u128::MAX; (n as usize) + 1];
                cache[0] = 0;
                cache[1] = 1;
                fibo_2(black_box(n as u32), &mut cache)
            });
        });
    }

    // fibo_3 iterative
    for &n in medium_inputs.iter().chain(large_inputs.iter()) {
        group.bench_with_input(BenchmarkId::new("fibo_3", n), &n, |b, &n| {
            b.iter(|| {
                let _ = fibo_3(black_box(n));
            });
        });
    }

    for &n in small_inputs
        .iter()
        .chain(medium_inputs.iter())
        .chain(large_inputs.iter())
    {
        group.bench_with_input(BenchmarkId::new("fibo_3_large", n), &n, |b, &n| {
            b.iter(|| {
                let _ = fibo_3_large(black_box(n));
            });
        });
    }

    // fibo_4 fast doubling
    for &n in small_inputs
        .iter()
        .chain(medium_inputs.iter())
        .chain(large_inputs.iter())
        .chain(huge_inputs.iter())
    {
        group.bench_with_input(BenchmarkId::new("fibo_4", n), &n, |b, &n| {
            b.iter(|| {
                let _: BigUint = fibo_4(black_box(n));
            });
        });
    }

    group.finish();
}

criterion_group!(benches, bench_fibo);
criterion_main!(benches);
