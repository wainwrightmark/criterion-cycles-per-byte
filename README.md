# criterion-cycles-per-byte

`CyclesPerByte` measures clock cycles using the x86 or x86_64 `rdtsc` instruction.
//!
```rust
# fn fibonacci_slow(_: usize) {}
# fn fibonacci_fast(_: usize) {}
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use criterion_cycles_per_byte::CyclesPerByte;
//!
fn bench(c: &mut Criterion<CyclesPerByte>) {
    let mut group = c.benchmark_group("fibonacci");
//!
    for i in 0..20 {
        group.bench_function(BenchmarkId::new("slow", i), |b| b.iter(|| fibonacci_slow(i)));
        group.bench_function(BenchmarkId::new("fast", i), |b| b.iter(|| fibonacci_fast(i)));
    }
//!
    group.finish()
}
//!
criterion_group!(
    name = my_bench;
    config = Criterion::default().with_measurement(CyclesPerByte);
    targets = bench
);
criterion_main!(my_bench);
```
