use serde_json::Value;
use types::Selection;

pub fn walker(
    json: &Value,
    selector: Option<&str>,
) -> Option<Selection> {
    let mut inner_json = json;
    if let Some(selector) = selector {
        let selector: Vec<&str> = selector.split('.').collect();
        // Returns Result of values or Err early on, stopping the iteration.
        let items: Selection = selector
            .iter()
            .enumerate()
            .map(|(i, s)| -> Result<Value, String> {
                if let Ok(index) = s.parse::<isize>() {
                    if (index).is_negative() {
                        Err(String::from("Invalid negative array index"))
                    } else if inner_json[index as usize] == Value::Null {
                        let error_message = match inner_json.as_array() {
                            Some(array) => [
                                "Index (",
                                s,
                                ") is out of bound, node (",
                                selector[i - 1],
                                ") has a length of",
                                &(array.len()).to_string(),
                            ]
                                .join(" "),
                            // Trying to access an index on a node which is not
                            // an array.
                            None => {
                                ["Node (", selector[i - 1], ") is not an array"]
                                    .join(" ")
                            }
                        };
                        println!("# {:?} #", inner_json.as_array());
                        Err(error_message)
                    } else {
                        inner_json = &inner_json[index as usize];
                        Ok(inner_json.clone())
                    }
                } else if s.is_empty() {
                    Err(String::from("Unterminated selector found"))
                } else if inner_json[s] == Value::Null {
                    if i == 0 {
                        Err(["Node (", s, ") is not the root element"]
                            .join(" "))
                    } else {
                        Err([
                            "Node (",
                            s,
                            ") not found on parent (",
                            selector[i - 1],
                            ")",
                        ]
                            .join(" "))
                    }
                } else {
                    inner_json = &inner_json[s];
                    Ok(inner_json.clone())
                }
            }).collect();
        Some(items)
    } else {
        None
    }
}
