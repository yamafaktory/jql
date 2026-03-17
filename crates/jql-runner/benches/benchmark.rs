use criterion::{
    BenchmarkId,
    Criterion,
    criterion_group,
    criterion_main,
};
use jql_runner::runner::raw;
use serde_json::{
    Value,
    json,
};

fn array_range_selector(c: &mut Criterion) {
    c.bench_function("Array range selector", move |b| {
        b.iter(|| raw("[2,0]", &json!([0, 1, 2, 3, 4, 5, 6, 7, 8, 9])))
    });
}

fn flatten_operator(c: &mut Criterion) {
    c.bench_function("Flatten operator", move |b| {
        b.iter(|| raw("..", &json!([[[[[[[0]]], 1, [[[[2]]]], 3]]]])))
    });
}

fn group_separator(c: &mut Criterion) {
    c.bench_function("Group separator", move |b| {
        b.iter(|| raw(r#""a","b","c""#, &json!({ "a": 1, "b": 2, "c": 3 })))
    });
}

fn key_selector(c: &mut Criterion) {
    c.bench_function("Key selector", move |b| {
        b.iter(|| {
            raw(
                r#""props""a""b""c""#,
                &json!({ "props": { "a": { "b": { "c": 1} } } }),
            )
        })
    });
}

fn pipe_operators(c: &mut Criterion) {
    c.bench_function("Pipe operators", move |b| {
        b.iter(|| {
            raw(
                r#""nested"|>"laptop""brand"<|[1]"#,
                &json!({
                    "nested": [
                        {
                            "laptop": {
                                "brand": "Apple"
                            }
                        },
                        {
                            "laptop": {
                                "brand": "Asus"
                            }
                        }
                    ]
                }),
            )
        })
    });
}

fn flatten_threshold(c: &mut Criterion) {
    let mut group = c.benchmark_group("Flatten threshold");

    for size in [1, 2, 4, 8, 16, 32, 64, 80, 96, 128] {
        // Each element is itself a single-element array so every item recurses.
        let input: Value = json!((0..size).map(|i| json!([i])).collect::<Vec<_>>());

        group.bench_with_input(BenchmarkId::from_parameter(size), &input, |b, json| {
            b.iter(|| raw("..", json))
        });
    }

    group.finish();
}

fn pipe_threshold(c: &mut Criterion) {
    let mut group = c.benchmark_group("Pipe threshold");

    for size in [1, 2, 4, 8, 16, 32, 64, 80, 96, 128] {
        let items: Value = json!((0..size).map(|i| json!({ "a": i })).collect::<Vec<_>>());
        let input: Value = json!({ "items": items });

        group.bench_with_input(BenchmarkId::from_parameter(size), &input, |b, json| {
            b.iter(|| raw(r#""items"|>"a"<|[0]"#, json))
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    array_range_selector,
    flatten_operator,
    flatten_threshold,
    group_separator,
    key_selector,
    pipe_operators,
    pipe_threshold,
);

criterion_main!(benches);
