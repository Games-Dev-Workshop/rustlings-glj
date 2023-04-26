// variables5.rs
// Execute `rustlings hint variables5` or use the `hint` watch subcommand for a hint.


fn main() {
    let number = "T-H-R-E-E"; // don't change this line
    println!("Spell a Number : {}", number);
    let number = 3; // don't rename this variable
    println!("Number plus two is : {}", number + 2);

    // Messing about with avoiding autos - I don't trust autos!
    let number : String = String::from("T-H-R-E-E"); // don't change this line
    println!("Spell a Number : {}", number);
    let number : u8 = 3; // don't rename this variable
    println!("Number plus two is : {}", number + 2);
}
