// functions4.rs
// Execute `rustlings hint functions4` or use the `hint` watch subcommand for a hint.

// This store is having a sale where if the price is an even number, you get
// 10 Rustbucks off, but if it's an odd number, it's 3 Rustbucks off.
// (Don't worry about the function bodies themselves, we're only interested
// in the signatures for now. If anything, this is a good way to peek ahead
// to future exercises!)


fn main() {
    let original_price = 51;
    println!("Your sale price is {}", sale_price(original_price));
    println!("Your sale price is {}", sale_price2(original_price));
}

//A return marks the end of an execution path in a function:
fn sale_price(price: i32) -> i32 {
    if is_even(price) {
        return price - 10
    } else {
        return price - 3
    }
}

fn sale_price2(price: i32) -> i32 {
    let mut sp : i32;
    if is_even(price) {
        sp = price - 10
    } else {
        sp = price - 3
    }

    return sp;
}

// return is not needed when the returned value is the last expression in the function. In this case the ; is omitted:
fn is_even(num: i32) -> bool {
    num % 2 == 0
}
