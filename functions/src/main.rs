fn main() {
    some_parameterless_fn();
    arbitrary_conversion(43, 5);

    // statements: instructions that DO NOT return a value
    // expressions: evaluate to a resultant value

    // here's a statement:
    let x = 5;

    // unlike in javascript, the assignment does not return the value of the assignment
    // thus this is illegal:
    /*
    let x = let y = 4;
     */
    // and you cannot assign two variables this way.

    // here's an expression:
    let y = {
        let x = 5;
        x
    };
    println!("x: {x}, y: {y}");

    // note that the `x` doesn't have a semicolon after it
    // this means that it's an expression - adding a semicolon would turn it into a (void) statement

    let plus_one_res = plus_one(x);
    println!("plus_one(5) result: {plus_one_res}");
}

// here's the syntax for a function
fn some_parameterless_fn() {
    println!("hello other world!!");
}

// here's how to specify args: you need to specify their type!
fn arbitrary_conversion(x: i32, offset: i32) {
    let ret = x * 4 + offset;
    println!("result is: {ret}");
}

// here's a function that returns something via an expression
fn plus_one(x: i32) -> i32 {
    x + 1
}
