use criterion::{
    criterion_group,
    criterion_main,
    Criterion,
};
use jql_runner::runner::raw_runner;
use serde_json::json;

fn array_range_selector(c: &mut Criterion) {
    c.bench_function("Array range selector", move |b| {
        b.iter(|| raw_runner("[2,0]", &json!([0, 1, 2, 3, 4, 5, 6, 7, 8, 9])))
    });
}

fn flatten_operator(c: &mut Criterion) {
    c.bench_function("Flatten operator", move |b| {
        b.iter(|| raw_runner("..", &json!([[[[[[[0]]], 1, [[[[2]]]], 3]]]])))
    });
}

fn group_separator(c: &mut Criterion) {
    c.bench_function("Group separator", move |b| {
        b.iter(|| raw_runner(r#""a","b","c""#, &json!({ "a": 1, "b": 2, "c": 3 })))
    });
}

fn key_selector(c: &mut Criterion) {
    c.bench_function("Key selector", move |b| {
        b.iter(|| {
            raw_runner(
                r#""props""a""b""c""#,
                &json!({ "props": { "a": { "b": { "c": 1} } } }),
            )
        })
    });
}

fn pipe_operators(c: &mut Criterion) {
    c.bench_function("Pipe operators", move |b| {
        b.iter(|| {
            raw_runner(
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

criterion_group!(
    benches,
    array_range_selector,
    flatten_operator,
    group_separator,
    key_selector,
    pipe_operators,
);

criterion_main!(benches);
