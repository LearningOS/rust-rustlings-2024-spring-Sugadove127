// tests8.rs
//
// This execrise shares `build.rs` with the previous exercise.
// You need to add some code to `build.rs` to make both this exercise and
// the previous one work.
//
// Execute `rustlings hint tests8` or use the `hint` watch subcommand for a
// hint.



fn main() {}

#[cfg(test)]
mod tests {
    // 不需要使用 super::*，因为测试代码中并没有使用到外部定义的内容
    #[test]
    fn test_success() {
        // 如果未设置名为 "pass" 的特性，则直接返回成功
        #[cfg(not(feature = "pass"))]
        return;

        panic!("no cfg set");
        
    }
}

