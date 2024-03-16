use sha2::{Sha256, Digest};
use std::io::{self, Write};

fn main() {
    println!("Введите текст для генерации хеша SHA-256:");

    // Создаем буфер для хранения введенных данных
    let mut input = String::new();

    // Считываем данные из stdin
    io::stdin()
        .read_line(&mut input)
        .expect("Ошибка при чтении строки");

    // Убираем символ новой строки из ввода, если он есть
    let input = input.trim_end();

    // Инициализируем новый хеширующий объект
    let mut hasher = Sha256::new();

    // Добавляем введенные данные в хеширующий объект
    hasher.update(input);

    // Получаем хеш-значение
    let result = hasher.finalize();

    // Выводим результат в шестнадцатеричном формате
    println!("Хеш SHA-256 для \"{}\" это {:x}", input, result);
}
