use std::collections::HashMap;

pub enum ConfigValue {
    String(String),
    Number(i64),
    Boolean(bool),
    Object(HashMap<String, Box<ConfigValue>>),
    Null,
}

pub struct Config(ConfigValue);

impl Config {
    pub fn new() -> Self {
        Self(ConfigValue::Null)
    }

    pub fn set(&mut self, value: ConfigValue) {
        self.0 = value;
    }
}

pub fn config_from_lines(lines: Vec<impl ToString>) -> Config {
    lines.iter().fold(Config::new(), |acc, line| {
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
        set_config(acc, keys, value)
    })
}

fn set_config(mut config: Config, keys: Vec<&str>, value: &str) -> Config {
    let mut current = &mut config.0;

    for (i, &k) in keys.iter().enumerate() {
        if i == keys.len() - 1 {
            let parsed_value = config_value_from_str(value);
            match current {
                ConfigValue::Object(obj) => {
                    obj.insert(k.into(), Box::new(parsed_value));
                }
                _ => {
                    let mut obj = HashMap::new();
                    obj.insert(k.into(), Box::new(parsed_value));
                    *current = ConfigValue::Object(obj);
                }
            }
        } else {
            let obj = match current {
                ConfigValue::Object(obj) => obj,
                _ => {
                    let obj = HashMap::new();
                    *current = ConfigValue::Object(obj);
                    if let ConfigValue::Object(obj) = current {
                        obj
                    } else {
                        unreachable!()
                    }
                }
            };
            if !obj.contains_key(k) {
                obj.insert(k.into(), Box::new(ConfigValue::Object(HashMap::new())));
            }
            current = obj.get_mut(k).unwrap();
        }
    }
    return config;
}

fn config_value_from_str(s: &str) -> ConfigValue {
    match s {
        "true" => ConfigValue::Boolean(true),
        "false" => ConfigValue::Boolean(false),
        _ => {
            if let Ok(n) = s.parse::<i64>() {
                ConfigValue::Number(n)
            } else {
                ConfigValue::String(s.into())
            }
        }
    }
}
