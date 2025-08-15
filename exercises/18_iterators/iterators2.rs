// In this exercise, you'll learn some of the unique advantages that iterators
// can offer.
// 在这个练习中，你将学到迭代器所能提供的一些独特优势。

// TODO: Complete the `capitalize_first` function.
// TODO: 完成 `capitalize_first` 函数。
// "hello" -> "Hello"

fn capitalize_first(input: &str) -> String {
    let mut chars = input.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => todo!(),
    }
}

// TODO: Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// TODO: 将 `capitalize_first` 函数应用于字符串切片切片。
// 返回一个字符串向量。
// ["hello", "world"] -> ["Hello", "World"]
fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    // ???
}

// TODO: Apply the `capitalize_first` function again to a slice of string
// slices. Return a single string.
// TODO: 再次将 `capitalize_first` 函数应用于字符串切片切片。
// 返回一个单独的字符串。
// ["hello", " ", "world"] -> "Hello World"
fn capitalize_words_string(words: &[&str]) -> String {
    // ???
}

fn main() {
    // You can optionally experiment here.
    // 你可以在这里进行可选的实验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
