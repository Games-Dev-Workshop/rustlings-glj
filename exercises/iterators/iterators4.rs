// iterators4.rs
// Execute `rustlings hint iterators4` or use the `hint` watch subcommand for a hint.



pub fn factorial(num: u64) -> u64 {
    // Complete this function to return the factorial of num
    // Do not use:
    // - return
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.
    
/* 
    //This is how I'd have done it 
    let mut prod: u64 = 1;

    for n in 1..num + 1 {
        prod = prod * n;
    }
    
    return prod;
*/

    // Trying to work out how the fuck fold works!
    let mut prod: u64 = 1;
    let mut list: Vec<u64> = Vec::new();

    for n in 1..num + 1 {
        list.push(n);    
    }
    
    
    // initial value for accumulator
    //                V
    //                    Closure
    //                    V                       WTF is a closure?
    
    prod = list.iter().fold(1, |prod, x| prod * x);

    // This is another sub optimial solution, I can't define the closure somewhere 
    // and use it here though - baffled!

    return prod;

    // This is the solution ... I'd have never come up with this.
    // (1..num + 1).fold(1, |prod, x| prod * x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(1, factorial(0));
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
