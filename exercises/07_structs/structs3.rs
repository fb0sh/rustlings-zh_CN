// Structs contain data, but can also have logic. In this exercise, we have
// defined the `Package` struct, and we want to test some logic attached to it.
// 结构体不仅可以包含数据，还可以包含逻辑。在这个练习中，我们定义了 `Package` 结构体，
// 并希望测试附加在它上的一些逻辑。

#[derive(Debug)]
struct Package {
    sender_country: String,
    recipient_country: String,
    weight_in_grams: u32,
}

impl Package {
    fn new(sender_country: String, recipient_country: String, weight_in_grams: u32) -> Self {
        if weight_in_grams < 10 {
            // This isn't how you should handle errors in Rust, but we will
            // learn about error handling later.
            // 这不是在 Rust 中处理错误的正确方式，但我们稍后会学习错误处理。
            panic!("Can't ship a package with weight below 10 grams");
        }

        Self {
            sender_country,
            recipient_country,
            weight_in_grams,
        }
    }

    // TODO: Add the correct return type to the function signature.
    // TODO: 在函数签名中添加正确的返回类型。
    fn is_international(&self) {
        // TODO: Read the tests that use this method to find out when a package
        // is considered international.
        // TODO: 阅读使用此方法的测试，以了解什么时候包裹被认为是国际包裹。
    }

    // TODO: Add the correct return type to the function signature.
    // TODO: 在函数签名中添加正确的返回类型。
    fn get_fees(&self, cents_per_gram: u32) {
        // TODO: Calculate the package's fees.
        // TODO: 计算包裹的费用。
    }
}

fn main() {
    // You can optionally experiment here.
    // 你可以在这里进行可选的实验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn fail_creating_weightless_package() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Austria");

        Package::new(sender_country, recipient_country, 5);
    }

    #[test]
    fn create_international_package() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Russia");

        let package = Package::new(sender_country, recipient_country, 1200);

        assert!(package.is_international());
    }

    #[test]
    fn create_local_package() {
        let sender_country = String::from("Canada");
        let recipient_country = sender_country.clone();

        let package = Package::new(sender_country, recipient_country, 1200);

        assert!(!package.is_international());
    }

    #[test]
    fn calculate_transport_fees() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Spain");

        let cents_per_gram = 3;

        let package = Package::new(sender_country, recipient_country, 1500);

        assert_eq!(package.get_fees(cents_per_gram), 4500);
        assert_eq!(package.get_fees(cents_per_gram * 2), 9000);
    }
}
