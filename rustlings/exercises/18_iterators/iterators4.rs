fn factorial(num: u64) -> u64 {
    // TODO: Complete this function to return the factorial of `num` which is
    // defined as `1 * 2 * 3 * … * num`.
    // https://en.wikipedia.org/wiki/Factorial
    //
    // Do not use:
    // - early returns (using the `return` keyword explicitly)
    // Try not to use:
    // - imperative style loops (for/while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    let mut total_product = 1;
    for n in 0..num+1 {
        if n == 0 {
            total_product = total_product * 1;
        } else {
            total_product = total_product * n;
        }
    }
    total_product
}

fn main() {
    // You can optionally experiment here.
    let mut total_product = 1;
    let num = 2;
    for n in 0..num+1 {
        if n == 0 {
            total_product = total_product * 1;
            println!("{}", total_product);
        } else {
            total_product = total_product * n;
            println!("{}", total_product);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(factorial(1), 1);
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(factorial(4), 24);
    }
}
