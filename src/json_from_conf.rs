use serde_json::{json, Value};

/// Parse separated lines from configuration file to json value.
/// Usage:
/// ```
/// use serde_json::json;
/// use conf_reader::json_from_conf;
///
/// let lines = vec![
///    "endpoint = localhost:3000",
///    "debug = true",
///    "log.file = /var/log/console.log",
///    "log.name = default.log"
///    ];
/// assert_eq!(json_from_conf(lines), json!({
///    "endpoint": "localhost:3000",
///    "debug": true,
///    "log": {
///    "file": "/var/log/console.log",
///    "name": "default.log",
///    },
/// }));
pub fn json_from_conf(lines: Vec<impl ToString>) -> Value {
    lines.iter().fold(json!({}), |acc, line| {
        let line = line.to_string();

        // Comment or empty line will be skipped
        if line.starts_with('#') || line.starts_with(';') || line.trim().is_empty() {
            return acc;
        }

        // Get key and value from line
        let [key, value, ..] = line.splitn(2, '=').map(|x| x.trim()).collect::<Vec<&str>>()[..]
        else {
            return acc;
        };
        let keys: Vec<&str> = key.split('.').collect();

        // Parse to json value
        set_json(acc, keys, value)
    })
}

fn set_json(mut json_value: Value, keys: Vec<&str>, value: &str) -> Value {
    let mut current = &mut json_value;

    for (i, &k) in keys.iter().enumerate() {
        if i == keys.len() - 1 {
            let parsed_value = value_from_str(value);
            current[k] = parsed_value;
        } else {
            if !current.get(k).is_some() {
                current[k] = json!({});
            }
            current = current.get_mut(k).unwrap();
        }
    }
    return json_value;
}

fn value_from_str(s: &str) -> Value {
    match s {
        "true" => Value::Bool(true),
        "false" => Value::Bool(false),
        _ => {
            if let Ok(n) = s.parse::<i64>() {
                Value::Number(n.into())
            } else {
                Value::String(s.into())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use serde_json::json;

    use super::*;

    #[rstest]
    #[case::case_1_simple_case_with_nested(vec![
        "endpoint = localhost:3000",
        "debug = true",
        "log.file = /var/log/console.log",
    ],
    json!({
        "endpoint": "localhost:3000",
        "debug": true,
        "log": {
            "file": "/var/log/console.log",
        },
    }))]
    #[case::case_2_skipped_comment_out_and_double_nested(vec![
        "endpoint = localhost:3000",
        "# debug = true",
        "; This is also a comment",
        "log.file = /var/log/console.log",
        "log.name = default.log"
    ],
    json!({
        "endpoint": "localhost:3000",
        "log": {
            "file": "/var/log/console.log",
            "name": "default.log",
        },
    }))]
    #[case::case_3_deep_nested(vec![
        "endpoint = localhost:3000",
        "log.file = /var/log/console.log",
        "revalidate = 60",
        "-this.will.not.fail = true",
    ],
    json!({
        "endpoint": "localhost:3000",
        "log": {
            "file": "/var/log/console.log",
        },
        "revalidate": 60,
        "-this": {
            "will": {
                "not": {
                    "fail": true,
                },
            },
        },
    }))]
    fn success_test(#[case] lines: Vec<impl ToString>, #[case] expected: Value) {
        let res = json_from_conf(lines);
        assert_eq!(res, expected);
    }
}
