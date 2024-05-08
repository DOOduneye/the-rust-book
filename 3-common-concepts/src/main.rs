// Constants
const _THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    // Mutability
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // Shadowing
    // When you can declare a new varianle with the same name as the previous variable
    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}");

    // another example
    let spaces = "   ";
    let _spaces = spaces.len();

    // Data types
    let _guess: u32 = "42".parse().expect("Not a number!");

    // Scalar types represents a single value.
    // [ integers, floating-point numbers, Booleans, characters ]

    // Tuple types
    // A compound type that cannot grow or shrink
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // We can destructure the type as well
    let (a, b, c) = tup;
    println!("{a}, {b}, {c}");

    // We can also destructure indiviually
    let d = tup.1;
    println!("{d}");

    // Arrays are fixed length whereas a vector type is of dynamic length
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    let _array2 = [3; 5]; // [3, 3, 3, 3, 3]
    let _array_first_value = array[1];

    // Functions
    paramaterized_function(5, 'k');

    // Statements and Expressions

    // Statements: are instructions that perform some action and do not return a value.
    // Creating a variable and assigning a value to it with the let keyword is a statement.
    // So are functions
    let _var = 6;

    // Expressions evaluate to a resultant value.
    let exp = {
        let x = 3;
        x + 1 // expressions do not have ending semicolons
    };

    println!("The value of exp is: {exp}");

    with_return_type();

    // if Expressions are generally the same as most other languages
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // Since expressions evaluate to a result we can use it on the right side of a variable declaration
    let condition = true;
    let number = if condition { "4" } else { "7" };

    println!("The value of number is: {number}");

    // Repititon of code

    // Loops run forever
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("result: {result}");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    // While loops, general
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
    
    println!("------");

    // For (in) loops
    for element in a {
        println!("the value is: {element}");
    }

    // Ranged For (in) loops
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");

    let cel = to_celsius(50.0);
    let far = to_fahrenheit(10.0);

    println!("Conversions cel: {cel}, far: {far}");

    let fib_5 = fib(5);

    println!("fib_5 {fib_5}");
}

fn paramaterized_function(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}")
}

fn with_return_type() -> i32 {
    5 // expressions do not have ending semicolons, equivelent to return 5
}

fn to_celsius(f: f32) -> f32 {
    (f - 32.0) * 5.0 / 9.0
}

fn to_fahrenheit(c: f32) -> f32 {
    (c * 9.0 / 5.0) + 32.0
}

fn fib(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }

    fib(n - 1) + fib(n - 2)
}