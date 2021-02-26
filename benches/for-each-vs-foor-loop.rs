use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn build_vector(size: usize) -> Vec<usize> {
    (0..size).collect::<Vec<_>>()
}

fn for_loop(list: Vec<usize>) {
    for item in list.iter() {
        debug_assert!(*item >= 0);
    }
}

fn for_each(list: Vec<usize>) {
    list.iter().for_each(|item| {
        debug_assert!(*item >= 0);
    });
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("iterate for_loop (1_000_000)", |b| {
        b.iter(|| for_loop(black_box(build_vector(1_000_000))))
    });
    c.bench_function("iterate for_each (1_000_000))", |b| {
        b.iter(|| for_each(black_box(build_vector(1_000_000))))
    });
    c.bench_function("iterate for_loop (500_000)", |b| {
        b.iter(|| for_loop(black_box(build_vector(500_000))))
    });
    c.bench_function("iterate for_each (500_000))", |b| {
        b.iter(|| for_each(black_box(build_vector(500_000))))
    });
    c.bench_function("iterate for_loop (100_000)", |b| {
        b.iter(|| for_loop(black_box(build_vector(100_000))))
    });
    c.bench_function("iterate for_each (100_000))", |b| {
        b.iter(|| for_each(black_box(build_vector(100_000))))
    });
    c.bench_function("iterate for_loop (50_000)", |b| {
        b.iter(|| for_loop(black_box(build_vector(50_000))))
    });
    c.bench_function("iterate for_each (50_000))", |b| {
        b.iter(|| for_each(black_box(build_vector(50_000))))
    });
    c.bench_function("iterate for_loop (10_000)", |b| {
        b.iter(|| for_loop(black_box(build_vector(10_000))))
    });
    c.bench_function("iterate for_each (10_000))", |b| {
        b.iter(|| for_each(black_box(build_vector(10_000))))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
