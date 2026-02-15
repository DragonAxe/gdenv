use colored::*;

pub fn success(msg: &str) {
    println!("{} {}", "[✓]".green(), msg);
}

pub fn error(msg: &str) {
    println!("{} {}", "[E]".red(), msg);
}

pub fn info(msg: &str) {
    println!("{} {}", "[ ]".blue(), msg);
}

pub fn warning(msg: &str) {
    println!("{} {}", "[W]".yellow(), msg);
}

pub fn helpful(msg: &str) {
    println!("{} {}", "[i]".cyan(), msg);
}

pub fn question(msg: &str) {
    println!("{} {}", "[?]".magenta(), msg);
}
