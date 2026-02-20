# AGENTS.md - Developer Guide for my_gpui_app

Native desktop GUI application using [gpui](https://github.com/zed-industries/gpui) + [gpui-component](https://longbridge.github.io/gpui-component/docs/getting-started).

## Project Overview

- **Framework**: gpui + gpui-component
- **Edition**: Rust 2024
- **Dependencies**: anyhow, gpui, gpui-component, gpui-component-assets

## Build, Lint, and Test Commands

### Building

```bash
cargo build              # Debug build
cargo build --release    # Release build
cargo run                # Run application
```

### Testing

```bash
cargo test                           # Run all tests
cargo test test_name                 # Run single test by name
cargo test -- --nocapture            # Show output
cargo test --doc                     # Doc tests
```

### Linting & Formatting

```bash
cargo clippy                         # Lint
cargo clippy -- -D warnings          # CI mode (warnings as errors)
cargo fmt                            # Format
cargo fmt -- --check                 # Check formatting
cargo fmt && cargo clippy            # Run both
```

### Checking

```bash
cargo check              # Check compilation
cargo doc --no-deps      # Generate docs
```

## Code Style Guidelines

### Imports

```rust
use std::sync::Arc;
use gpui::*;
use gpui_component::{
    button::{Button, ButtonGroup},
    table::{Column, Table, TableDelegate, TableState},
    *,
};
```

### Formatting

- Run `cargo fmt` before committing
- Use 4 spaces, lines under 100 chars
- Use trailing commas in multi-line collections

### Naming

- Functions/variables: `snake_case`
- Structs/Enums/Traits: `PascalCase`
- Constants: `SCREAMING_SNAKE_CASE`
- Files: `snake_case.rs`

### Types

- Prefer explicit types for public API
- Use `Arc<T>` for shared ownership
- Derive `Clone` only when necessary

### Error Handling

Use `anyhow` for application errors:

```rust
fn load_config() -> Result<Config, anyhow::Error> {
    let content = std::fs::read_to_string("config.json")?;
    let config: Config = serde_json::from_str(&content)
        .context("Failed to parse config")?;
    Ok(config)
}
```

### GUI Patterns (gpui)

- Implement `Render` trait for views
- Use `Entity<T>` for reactive state
- Pass `&mut Window` and `&mut Context<T>` to constructors
- Use `cx.new(|cx| ...)` for entity creation

```rust
impl Render for MyView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().p_4().child(
            Button::new("click-me").on_click(|_, cx| cx.notify())
        )
    }
}
```

- Pass references to gpui components: `Table::new(&self.table_state)`

### Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_something() { /* Arrange, Act, Assert */ }
}
```

## Common Tasks

```bash
cargo add <crate>     # Add dependency
cargo update          # Update dependencies
cargo clean           # Clean build artifacts
```

## CI/CD

```bash
cargo fmt -- --check && cargo clippy -- -D warnings && cargo test && cargo build --release
```
