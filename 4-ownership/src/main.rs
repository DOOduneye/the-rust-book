fn main() {
    let mut s = String::from("hello"); // `s` owns the "hello" string
    let t = s; // Ownership of the string is moved to `t`. `s` is no longer valid

    // Uncommenting the next line would cause a compile-time error
    // println!("s is {}", s);  // Error: use of moved value: `s`

    println!("t is {}", t); // This is valid and will print: t is hello

    // To use `s` again, we must reinitialize it
    s = String::from("world"); // Now `s` is valid again with a new value
    println!("s is {}", s); // This prints: s is world

    // let mut num = 5;
    // let mut ref1 = &mut num;
    // *ref1 += 1;

    // let ref2 = &mut ref1;
    // println!("{} and point to the same value: {}", ref1, ref2);

    let mut num = 5;
    {
        let ref1 = &mut num;
        *ref1 += 100;
        println!("{} point to the same value as num", ref1);
    }

    {
        let ref2 = &num;
        println!("{ref2}");
    }

    test_print(&String::from("car"));

    let mut s1 = 4;
    let s2 = s1;

    s1 += 1;

    println!("{s1}, {s2}");

    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
    println!("x addr {:p}, y addr {:p}", &x, &y);

    // let mut p1 = Point { x: 10, y: 20 };
    // let p2 = p1;

    // println!("p1: {:?}, p2: {:?}", p1, p2);

    // p1.x += 5;
    // p1.y += 7;

    // println!("p1: {:?}, p2: {:?}", p1, p2);

    let mut p1 = Point { x: 10, y: 20 };

    // Accessing coordinates through references
    let x_ref = &p1.x;
    let y_ref = &p1.y;

    println!("p1: ({}, {})", x_ref, y_ref);

    // Modifying through a mutable reference
    let ref_to_modify = &mut p1.x; // Borrow a mutable reference to x
    *ref_to_modify += 5;

    println!("p1 after modification: ({}, {})", ref_to_modify, y_ref); // x_ref will still reflect the change

    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // x's value moves into the function...
                        // ... and so is no longer valid here

    // println!("{s}"); Errors as we gave ownership to take_ownership and then it gets dropped so to save yourself from a dangling pointer

    let x = 5; // x comes into scope

    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it's okay to still
                   // use x afterward

    println!("{x}"); // Works because it's stored in the stack + uses the Copy trait and will inherently be copied

    let s1 = gives_ownership(); // gives_ownership moves its return
                                // value into s1
    println!("{s1}");
    let s2 = String::from("hello"); // s2 comes into scope
    println!("{s2}");
    let s3 = takes_and_gives_back(s2); // s2 is moved into
                                       // takes_and_gives_back, which also
                                       // moves its return value into s3
    println!("{s3}");

    let value1 = String::from("hello");

    let (value1, len) = calculate_length(value1);

    println!("The length of '{}' is {}.", value1, len);

    let mut m = 7;
    let ref_m = &mut m;
    println!("ref1 {ref_m}");

    let ref_m2 = &mut m;
    println!("ref2 {ref_m2}");

    // OK
    let s = String::from("hello");

    let r1 = &s;
    let r2 = &s;

    println!("{}, {}", r1, r2);

    // OK
    let mut s = String::from("hello your mom");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);

    // let reference_to_nothing = dangle();
    fn first_word(s: &String) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                // return i;
                return &s[0..i];
            }
        }

        &s[..]
    }

    let s = String::from("hello world");

    let word = first_word(&s);

    // s.clear(); // error! would pass if we didnt use a slice

    println!("the first word is: {}", word);

    let _ = no_dangle();
}

// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }
fn no_dangle() -> String {
    String::from("hello")
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn test_print(string: &String) {
    println!("{string}");
}

// #[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    // some_string comes into scope

    String::from("yours") // some_string is returned and
                          // moves out to the calling
                          // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}
