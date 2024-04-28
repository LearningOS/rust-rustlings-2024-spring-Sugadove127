// quiz2.rs
//
// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums
//
// Let's build a little machine in the form of a function. As input, we're going
// to give a list of strings and commands. These commands determine what action
// is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
// The exact form of this will be:
// - The input is going to be a Vector of a 2-length tuple,
//   the first element is the string, the second one is the command.
// - The output element is going to be a Vector of strings.
//
// No hints this time!



pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // 完成函数签名
    pub fn transformer(input: Vec<(&str, Command)>) -> Vec<String> {
        // 完成输出声明
        let mut output: Vec<String> = vec![];
        for (string, command) in input.iter() {
            // 完成函数体
            let mut modified_string = String::from(*string);
            match command {
                Command::Uppercase => {
                    modified_string = modified_string.to_uppercase();
                }
                Command::Trim => {
                    modified_string = modified_string.trim().to_string();
                }
                Command::Append(times) => {
                    for _ in 0..*times {
                        modified_string.push_str("bar");
                    }
                }
            }
            output.push(modified_string);
        }
        output
    }
}

#[cfg(test)]
mod tests {
    // transformer在这个作用域中所需的导入
    use super::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let output = transformer(vec![
            ("hello", Command::Uppercase),
            (" all roads lead to rome! ", Command::Trim),
            ("foo", Command::Append(1)),
            ("bar", Command::Append(5)),
        ]);
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}

