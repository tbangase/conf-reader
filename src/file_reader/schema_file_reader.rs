use std::{
    fs::File,
    io::{BufReader, Read},
};

use thiserror::Error;

use crate::{Schema, SchemaError};

#[derive(Debug, Error)]
pub enum SchemaFromPathError {
    #[error(transparent)]
    SchemaError(#[from] SchemaError),
    #[error("Failed to read file: {0}")]
    FailToReadFile(String),
}

pub fn schema_from_path(path: impl ToString) -> Result<Schema, SchemaFromPathError> {
    let file = File::open(path.to_string()).expect("file not found");
    let mut reader = BufReader::new(file);

    let mut buf = String::new();

    reader
        .read_to_string(&mut buf)
        .expect("Failed to read file");

    let lines = buf.split('\n').map(|s| s.to_string()).collect::<Vec<_>>();

    Ok(lines.into_iter().try_fold(Schema::default(), |acc, line| {
        let [key, value_type, ..] = line.splitn(2, "->").map(|x| x.trim()).collect::<Vec<_>>()[..]
        else {
            return Ok(acc);
        };
        acc.add_rule(key, value_type)
    })?)
}

#[cfg(test)]
mod schema_file_reader_tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn success_to_read_schema_file() {
        let schema = schema_from_path("./test_data/test.schema").unwrap();
        assert_eq!(
            schema,
            Schema::new(HashMap::from([
                (
                    "endpoint".to_string(),
                    "String".to_string().parse().unwrap()
                ),
                ("debug".to_string(), "Bool".to_string().parse().unwrap()),
                (
                    "log.file".to_string(),
                    "String".to_string().parse().unwrap()
                )
            ]))
        );
    }
}
