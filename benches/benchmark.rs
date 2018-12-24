use criterion::{criterion_group, criterion_main, Criterion};
use jql::walker;
use serde_json::Value;

const DATA: &str = r#"{
    "array": [[[[["c", "a", "c"]]]], "g", [[["a", ["t"]]]]]
}"#;

fn criterion_benchmark(c: &mut Criterion) {
    let json: Value = serde_json::from_str(DATA).unwrap();
    let selector = Some(r#".."array""#);
    c.bench_function("Flatten an array", move |b| {
        b.iter(|| walker(&json, selector))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
