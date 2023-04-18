fn main() {
    /*
    let x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // we get this error
    // ^^^^^ cannot assign twice to immutable variable
    // variables are *immutable* by default in rust
    // so we can't reassign x to 6 after initial assignment
    // let's try something different
    */

    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // this works!

    // we also have constants - always immutable, always annotated with type, and always calculated at compile time
    // use underscores and caps
    const _THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // we can also do *shadowing* by reassigning variable names
    // this is different from mutability, since y stays immutable after initial modifications
    // also, y can be reassigned to a different type with shadowing (which doesn't work w/ mutable variables)
    // shadowing in diffent blocks works as expected

    let y = 5;
    let y = y + 1;

    {
        let y = "abcde";
        println!("y in inner block: {y}");
    }

    println!("y in outer block: {y}");

    // rust is statically typed: it needs to know the type at compile time
    // i.e. with this line
    /*
    let _guess = "42".parse().expect("not a number!");
     */
    // it gives us that a type annotation is needed, since we can't assume guess's type until runtime
    // this works:
    let _guess: u32 = "42".parse().expect("not a number!");

    // scalar types: integer, floats, booleans, characters
    // integers are either signed or unsigned (i or u) and have a defined bit length
    // i.e. u8, i16, u32, i128
    let num = 443;
    let num_signed: i32 = -2342;
    println!("num: {num},    num_signed: {num_signed}");

    // also have floats (f32, f64)
    // booleans (bool)
    // character (char) specified within single quotes
    let c = '🤠';
    println!("char: {c}");

    // tuples: comma-separated parenthesized list of values - can be multiple types and can have annotations
    let tup = ('a', "bee", 3);
    let _tup_signed: (char, &str, u8) = ('d', "ee", 4);

    // destructuring:
    let (t0, _t1, _t2) = tup;
    println!("t0: {t0}");

    // accessing
    let t1 = tup.1;
    println!("t1: {t1}");

    // array types! must be of same type, have fixed length
    // allocated on STACK rather than heap
    let _nums = [1, 2, 3, 4];
    let _nums_signed: [i8; 5] = [3, 4, 5, -3, -11];
    // repeating 3 five times
    let five_threes = [3; 5];

    //accessing
    let index_four = five_threes[4];
    println!("five_threes index 4: {index_four}");
}
