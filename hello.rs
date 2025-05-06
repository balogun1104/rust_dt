fn main() {
    let x: i32 = -42;
    let y: u64 = 128;

    println!("unSigned Int: {}", x);
    println!("Signed Int: {}", y);

    let e: i32 = 214748349;
    let i: i64 = 922337206854775808;

    println!("max for i32: {}", e);
    println!("max for i64: {}", i);

    let pi: f32 = 3.14;
    println!("Value of pi: {}", pi);

    // Boolean
    let is_snowing: bool = false;
    println!("Is it snowing? {}", is_snowing);

    // character Type - char
    let letter: char = 'a';
    println!("First letter: {}", letter);

    // Mutability - variables are immutable by default
    let mut count = 0;
    count = count + 1;
    println!("Count: {}", count);

    // Constants - always immutable, require type annotation
    const MAX_POINTS: u32 = 100_000;
    println!("Max points: {}", MAX_POINTS);

    // Shadowing - reuse variable names
    let spaces = "   ";
    let spaces = spaces.len();
    println!("Number of spaces: {}", spaces);

    // Tuples - fixed-length collection of different types
    let tup: (i32, f64, char) = (500, 6.4, 'z');
    let (x, y, z) = tup; // Destructuring
    println!("Tuple values: {}, {}, {}", x, y, z);
    println!("Access by index: {}", tup.0); // Access by index

    // Arrays - fixed-length collection of same type
    let arr = [1, 2, 3, 4, 5];
    println!("First element: {}", arr[0]);
    
    // Functions
    print_hello();
    let sum = add(5, 10);
    println!("Sum: {}", sum);
    
    // Conditionals
    let number = 7;
    if number < 5 {
        println!("Number is less than 5");
    } else if number == 5 {
        println!("Number is 5");
    } else {
        println!("Number is greater than 5");
    }
    
    // Loops
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // Return value from loop
        }
    };
    println!("Loop result: {}", result);
    
    // While loop
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    
    // For loop
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("Value: {}", element);
    }
    
    // Range-based for loop
    for number in 1..4 {
        println!("{}!", number);
    }
    
    // Ownership - Rust's unique feature
    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved to s2, s1 no longer valid
    // println!("{}", s1); // This would cause an error
    println!("{}", s2);
    
    // References and Borrowing
    let s3 = String::from("hello");
    let len = calculate_length(&s3); // Borrow s3
    println!("The length of '{}' is {}.", s3, len);
    
    // Mutable references
    let mut s4 = String::from("hello");
    change(&mut s4);
    println!("Changed string: {}", s4);
}

// Function definition
fn print_hello() {
    println!("Hello, Rust!");
}

// Function with parameters and return value
fn add(a: i32, b: i32) -> i32 {
    a + b // No semicolon means this is the return value
}

// Function with reference parameter
fn calculate_length(s: &String) -> usize {
    s.len()
}

// Function with mutable reference
fn change(s: &mut String) {
    s.push_str(", world");
}
