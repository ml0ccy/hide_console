# Hide Console

[![crates.io](https://img.shields.io/crates/v/hide_console.svg)](https://crates.io/crates/hide_console)
[![Documentation](https://docs.rs/hide_console/badge.svg)](https://docs.rs/hide_console)
[![MIT License](https://img.shields.io/crates/l/hide_console.svg)](https://github.com/ml0ccy/hide_console/blob/main/LICENSE)

A library for hiding console windows in Rust applications. Perfect for creating background applications or applications with graphical interface without visible console window.

## Features

- Hiding console window on Windows platform
- Cross-platform support (works safely on all platforms)
- Minimal dependencies
- Simple and clear API

## Documentation

Full documentation is available at [docs.rs/hide_console](https://docs.rs/hide_console).

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
hide_console = "0.1.0"
```

Or use the command:

```
cargo add hide_console
```

## Usage

### Basic Example

```rust
use hide_console::hide_console;

fn main() {
    // Perform necessary actions before hiding the console
    println!("This message will be visible");
    
    // Hide the console window
    hide_console();
    
    // Continue program execution without visible console
    println!("This message won't be visible in the console, but will be written to stdout");
}
```

### Checking Console Hiding Support

```rust
use hide_console::is_hide_console_supported;

fn main() {
    if is_hide_console_supported() {
        println!("Console hiding is supported on this platform");
    } else {
        println!("Console hiding is not supported on this platform");
    }
}
```

### Examples

The library contains several examples that you can run:

```
# Simple console hiding example
cargo run --example simple

# GUI application emulation
cargo run --example gui_emulation
```

## Platforms

- **Windows**: Full support for hiding console.
- **macOS, Linux, and others**: The `hide_console()` function doesn't perform any actions, but doesn't cause errors.

## How It Works

On Windows platform, the library uses WinAPI to hide the console window:

1. Gets the console window handle using `GetConsoleWindow()`
2. Hides the window using `ShowWindow()` with the `SW_HIDE` parameter

On other platforms, the `hide_console()` function simply returns without performing any actions.

## License

MIT

## Contributing

Contributions are welcome! Please submit pull requests or create issues on GitHub.

## Social Media

twitch.tv/mloccy_

t.me/mloccy