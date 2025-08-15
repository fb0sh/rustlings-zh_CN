// You can bring module paths into scopes and provide new names for them with
// the `use` and `as` keywords.
// 你可以使用 `use` 和 `as` 关键字将模块路径引入作用域，并为它们提供新的名称。

mod delicious_snacks {
    // TODO: Add the following two `use` statements after fixing them.
    // TODO: 修复后添加以下两个 `use` 语句

    // use self::fruits::PEAR as ???;
    // use self::veggies::CUCUMBER as ???;

    mod fruits {
        pub const PEAR: &str = "Pear";
        pub const APPLE: &str = "Apple";
    }

    mod veggies {
        pub const CUCUMBER: &str = "Cucumber";
        pub const CARROT: &str = "Carrot";
    }
}

fn main() {
    println!(
        "favorite snacks: {} and {}",
        delicious_snacks::fruit,
        delicious_snacks::veggie,
    );
}
