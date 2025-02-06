# criterion-cycles-per-byte

![GITHUB](https://img.shields.io/github/last-commit/wainwrightmark/criterion-cycles-per-byte)
![Crates.io](https://img.shields.io/crates/v/criterion-cycles-per-byte)
![docs](https://img.shields.io/docsrs/criterion-cycles-per-byte)


`CyclesPerByte` measures ticks using the CPU read time-stamp counter instruction.

## Cycle measurement instructions

| Architecture |  Instruction  |
| ------------ | ------------- |
| x86          | rdtsc / rdpru |
| x86_64       | rdtsc / rdpru |
| aarch64 (running GNU/Linux kernel)     | pmccntr     |
| loongarch64  | rdtime.d      |
| riscv64      | rdtime        |

The RDPRU instruction is available only on AMD CPUs since Zen 2 and it is not used by default.
To enable it use the `rdpru` configuration flag, e.g. by using `RUSTFLAGS="--cfg rdpru"`.
Note that this crate does not check availability of the instruction at runtime,
which may result in the "illegal instruction" exception during benchmark execution.

After enabling `rdpru` it is also strongly recommended to pin benchmarks to one core, e.g. by using
`taskset`: `RUSTFLAGS="--cfg rdpru" taskset --cpu-list 0 cargo bench`. Otherwise, the crate may
produce wildly incorrect measurments caused by benchmark thread migration across CPU cores.

### Warnings: x86

Unless `rdpru` is enabled, this crate measures clock ticks rather than cycles.
It will not provide accurate results on modern machines unless you calculate the ratio of ticks
to cycles and take steps to ensure that that ratio remains consistent.

### Warnings: aarch64

In case you're planning to use this library on an `aarch64` target, running GNU/Linux kernel,
I advise you to read [src/lib.rs#L61-L68](src/lib.rs#L61-L68).

## Example

```rust
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use criterion_cycles_per_byte::CyclesPerByte;

fn bench(c: &mut Criterion<CyclesPerByte>) {
    let mut group = c.benchmark_group("fibonacci");

    for i in 0..20 {
        group.bench_function(BenchmarkId::new("slow", i), |b| b.iter(|| fibonacci_slow(i)));
        group.bench_function(BenchmarkId::new("fast", i), |b| b.iter(|| fibonacci_fast(i)));
    }

    group.finish()
}

criterion_group!(
    name = my_bench;
    config = Criterion::default().with_measurement(CyclesPerByte);
    targets = bench
);
criterion_main!(my_bench);
```

## Maintainence status

I am not the original writer but am maintaining this crate because it is still being used
in several places. I plan to do version updates and bug fixes as necessary but not to add
features or attempt fix the (potentially intractable) problems with this method of measurement.


## Compatibility

| Criterion version | Cycles Per Byte Version |
|-------------------|-------------------------|
| 0.5               | 0.6                     |
| 0.4               | 0.4                     |