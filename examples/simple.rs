use hide_console::{hide_console, is_hide_console_supported};
use std::thread;
use std::time::Duration;

fn main() {
    println!("This message will be visible before hiding the console");
    
    println!("Checking console hiding support: {}", 
        if is_hide_console_supported() { "Supported" } else { "Not supported" });
    
    // Hiding the console window after 2 seconds delay
    thread::sleep(Duration::from_secs(2));
    
    hide_console();
    
    // This text will be written to stdout, but won't be visible in the console
    println!("Console is hidden!");
    println!("The program will continue running for 5 more seconds...");
    
    // Continue working in background mode...
    thread::sleep(Duration::from_secs(5));
    
    println!("Program completed!");
} 