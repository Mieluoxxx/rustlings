// `From` trait 用于值到值的转换。如果实现了 `From`，
// 则会自动提供 `Into` 的实现。
// 你可以在文档中阅读更多相关信息：
// https://doc.rust-lang.org/std/convert/trait.From.html

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// 我们实现了 Default trait，以便在提供的
// 字符串无法转换为 `Person` 对象时将其用作回退。
impl Default for Person {
    fn default() -> Self {
        Self {
            name: String::from("John"),
            age: 30,
        }
    }
}

// TODO: 完成此 `From` 实现，以便能够从
// "Mark,20" 形式的字符串中解析出 `Person`。
// 请注意，你需要使用类似 `"4".parse::<u8>()` 的方式将年龄部分解析为 `u8`。
//
// 步骤：
// 1. 根据逗号分割给定的字符串。
// 2. 如果分割操作返回少于或多于 2 个元素，则返回 `Person` 的默认值。
// 3. 使用分割操作的第一个元素作为名称。
// 4. 如果名称为空，则返回 `Person` 的默认值。
// 5. 将分割操作的第二个元素解析为 `u8` 作为年龄。
// 6. 如果解析年龄失败，则返回 `Person` 的默认值。
impl From<&str> for Person {
    fn from(s: &str) -> Self {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 2 {
            return Person::default();
        }
        let name = parts[0];
        if name.is_empty() {
            return Person::default();
        }
        if let Ok(age) = parts[1].parse() {
            Person {
                name: name.to_string(),
                age,
            }
        } else {
            Person::default()
        }
    }
}

fn main() {
    // 使用 `from` 函数。
    let p1 = Person::from("Mark,20");
    println!("{p1:?}");

    // 由于为 Person 实现了 `From`，因此我们能够使用 `Into`。
    let p2: Person = "Gerald,70".into();
    println!("{p2:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let dp = Person::default();
        assert_eq!(dp.name, "John");
        assert_eq!(dp.age, 30);
    }

    #[test]
    fn test_bad_convert() {
        let p = Person::from("");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_good_convert() {
        let p = Person::from("Mark,20");
        assert_eq!(p.name, "Mark");
        assert_eq!(p.age, 20);
    }

    #[test]
    fn test_bad_age() {
        let p = Person::from("Mark,twenty");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_comma_and_age() {
        let p: Person = Person::from("Mark");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_age() {
        let p: Person = Person::from("Mark,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name() {
        let p: Person = Person::from(",1");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_age() {
        let p: Person = Person::from(",");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_invalid_age() {
        let p: Person = Person::from(",one");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma() {
        let p: Person = Person::from("Mike,32,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma_and_some_string() {
        let p: Person = Person::from("Mike,32,dog");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
}
