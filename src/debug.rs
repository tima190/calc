
pub fn debugging(formule: &str) {
    let mut numbers: Vec<i32> = vec![]; // Массив для хранения чисел
    let mut chars = formule.chars().peekable(); // Создаем peekable-итератор

    if let Some(first_char) = chars.next() {
        if first_char == '!' as char {
            while let Some(c) = chars.next() {
                if c >= 'a' as char {
                    continue;
                } else if c == ' ' {
                    continue;
                } else {
                    if String::from(c) == "+" || String::from(c) == "-" || String::from(c) == "" || String::from(c) == "+" {

                    }
                    numbers.push(c as i32 - 48); // Заполняем массив числами
                    println!("symbol: {}", c); // Выводим символ

                    // Выводим следующий символ, если он есть
                    if let Some(next_char) = chars.peek() {
                        println!("next symbol: {}", next_char);
                    } else {
                        println!("next symbol: None");
                    }

                    println!("size: {}", numbers.len()); // Выводим текущий размер массива
                    {
                        let a = c as i32;
                        // Здесь вы можете использовать переменную `a`
                        println!("a = {}", a - 48);
                        println!("-----");
                    }
                }
            }
            println!("DEBUG numbers: {:?}", numbers); // Выводим массив чисел
        } else {
            // па преколу
        }
    }
}