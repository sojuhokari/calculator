



fn main() {
    // Variables
    // Variables in Rust are immutable by default – so the value can't change once you assign a value to them
    let hello_string = "Hello, World! 🏳️‍⚧️";
    
    // To print out a variable:
    println!("{}", hello_string);

    // Variables in rust are immutable by default so you can't change the value:
    //hello_string = "hi";

    // To make a mutable variable:
    let mut mutable_variable = "Hi!";
    println!("{}", mutable_variable);
    mutable_variable = "Hello!";
    println!("{}", mutable_variable);

    // Ok. Basic types in Rust:
    // String:
    let string_var = "String";
    // Rust tries to deduce from what you've written what type of variable it is. So in the case above, Rust assumes because there are quotation marks, that the variable "string_var" is of type "String"
    // To explicitly declare the type of a variable, use a semicolon after the name:
    let string_var_explicit: &str = "String";

    // Integers:
    let integer = 6;

    // There are a TON of types of integers
    // i8, i16, i32, i64, i128, isize
    // Each of them describes the binary size of the integer. So for instance, i8 means it's 8 bits, which means that it covers all numbers between -127 and 127.

    // If you don't specify the type of integer, I'm pretty sure it defaults to "isize", which is i32 on 32-bit machines, and i64 on 64-bit machines.

    // So this will be of type "isize"
    let integer_64 = 7;

    // If you want it to be of type i8:
    let integer_8: i8 = 8;
    // If it's of type "i8", then you can't go over the limit. So the following won't run
    //let integer_8_too_big: i8 = 500;

    // The other thing is that different types of integers can't be added to each other without being converted:

    let integer_32: i32 = 50;
    //let integer_combined = integer_8 + integer_32;

    // So instead of simply adding, you have to convert one to the other using "as"
    let integer_combined = integer_8 + integer_32 as i8;
    println!("{}", integer_combined);

    // Other types of numbers: u8, u16, u32, u64, u128, and usize are "unsigned" integers, which means that they don't have a +/- sign. So u8 goes from 0 to 255.
    // More types of numbers: f32, f64, f128 are floating-point numbers, so they can be decimals:
    let float: f32 = 28.43243;

    // If you don't specify the type of the number, if there is no decimal, Rust assumes it's an integer, and if there is, Rust assumes it's a float.
    // So this will be a float:
    let float_two = 45.234;

    println!("{}", float_two);

    // And then there's booleans:
    // They can be true or false
    let boolean = true;
    let boolean_two = false;

    let boolean_three: bool = true;

    // In Rust, you can declare a variable without assigning anything to it
    let boolean_four: bool;
    // And you won't be able to use it until you assign something to it, so the following won't compile:
    //println!("{}", boolean_four);
    // So you have to assign like this:
    boolean_four = true;
    // And then you can print it:
    println!("{}", boolean_four);

    // If Else Statements...
    // So in Rust, there are two different types of things you can write. There are statements, which end in a semicolon, and there are expressions, which do not, and evaluate to something.
    // Basically, statements *do* something, while expressions *can be evaluated*
    // So "let var_name = 7;" is a statement
    // "7 + 11" is an expression
    // "true" is an expression
    // "9" is an expression
    // "8 - 32 * 6" is an expression

    // This is important because if-else in Rust is actually an expression, and you write it like this:
    // if "expression that evaluates to a boolean" { "then do this" } else { "then do that" }
    // So for example:

    if boolean_four {
        println!("Boolean four is true");
    } else {
        println!("Boolean four is false");
    }

    // And this also works:
    if integer_32 < 20 {
        println!("Integer 32 is less than 20");
    } else {
        println!("Integer 32 is more than 20");
    }
    // And "integer_32 < 20" is an expression that evaluates to a boolean; if the boolean is true, it runs the first branch, if false, it runs the second branch.

    // String interpolation!!!
    println!("Integer 32 is {}", integer_32);
    // The above will print "Integer 32 is 50"
    println!("Integer 32 is {} and more than 20", integer_32);
    // The above will print "Integer 32 is 50 and more than 20"

    // To interpolate multiple things into a string,
    println!("Integer 32 is {}. Is Integer 32 bigger than 20? {}", integer_32, integer_32 > 20);
    // The above will print "Integer 32 is 50. Is Integer 32 bigger than 20? true"
    // The interpolation happens in order
    // You can interpolate anything that is an expression into it, so in the above println statement, "integer_32 > 20" acts as an expression that evaluates to a boolean
    // To print out simple curlicues, just double the curlicue, so "{{"
    
    // Assignment is one =, comparing for equality is ==
    // And anything with "=" will be a statement, and anything with "==" will be an expression

    // To escape characters in a string, use a backslash, so: "\\" = "\", "\t" = tab, "|n" = newline

    // So the following will print "\hello  
    //"
    println!("\\hello\t\n");

    // AND ALSO! Strings in Rust can cover multiple lines!
    let multiline_string = "This is a valid &str
    
    and it continues here";

    // So this:
    println!("hello");
    // is the same as this:
    print!("hello\n");

    let mut bedtime = false;
    bedtime = true;
    if bedtime {
        println!("Go to sleep");
    } else {
        println!("Stay awake lol");
    }
}


