// functions3.rs
//
// Execute `rustlings hint functions3` or use the `hint` watch subcommand for a
// hint.

fn call_me(num: u32) {
    for i in 0..num {
        println!("{}", i);
    }
}

fn main() {
    call_me(5);
}
