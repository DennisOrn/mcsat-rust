use criterion::{criterion_group, criterion_main, Criterion};
use mcsat_rust::{example_solver_unsat_1, example_solver_unsat_2};

fn benchmark_unsat_1(c: &mut Criterion) {
    c.bench_function("example_unsat_1", |b| {
        b.iter(|| example_solver_unsat_1().run())
    });
}

fn benchmark_unsat_2(c: &mut Criterion) {
    c.bench_function("example_unsat_2", |b| {
        b.iter(|| example_solver_unsat_2().run())
    });
}

criterion_group!(benches, benchmark_unsat_1, benchmark_unsat_2);
criterion_main!(benches);
