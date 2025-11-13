# json_parser

Minimal, dependency-free JSON parser in Rust.

Overview

- Parses JSON strings into a simple `Value` enum.
- No external dependencies.

Install (local path)

Add to your `Cargo.toml`:

```toml
[dependencies]
json_parser = { path = "path/to/json_parser" }
```

Quick usage

```rust
use json_parser::Value;

let json = r#"{"name":"Alice","age":30}"#;
let v = Value::from_str(json).unwrap();
println!("{:?}", v);
```

Value enum

```rust
pub enum Value {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Value>),
    Object(std::collections::HashMap<String, Value>),
}
```

Key function

- `Value::from_str(input: &str) -> Result<Value, String>` — parse input into `Value` or return an error string.

Errors

Returns short, descriptive strings (e.g. "Unexpected end of input", "Unterminated string", "Expected ':' after object key").

Testing

Run the test suite:

```bash
cargo test
```

Limitations

- Numbers use `f64` (possible precision loss for very large integers).
- No streaming API; input is parsed in-memory.
- No serializer (Value → JSON string).

License

MIT
