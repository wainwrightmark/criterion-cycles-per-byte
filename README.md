# criterion-cycles-per-byte

![GITHUB](https://img.shields.io/github/last-commit/wainwrightmark/criterion-cycles-per-byte)
![Crates.io](https://img.shields.io/crates/v/criterion-cycles-per-byte)
![docs](https://img.shields.io/docsrs/criterion-cycles-per-byte)

`CyclesPerByte` measures ticks using the x86 or x86_64 `rdtsc` instruction.

> **Warning**
This crate measures clock ticks rather than cycles. It will not provide accurate results on modern machines unless you calculate the ratio of ticks to cycles and take steps to ensure that that ratio remains consistent.

<br>


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

<br>

> **Note**
I am not the original writer but am maintaining this crate because it is still being used in several places. I plan to do version updates and bug fixes as necessary but not to add features or attempt fix the (potentially intractable)  problems with this method of measurement.