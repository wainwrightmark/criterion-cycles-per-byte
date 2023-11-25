//! `CyclesPerByte` measures clock cycles using the CPU read time-stamp counter instruction.
//!
//! ```rust
//! # fn fibonacci_slow(_: usize) {}
//! # fn fibonacci_fast(_: usize) {}
//! use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
//! use criterion_cycles_per_byte::CyclesPerByte;
//!
//! fn bench(c: &mut Criterion<CyclesPerByte>) {
//!     let mut group = c.benchmark_group("fibonacci");
//!
//!     for i in 0..20 {
//!         group.bench_function(BenchmarkId::new("slow", i), |b| b.iter(|| fibonacci_slow(i)));
//!         group.bench_function(BenchmarkId::new("fast", i), |b| b.iter(|| fibonacci_fast(i)));
//!     }
//!
//!     group.finish()
//! }
//!
//! criterion_group!(
//!     name = my_bench;
//!     config = Criterion::default().with_measurement(CyclesPerByte);
//!     targets = bench
//! );
//! criterion_main!(my_bench);
//! ```

use criterion::{
    measurement::{Measurement, ValueFormatter},
    Throughput,
};

#[cfg(not(any(
    target_arch = "x86_64",
    target_arch = "x86",
    target_arch = "aarch64",
    target_arch = "loongarch64"
)))]
compile_error!(
    "criterion-cycles-per-byte currently works only on x86 or x86_64 or aarch64 or loongarch64."
);

/// `CyclesPerByte` measures clock cycles using the CPU read time-stamp counter instruction. `cpb` is
/// the preferred measurement for cryptographic algorithms.
pub struct CyclesPerByte;

// WARN: does not check for the cpu feature; but we'd panic anyway so...
fn rdtsc() -> u64 {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        core::arch::x86_64::_mm_mfence();
        core::arch::x86_64::_rdtsc()
    }

    #[cfg(target_arch = "x86")]
    unsafe {
        core::arch::x86::_mm_mfence();
        core::arch::x86::_rdtsc()
    }

    #[cfg(all(target_arch = "aarch64", target_os = "linux"))]
    unsafe {
        // If a aarch64 CPU, running GNU/Linux kernel, executes following instruction,
        // it'll *probably* panic with message "illegal instruction executed", because userspace
        // isn't allowed to execute that instruction without installing a Linux Kernel Module.
        //
        // I've tested the LKM @ https://github.com/jerinjacobk/armv8_pmu_cycle_counter_el0
        // on a Raspberry Pi 4b ( i.e. ARM Cortex-A72, running kernel version 6.5.0-1006-raspi )
        // and it works like charm. While extending support of this library for aarch64 targets,
        // I found https://github.com/pornin/crrl#benchmarks pretty helpful.
        let mut counter: u64;
        core::arch::asm!("dsb sy", "mrs {}, pmccntr_el0", out(reg) counter);
        counter
    }

    #[cfg(target_arch = "loongarch64")]
    unsafe {
        let counter: u64;
        core::arch::asm!("rdtime.d {0}, $zero", out(reg) counter);
        counter
    }
}

impl Measurement for CyclesPerByte {
    type Intermediate = u64;
    type Value = u64;

    fn start(&self) -> Self::Intermediate {
        rdtsc()
    }

    fn end(&self, i: Self::Intermediate) -> Self::Value {
        rdtsc().saturating_sub(i)
    }

    fn add(&self, v1: &Self::Value, v2: &Self::Value) -> Self::Value {
        v1 + v2
    }

    fn zero(&self) -> Self::Value {
        0
    }

    fn to_f64(&self, value: &Self::Value) -> f64 {
        *value as f64
    }

    fn formatter(&self) -> &dyn ValueFormatter {
        &CyclesPerByteFormatter
    }
}

struct CyclesPerByteFormatter;

impl ValueFormatter for CyclesPerByteFormatter {
    fn format_value(&self, value: f64) -> String {
        format!("{:.4} cycles", value)
    }

    fn format_throughput(&self, throughput: &Throughput, value: f64) -> String {
        match throughput {
            Throughput::Bytes(b) => format!("{:.4} cpb", value / *b as f64),
            Throughput::Elements(b) => format!("{:.4} cycles/{}", value, b),
            Throughput::BytesDecimal(b) => format!("{:.4} cpb (decimal)", value / *b as f64),
        }
    }

    fn scale_values(&self, _typical_value: f64, _values: &mut [f64]) -> &'static str {
        "cycles"
    }

    fn scale_throughputs(
        &self,
        _typical_value: f64,
        throughput: &Throughput,
        values: &mut [f64],
    ) -> &'static str {
        match throughput {
            Throughput::Bytes(n) => {
                for val in values {
                    *val /= *n as f64;
                }
                "cpb"
            }
            Throughput::Elements(n) => {
                for val in values {
                    *val /= *n as f64;
                }
                "c/e"
            }
            Throughput::BytesDecimal(n) => {
                for val in values {
                    *val /= *n as f64;
                }
                "cpb (decimal)"
            }
        }
    }

    fn scale_for_machines(&self, _values: &mut [f64]) -> &'static str {
        "cycles"
    }
}
