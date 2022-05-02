use criterion::{criterion_group, criterion_main, Criterion};

fn parse(s: &str) -> Result<bool, std::num::ParseIntError> {
  s.parse::<u64>().map(|n| n > 100_u64)
}

fn bytes_cmp(s: &str) -> bool {
  let n = 100_u64.to_string();
  s.as_bytes().gt(n.as_bytes())
}

fn parse_vs_bytes_cmp_benchmark(c: &mut Criterion) {
  let mut group = c.benchmark_group("number_parse");

  group.bench_function("parse", |b| b.iter(|| parse("10")));

  group.bench_function("bytes_cmp", |b| b.iter(|| bytes_cmp("10")));
  group.finish();
}

criterion_group!(benches, parse_vs_bytes_cmp_benchmark,);
criterion_main!(benches);
