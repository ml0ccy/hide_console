use hide_console::{hide_console, is_hide_console_supported};
use std::thread;
use std::time::Duration;

fn main() {
    println!("Это сообщение будет видно до скрытия консоли");
    
    println!("Проверка поддержки скрытия консоли: {}", 
        if is_hide_console_supported() { "Поддерживается" } else { "Не поддерживается" });
    
    // Скрываем консольное окно после 2 секунд ожидания
    thread::sleep(Duration::from_secs(2));
    
    hide_console();
    
    // Этот текст будет записан в stdout, но не будет виден в консоли
    println!("Консоль скрыта!");
    println!("Программа будет работать еще 5 секунд...");
    
    // Продолжаем работу в фоновом режиме...
    thread::sleep(Duration::from_secs(5));
    
    println!("Программа завершена!");
} 