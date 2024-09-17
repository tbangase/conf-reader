use std::{
    fs::File,
    io::{BufReader, Read},
};

/// Read lines from a file from relative path from executed path.
/// Returned value is a vector of strings, each string is a line from the file, possible to be empty.
/// TODO: Panics: If file is not found or failed to read file.
pub fn lines_from_path(path: impl ToString) -> Vec<String> {
    let file = File::open(path.to_string()).expect("file not found");
    let mut reader = BufReader::new(file);

    let mut buf = String::new();

    reader
        .read_to_string(&mut buf)
        .expect("Failed to read file");

    buf.split('\n').map(|s| s.to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_lines() {
        let lines = lines_from_path("./test_data/file_read_test.conf");
        assert_eq!(lines[0], "endpoint = localhost:3000");
        assert_eq!(lines[1], "debug = true");
        assert_eq!(lines[2], "log.file = /var/log/console.log");
    }
}
