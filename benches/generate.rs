#[macro_use]
extern crate criterion;
extern crate rpwgen;

use criterion::Criterion;
use rpwgen::{generate, Config};

fn bench_generate(c: &mut Criterion) {
    c.bench_function("generate", |b| {
        let config = Config::default();
        b.iter(|| generate(&config))
    });
}

criterion_group!(benches, bench_generate);
criterion_main!(benches);
