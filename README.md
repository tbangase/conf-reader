# Conf file Reader

This library is used to read configuration files in the format of key-value pairs such as the `sysctl.conf` file.
The library supports reading from a file or lines of the data.

# Usage

To run a simple CLI using the library, you can use the following command:

```
cargo run -- -c <config_file_path> -s <schema_file_path>
```

There is simple test data file: `test_data/file_read_test.conf`, and simple schema file: `test_data/test.schema`.
So the fastest way to test the library is to run the following command:

```
cargo run -- -c test_data/file_read_test.conf -s test_data/test.schema
```
