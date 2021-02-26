use criterion::{criterion_group, criterion_main, Criterion};
use rand::{distributions::Alphanumeric, Rng};

fn build_string(size: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(size)
        .map(char::from)
        .collect()
}

fn vec_join(count: usize, word_size: usize) {
    let mut result = vec![];
    for _ in 0..count {
        result.push(build_string(word_size));
    }
    result.join(" ");
}

fn string_push(count: usize, word_size: usize) {
    let mut result = String::default();
    for _ in 0..count {
        result.push_str(&build_string(word_size));
    }
}

fn string_concat(count: usize, word_size: usize) {
    let mut result = String::default();
    for _ in 0..count {
        result = result + &build_string(word_size);
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    for count in [10, 50, 100, 500, 1_000].iter() {
        for word_size in [5, 10, 20].iter() {
            c.bench_function(
                format!("vec join ({}, {})", count, word_size).as_str(),
                |b| b.iter(|| vec_join(*count, *word_size)),
            );
            c.bench_function(
                format!("string push ({}, {})", count, word_size).as_str(),
                |b| b.iter(|| string_push(*count, *word_size)),
            );
            c.bench_function(
                format!("string concat ({}, {})", count, word_size).as_str(),
                |b| b.iter(|| string_concat(*count, *word_size)),
            );
        }
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
