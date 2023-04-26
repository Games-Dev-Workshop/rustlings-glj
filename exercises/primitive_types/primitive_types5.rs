// primitive_types5.rs
// Destructure the `cat` tuple so that the println will work.
// Execute `rustlings hint primitive_types5` or use the `hint` watch subcommand for a hint.


fn main() {
    let cat = ("Furry McFurson", 3.5);
    let (name, age) = cat;

    let dog: (&str, i32) = ("Colin", 2);

    println!("{} is {} years old.", name, age);

    let (name, age) = dog;

    println!("{} is {} years old.", name, age);
}
