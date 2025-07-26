// 这类似于之前的 `from_into` 练习。但这一次，我们将
// 实现 `FromStr` 并返回错误，而不是回退到默认值。
// 此外，在实现 `FromStr` 后，你可以对字符串使用 `parse` 方法
// 来生成一个实现了该 trait 的类型的对象。你可以在文档中阅读更多相关信息：
// https://doc.rust-lang.org/std/str/trait.FromStr.html

use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: u8,
}

// 我们将为 `FromStr` 实现使用这个错误类型。
#[derive(Debug, PartialEq)]
enum ParsePersonError {
    // 字段数量不正确
    BadLen,
    // 名称字段为空
    NoName,
    // 来自 parse::<u8>() 的包装错误
    ParseInt(ParseIntError),
}

// TODO: 完成此 `FromStr` 实现，以便能够从 "Mark,20" 形式的
// 字符串中解析出 `Person`。
// 请注意，你需要使用类似 `"4".parse::<u8>()` 的方式将年龄部分解析为 `u8`。
//
// 步骤：
// 1. 根据逗号分割给定的字符串。
// 2. 如果分割操作返回少于或多于 2 个元素，则返回错误 `ParsePersonError::BadLen`。
// 3. 使用分割操作的第一个元素作为名称。
// 4. 如果名称为空，则返回错误 `ParsePersonError::NoName`。
// 5. 将分割操作的第二个元素解析为 `u8` 作为年龄。
// 6. 如果解析年龄失败，则返回错误 `ParsePersonError::ParseInt`。
impl FromStr for Person {
    type Err = ParsePersonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 2 {
            return Err(ParsePersonError::BadLen);
        }
        let name = parts[0];
        if name.is_empty() {
            return Err(ParsePersonError::NoName);
        }
        match parts[1].parse::<u8>() {
            Ok(age) => Ok(Person {
                name: name.to_string(),
                age,
            }),
            Err(e) => Err(ParsePersonError::ParseInt(e)),
        }
    }
}

fn main() {
    let p = "Mark,20".parse::<Person>();
    println!("{p:?}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use ParsePersonError::*;

    #[test]
    fn empty_input() {
        assert_eq!("".parse::<Person>(), Err(BadLen));
    }

    #[test]
    fn good_input() {
        let p = "John,32".parse::<Person>();
        assert!(p.is_ok());
        let p = p.unwrap();
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 32);
    }

    #[test]
    fn missing_age() {
        assert!(matches!("John,".parse::<Person>(), Err(ParseInt(_))));
    }

    #[test]
    fn invalid_age() {
        assert!(matches!("John,twenty".parse::<Person>(), Err(ParseInt(_))));
    }

    #[test]
    fn missing_comma_and_age() {
        assert_eq!("John".parse::<Person>(), Err(BadLen));
    }

    #[test]
    fn missing_name() {
        assert_eq!(",1".parse::<Person>(), Err(NoName));
    }

    #[test]
    fn missing_name_and_age() {
        assert!(matches!(",".parse::<Person>(), Err(NoName | ParseInt(_))));
    }

    #[test]
    fn missing_name_and_invalid_age() {
        assert!(matches!(
            ",one".parse::<Person>(),
            Err(NoName | ParseInt(_)),
        ));
    }

    #[test]
    fn trailing_comma() {
        assert_eq!("John,32,".parse::<Person>(), Err(BadLen));
    }

    #[test]
    fn trailing_comma_and_some_string() {
        assert_eq!("John,32,man".parse::<Person>(), Err(BadLen));
    }
}
