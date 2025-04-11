# CSV to JSON Converter

A Rust-based CLI tool for converting CSV files to JSON format with data validation.

## Features

- Convert CSV files to JSON format
- Validate CSV data during conversion:
  - Name cannot be empty
  - Age must be greater than 0
  - Email must be valid (contains '@')
- Create and manage user records in JSON format

## Usage

### Converting CSV to JSON

```bash
cargo run -- convert --input example.csv --output output.json
```

### Creating a User

```bash
cargo run -- user create --name "John Doe" --age 30 --email "john@example.com"
```

## CSV Format

The input CSV file must contain the following columns:
- name
- age
- email

Example:
```csv
name,age,email
John Doe,30,john@example.com
Jane Smith,25,jane@example.com
```

## Future Features

- [ ] Add more data validation rules
- [ ] Support for different CSV formats
- [ ] User management commands (list, update, delete)
- [ ] Custom output formatting options
- [ ] Batch processing capabilities 