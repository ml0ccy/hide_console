use hide_console::{hide_console, show_console, is_hide_console_supported};
use std::thread;
use std::time::Duration;
use std::io::{self, Write};

fn main() {
    // Check console hiding/showing support
    if !is_hide_console_supported() {
        println!("Console hiding/showing is not supported on this platform.");
        println!("This example will simply emulate the behavior.");
    }

    println!("Console Visibility Toggle Demonstration");
    println!("======================================");
    println!("The console will be hidden in 3 seconds...");
    
    // Display countdown text
    for i in (1..=3).rev() {
        print!("\rHiding in {} seconds...", i);
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs(1));
    }
    println!("\rHiding console...                 ");
    
    // Hide the console
    hide_console();
    
    // Background work continues (console is invisible)
    thread::sleep(Duration::from_secs(5));
    
    // Show the console
    show_console();
    
    println!("Console is visible again!");
    println!("This example demonstrates the ability to:");
    println!("1. Hide the console for background work");
    println!("2. Show the console when user input is needed");
    println!();
    println!("Press Enter to hide the console again...");
    
    // Wait for Enter key
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    println!("The console will be hidden for 3 seconds...");
    
    // Hide the console again
    hide_console();
    
    // Pause
    thread::sleep(Duration::from_secs(3));
    
    // Show the console again
    show_console();
    
    println!("Example completed!");
} 