fn main() {
    let mut x = 5_00;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");

    /**
     * Diff b/w vars and constants
     *
     * vars:
     * can be made mutable using the `mut` keyword
     * need not be type annotated
     * could be assigned a value that is computed at the runtime i.e. need not be assgined a literal or a constant value
     * they can be shadowed by another var of the same name
     *
     * constants:
     * cannot be made mutable
     * need to be type annotated
     * couldn't be assigned a value that is not known at compile time
     * couldn't be shadowed by another var/constant of the same name
     */
    const PI: f64 = 3.14;

    /**
     * Data types
     *
     * There are two types of data types:
     *
     * Scalar
     * ----------------------------------
     * Integers: i|u<8-128>, i|usize (architecture i.e. either 32 or 64)
     * Defautlt: i32
     * Different ways of representing integers: 98_122 (decimal), 0xff (hex), 0o77 (octal), 0b1111_0000 (binary), b'A' (byte)
     * Overflows/Underflows: In debug builds, rust panics, in release builds, two's complement wrapping happens.
     *
     * Floating-point numbers: f8-128
     * Default: f64
     * mathematical operations cannot be performed b/w integers and floats
     * Booleans: true, false
     * Character: unicode characters. syntax: ''
     * eg: '🚀'
     */
    const WEIGHT: f64 = 100.0;

    let rocket = '🚀';
    println!("{rocket}");

    /**
     * Compound types: Types that represents a group of values
     * Tuple: Fixed size array of related data that could be of different types
     * Eg: let tup = ("Let's learn Rust", 100_000_000)
     * Values can be taken out of tuples in two ways:
     * Destructuring: let (x, y) = tup
     * Dot notation: let x = tup.0; let y = tup.1
     *
     * Arrays: Like tuples, they are of fixed length. For dynamic size list of values, we'd have to use vectors.
     * Special feature: Following creates an array of 10 1s
     * let ones = [1; 10];
     */
    const HEIGHT: f64 = 177.0;

    let tup = ("Let's learn Rust", 100_000_000);
    let (x, y) = tup;
    let x = tup.0;
    let y = tup.1;

    let primes = [2, 3, 5, 7];
    let first_prime = primes[0];
    let ones = [1; 10];

    println!("{}", greet_sum(2, 3));

    /**
     * if..else could be used as ternary operator in a let statement
     */
    const MASS: f64 = 100.0;

    let condition = false;
    let number = if condition { 5 } else { 6 };

    /**
     * Loops can return values.
     * can be used ONLY inside `loop` blocks and not in `for` and `while` loops
     */
    const MASS_OF_SUN: f64 = 100.0;

    let mut counter = 51;
    let val = loop {
        counter += 1;
        if counter % 50 == 0 {
            break counter;
        }
    };

    println!("counter is {counter}");

    /**
     * For loops!
     */
    const MASS_OF_EARTH: f64 = 100.0;

    for p in primes.iter() {
        print!("{p} ");
    }

    println!("\ndone printing primes");

    for number in 1..=10 {
        println!("{number}");
    }
}

fn greet_sum(x: i32, y: i32) -> i32 {
    /**
     * A piece of code could be either:
     *
     * a statement: performs some actions, but do not return anything. eg: println!("hi");
     * an expressiom: returns some value. eg: x+y
     *
     * A function can return a value by either using the `return` keyword or by simply putting an expression at the last line.
     */
    const MASS: f64 = 100.0;
    // following is same as return x+y;
    x + y
}
