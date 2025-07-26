fn main() {
    let cat = ("Furry McFurson", 3.5);

    // TODO: 在一个语句中解构元组 `cat`，使 println 能够正常工作。
    // let /* your pattern here */ = cat;
    let (name, age) = cat;
    println!("{name} is {age} years old");
}
