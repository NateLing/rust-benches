use std::sync::Arc;

use bytes::Buf;
use criterion::{criterion_group, criterion_main, Criterion};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct User {
  name:  String,
  age:   u8,
  email: String,
  add:   String,
}

fn serde_with_reader(s: &str) -> Result<User, serde_json::Error> {
  let b = s.as_bytes();
  let rdr = b.reader();

  serde_json::from_reader(rdr)
}

fn serde_with_slice(s: &str) -> Result<User, serde_json::Error> {
  let b = s.as_bytes();
  serde_json::from_slice(b)
}

fn reader_vs_slice_benchmark(c: &mut Criterion) {
  let user = r#"{{
    "name":"Jim",
    "age":18,
    "email":"jim@gmail.com",
    "add":"Yosemite Village, CA 95389"
  }}"#;

  let user = Arc::new(user);
  let mut group = c.benchmark_group("Serde");

  group.bench_with_input("reader", &user, |b, u| b.iter(|| serde_with_reader(u)));

  group.bench_with_input("slice", &user, |b, u| b.iter(|| serde_with_slice(u)));
  group.finish();
}

fn large_reader_vs_slice_benchmark(c: &mut Criterion) {
  let user = r#"{{
    "name":"Jim",
    "age":18,
    "email":"jim@gmail.com",
    "add":"Yosemite Village, CA 95389"
  }}"#;

  let users = vec![user; 100];

  let users = Arc::new(users.join(","));
  let mut group = c.benchmark_group("Large serde");

  group.bench_with_input("reader", &users, |b, u| b.iter(|| serde_with_reader(u)));

  group.bench_with_input("slice", &users, |b, u| b.iter(|| serde_with_slice(u)));
  group.finish();
}

criterion_group!(
  benches,
  reader_vs_slice_benchmark,
  large_reader_vs_slice_benchmark
);
criterion_main!(benches);
