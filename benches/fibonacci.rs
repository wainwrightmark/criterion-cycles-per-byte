use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use criterion_cycles_per_byte::CyclesPerByte;

fn fibonacci_slow(n: usize) -> usize {
    match n {
        0 => panic!("zero is not a good argument to fibonacci_slow()!"),
        1 | 2 => 1,
        3 => 2,
        /*
        50    => 12586269025,
        */
        _ => fibonacci_slow(n - 1) + fibonacci_slow(n - 2),
    }
}
fn fibonacci_fast(n: usize) -> usize {
    if n == 0 {
        panic!("zero is not a right argument to fibonacci_fast()!");
    } else if n == 1 {
        return 1;
    }

    let mut sum = 0;
    let mut last = 0;
    let mut current = 1;
    for _i in 1..n {
        sum = last + current;
        last = current;
        current = sum;
    }
    sum
}

fn bench(c: &mut Criterion<CyclesPerByte>) {
    let mut group = c.benchmark_group("fibonacci");
    for i in 1..20 {
        group.bench_function(BenchmarkId::new("slow", i), |b| {
            b.iter(|| fibonacci_slow(i))
        });
        group.bench_function(BenchmarkId::new("fast", i), |b| {
            b.iter(|| fibonacci_fast(i))
        });
    }

    group.finish()
}

criterion_group!(
    name = my_bench;
    config = Criterion::default().with_measurement(CyclesPerByte);
    targets = bench
);
criterion_main!(my_bench);
