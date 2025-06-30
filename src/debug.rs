use colored::Colorize;

pub fn debugging(formule: &str) {
    let mut numbers: Vec<u32> = Vec::new();
    let mut chars = formule.chars().peekable();

    if let Some('!') = chars.next() {
        println!("{}", "--- Debugging mode activated ---".bold().yellow());

        let mut index = 0;

        while let Some(c) = chars.next() {
            index += 1;

            if c.is_ascii_whitespace() {
                continue;
            }

            print!(
                "{} ",
                format!("pos {:>2}:", index).dimmed()
            );

            if c.is_ascii_digit() {
                let digit = c.to_digit(10).unwrap();
                numbers.push(digit);

                println!(
                    "{} = {} {}",
                    format!("digit '{}'", c).green().bold(),
                    digit.to_string().green(),
                    format!("(stack size: {})", numbers.len()).dimmed()
                );
            } else if "+-*/^".contains(c) {
                println!("{}", format!("operator '{}'", c).cyan().bold());
            } else if "()".contains(c) {
                println!("{}", format!("bracket '{}'", c).blue().bold());
            } else {
                println!("{}", format!("ignored '{}'", c).red().bold());
            }

            if let Some(peek) = chars.peek() {
                println!(
                    "{} {}",
                    "→ next:".dimmed(),
                    format!("'{}'", peek).yellow()
                );
            } else {
                println!("{}", "→ next: None".dimmed());
            }

            println!("{}", "—".repeat(32).dimmed());
        }

        println!(
            "{} {:?}",
            "DEBUG numbers stack:".bold().green(),
            numbers
        );
    } else {

    }
}
