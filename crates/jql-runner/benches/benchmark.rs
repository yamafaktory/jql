use criterion::{
    criterion_group,
    criterion_main,
    Criterion,
};
use jql_runner::runner::raw_runner;
use serde_json::json;

// const DATA: &str = r#"{
//     "array": [1, 2, 3, 4 , 5 , 6, 7],
//     "flatten-array": [[[[["c", "a", "c"]]]], "g", [[["a", ["t"]]]]],
//     "props": { "a": { "b": { "c" : 777 } } },
//     "nested-filter": [
//         {
//             "laptop": {
//                 "brand": "Apple"
//             }
//         },
//         {
//             "laptop": {
//                 "brand": "Asus"
//             }
//         }
//     ]
// }"#;

fn access_properties_benchmark(c: &mut Criterion) {
    c.bench_function("Access properties", move |b| {
        b.iter(|| {
            raw_runner(
                r#""props"."a"."b"."c""#,
                &json!({ "props": { "a": { "b": { "c": 1} } } }),
            )
        })
    });
}

fn pipes_benchmark(c: &mut Criterion) {
    c.bench_function("Pipes", move |b| {
        b.iter(|| {
            raw_runner(
                r#""nested"|>"laptop""brand"<|[1]"#,
                &json!({ "nested": [
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
                ]}),
            )
        })
    });
}

// fn flatten_array_benchmark(c: &mut Criterion) {
//     let json: Value = serde_json::from_str(DATA).unwrap();
//     let selector = r#".."flatten-array""#;
//     c.bench_function("Flatten an array", move |b| {
//         b.iter(|| walker(&json, selector))
//     });
// }
//
// fn range_array_benchmark(c: &mut Criterion) {
//     let json: Value = serde_json::from_str(DATA).unwrap();
//     let selector = r#""array".[5:3]"#;
//     c.bench_function("Get the range of an array", move |b| {
//         b.iter(|| walker(&json, selector))
//     });
// }
//
// fn group_benchmark(c: &mut Criterion) {
//     let json: Value = serde_json::from_str(DATA).unwrap();
//     let selector = r#""array","flatten-array","props""#;
//     c.bench_function("Get multiple groups", move |b| {
//         b.iter(|| walker(&json, selector))
//     });
// }

criterion_group!(
    benches,
    access_properties_benchmark,
    pipes_benchmark,
    // flatten_array_benchmark,
    // range_array_benchmark,
    // group_benchmark
);

criterion_main!(benches);
