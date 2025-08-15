trait Licensed {
    // TODO: Add a default implementation for `licensing_info` so that
    // implementors like the two structs below can share that default behavior
    // without repeating the function.
    // The default license information should be the string "Default license".
    // TODO: 为 `licensing_info` 添加一个默认实现，以便像下面两个结构体这样的
    // 实现者可以共享该默认行为，而无需重复该函数。默认的许可证信息应该是
    // 字符串 "Default license"。
    fn licensing_info(&self) -> String;
}

struct SomeSoftware {
    version_number: i32,
}

struct OtherSoftware {
    version_number: String,
}

impl Licensed for SomeSoftware {} // Don't edit this line. // 不要修改当前行
impl Licensed for OtherSoftware {} // Don't edit this line. // 不要修改当前行

fn main() {
    // You can optionally experiment here.
    // 你可以在这里进行可选的实验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_licensing_info_the_same() {
        let licensing_info = "Default license";
        let some_software = SomeSoftware { version_number: 1 };
        let other_software = OtherSoftware {
            version_number: "v2.0.0".to_string(),
        };
        assert_eq!(some_software.licensing_info(), licensing_info);
        assert_eq!(other_software.licensing_info(), licensing_info);
    }
}
