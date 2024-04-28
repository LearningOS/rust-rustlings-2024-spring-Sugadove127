// functions4.rs
//
// This store is having a sale where if the price is an even number, you get 10
// Rustbucks off, but if it's an odd number, it's 3 Rustbucks off. (Don't worry
// about the function bodies themselves, we're only interested in the signatures
// for now. If anything, this is a good way to peek ahead to future exercises!)
//
// Execute `rustlings hint functions4` or use the `hint` watch subcommand for a
// hint.



fn sale_price(price: i32) -> i32 {
    let discount = 10;
    price - discount
}

fn main() {
    let original_price = 100;
    let final_price = sale_price(original_price);
    println!("Final price after discount: {}", final_price);
}

fn is_even(num: i32) -> bool {
    num % 2 == 0
}
