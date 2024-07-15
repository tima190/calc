use colored::Colorize; // Импортируем библиотеку для работы с цветами
use std::io::{self, Write}; // Импортируем библиотеки для работы с вводом и выводом

//import
mod debug; // Импортируем модуль debug.rs
use crate::debug::debugging;

static mut DEBUG_MODE: bool = false;

fn main() {
    println!(
        "{}, ver: {}, by: {} ; print 'H' to see help ; to exit print 'Q'",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION").bold().cyan(),
        env!("CARGO_PKG_AUTHORS").bold().cyan()
    );
    begin(); // запуск кода
}

fn begin() {
    let prefix = ">> "; // Префикс для ввода
    print!("{}", prefix); // Выводим префикс
    io::stdout().flush().unwrap(); // Принудительный вывод префикса (необходимо для корректного отображения префикса)

    let mut input = String::new(); // Строка для ввода

    io::stdin() // Ввод с клавиатуры
        .read_line(&mut input)
        .expect("Failed to read line");

    

    if input.chars().nth(0).unwrap() == 'H' || input.chars().nth(0).unwrap() == 'h' {
        // Выводим помощь
        println!(
            "{}\n{}\n{}\n{}\n{}\n{}\n{}",
            "calc help".bold().red(),
            "'!' = debug mode".italic(),
            "'+' = addition".italic(),
            "'-' = subtraction".italic(),
            "'*' = multiplication".italic(),
            "'/' = division".italic(),
            "'(' or ')' = bracket".italic()
        );
        begin()
    }

    if input.trim().is_empty() {
        std::process::exit(0); // Выход
    }
    else {
        println!("You entered: {}", input.trim());
        debugging(&input.trim()); // Вызываем функцию дебаг
        calculate(formatting(&input.trim())); // Вызываем функцию вычисления
        unsafe {
            DEBUG_MODE = false;
        }
        let_start_again()
    }
}

fn formatting(formule: &str) -> Vec<String> {
    if formule == "exit" || formule == "quit" || formule == "q" || formule == "Q" {
        println!("{}", "Bye!".bold().on_green());
        std::process::exit(0); // Выход
    }

    let mut stack: Vec<String> = vec![String::from("")]; // Массив для хранения чисел в виде строк
    let mut dot_count = 0; // Счетчик точек в текущем числе
    let mut last_char = ' '; // Переменная для отслеживания последнего символа

    for c in formule.chars() {
        if c.is_digit(10) {
            // Если символ является цифрой
            stack.last_mut().unwrap().push(c);
        } else if c == '.' {
            // Если символ является точкой
            if dot_count > 0 {
                // Если уже есть одна точка в текущем числе, это ошибка
                println!(
                    "{}",
                    "Error: Invalid number format with multiple decimal points."
                        .bold()
                        .red()
                );
                begin();
            } else {
                stack.last_mut().unwrap().push(c);
                dot_count += 1; // Увеличиваем счетчик точек
            }
        } else if c == '+' || c == '*' || c == '/' || c == '(' || c == ')' || c == '^'{
            // Если символ является оператором (кроме '-')
            stack.push(String::from(c)); // Добавляем оператор в стек
            stack.push(String::from("")); // Добавляем пустую строку в стек
            dot_count = 0; // Сбрасываем счетчик точек при переходе к новому числу
        } else if c == '-' {
            // Если символ является минусом
            if last_char == ' ' || last_char == '(' || "+-*/(".contains(last_char) {
                // Минус как часть числа (отрицательное число)
                stack.last_mut().unwrap().push(c);
            } else {
                // Минус как оператор
                stack.push(String::from(c)); // Добавляем оператор в стек
                stack.push(String::from("")); // Добавляем пустую строку в стек
            }
            dot_count = 0; // Сбрасываем счетчик точек при переходе к новому числу
        } else if c == ' ' {
            // Если символ является пробелом
            continue;
        } else if c == '!' {
            unsafe { DEBUG_MODE = true }
        } else {
            println!("{}", "Error: Invalid characters entered!".bold().red());
            begin();
        }

        last_char = c; // Обновляем последний символ
    }

    if formule.chars().nth(0).unwrap() == '!' {
        println!("DEBUG: {:?}", formule);
        println!("DEBUG len: {:?}", formule.len());
    }

    stack // спасибо чату жпт что подсказал что надо поставить эту хуйню :D
}


fn calculate(formula: Vec<String>) -> f32 {
    let mut values: Vec<f32> = Vec::new(); // стек чисел
    let mut operators: Vec<String> = Vec::new(); // стек операторов

    let mut i = 0; //счетчик
    while i < formula.len() {
        let token = &formula[i]; // число или оператор
        if let Ok(num) = token.parse::<f32>() {
            //если число то прибавляет
            values.push(num);
        } else if token == "(" {
            // если "(" то добавляет в операторы
            operators.push(token.clone());
        } else if token == ")" {
            while let Some(op) = operators.pop() {
                if op == "(" {
                    break;
                }
                apply_operator(&mut values, &op);
            }
        } else if token == "+" || token == "-" || token == "*" || token == "/" || token == "^" {
            while let Some(last_op) = operators.last() {
                if precedence(token) <= precedence(last_op) {
                    let op = operators.pop().unwrap();
                    apply_operator(&mut values, &op);
                } else {
                    break;
                }
            }
            operators.push(token.clone());
        }
        i += 1;
        unsafe {
            if DEBUG_MODE == true {
                println!("-- {}", i);
                println!("!operators: {:?}", operators);
                println!("!values: {:?}", values);
            }
        }
    }

    while let Some(op) = operators.pop() {
        apply_operator(&mut values, &op);
        unsafe {
            if DEBUG_MODE == true {
                println!("-- op");
                println!("!operators: {:?}", operators);
                println!("!values: {:?}", values);
            }
        }
    }
    
    let result = values.pop().unwrap();
    println!("Result: {}", result);
    result
}

fn apply_operator(values: &mut Vec<f32>, op: &str) {
    let right = values.pop().unwrap();
    let left = values.pop().unwrap();
    let result = match op {
        "+" => left + right,
        "-" => left - right,
        "*" => left * right,
        "/" => left / right,
        "^" => left.powf(right),
        _ => panic!("Unknown operator"),
    };
    values.push(result);
}

fn precedence(op: &str) -> i32 {
    match op {
        "+" | "-" => 1,
        "*" | "/" => 2,
        "(" => 0,
        _ => panic!("Unknown operator"),
    }
}

fn let_start_again() {
    begin()
}
