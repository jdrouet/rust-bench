//! Given results
//!
//! ```
//! write_by_chunks without capacity
//!                         time:   [356.30 µs 356.83 µs 357.39 µs]
//! Found 3 outliers among 100 measurements (3.00%)
//!   2 (2.00%) high mild
//!   1 (1.00%) high severe
//!
//! write_by_chunks with capacity
//!                         time:   [366.91 µs 368.68 µs 370.33 µs]
//! Found 1 outliers among 100 measurements (1.00%)
//!   1 (1.00%) high mild
//!
//! write_each without capacity
//!                         time:   [3.8138 ms 3.8188 ms 3.8243 ms]
//! Found 9 outliers among 100 measurements (9.00%)
//!   6 (6.00%) high mild
//!   3 (3.00%) high severe
//!
//! write_each with capacity
//!                         time:   [3.8538 ms 3.8632 ms 3.8726 ms]
//! ```
//!
//! We can see that writing by chunks is way faster.

use criterion::{criterion_group, criterion_main, Criterion};

const FRANKENSTEIN: &str = include_str!("../resources/frankenstein.txt");

const CONTENT_ESCAPE: &str = "&<>\"'/";

pub fn write_by_chunks<W: std::io::Write>(f: &mut W, value: &str) -> Result<(), std::io::Error> {
    if value.is_empty() {
        return Ok(());
    }
    if let Some(index) = value.find(|c| CONTENT_ESCAPE.contains(c)) {
        // f.write(value[..index].as_bytes())?;
        write!(f, "{}", &value[..index])?;
        match &value[index..(index + 1)] {
            "&" => f.write(b"&amp;")?,
            "<" => f.write(b"&lt;")?,
            ">" => f.write(b"&gt;")?,
            "\"" => f.write(b"&quot;")?,
            "'" => f.write(b"&#x27;")?,
            "/" => f.write(b"&#x2F;")?,
            // this should be unreachable
            other => f.write(other.as_bytes())?,
        };
        write_by_chunks(f, &value[(index + 1)..])?;
    } else {
        f.write(value.as_bytes())?;
    }
    Ok(())
}

pub fn write_each<W: std::io::Write>(f: &mut W, value: &str) -> Result<(), std::io::Error> {
    for c in value.chars() {
        match c {
            '&' => {
                f.write(b"&amp;")?;
            }
            '<' => {
                f.write(b"&lt;")?;
            }
            '>' => {
                f.write(b"&gt;")?;
            }
            '"' => {
                f.write(b"&quot;")?;
            }
            '\'' => {
                f.write(b"&#x27;")?;
            }
            '/' => {
                f.write(b"&#x2F;")?;
            }
            other => write!(f, "{other}")?,
        }
    }
    Ok(())
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("write_by_chunks without capacity", |b| {
        b.iter(|| {
            let mut buf: Vec<u8> = Vec::new();
            write_by_chunks(&mut buf, FRANKENSTEIN).unwrap();
        })
    });
    c.bench_function("write_by_chunks with capacity", |b| {
        b.iter(|| {
            let mut buf: Vec<u8> = Vec::with_capacity(FRANKENSTEIN.len());
            write_by_chunks(&mut buf, FRANKENSTEIN).unwrap();
        })
    });
    c.bench_function("write_each without capacity", |b| {
        b.iter(|| {
            let mut buf: Vec<u8> = Vec::new();
            write_each(&mut buf, FRANKENSTEIN).unwrap();
        })
    });
    c.bench_function("write_each with capacity", |b| {
        b.iter(|| {
            let mut buf: Vec<u8> = Vec::with_capacity(FRANKENSTEIN.len());
            write_each(&mut buf, FRANKENSTEIN).unwrap();
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
