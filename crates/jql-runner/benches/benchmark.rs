#![allow(missing_docs)]

use criterion::{
    BatchSize,
    BenchmarkId,
    Criterion,
    Throughput,
    criterion_group,
    criterion_main,
};
use jql_runner::{
    lazy,
    runner::raw,
};
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

fn to_bytes(value: &Value) -> Vec<u8> {
    serde_json::to_vec(value).unwrap()
}

fn bench_lazy(c: &mut Criterion, name: &str, query: &'static str, value: &Value) {
    let input = to_bytes(value);

    c.bench_function(name, move |b| {
        b.iter_batched_ref(
            || input.clone(),
            |json| lazy::raw(query, json),
            BatchSize::SmallInput,
        )
    });
}

fn lazy_array_range_selector(c: &mut Criterion) {
    bench_lazy(
        c,
        "Lazy array range selector",
        "[2,0]",
        &json!([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]),
    );
}

fn lazy_flatten_operator(c: &mut Criterion) {
    bench_lazy(
        c,
        "Lazy flatten operator",
        "..",
        &json!([[[[[[[0]]], 1, [[[[2]]]], 3]]]]),
    );
}

fn lazy_group_separator(c: &mut Criterion) {
    bench_lazy(
        c,
        "Lazy group separator",
        r#""a","b","c""#,
        &json!({ "a": 1, "b": 2, "c": 3 }),
    );
}

fn lazy_key_selector(c: &mut Criterion) {
    bench_lazy(
        c,
        "Lazy key selector",
        r#""props""a""b""c""#,
        &json!({ "props": { "a": { "b": { "c": 1} } } }),
    );
}

fn lazy_pipe_operators(c: &mut Criterion) {
    bench_lazy(
        c,
        "Lazy pipe operators",
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
    );
}

fn repositories(count: usize) -> Value {
    json!({
        "items": (0..count)
            .map(|index| json!({
                "id": index,
                "name": format!("repository-{index}"),
                "owner": {
                    "id": index,
                    "login": format!("user-{index}"),
                },
                "description": "A repository standing in for a realistic payload.",
                "topics": ["cli", "json", "rust"],
                "stargazers_count": index,
            }))
            .collect::<Vec<_>>()
    })
}

const DEEP_QUERY: &str = r#""items"[0]"owner""login""#;

fn large_document(c: &mut Criterion) {
    let value = repositories(5_000);
    let input = to_bytes(&value);
    let mut group = c.benchmark_group("Large document");

    group.throughput(Throughput::Bytes(input.len() as u64));

    group.bench_function("serde_json and runner", |b| {
        b.iter(|| {
            raw(
                DEEP_QUERY,
                &serde_json::from_slice::<Value>(&input).unwrap(),
            )
        })
    });

    group.bench_function("lazy", |b| {
        b.iter_batched_ref(
            || input.clone(),
            |json| lazy::raw(DEEP_QUERY, json),
            BatchSize::LargeInput,
        )
    });

    group.finish();
}

fn multi_document(c: &mut Criterion) {
    let document = to_bytes(&repositories(200));
    let mut input = Vec::new();

    for _ in 0..10 {
        input.extend_from_slice(&document);
        input.push(b'\n');
    }

    let mut evaluator = lazy::Evaluator::new();
    let mut group = c.benchmark_group("Multi document");

    group.throughput(Throughput::Bytes(input.len() as u64));

    group.bench_function("lazy", |b| {
        b.iter_batched_ref(
            || input.clone(),
            |json| evaluator.raw_all(DEEP_QUERY, json),
            BatchSize::LargeInput,
        )
    });

    group.finish();
}

criterion_group!(
    benches,
    array_range_selector,
    flatten_operator,
    flatten_threshold,
    group_separator,
    key_selector,
    large_document,
    lazy_array_range_selector,
    lazy_flatten_operator,
    lazy_group_separator,
    lazy_key_selector,
    lazy_pipe_operators,
    multi_document,
    pipe_operators,
    pipe_threshold,
);

criterion_main!(benches);
