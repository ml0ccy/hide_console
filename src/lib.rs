/*!
 * # Hide Console
 * 
 * Библиотека для скрытия консольного окна в приложениях Rust.
 * 
 * Основное предназначение - создание фоновых приложений или
 * приложений с графическим интерфейсом без видимого консольного окна.
 * 
 * ## Пример использования
 * 
 * ```rust
 * use hide_console::hide_console;
 * 
 * fn main() {
 *     // Скрыть консольное окно
 *     hide_console();
 *     
 *     // Продолжить выполнение приложения...
 *     println!("Консоль скрыта, но этот текст будет записан в stdout");
 * }
 * ```
 */

extern crate cfg_if;

#[cfg(windows)]
extern crate winapi;

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(windows)] {
        use winapi::um::wincon::GetConsoleWindow;
        use winapi::um::winuser::{ShowWindow, SW_HIDE};
    }
}

/// Скрывает консольное окно приложения.
///
/// # Платформы
///
/// - **Windows**: Функция полностью поддерживается и скрывает консольное окно.
/// - **Не Windows**: На других платформах функция не выполняет никаких действий.
///
/// # Примеры
///
/// ```
/// use hide_console::hide_console;
///
/// // Скрыть консольное окно
/// hide_console();
/// ```
///
/// # Безопасность
///
/// Функция использует unsafe код для вызова WinAPI, 
/// но обеспечивает безопасное взаимодействие с API операционной системы.
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
            // На не-Windows платформах функция ничего не делает
        }
    }
}

/// Проверяет, поддерживается ли скрытие консоли на текущей платформе.
///
/// # Возвращаемое значение
///
/// - `true` - если скрытие консоли поддерживается (Windows)
/// - `false` - если скрытие консоли не поддерживается (не Windows)
///
/// # Примеры
///
/// ```
/// use hide_console::is_hide_console_supported;
///
/// if is_hide_console_supported() {
///     println!("Скрытие консоли поддерживается на этой платформе");
/// } else {
///     println!("Скрытие консоли не поддерживается на этой платформе");
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_hide_console_supported() {
        // Просто убедимся, что функция выполняется без ошибок
        let result = is_hide_console_supported();
        
        // Результат будет зависеть от платформы
        #[cfg(windows)]
        assert_eq!(result, true);
        
        #[cfg(not(windows))]
        assert_eq!(result, false);
    }
}
