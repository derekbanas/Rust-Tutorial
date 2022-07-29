/*
Rust provides high performance similar to C++ and is a systems
programming language that is excellent at reducing memory
related errors. (Systems Programming == Low Level Details
like Hardware and Memory)
*/

/*
Garbage collection is not necessary. Memory errors often lead to
security breaches which makes Rust very attractive. Rust programs
also tend to require much less memory in comparison to other
languages.
*/

/*
Also Rust excels when it comes to concurrent programming. At compile
time many possible concurrent programming problems are pointed out.
Rusts compiler is so robust that it normally finds errors not found
by other languages compilers. On top of that, these error messages
are normally very easy to understand!
*/

/*
Cargo is the Rust package manager and build system. It downloads
libraries and builds them. You can verify it is installed with cargo --version
Installation Details are at the end of the video
Create Project with Cargo :
cargo new rust_tutorial
cd rust_tutorial
*/

/*
This generates a Git repository, src directory and a TOML file
TOML (Tom's Obvious Minimal Language) is the Cargo config file
Contains info on configuring the package along with Dependencies
Cargo.lock stores versions for all dependencies
(cargo update updates all dependencies to latest versions)
*/

/*
If you want to compile on any OS open your terminal, go to the directory
cd /D D:\Tutorials\Rust Tutorial
rustc main.rs
./main (or .\main.exe on Windows)
*/

/*
To compile using Cargo (cargo run) or
cargo build
cd D:\Tutorials\Rust Tutorial\rust_tut\target\debug
rust_tut.exe
Cargo is best to use when you have multiple Crates (Packages of Code)
*/

// You can check if the code compiles without building
// cargo check

// When your code is ready for release compile with optimizations
// cargo build --release

// You'll get warnings if you have unused variables
// This gets rid of them
#![allow(unused)]

// ----- LIBRARIES -----

// Define that you want to use the input / output library
use std::io;

// You could bring all public items under io into scope
// use std::io::*;

// Generate random numbers (Add rand crate to Cargo.toml)
use rand::Rng;

// Used for working with files
// You could also use nested paths like this
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;

// Ordering compares values
use std::cmp::Ordering;

// main is where execution begins for your program
// fn : Declares a new function
// () : Is were the parameters would go but there are none
// {} : Surrounds the code in the function

// Declare that we want to use the restaurant module here
mod restaurant;

// Declare a specific function we'll use to access the
// pizza_order module
use crate::restaurant::order_food;

// ----- FUNCTIONS -----
// You can define functions before or after main
fn say_hello(){
    println!("Hello");
}

// You can pass arguments to functions
fn get_sum(x: i32, y: i32){
    println!("{} + {} = {}", x, y, x+y);
}

// Return a value
fn get_sum_2(x: i32, y: i32) -> i32 {
    // This expression is returned
    // If you used a semicolon you'd get an error because
    // a statement don't evaluate to a value
    x + y
}

// You can also use return
fn get_sum_3(x: i32, y: i32) -> i32 {
    return x + y;
}

// Return multiple values
fn get_2(x: i32) -> (i32, i32){
    return (x+1, x+2);
}

fn print_str(x: String){
    println!("A string {}", x);
}

fn print_return_str(x: String) -> String{
    println!("A string {}", x);
    x
}

fn change_string(name: &mut String){
    name.push_str(" is Happy");
    println!("Message : {}", name);
}

// This function sums values in a list (Receives reference to list)
fn sum_list(list: &[i32]) -> i32 {
    let mut sum = 0;
    for &val in list.iter(){
        sum += &val;
    }
    sum
}

// ----- GENERIC FUNCTION -----
// When defining a function that uses a generic place the name
// inside angled brackets after the function name

// The add trait specifies the functionality of + for different
// types
use std::ops::Add;

// We get 2 generic types of the same type and return that same type
fn get_sum_gen<T:Add<Output = T>>(x: T, y: T) -> T {
    return x + y;
}

fn main() {
    // It is common to indent with 4 spaces
    // You can tell println is a macro because of the !
    // and not a function
    println!("What is your name?");

    // Define an mutable variable (Value can changed)
    // String::new : A function that returns an empty string
    let mut name = String::new();

    /*
    By default variables are immutable (Value can't Change)
    but it is possible to use mutable variables
    It is beneficial to use immutable variables because then
    you don't have to track down how values change
    */

    // This string is immutable
    // Define it is a string with double quotes
    let greeting = "Nice to meet you";

    /*
    Receive input from the user with read_line
    name is passed as an argument to read_line
    & defines that this variable is a reference to the variable
    which allows read_line to save values directly to name
    You use mut to define that name is a mutable variable
    */

    /*
    read_line returns io::Result which is an enum
    Enums have a fixed number of specific values (Ok or Err)
    If Err is returned the operation failed and Err can tell you why
    Result has an expect function that returns any error that occurred
    (We should handle this error, but we'll cover that later)
    */
    io::stdin().read_line(&mut name)
        .expect("Didn't Receive Input");

    // Were you have {} your variable values will be placed
    // To remove the newline after name use trim_end
    println!("Hello {}! {}", name.trim_end(), greeting);

    // ----- VARIABLES -----

    // Constants can be declared in any scope and used globally
    // They differ from immutable variables in that their value
    // can't be defined at runtime (based on a function call for example)
    const ONE_MIL: u32 = 1_000_000;
    const PI: f32 = 3.141592;

    // You can define variables with the same name but with different
    // data types (Shadowing)
    let age = "47";

    // Trim eliminates white space and parse converts the string into an int
    // We expect age to have an integer value and expect will throw an
    // error if this isn't true (We'll get more into error handling later)
    let mut age: u32 = age.trim().parse()
        .expect("Age wasn't assigned a number");
    age = age + 1;

    println!("I'm {} and I want ${}", age, ONE_MIL);

    // ----- DATA TYPES -----
    // Rust is statically typed which means all types must be defined
    // These types are autogenerated by the compiler or defined explicitly

    // Unsigned integer : u8, u16, u32, u64, u128, usize
    // Signed integer : i8, i16, i32, i64, i128, isize
    let max_u32 = u32::MAX;
    println!("Max u32 : {}", max_u32);
    println!("Max u64 : {}", u64::MAX);
    // usize depends on your computer (If 64 bit then it's 64 bit)
    println!("Max usize : {}", usize::MAX);
    println!("Max u128 : {}", u128::MAX);

    // Floating Points : f32, f64
    println!("Max f32 : {}", f32::MAX);
    println!("Max f64 : {}", f64::MAX);

    // Booleans can have either the value true or false
    let _is_true = true;

    // Characters are defined with single quotes
    // They can store most any type of character from any language
    let _my_grade = 'A';

    // ----- MATH -----

    // f32 has 6 digits of precision
    let num_1: f32 = 1.111111111111111;
    println!("f32 : {}", num_1 + 0.111111111111111);

    // f64 has 14 digits of precision
    let num_2: f64 = 1.111111111111111;
    println!("f64 : {}", num_2 + 0.111111111111111);

    // Basic math operators
    let num_3: u32 = 5;
    let num_4: u32 = 4;
    println!("5 + 4 = {}", num_3 + num_4);
    println!("5 - 4 = {}", num_3 - num_4);
    println!("5 * 4 = {}", num_3 * num_4);
    println!("5 / 4 = {}", num_3 / num_4);
    println!("5 % 4 = {}", num_3 % num_4); // Remainder

    // You can use var+= 1 instead of var = var + 1

    // Generate random values between 1 and 100
    let random_num = rand::thread_rng().gen_range(1..101);
    println!("Random : {}", random_num);

    // ----- IF EXPRESSIONS -----
    let age = 8;

    if (age >= 1) && (age <= 18){
        println!("Important Birthday");
    } else if (age == 21) || (age == 50){
        println!("Important Birthday");
    } else if age >= 65 {
        println!("Important Birthday");
    } else {
        println!("Not an Important Birthday");
    }

    // ----- TERNARY OPERATOR -----
    let mut my_age = 47;
    let can_vote = if my_age >= 18 {
        true
    } else {
        false
    };
    println!("Can Vote : {}", can_vote);

    // ----- MATCH -----
    // Match runs different code depending on conditions
    // The pattern and the code to be executed is called an arm
    // A match must match all possible values!

    // You can do what we did with if
    let age2 = 8;
    match age2 {
        1..=18 => println!("Important Birthday"), // 1 through 18
        21 | 50 => println!("Important Birthday"), // 21 or 50
        65..=i32::MAX => println!("Important Birthday"), // > 65
        _ => println!("Not an Important Birthday"), // Default
    };

    // Compares age to valid age and cmp returns an Ordering which
    // has either the value Less, Greater, or Equal
    my_age = 18;
    let voting_age = 18;
    match my_age.cmp(&voting_age) {
        Ordering::Less => println!("Can't Vote"),
        Ordering::Greater => println!("Can Vote"),
        Ordering::Equal => println!("You just gained the right to vote!"),
    };

    // ----- ARRAYS -----
    // Elements in an array must be of the same type
    // and have a fixed size
    let arr_1 = [1,2,3,4];

    // Get value by index starting at 0
    println!("1st : {}", arr_1[0]);

    // Get array length
    println!("Length : {}", arr_1.len());

    // ----- LOOP -----
    // Create an infinite loop that ends when break is called
    let arr_2 = [1,2,3,4,5,6,7,8,9];
    let mut loop_idx = 0;
    loop {
        if arr_2[loop_idx] % 2 == 0 {
            loop_idx += 1;
            continue; // Goes to beginning of loop
        }

        if arr_2[loop_idx] == 9 {
            break; // Breaks out of loop
        }

        println!("Val : {}", arr_2[loop_idx]);
        loop_idx += 1;
    }

    // ----- WHILE LOOP -----
    // Looping based on a condition
    loop_idx = 0;
    while loop_idx < arr_2.len(){
        println!("Arr : {}", arr_2[loop_idx]);
        loop_idx += 1;
    }

    // ----- FOR LOOP -----
    // For works better for cycling through collections
    for val in arr_2.iter() {
        println!("Val : {}", val);
    }
    // Start Here
    // ----- TUPLES -----
    // Tuples can contain multiple data types in a list of fixed size
    // We convert to strings with to_string()
    let my_tuple: (u8, String, f64) = (47, "Derek".to_string(), 50_000.00);

    // You can get values by index starting at 0
    println!("Name : {}", my_tuple.1);

    // You can assign values to multiple variables
    let (v1, _v2, _v3) = my_tuple;
    println!("Age : {}", v1);

    // ----- STRINGS -----
    // There are 2 types of strings
    // 1. String : Vector of bytes that can be changed
    // 2. &str : Points to the string and allows for viewing

    // Create an empty growable string
    let mut st1 = String::new();

    // Insert a character at the end of a string
    st1.push('A');

    // Insert a string at the end
    st1.push_str(" word");

    // Iterate through words by splitting at whitespace
    for word in st1.split_whitespace() {
        println!("{}", word);
    }

    // Replace a string (Use "" for deleting)
    let st2 = st1.replace("A", "Another");
    println!("{}", st2);

    // Create string of characters
    let st3 = String::from("x r t b h k k a m c");

    // Convert to a vector
    let mut v1: Vec<char> = st3.chars().collect();

    // Sort characters
    v1.sort();

    // Remove duplicates
    v1.dedup();

    // Cycle through vector
    for char in v1 {
        println!("{}", char);
    }

    // Create a string literal
    let st4: &str = "Random string";

    // Convert to heap allocated String
    let mut st5: String = st4.to_string();
    println!("{}", st5);

    // Convert string into an array of bytes
    let _byte_arr1 = st5.as_bytes();

    // Get a slice of a string from index 0 to 5
    let st6 = &st5[0..6];
    println!("{}", st6);

    // Get length of string
    println!("String Length : {}", st6.len());

    // Delete values in a string if mutable
    st5.clear();

    // Combine strings
    let st6 = String::from("Just some");
    let st7 = String::from("words");

    // st6 can no longer be used
    // You can only add a reference to a string to another
    let st8 = st6 + &st7;

    // Cycle through letters in a string and print unicode
    for char in st8.bytes() {
        println!("{}", char);
    }

    // Cycle through letters in a string and print characters
    for char in st8.chars() {
        println!("{}", char);
    }

    // ----- CASTING WITH AS -----
    // You can convert to different types in multiple ways
    let int_u8: u8 = 5;
    let int2_u8: u8 = 4;
    // Cast using as
    let int3_u32: u32 = (int_u8 as u32) + (int2_u8 as u32);

    // ----- ENUMS -----
    // Enums allow for the definition of custom data types

    // Create an Enum for days of week
    enum Day {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday
    }

    // Define function for Day enum
    impl Day {
        fn is_weekend(&self) -> bool {
            match self {
                Day::Saturday | Day::Sunday => true,
                _ => false
            }
        }
    }

    // Use enum to store todays day
    let today:Day = Day::Monday;

    // Perform different actions based on day
    match today {
        Day::Monday => println!("Everyone hates Monday"),
        Day::Tuesday => println!("Donut day"),
        Day::Wednesday => println!("Hump day"),
        Day::Thursday => println!("Pay day"),
        Day::Friday => println!("Almost Weekend"),
        Day::Saturday => println!("Weekend!!!"),
        Day::Sunday => println!("Weekend!!!"),
    }

    // Check if today is a weekend
    println!("Is today the weekend {}", today.is_weekend());

    // ----- VECTORS -----
    // Vectors are like arrays that can grow if mutable
    // They only store values of the same type

    // Create an empty vector with i32
    let _vec1: Vec<i32> = Vec::new();

    // Create a vector with defined values
    let mut vec2 = vec![1, 2, 3, 4];

    // Add values to the end of a vector
    vec2.push(5);

    // Get value by index
    println!("1st : {}", vec2[0]);

    // Verify value exists
    let _second: &i32 = &vec2[1];
    match vec2.get(1) {
        Some(second) => println!("2nd : {}", second),
        None => println!("No 2nd value"),
    };

    // Cycle and change values
    for i in &mut vec2 {
        *i *= 2;
    }

    // Cycle through vector values
    for i in &vec2 {
        println!("{}", i);
    }

    // Get number of values in a vector
    println!("Vec Length : {}", vec2.len());

    // Remove and return the last value
    println!("Pop {:?}", vec2.pop());

// START HERE

    // ----- FUNCTIONS -----
    say_hello();
    get_sum(4, 5);
    println!("{} + {} = {}", 5, 3, get_sum_2(5, 3));
    println!("{} + {} = {}", 7, 8, get_sum_3(7, 8));

    // Get multiple values
    let (val_1, val_2) = get_2(3);
    println!("Nums : {} {}", val_1, val_2);

    let num_list = vec![1,2,3,4,5];
    println!("Sum of list = {}", sum_list(&num_list));

    // ----- GENERIC TYPES -----
    // We can specify the data type to be used at a later time with generics
    // It is mainly used when we want to create functions that can work with
    // multiple data types. It is used with structs, enums, traits, etc.
    // which we'll talk about later
    println!("5 + 4 = {}", get_sum_gen(5,4));
    println!("5.2 + 4.6 = {}", get_sum_gen(5.2,4.6));

    // ----- OWNERSHIP -----
    // Memory is managed through a system of ownership with
    // rules that are checked at compile time.
    // To understand this you must understand the Stack & Heap
    // Both are parts of memory

    // Stack : Stores values in a last in first out format
    // Data on the stack must have a defined fixed size

    // Heap : When putting data on the heap you request a certain
    // amount of space. The OS finds space available and returns
    // an address for that space called a pointer.

    // RULES
    // 1. Each value has a variable that's called its owner
    // 2. There is only one owner at a time
    // 3. When the owner goes out of scope the value disappears

    // While automatic deallocation of resources is great problems
    // can occur. Imagine you copied a string. If you do the string
    // just stores a pointer to the 1st index, the memory required
    // for each character and the number of characters. What happens if
    // we delete one of those strings? That information is deallocated
    // for both strings. That causes a problem called a double free error.
    // Because of that once you copy a string you can no longer access
    // the original as you see here :
    // let str1 = String::from("World");
    // let srt2 = str1;
    // println!("Hello {}", str1);

    // If you want 2 copies use clone
    let str1 = String::from("World");
    let _str2 = str1.clone();
    println!("Hello {}", str1);

    // The above doesn't apply with data types :
    // Integers, bool, char, floats, tuples with the above data types only

    // Here the string was borrowed by the function
    let str3: String = String::from("World");
    print_str(str3);

    // This throws an error because the string was dropped when the
    // function ends
    // println!("str3 = {}", str3);

    // You can avoid this by passing a reference to a variable without
    // transferring ownership (You could also return the variable from
    // the function) (Passing by reference is called Borrowing)
    let str4: String = String::from("World");
    let str5 = print_return_str(str4);
    println!("str5 = {}", str5);

    // If a function borrows a reference it can't change it unless we
    // create a mutable version of it (You can only create one mutable
    // version in the function)
    let mut str6: String = String::from("Derek");
    change_string(&mut str6);

    // ----- HASH MAPS -----
    // Hash maps are used to store key / value pairs
    use std::collections::HashMap;

    // Create an empty hash map
    let mut heroes = HashMap::new();

    // Insert in hashmap (To overwrite use the same key)
    heroes.insert("Superman", "Clark Kent");
    heroes.insert("Batman", "Bruce Wayne");
    heroes.insert("The Flash", "Barry Allen");

    // Iterate over hashmap
    for(k, v) in heroes.iter(){
        println!("{} = {}", k, v);
    }

    // Length of hashmap
    println!("Length : {}", heroes.len());

    // Check for key in hashmap
    if heroes.contains_key(&"Batman"){
        // Get value with key
        let the_batman = heroes.get(&"Batman");
        match the_batman {
            Some(_x) => println!("Batman is a hero"),
            None => println!("Batman is not a hero"),
        }
    }

    // ----- STRUCTS -----
    // A struct is a custom data type that stores multiple
    // types of data
    struct Customer{
        name: String,
        address: String,
        balance: f32,
    }

    // Create struct
    let mut bob = Customer {
        name: String::from("Bob Smith"),
        address: String::from("555 Main St"),
        balance: 234.50
    };

    // Change a value
    bob.address = String::from("505 Main St");
    println!("Address : {}", bob.address);

    // You could accept multiple data types using generics like
    // we did with functions. If we had a rectangle struct
    // that could accept floats or ints we would need 2 generics
    /* struct Rectangle<T, U> {
        length: T,
        height: U
    }
    The data type is defined when the struct is created
    let rec = Rectangle {length: 4, height: 10.5};
    */

    // We can tie struct properties to functions using Traits
    // You can create functions that can be used by any structs
    // that implement the right traits
    trait Shape {
        // This is a constructor which returns a Shape
        fn new(length: f32, width: f32) -> Self;

        // An area function that belongs to this trait
        fn area(&self) -> f32;
    }

    // Define rectangle and circle struct
    struct Rectangle {length: f32, width: f32}
    struct Circle {length: f32, width: f32}

    // Implement the trait for rectangle
    impl Shape for Rectangle{
        // Constructor
        fn new(length: f32, width: f32) -> Rectangle {
            return Rectangle{length, width};
        }

        // self allows us to refer to parameters for this struct
        fn area(&self) -> f32{
            return self.length * self.width;
        }
    }

    // Implement the trait for circle
    impl Shape for Circle{
        // Constructor
        fn new(length: f32, width: f32) -> Circle {
            return Circle{length, width};
        }

        fn area(&self) -> f32{
            return (self.length / 2.0).powf(2.0) * PI;
        }
    }

    // Create circle and rectangle with Shape
    let rec: Rectangle = Shape::new(10.0, 10.0);
    let circ: Circle = Shape::new(10.0, 10.0);

    println!("Rec Area : {}", rec.area());
    println!("Circ Area : {}", circ.area());

    // We can implement methods on structs using generics
    // impl<T, U>Shape<T, U> ...

    // ----- PACKAGES CRATES & MODULES -----
    // It is very important to keep your code organized
    // You can split code into multiple files
    // Packages can contain multiple crates
    // You can define what code is public and which is private

    // Create a file called mod.rs in a directory named restaurant
    // in the src directory

    // Crates : Modules that produce a library or executable
    // Modules : Organize and handle privacy
    // Packages : Build, test and share crates
    // Paths : A way of naming an item such as a struct, function

    // Packages can contain 0 or 1 library crate and as many binary crates
    // as you want. If you want more binary crates create a folder
    // called bin (Create bin directory in src and create file in it
    // named more_stuff.rs)

    // Call for the public function that will allow us access to
    // the module
    order_food();

    // ----- READING & WRITING TO FILES & ERROR HANDLING -----
    // Rust doesn't have exceptions like other languages. It handles
    // recoverable errors with Result and the panic! macro for
    // unrecoverable errors

    // When the panic! macro executes your program prints an error
    // memory is cleaned up and the program quits
    // panic!("Terrible Error");

    // Accessing an index that doesn't exist calls panic
    // let lil_arr = [1,2];
    // println!("{}", lil_arr[10]);

    // File to create
    let path = "lines.txt";

    // Result has 2 varients Ok and Err
    // enum Result<T, E> {
    // Ok(T),
    // Err(E), }
    // Where T represents the data typeof the value returns and E
    // the type of error

    // Create file and handle errors with match
    let output = File::create(path);
    let mut output = match output {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem creating file : {:?}", error);
        }
    };

    // Write to file and define the panic! error message with expect
    write!(output, "Just some\nRandom Words").expect("Failed to write to file");

    // Open the file and if everything is ok unwrap returns the file
    // and if not panic! triggers an error (You could replace unwrap with ?)
    // Read file using buffering
    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);

    // Cycle through and print the lines
    for line in buffered.lines() {
        println!("{}", line.unwrap());
    }

    // You can also catch specific errors
    // Here I'll try to open a file and trigger an error if the file
    // couldn't be created, or use a default
    let output2 = File::create("rand.txt");
    let output2 = match output2 {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("rand.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Can't create file: {:?}", e),
            },
            _other_error => panic!("Problem opening file : {:?}", error),
        },
    };

    // ----- ITERATORS -----
    // We covered iterators before. They help us cycle through values in
    // arrays, vectors, maps, etc.
    // An iterator cycles through values by borrowing, so the collection
    // is not moved (You can't change values)
    let mut arr_it = [1,2,3,4];
    for val in arr_it.iter() {
        println!("{}", val);
    }

    // You can create an iterator
    let mut iter1 = arr_it.iter();
    // And call for each value with next
    println!("1st : {:?}", iter1.next());

    // You could consume the collection with
    // arr_it.into_iter() but you'll no longer be able to use the collection

    // ----- CLOSURES -----
    // A closure is a function without a name and they are sometimes
    // stored in a variable (They can be used to pass a function into
    // another function)
    // let var_name = |parameters| -> return_type {BODY}

    // Create a closure that defines if someone can vote
    let can_vote = |age: i32| {
        age >= 18
    };
    println!("Can vote : {}", can_vote(8));

    // Closures can access variables outside of its body with borrowing
    let mut samp1 = 5;
    let print_var = || println!("samp1 = {}", samp1);
    print_var();
    samp1 = 10;

    // You can change values if you mark the closure mutable
    let mut change_var = || samp1 += 1;
    change_var();
    println!("samp1 = {}", samp1);
    samp1 = 10;
    println!("samp1 = {}", samp1);

    // You can pass closures to functions
    fn use_func<T>(a: i32, b: i32, func: T) -> i32 where T: Fn(i32, i32) -> i32 {
        func(a, b)
    }

    let sum = |a, b| a + b;
    let prod = |a, b| a * b;

    println!("5 + 4 = {}", use_func(5, 4, sum));
    println!("5 * 4 = {}", use_func(5, 4, prod));

    // ----- SMART POINTERS -----
    // A pointer is an address to a location in memory. We have been
    // using them when we used the reference operator(&) to borrow
    // a value.

    // Strings and vectors are smart pointers. They own
    // data and also have functions for manipulating that data.

    // Smart pointers provide functionality beyond referencing locations
    // in memory. They can be used to track who has ownership of data.
    // Ownership is very important with Rust.

    // ----- BOX -----

    // The Box smart pointer stores data on the heap instead of the stack.
    // All values are stored on the stack by default

    // Stack : Stores values in a last in first out format
    // Data on the stack must have a defined fixed size

    // Heap : When putting data on the heap you request a certain
    // amount of space. The OS finds space available and returns
    // an address for that space called a pointer.

    // A Box is normally used when you have a large amount of data stored
    // on the heap and then you pass pointers to it on the stack.

    // Create a Box with value 10
    let b_int1 = Box::new(10);

    // Get the value
    println!("b_int1 = {}", b_int1);

    // If we try to create a Binary tree we get the error
    // the size for values of type `str` cannot be known at
    // compilation time within `TreeNode<T>`

    // This is saying we can't include nodes in a node because
    // the size of node depends on the size of multiple nodes
    // which confuses the compiler
    // struct TreeNode<T> {
    //     pub left: TreeNode<T>,
    //     pub right: TreeNode<T>,
    //     pub key: T,
    // }

    // We have other problems in that Binary Trees eventually end
    // and Rust doesn't like Null values so we have to use Option

    // We can use a Box here because it has a pointer to data and
    // a fixed size

    struct TreeNode<T> {
        pub left: Option<Box<TreeNode<T>>>,
        pub right: Option<Box<TreeNode<T>>>,
        pub key: T,
    }

    // Create functions for creating nodes and adding left & right
    impl<T> TreeNode<T> {
        pub fn new(key: T) -> Self {
            TreeNode {
                left: None,
                right: None,
                key,
            }
        }

        pub fn left(mut self, node: TreeNode<T>) -> Self {
            self.left = Some(Box::new(node));
            self
        }

        pub fn right(mut self, node: TreeNode<T>) -> Self {
            self.right = Some(Box::new(node));
            self
        }
    }

    // Create the root node with left and right
    let node1 = TreeNode::new(1)
    .left(TreeNode::new(2))
    .right(TreeNode::new(3));


    // Used to test original
    // let mut boss = TreeNode {
    //     left: None,
    //     right: None,
    //     key: 50,
    // };

    // ----- CONCURRENCY -----
    // Concurrent programming envolves executing different blocks of code
    // independently, while parallel programming is when different
    // code executes at the same time. A thread handles scheduling
    // and execution of these blocks of code.

    // Common problems with parallel programming involve :
    // 1. Thread are accessing data in the wrong order
    // 2. Threads are blocked from executing because of confusion
    // over requirements to proceed with execution

    use std::thread;
    use std::time::Duration;

    // // Create a thread with spawn
    // thread::spawn(|| {
    //     for i in 1..25 {
    //         println!("Spawned thread : {}", i);
    //         // Forces thread to sleep and allow another thread to execute
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });

    // // There are no guarantees on when the threads will execute and
    // // that they will complete execution
    // for i in 1..20 {
    //     println!("Main thread : {}", i);
    //     thread::sleep(Duration::from_millis(1));
    // }

    // If we assign the return value for this thread to a variable
    // and then call join on it our program will wait for it to stop
    // executing
    let thread1 = thread::spawn(|| {
        for i in 1..25 {
            println!("Spawned thread : {}", i);
            // Forces thread to sleep and allow another thread to execute
            thread::sleep(Duration::from_millis(1));
        }
    });

    // There are no guarantees on when the threads will execute and
    // that they will complete execution
    for i in 1..20 {
        println!("Main thread : {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    // We call join here so that the main thread executes with thread1
    // unwrap handles the option Result which is Ok or Err
    thread1.join().unwrap();

    // ----- BANK ACCOUNT EXAMPLE -----
    // We will create a bank account that multiple customers will try
    // to withdraw money from

    // Bank struct just contains current balance
    pub struct Bank {
        balance: f32
    }

    // Allows for withdrawing money
    // Pass a mutable reference so bank can be used elsewhere
    // fn withdraw(the_bank: &mut Bank, amt: f32) {
    //         the_bank.balance -= amt;
    //     }


    // Create bank struct
    // let mut bank = Bank{balance: 100.00};
    // withdraw(&mut bank, 5.00);
    // println!("Balance : {}", bank.balance);

    // Create a customer thread that withdraws money
    // THIS WON'T WORK
    // fn customer(the_bank: &mut Bank){
    //     withdraw(the_bank, 5.00)
    // }

    // Can't do this closure may outlive the current function,
    // but it borrows `bank`, which is owned by the current function
    // If a thread can outlive the main function and the main function
    // has the bank which causes problems
    // thread::spawn(|| {
    //     customer(&mut bank)
    // }).join().unwrap();

    // The fix that allows multiple owners and blocks access when needed
    // A smart pointer Rc<RefCell<T>> allows multiple owners with mutable
    // access to the same data
    use std::rc::Rc;
    use std::cell::RefCell;
    // Arc<T> provides shared ownership of a value
    // Mutex blocks threads waiting for lock to be available
    use std::sync::{Arc, Mutex};

    fn withdraw(the_bank: &Arc<Mutex<Bank>>, amt:f32){
        let mut bank_ref = the_bank.lock().unwrap();

        if bank_ref.balance < 5.00{
            println!("Current Balance : {} Withdrawal a smaller amount",
            bank_ref.balance);
        } else {
            bank_ref.balance -= amt;
            println!("Customer withdrew {} Current Balance {}",
            amt, bank_ref.balance);
        }
    }

    fn customer(the_bank: Arc<Mutex<Bank>>) {
        withdraw(&the_bank, 5.00);
    }

    let bank: Arc<Mutex<Bank>> =
      Arc::new(Mutex::new(Bank { balance: 20.00 }));

    // Creates 10 customer threads
    let handles = (0..10).map(|_| {

        // Clone duplicates an the bank object
        let bank_ref = bank.clone();
        thread::spawn(|| {
            customer(bank_ref)
        })
    });

    // Wait for all customers to finish
    for handle in handles {
        handle.join().unwrap();
    }

  println!("Total: {}", bank.lock().unwrap().balance);

    // ----- INSTALLATION ------
    // Install rustup on Mac or Linux
    // curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh

}
