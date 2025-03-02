use hide_console::hide_console;
use std::thread;
use std::time::Duration;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

// Эмуляция GUI-приложения без реальной графики
fn main() {
    println!("Запуск GUI-приложения...");
    println!("Сейчас будет скрыта консоль");
    thread::sleep(Duration::from_secs(2));
    
    // Скрываем консоль, чтобы приложение выглядело как полноценное GUI-приложение
    hide_console();
    
    // Флаг для управления работой приложения
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    
    // Обработчик сигнала завершения (Ctrl+C)
    ctrlc::set_handler(move || {
        println!("Получен сигнал завершения, закрываем приложение...");
        r.store(false, Ordering::SeqCst);
    }).expect("Ошибка при установке Ctrl-C обработчика");
    
    // Эмуляция обработки событий в GUI-приложении
    println!("GUI-приложение запущено в фоновом режиме");
    println!("Для завершения работы нажмите Ctrl+C (если консоль видна) или завершите процесс через диспетчер задач");
    
    let mut counter = 0;
    while running.load(Ordering::SeqCst) {
        // Эмуляция обработки событий GUI
        counter += 1;
        if counter % 10 == 0 {
            println!("Обработано {} событий", counter);
        }
        
        // Пауза между итерациями цикла событий
        thread::sleep(Duration::from_millis(500));
    }
    
    println!("GUI-приложение завершило работу");
} 