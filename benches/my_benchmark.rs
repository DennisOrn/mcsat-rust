use criterion::{criterion_group, criterion_main, Criterion};
use mcsat_rust::{
    example_solver_pigeonhole_1, example_solver_sat_1, example_solver_sat_2,
    example_solver_unsat_1, example_solver_unsat_2,
};

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

fn benchmark_sat_1(c: &mut Criterion) {
    c.bench_function("example_sat_1", |b| b.iter(|| example_solver_sat_1().run()));
}

fn benchmark_sat_2(c: &mut Criterion) {
    c.bench_function("example_sat_2", |b| b.iter(|| example_solver_sat_2().run()));
}

fn benchmark_pigeonhole_1(c: &mut Criterion) {
    c.bench_function("example_pigeonhole_1", |b| {
        b.iter(|| example_solver_pigeonhole_1().run())
    });
}

criterion_group!(
    benches,
    benchmark_unsat_1,
    benchmark_unsat_2,
    benchmark_sat_1,
    benchmark_sat_2,
    benchmark_pigeonhole_1,
);
criterion_main!(benches);
