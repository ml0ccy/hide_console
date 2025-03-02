/*!
 * # Hide Console
 * 
 * A library for hiding console windows in Rust applications.
 * 
 * Main purpose - creating background applications or
 * applications with graphical interface without visible console window.
 * 
 * ## Features
 * 
 * - Hiding console window on Windows platform
 * - Showing console window back when needed
 * - Cross-platform support (works safely on all platforms)
 * - Minimal dependencies
 * - Simple and clear API
 * 
 * ## Usage Example
 * 
 * ```rust
 * use hide_console::hide_console;
 * 
 * fn main() {
 *     // Hide console window
 *     hide_console();
 *     
 *     // Continue application execution...
 *     println!("Console is hidden, but this text will be written to stdout");
 * }
 * ```
 * 
 * ## Show/Hide Console Example
 * 
 * ```rust
 * use hide_console::{hide_console, show_console};
 * use std::thread;
 * use std::time::Duration;
 * 
 * fn main() {
 *     // Hide console window
 *     hide_console();
 *     
 *     // Do some work without visible console
 *     thread::sleep(Duration::from_secs(5));
 *     
 *     // Show console when user interaction needed
 *     show_console();
 *     println!("Please check the information and press Enter to continue...");
 *     let mut input = String::new();
 *     std::io::stdin().read_line(&mut input).unwrap();
 *     
 *     // Hide console again
 *     hide_console();
 * }
 * ```
 * 
 * ## Compatibility with GUI Libraries
 * 
 * This library can be used with popular GUI frameworks:
 * 
 * ```rust
 * use hide_console::hide_console;
 * 
 * fn main() {
 *     // Hide console before GUI initialization
 *     hide_console();
 *     
 *     // Your GUI framework initialization code goes here
 *     // ...
 * }
 * ```
 * 
 * ## Platforms
 * 
 * - **Windows**: Full support for hiding and showing console
 * - **macOS, Linux and others**: Functions do nothing but don't cause errors
 */

extern crate cfg_if;

#[cfg(windows)]
extern crate winapi;

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(windows)] {
        use winapi::um::wincon::GetConsoleWindow;
        use winapi::um::winuser::{ShowWindow, SW_HIDE, SW_SHOW};
    }
}

/// Hides the application's console window.
///
/// This function allows hiding the console window in Windows applications.
/// Especially useful when creating applications with graphical interface or background services,
/// where the console window only interferes with the user.
///
/// # Platforms
///
/// - **Windows**: Function is fully supported and hides the console window.
/// - **Non-Windows**: On other platforms, the function does not perform any actions.
///
/// # Examples
///
/// Simple usage:
/// ```
/// use hide_console::hide_console;
///
/// // Hide the console window
/// hide_console();
/// ```
///
/// With platform support checking:
/// ```
/// use hide_console::{hide_console, is_hide_console_supported};
///
/// if is_hide_console_supported() {
///     println!("Console will be hidden");
///     hide_console();
/// } else {
///     println!("Hiding console is not supported on this platform");
/// }
/// ```
///
/// # Technical Details
///
/// On Windows, the function uses WinAPI calls:
/// 1. `GetConsoleWindow()` to get the console window handle
/// 2. `ShowWindow(window, SW_HIDE)` to hide the window
///
/// # Safety
///
/// The function uses unsafe code to call WinAPI,
/// but ensures safe interaction with the operating system API.
pub fn hide_console() {
    cfg_if! {
        if #[cfg(windows)] {
            unsafe {
                let window = GetConsoleWindow();
                if !window.is_null() {
                    ShowWindow(window, SW_HIDE);
                }
            }
        } else {
            // On non-Windows platforms, function does nothing
        }
    }
}

/// Checks if hiding the console is supported on the current platform.
///
/// This function allows an application to determine whether the
/// `hide_console()` function will work on the current platform.
///
/// # Return Value
///
/// - `true` - if hiding console is supported (Windows)
/// - `false` - if hiding console is not supported (non-Windows)
///
/// # Examples
///
/// Basic usage:
/// ```
/// use hide_console::is_hide_console_supported;
///
/// if is_hide_console_supported() {
///     println!("Hiding console is supported on this platform");
/// } else {
///     println!("Hiding console is not supported on this platform");
/// }
/// ```
///
/// Conditional code execution:
/// ```
/// use hide_console::{hide_console, is_hide_console_supported};
///
/// fn main() {
///     // Perform different actions depending on support
///     if is_hide_console_supported() {
///         // Code for Windows
///         hide_console();
///         println!("Console is hidden");
///     } else {
///         // Code for other platforms
///         println!("Running in console mode");
///     }
/// }
/// ```
pub fn is_hide_console_supported() -> bool {
    cfg_if! {
        if #[cfg(windows)] {
            true
        } else {
            false
        }
    }
}

/// Shows the application's console window if it was previously hidden.
///
/// This function allows showing the console window in Windows applications.
/// Useful when you need to temporarily display the console to show information 
/// to the user and then hide it again.
///
/// # Platforms
///
/// - **Windows**: Function is fully supported and shows the console window.
/// - **Non-Windows**: On other platforms, the function does not perform any actions.
///
/// # Examples
///
/// Simple usage:
/// ```
/// use hide_console::{hide_console, show_console};
///
/// // Hide the console window
/// hide_console();
/// 
/// // Do some work...
/// 
/// // Show console again when needed
/// show_console();
/// ```
///
/// With platform support checking:
/// ```
/// use hide_console::{show_console, is_hide_console_supported};
///
/// if is_hide_console_supported() {
///     println!("Console will be shown");
///     show_console();
/// } else {
///     println!("Showing console is not supported on this platform");
/// }
/// ```
///
/// # Technical Details
///
/// On Windows, the function uses WinAPI calls:
/// 1. `GetConsoleWindow()` to get the console window handle
/// 2. `ShowWindow(window, SW_SHOW)` to show the window
///
/// # Safety
///
/// The function uses unsafe code to call WinAPI,
/// but ensures safe interaction with the operating system API.
pub fn show_console() {
    cfg_if! {
        if #[cfg(windows)] {
            unsafe {
                let window = GetConsoleWindow();
                if !window.is_null() {
                    ShowWindow(window, SW_SHOW);
                }
            }
        } else {
            // On non-Windows platforms, function does nothing
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_hide_console_supported() {
        // Just make sure the function executes without errors
        let result = is_hide_console_supported();
        
        // Result will depend on the platform
        #[cfg(windows)]
        assert_eq!(result, true);
        
        #[cfg(not(windows))]
        assert_eq!(result, false);
    }
    
    #[test]
    fn test_hide_show_console() {
        // Test that both functions execute without errors
        hide_console();
        show_console();
        
        // We can't easily verify window visibility in an automated test,
        // but we ensure that the functions don't panic
    }
}
