use criterion::{criterion_group, criterion_main, Criterion};

const FRANKENSTEIN: &str = include_str!("../resources/frankenstein.txt");

const CONTENT_ESCAPE: &str = "&<>\"'/";

pub fn fmtwrite<W: std::fmt::Write>(f: &mut W, value: &str) -> std::fmt::Result {
    if value.is_empty() {
        return Ok(());
    }
    if let Some(index) = value.find(|c| CONTENT_ESCAPE.contains(c)) {
        f.write_str(&value[..index])?;
        match &value[index..(index + 1)] {
            "&" => f.write_str("&amp;")?,
            "<" => f.write_str("&lt;")?,
            ">" => f.write_str("&gt;")?,
            "\"" => f.write_str("&quot;")?,
            "'" => f.write_str("&#x27;")?,
            "/" => f.write_str("&#x2F;")?,
            // this should be unreachable
            other => f.write_str(other)?,
        };
        fmtwrite(f, &value[(index + 1)..])?;
    } else {
        f.write_str(value)?;
    }
    Ok(())
}

pub fn iowrite<W: std::io::Write>(f: &mut W, value: &str) -> Result<(), std::io::Error> {
    if value.is_empty() {
        return Ok(());
    }
    if let Some(index) = value.find(|c| CONTENT_ESCAPE.contains(c)) {
        f.write(value[..index].as_bytes())?;
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
        iowrite(f, &value[(index + 1)..])?;
    } else {
        f.write(value.as_bytes())?;
    }
    Ok(())
}

fn criterion_benchmark(c: &mut Criterion) {
    let lines: Vec<_> = FRANKENSTEIN.split('\n').collect();
    //
    let mut g = c.benchmark_group("big chunk without capacity");
    g.bench_function("fmtwrite", |b| {
        b.iter_with_large_drop(|| {
            let mut buf = String::new();
            fmtwrite(&mut buf, FRANKENSTEIN).unwrap();
            buf
        })
    });
    g.bench_function("iowrite", |b| {
        b.iter_with_large_drop(|| {
            let mut buf: Vec<u8> = Vec::new();
            iowrite(&mut buf, FRANKENSTEIN).unwrap();
            buf
        })
    });
    g.finish();
    //
    let mut g = c.benchmark_group("big chunk with capacity");
    g.bench_function("fmtwrite", |b| {
        b.iter_with_large_drop(|| {
            let mut buf = String::with_capacity(FRANKENSTEIN.len());
            fmtwrite(&mut buf, FRANKENSTEIN).unwrap();
            buf
        })
    });
    g.bench_function("iowrite", |b| {
        b.iter_with_large_drop(|| {
            let mut buf: Vec<u8> = Vec::with_capacity(FRANKENSTEIN.len());
            iowrite(&mut buf, FRANKENSTEIN).unwrap();
            buf
        })
    });
    g.finish();
    //
    let mut g = c.benchmark_group("small chunks without capacity");
    g.bench_function("fmtwrite", |b| {
        b.iter_with_large_drop(|| {
            let mut buf = String::new();
            for line in lines.iter() {
                fmtwrite(&mut buf, line).unwrap();
            }
            buf
        })
    });
    g.bench_function("iowrite", |b| {
        b.iter_with_large_drop(|| {
            let mut buf: Vec<u8> = Vec::new();
            for line in lines.iter() {
                iowrite(&mut buf, line).unwrap();
            }
            buf
        })
    });
    g.finish();
    //
    let mut g = c.benchmark_group("small chunks with capacity");
    g.bench_function("fmtwrite", |b| {
        b.iter_with_large_drop(|| {
            let mut buf = String::with_capacity(FRANKENSTEIN.len());
            for line in lines.iter() {
                fmtwrite(&mut buf, line).unwrap();
            }
            buf
        })
    });
    g.bench_function("iowrite", |b| {
        b.iter_with_large_drop(|| {
            let mut buf: Vec<u8> = Vec::with_capacity(FRANKENSTEIN.len());
            for line in lines.iter() {
                iowrite(&mut buf, line).unwrap();
            }
            buf
        })
    });
    g.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default().significance_level(0.1).sample_size(10_000);
    targets = criterion_benchmark
);
criterion_main!(benches);
