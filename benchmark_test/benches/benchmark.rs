use criterion::{criterion_group, criterion_main, Criterion};

use mylib::comp;

fn bm(c: &mut Criterion) {
    c.bench_function("Stupid Fibonacci", |b| {
        b.iter(|| {
            comp::stupid_fibonacci(4);
            comp::stupid_fibonacci(30);
            comp::stupid_fibonacci(45);
        })
    });
}

criterion_group!(benches, bm);
criterion_main!(benches);
