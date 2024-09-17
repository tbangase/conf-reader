use serde_json::{json, Value};
use thiserror::Error;

use crate::Schema;

#[derive(Error, Debug)]
pub enum JsonFromConfError {
    #[error("Config not match to the Schema: {0}")]
    NotMatchToTheSchema(String),
}

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
/// assert_eq!(json_from_conf(lines, None), json!({
///    "endpoint": "localhost:3000",
///    "debug": true,
///    "log": {
///    "file": "/var/log/console.log",
///    "name": "default.log",
///    },
/// }));
/// ```
pub fn json_from_conf(
    lines: Vec<impl ToString>,
    schema: Option<Schema>,
) -> Result<Value, JsonFromConfError> {
    lines.iter().try_fold(json!({}), |acc, line| {
        let line = line.to_string();

        // Comment or empty line will be skipped
        if line.starts_with('#') || line.starts_with(';') || line.trim().is_empty() {
            return Ok(acc);
        }

        // Get key and value from line
        let [key, value, ..] = line.splitn(2, '=').map(|x| x.trim()).collect::<Vec<_>>()[..] else {
            return Ok(acc);
        };
        let key_path: Vec<&str> = key.split('.').collect();

        if let Some(schema) = &schema {
            if !schema.is_valid(&key_path, &value_from_str(value)) {
                return Err(JsonFromConfError::NotMatchToTheSchema(format!(
                    "key: {key}, value: {value}"
                )));
            }
        }

        // Parse to json value
        Ok(set_json(acc, key_path, value))
    })
}

fn set_json(mut json_value: Value, key_path: Vec<&str>, value: &str) -> Value {
    let mut current = &mut json_value;

    for (i, &k) in key_path.iter().enumerate() {
        if i == key_path.len() - 1 {
            let parsed_value = value_from_str(value);
            current[k] = parsed_value;
        } else {
            if current.get(k).is_none() {
                current[k] = json!({});
            }
            current = current.get_mut(k).unwrap();
        }
    }
    json_value
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
    use std::collections::HashMap;

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
    fn conf_to_json_test(#[case] lines: Vec<impl ToString>, #[case] expected: Value) {
        let res = json_from_conf(lines, None);
        assert_eq!(res.unwrap(), expected);
    }

    #[rstest]
    #[case::case_1_simple_case_with_nested(vec![
        "endpoint = localhost:3000",
        "debug = true",
        "log.file = /var/log/console.log",
    ],
    Schema::new(HashMap::from([
        ("endpoint".to_string(), "String".to_string().parse().unwrap()),
        ("debug".to_string(), "Bool".to_string().parse().unwrap()),
        ("log.file".to_string(), "String".to_string().parse().unwrap()),
    ])))]
    #[should_panic]
    #[case::case_2_wrong_type(vec![
        "debug = true",
    ],
    Schema::new(HashMap::from([
        ("debug".to_string(), "String".to_string().parse().unwrap()),
    ])))]
    fn schema_validation_test(#[case] lines: Vec<impl ToString>, #[case] schema: Schema) {
        json_from_conf(lines, Some(schema)).unwrap();
    }
}
