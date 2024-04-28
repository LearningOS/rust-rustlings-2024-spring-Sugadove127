// functions5.rs
//
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a
// hint.



fn square(num: i32) -> i32 {
    num * num
}

fn main() {
    let number = 5;
    let result = square(number);
    println!("Square of {} is {}", number, result);
}

