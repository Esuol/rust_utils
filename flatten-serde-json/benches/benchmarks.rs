use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use flatten_serde_json::flatten;
use serde_json::json;

pub fn flatten_simple(c: &mut Criterion) {
    let input = json!({
      "a": {
        "b": "c",
        "d": "e",
        "f": "g"
      }
    });

    let object = input.as_object().unwrap();

    c.bench_with_input(
        BenchmarkId::new("flatten", "simple"),
        &object,
        |b, input| b.iter(|| flatten(&input)),
    );
}

pub fn flatten_complex(c: &mut Criterion) {
    let input = json!({
      "a": [
        "b",
        ["c", "d"],
        { "e": ["f", "g"] },
        [
            { "h": "i" },
            { "e": ["j", { "z": "y" }] },
        ],
        ["l"],
        "m",
      ]
    });

    let object = input.as_object().unwrap();

    c.bench_with_input(
        BenchmarkId::new("flatten", "flatten_complex"),
        &object,
        |b, input| b.iter(|| flatten(&input)),
    );
}
// 定义基准测试组
criterion_group!(benches, flatten_simple, flatten_complex);
// 定义基准测试的入口点
criterion_main!(benches);
