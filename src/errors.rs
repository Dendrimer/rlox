pub fn error(line: i32, message: &str) -> bool {
    report(line, "", message);
    true
}

fn report(line: i32, error: &str, message: &str) {
    println!("[line {}] Error {}: {}", line, error, message);
}
