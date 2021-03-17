use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn max_stock_benchmark(c: &mut Criterion) {
    let input = &[2.0, 3.0, 5.0, 6.0, 1.0, 2.0, 10.0];
    c.bench_function("max stockprice 20", |b| b.iter(|| stock_stats::max(black_box(input))));
}

criterion_group!(benches, max_stock_benchmark);
criterion_main!(benches);