// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.

fn main() {
    let data = "Rust is great!".to_string();

    let c = get_char(data.clone());
    println!("First character: {}", c);

    string_uppercase(&data);
    println!("Uppercase string: {}", data);
}

fn get_char(data: String) -> char {
    data.chars().next().unwrap()
}

fn string_uppercase(data: &String) {
    let upper = data.to_uppercase();
    println!("{}", upper);
}

