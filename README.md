# ggtools

A lightweight Rust utility library for common programming tasks. Simplifies file operations, configuration management, and AI integration.

## Features

- **Files** — Simple file reading and creation
- **Config** — Multi-format configuration support (JSON, TOML, YAML)
- **AI** — Easy integration with Ollama API

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
ggtools = { git = "https://github.com/i3667563-dot/ggtools" }
```

## Usage

### Files

```rust
use ggtools::files::{read, create, read_bytes};

// Read file contents
let content = read("example.txt")?;

// Create empty file
create("new_file.txt")?;

// Read as bytes
let bytes = read_bytes("data.bin")?;
```

### Config

```rust
use ggtools::config::{read_json, write_json, read_auto, write_auto};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Config {
    name: String,
    value: i32,
}

// Read JSON
let config: Config = read_json("config.json")?;

// Write JSON
write_json("config.json", &config)?;

// Auto-detect format by extension
let data = read_auto("settings.toml")?;
write_auto("settings.yaml", &config)?;
```

Supported formats: `JSON`, `TOML`, `YAML`.

### AI (Ollama)

```rust
use ggtools::ai::prompt;

// Simple prompt
let response = prompt("llama2", "Explain Rust in one sentence")?;
println!("{}", response);
```

For custom Ollama URL:

```rust
use ggtools::ai::prompt_to;

let response = prompt_to("http://localhost:11434", "llama2", "Hello!")?;
```

**Note:** Requires Ollama running locally. Default endpoint: `http://localhost:11434`

## Error Handling

All functions return `Result<T, io::Error>` for predictable error handling:

```rust
match read("file.txt") {
    Ok(content) => println!("{}", content),
    Err(e) => eprintln!("Error: {}", e),
}
```

## License

MIT
