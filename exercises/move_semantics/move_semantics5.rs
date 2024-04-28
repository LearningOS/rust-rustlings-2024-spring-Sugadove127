// move_semantics5.rs
//
// Make me compile only by reordering the lines in `main()`, but without adding,
// changing or removing any of them.
//
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand
// for a hint.



fn main() {
    let mut x = 42;

    {
        let y = &mut x; // 第一个可变引用的作用域
        *y += 100;
    } // 第一个可变引用的作用域结束

    let z = &mut x; // 第二个可变引用的作用域
    *z += 1000;

    println!("x = {}", x);
}

