use hide_console::hide_console;
use std::thread;
use std::time::Duration;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

// GUI application emulation without real graphics
fn main() {
    println!("Starting GUI application...");
    println!("The console will be hidden now");
    thread::sleep(Duration::from_secs(2));
    
    // Hide the console to make the application look like a full-fledged GUI application
    hide_console();
    
    // Flag to control application execution
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    
    // Termination signal handler (Ctrl+C)
    ctrlc::set_handler(move || {
        println!("Termination signal received, closing application...");
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting up Ctrl-C handler");
    
    // Emulating event processing in a GUI application
    println!("GUI application is running in background mode");
    println!("To exit, press Ctrl+C (if console is visible) or terminate the process through task manager");
    
    let mut counter = 0;
    while running.load(Ordering::SeqCst) {
        // Emulating GUI event processing
        counter += 1;
        if counter % 10 == 0 {
            println!("Processed {} events", counter);
        }
        
        // Pause between event loop iterations
        thread::sleep(Duration::from_millis(500));
    }
    
    println!("GUI application has terminated");
} 