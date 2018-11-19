use rayon::prelude::*;
use serde_json::json;
use serde_json::Value;

/// Flatten nested arrays.
pub fn flatten_array(array: &[Value]) -> Vec<Value> {
    fn deepen(value: Value) -> Value {
        // let mut r: Vec<Value> = Vec::new();
        // let t =
        //     value
        //         .as_array()
        //         .unwrap()
        //         .clone()
        //         .iter()
        //         .for_each(move |inner_value| {
        //             if inner_value.is_array() {
        //                 deepen(inner_value.clone());
        //             } else {
        //                 println!("======>    {:?}", inner_value.clone());
        //                 r.push(inner_value.clone());
        //                 //inner_value.clone()
        //             }
        //         });
        // let mut acc: Vec<Value> = Vec::new();
        // for inner_value in value.as_array().unwrap() {
        //     if inner_value.is_array() {
        //         println!("Called again");
        //         deepen(inner_value.clone());
        //     } else {
        //         println!("=> {:?}", inner_value.clone());
        //         acc.push(inner_value.clone());
        //         //inner_value.clone()
        //     }
        // }
        // println!("ACC: {:?}", acc);

        let t: Vec<Value> = value
            .as_array()
            .unwrap()
            .par_iter()
            .fold(
                || Vec::new(),
                |mut acc: Vec<Value>, inner_value: &Value| {
                    if inner_value.is_array() {
                        let deep = deepen(inner_value.clone());
                        if deep.is_array() {
                            acc.append(&mut deep.as_array().unwrap().clone());
                        } else {
                            acc.push(inner_value.clone());
                        }
                        // println!("acc: {:?}", acc);
                        acc
                    } else {
                        // println!("=> {:?}", inner_value.clone());
                        acc.push(inner_value.clone());
                        acc
                    }
                },
            ).flatten()
            .collect();

        json!(t)
    }

    array
        .iter()
        .map(|value| {
            if value.is_array() {
                println!("{:?}", deepen(value.clone()));
                deepen(value.clone())
            } else {
                println!("nope");
                value.clone()
            }
        }).collect::<Vec<Value>>()
}
