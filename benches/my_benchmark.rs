use criterion::{criterion_group, criterion_main, Criterion};

use mcsat_rust::example_solver_unsat;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("example_unsat", |b| b.iter(|| example_solver_unsat().run()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
