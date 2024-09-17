use std::{
    collections::HashMap,
    fmt::Display,
    fs::File,
    io::{BufReader, Read},
};

#[cfg_attr(test, derive(Debug))]
#[derive(PartialEq, Eq)]
pub struct Schema(HashMap<String, String>);

impl Default for Schema {
    fn default() -> Self {
        Self::new(HashMap::new())
    }
}

impl Schema {
    pub fn new(hash_map: HashMap<String, String>) -> Self {
        Schema(hash_map)
    }

    // TODO: handle doubled schema error
    pub fn add_rule(mut self, key: impl ToString, value_type: impl ToString) -> Self {
        self.0.insert(key.to_string(), value_type.to_string());
        self
    }
}

impl Display for Schema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0
            .iter()
            .try_for_each(|(k, v)| writeln!(f, "{k} -> {v}"))
    }
}

pub fn schema_from_path(path: impl ToString) -> Schema {
    let file = File::open(path.to_string()).expect("file not found");
    let mut reader = BufReader::new(file);

    let mut buf = String::new();

    reader
        .read_to_string(&mut buf)
        .expect("Failed to read file");

    let lines = buf.split('\n').map(|s| s.to_string()).collect::<Vec<_>>();

    lines.into_iter().fold(Schema::default(), |acc, line| {
        let [key, value_type, ..] = line.splitn(2, "->").map(|x| x.trim()).collect::<Vec<_>>()[..]
        else {
            return acc;
        };
        acc.add_rule(key, value_type)
    })
}

#[cfg(test)]
mod schema_file_reader_tests {
    use super::*;

    #[test]
    fn success_to_read_schema_file() {
        let schema = schema_from_path("./test_data/test.schema");
        assert_eq!(
            schema,
            Schema::new(HashMap::from([
                ("endpoint".to_string(), "string".to_string()),
                ("debug".to_string(), "bool".to_string()),
                ("log.file".to_string(), "string".to_string())
            ]))
        );
    }
}
