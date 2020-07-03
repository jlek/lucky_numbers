use ::count_lucky_numbers::count_lucky_numbers;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
  c.bench_function("count_lucky_numbers 1000", |b| {
    b.iter(|| count_lucky_numbers(black_box(0), black_box(1000), black_box(123)))
  });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
