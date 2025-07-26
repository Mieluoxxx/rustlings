// 你可以将模块路径带入作用域，并为它们提供新名称，
// 使用 `use` 和 `as` 关键字。

mod delicious_snacks {
    // TODO: 在修复下面两个 `use` 语句后添加它们。
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
