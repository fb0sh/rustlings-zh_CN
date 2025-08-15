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
//
// The exact form of this will be:
// - The input is going to be a Vector of 2-length tuples,
//   the first element is the string, the second one is the command.
// - The output element is going to be a vector of strings.

// 这是针对以下章节的测验：
// - 字符串（Strings）
// - 向量（Vecs）
// - 移动语义（Move semantics）
// - 模块（Modules）
// - 枚举（Enums）
//
// 我们将以函数的形式构建一个小机器。输入是一个字符串和命令的列表。
// 这些命令决定将对字符串执行的操作。可能的操作有：
// - 将字符串转换为大写
// - 修剪字符串
// - 将 "bar" 附加到字符串指定次数
//
// 具体形式如下：
// - 输入是一个包含 2 元组的向量，
//   第一个元素是字符串，第二个元素是命令。
// - 输出将是一个字符串向量。
enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // TODO: Complete the function as described above.
    // TODO: 根据上述描述完成函数。
    // pub fn transformer(input: ???) -> ??? { ??? }
}

fn main() {
    // You can optionally experiment here.
    // 你可以在这里进行可选的实验。
}

#[cfg(test)]
mod tests {
    // TODO: What do we need to import to have `transformer` in scope?
    // use ???;
    use super::Command;

    #[test]
    fn it_works() {
        let input = vec![
            ("hello".to_string(), Command::Uppercase),
            (" all roads lead to rome! ".to_string(), Command::Trim),
            ("foo".to_string(), Command::Append(1)),
            ("bar".to_string(), Command::Append(5)),
        ];
        let output = transformer(input);

        assert_eq!(
            output,
            [
                "HELLO",
                "all roads lead to rome!",
                "foobar",
                "barbarbarbarbarbar",
            ]
        );
    }
}
