fn main() {
    // Ownership is what allows Rust to be memory-safe without the use of garbage collector.
    // Ownership model is a way to manage memory. Two other ways are: Garbage collection, manual memory management

    // Ownership rules
    // 1. Each value in Rust has a variable that's called its owner.
    // 2. A value cannot have more than 1 owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.

    {
        // s is not valid here, it's not yet declared
        let s = String::from("hello"); // s is valid from this point forward
        // do stuff with  s
    } // this scope is now over, and s is no longer valid

    // --------------------MEMORY ALLOCATION----------------------------------

    let x = 10;
    let y = x; // copy

    let s1 = String::from("Hello");
    // s1 gets invalidated after being moved to a new var
    let s2 = s1; // s1 is moved over to s2
                 //  println!("{s1}"); // DOES NOT COMPILE!

    let s3 = s2.clone(); // s2 is not moved to s3
                         // a new "Hello" string is created in heap for s3
    println!("{s3}");

    // Rust defaults to moving a value

    takes_ownership(s3);

    // when we passed s3 to takes_ownership, it was same as if we assigned it to another variable. Here, s3 was assigned to s
    //  println!("{s3}"); // DOES NOT COMPILE!

    // -------------------- REFERENCES & BORROWING --------------------
    let s = String::from("hello");
    let ss = s.clone();

    let len = calc_length(s);
    // println!("the length of {s} is {len}"); // s is not found here as ss took the ownership of the value.

    // therefore, instead of passing in the string variable, we'll pass only a reference to it. This is also called borrowing
    let len = calc_length_with_ref(&ss);
    println!("the length of {ss} is {len}");

    let mut ms = String::from("hello");
    change(&mut ms);
    println!("{ms}");

    // you can only have one mutable reference to a particular piece of data in a particular scope
    let mut s = String::from("hi");

    let r1 = &s;
    let r2 = &s; // cannot borrow `s` as mutable more than once at a time
    // this prevents data races right at the compile time!
    // A data race occurs if we have two pointers pointing to the same piece of data and one of the pointers is used to write to the data and there's no method to syncronize the data access between those pointers.

    // let r3 = &mut s; // also, you cannot have mutable references if an immutable reference already exists.
    // this is because the immutable references don't expect underlying value to change which is likely going to happen if there's also a mutable reference.

    println!("{r1}, {r2}");

    // Scope of a reference starts when it's first introduced and ends when it's last used. Therefore, below we can freely create a mut reference to s as the immutable reference's scope ended above

    let r3 = &mut s;

    println!("{r3}");

    // -------------------- RULES OF REFERENCES --------------------

    // 1. At any given time, you can have either one mutable refernce or any number of immutable reference.
    // 2. Those references must be valid i.e. they must not be dangling references.

    // -------------------------- SLICING 🔪 --------------------------
    let s = String::from("hello world");
    let s2 = "hello world";
    // string literals are actually string slices. they are stored directly in the binary. s2 is a string slice to the location in binary where "hello world" is stored.

    let first_word = &s[..5];
    let second_word = &s[6..];
    let word = first_word_fn(&s); // String types can implicitly be converted into string slices. i.e. String -> &str (not `str`!)

    println!("{first_word}, {second_word}, {word}");

    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[..2];
}

fn takes_ownership(s: String) {
    println!("{s}");
}

fn calc_length(ss: String) -> usize {
    return ss.len();
}

fn calc_length_with_ref(ss: &String) -> usize {
    // references are immutable by default
    // ss.push('x'); // cannot do this. we'd have to make ss of &mut String type
    return ss.len();
}

fn change(s: &mut String) {
    // takes a string as input without taking the ownership of its underlying value and also modifying it at the end.
    s.push_str(" world");
}

// the following function does not compile cuz its trying to return a reference of a local variable and the value of that local var gets dropped after this function finishes execution, so there would have been a reference to an invalid value i.e. a dangling reference.
// fn dangle() -> &String {
//     let s = String::from("hello");

//     return &s;
// }

fn first_word_fn(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &c) in bytes.iter().enumerate() {
        if c == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
