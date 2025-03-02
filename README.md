# Hide Console

[![crates.io](https://img.shields.io/crates/v/hide_console.svg)](https://crates.io/crates/hide_console)
[![Documentation](https://docs.rs/hide_console/badge.svg)](https://docs.rs/hide_console)
[![MIT License](https://img.shields.io/crates/l/hide_console.svg)](https://github.com/ml0ccy/hide_console/blob/main/LICENSE)

A library for hiding console windows in Rust applications. Perfect for creating background applications or applications with graphical interface without visible console window.

## Features

- Hiding console window on Windows platform
- Showing console window when needed
- Cross-platform support (works safely on all platforms)
- Minimal dependencies
- Simple and clear API

## Documentation

Full documentation is available at [docs.rs/hide_console](https://docs.rs/hide_console).

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
hide_console = "0.2.1"
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

### Showing and Hiding the Console

```rust
use hide_console::{hide_console, show_console};
use std::io;

fn main() {
    // Hide console at application start
    hide_console();
    
    // Do some work without visible console
    
    // When user interaction is needed, show console again
    show_console();
    println!("Please enter your name:");
    
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read input");
    
    println!("Hello, {}!", name.trim());
    
    // Hide console again for background work
    hide_console();
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

# Toggle console visibility
cargo run --example toggle_console
```

## Platforms

- **Windows**: Full support for hiding and showing console.
- **macOS, Linux, and others**: The functions don't perform any actions, but don't cause errors.

## How It Works

On Windows platform, the library uses WinAPI to hide and show the console window:

1. Gets the console window handle using `GetConsoleWindow()`
2. Hides or shows the window using `ShowWindow()` with the `SW_HIDE` or `SW_SHOW` parameter

On other platforms, the functions simply return without performing any actions.

## License

MIT

## Contributing

Contributions are welcome! Please submit pull requests or create issues on GitHub.

## Social Media

twitch.tv/mloccy_

t.me/mloccy