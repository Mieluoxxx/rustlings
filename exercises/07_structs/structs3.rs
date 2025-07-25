// 结构体包含数据，但也可以有逻辑。在这个练习中，我们已经
// 定义了 `Package` 结构体，我们想要测试附加到它的一些逻辑。

#[derive(Debug)]
struct Package {
    sender_country: String,
    recipient_country: String,
    weight_in_grams: u32,
}

impl Package {
    fn new(sender_country: String, recipient_country: String, weight_in_grams: u32) -> Self {
        if weight_in_grams < 10 {
            // 这不是你在 Rust 中应该处理错误的方式，但我们将
            // 在后面学习错误处理。
            panic!("Can't ship a package with weight below 10 grams");
        }

        Self {
            sender_country,
            recipient_country,
            weight_in_grams,
        }
    }

    // TODO: 给函数签名添加正确的返回类型。
    fn is_international(&self) -> bool {
        // TODO: 阅读使用这个方法的测试，找出什么时候包裹
        // 被认为是国际包裹。
        self.sender_country != self.recipient_country
    }

    // TODO: 给函数签名添加正确的返回类型。
    fn get_fees(&self, cents_per_gram: u32) -> u32 {
        // TODO: 计算包裹的费用。
        self.weight_in_grams * cents_per_gram
    }
}

fn main() {
    // You can optionally experiment here.
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
