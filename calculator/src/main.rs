fn main() {
    let x = 25;
    // println!("{}", factorial(m)); <- for the normal function
    let calculate = &mut FactorialCalculator {n: 25};
    println!("{}", calculate.calculate_factorial());
    println!("{}", FactorialCalculator::factorial(x))
}

// The calculator in a normal function.
/* fn factorial(n: i128) -> i128 {
    if n == 0 || n == 1 {
        return 1;
    } else {
        return n * factorial(n - 1);
    }
} */

// The calculator in a structure with an implementation block.
struct FactorialCalculator {
    // Structure must be mutable to work properly.
    // Use &mut FactorialCalculator {} when creating new instance.
    n: i128,
}

impl FactorialCalculator {
    // Calculate using a method.
    fn calculate_factorial(self: &mut Self) -> i128 {
        if self.n == 0 || self.n == 1 {
            return 1;
        } else {
            self.n = self.n - 1;
            return (self.n + 1) * self.calculate_factorial()
        }
    }

    // Calculate using an associated function.
    fn factorial(n: i128) -> i128 {
        if n == 0 || n == 1 {
            return 1;
        } else {
            return n * Self::factorial(n - 1)
        }
    }
}