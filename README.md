# Hide Console

A library for hiding console windows in Rust applications. Perfect for creating background applications or GUI applications without visible console windows.

## Features

- Hide console windows on Windows platform
- Cross-platform support (safely works on all platforms)
- Minimal dependencies
- Simple and intuitive API

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
    
    // Continue running without visible console
    println!("This message won't be visible in the console, but will be written to stdout");
}
```

### Check for Console Hiding Support

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

The library includes several examples that you can run:

```
# Simple console hiding example
cargo run --example simple

# GUI application emulation
cargo run --example gui_emulation
```

## Platforms

- **Windows**: Full support for console hiding.
- **macOS, Linux, and others**: The hide_console() function doesn't perform any actions but doesn't cause errors.

## How It Works

On Windows platform, the library uses WinAPI to hide the console window:

1. Gets the console window handle using `GetConsoleWindow()`
2. Hides the window using `ShowWindow()` with the `SW_HIDE` parameter

On other platforms, the `hide_console()` function simply returns without performing any actions.

## License

MIT

## Contributing

Contributions are welcome! Please send pull requests or create issues on GitHub.

## Social Media

twitch.tv/mloccy_

t.me/mloccy
