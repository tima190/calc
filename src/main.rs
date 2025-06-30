use colored::Colorize; // Импортируем библиотеку для работы с цветами
use std::io::{self, Write}; // Импортируем библиотеки для работы с вводом и выводом

//import
mod debug; // Импортируем модуль debug.rs
use crate::debug::debugging;

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
    loop {
        let prefix = ">> ";
        print!("{}", prefix.cyan());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let trimmed = input.trim();

        if trimmed.eq_ignore_ascii_case("h") {
            println!(
                "{}\n{}\n{}\n{}\n{}\n{}\n{}",
                "calc help".bold().green(),
                "'!' = debug mode".italic(),
                "'+' = addition".italic(),
                "'-' = subtraction".italic(),
                "'*' = multiplication".italic(),
                "'/' = division".italic(),
                "'(' or ')' = bracket".italic()
            );
            continue;
        }

        if trimmed.is_empty() {
            println!("{}", "Exiting...".bold().green());
            break;
        }

        println!("You entered: {}", trimmed);
        debugging(trimmed);

        match formatting(trimmed) {
            Ok(tokens) => {
                calculate(tokens);
            }
            Err(e) => {
                println!("{}", e.red());
            }
        }
    }
}

fn formatting(formule: &str) -> Result<Vec<String>, String> {
    let mut stack: Vec<String> = vec![String::from("")];
    let mut dot_count = 0;
    let mut last_char = ' ';

    for c in formule.chars() {
        if c.is_ascii_digit() {
            stack.last_mut().unwrap().push(c);
        } else if c == '.' {
            if dot_count > 0 {
                return Err("Invalid number format: multiple decimal points.".to_string());
            }
            stack.last_mut().unwrap().push(c);
            dot_count += 1;
        } else if "+*/()^".contains(c) {
            stack.push(c.to_string());
            stack.push(String::new());
            dot_count = 0;
        } else if c == '-' {
            if last_char == ' ' || last_char == '(' || "+-*/(".contains(last_char) {
                stack.last_mut().unwrap().push(c);
            } else {
                stack.push(c.to_string());
                stack.push(String::new());
            }
            dot_count = 0;
        } else if c == ' ' {
            continue;
        
        } else if c == '!' {
            continue;
        
        } else {
            return Err(format!("Invalid character: '{}'", c));
        }

        last_char = c;
    }

    match formule.chars().next() {
    Some('!') => {
        println!("{}{:?}","DEBUG: ".bold().green(), formule);
        println!("{}{:?}","DEBUG len: ".bold().green(), formule.len());
    }
    _ => {}
    }

    Ok(stack)
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
    }

    while let Some(op) = operators.pop() {
        apply_operator(&mut values, &op);
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
        "/" => {
        if right == 0.0 {
            panic!("Division by zero");
        }
        left / right
        },
        "^" => left.powf(right),
        _ => panic!("Unknown operator"),
    };
    values.push(result);
}

fn precedence(op: &str) -> i32 {
    match op {
        "+" | "-" => 1,
        "*" | "/" => 2,
        "^" => 3,
        "(" => 0,
        _ => panic!("Unknown operator"),
    }
}
