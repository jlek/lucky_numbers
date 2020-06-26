use ::lucky_numbers::{count_lucky_numbers, LuckyNumbersRequest};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
  c.bench_function("count_lucky_numbers 1000", |b| {
    b.iter(|| {
      count_lucky_numbers(black_box(LuckyNumbersRequest {
        start: 0,
        end: 1000,
        sequence: 123,
      }))
    })
  });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
