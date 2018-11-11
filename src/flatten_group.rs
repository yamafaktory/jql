use serde_json::json;
use serde_json::Value;
use types::Selection;

// fn fwe(value: Value) -> String {
//     if value.is_array() {
//         value.as_array().unwrap().iter().for_each(|inner_value| {
//             println!("{:?}", inner_value);
//         });
//     } else {
//         value
//     }
// }

/// Flatten nested arrays.
pub fn flatten_group(selection: Selection) -> Selection {
    match selection {
        Ok(json) => {
            let result: Vec<Value> = json.iter().fold(
                Vec::with_capacity(json.len()),
                |mut acc, value| {
                    if value.is_array() {
                        value.as_array().unwrap().iter().for_each(|i| {
                            if i.is_array() {
                                i.as_array().unwrap().iter().for_each(|j| {
                                    println!("{:?}", j);
                                    acc.push(j.clone());
                                });
                            } else {
                                println!("asdasd");
                                acc.push(i.clone());
                            }
                        });
                        println!("-== {:?}", acc);
                        acc
                    } else {
                        println!("nope");
                        acc.push(value.clone());
                        acc
                    }
                },
            );

            Ok(result)
        }
        Err(error) => Err(error),
    }
}
