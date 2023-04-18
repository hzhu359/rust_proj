fn main() {
    // we learned before about scope and ownership:
    // when we create some variable, it leaves scope at the end of its block
    //  i.e. if we create a String (on the heap), it is deallocated after the block ends
    // each reference can only be referenced by one variable
    // i.e. if we have

    let s1 = String::from("hello");
    let s2 = s1;

    // println!("{}", s1);
    //                ^^ value borrowed here after move
    // this is a move! happens since String does not implement Copy

    let i1 = 32;
    let i2 = i1;

    println!("{i2}");
    // this is OK since i32 and integers implement copy - their *values* are copied over upon reassignment

    // for elements without the Copy trait, ownership is passed to a function when we pass the object to a function
    print_fun(s2);
    // println!("{s2}");
    //            ^^ value borrowed here after move
    // again, we give up ownership of s2 to the print_fun method

    // what if we don't want to give up ownership?
    // use references w/ &
    let mut s3 = String::from("helloooo");
    use_no_ownership(&s3);

    // pass a mutable reference to modify it
    modify_string(&mut s3);
    println!("{s3}");

    // NOTE: we cannot create a mutable reference while a reference (either mutable or immutable) is already valid
    // this prevents data races - we want mutations to be controlled
    // if an immutable reference exists, we don't want some other process modifying it via a mutable reference
    // we also don't want multiple mutable references modifying memory at once - data race!

    // dangling references - pointer that references memory that might be freed/given to someone else
    // rust ensures that data will not go out of scope before data does ex.

    /*
    fn dangle() -> &String {
        let s = String::from("");
        &s
    }
    */

    // this function's return type contains a borrowed value, but there is no value for it to be borrowed from
    // because s goes out of scope once we return
    // to fix this, why not just return the string

    fn _gimme_string() -> String {
        String::from("")
    }

    // SLICES: a reference to a contiguous sequence of elements in a collection
    // ex. in a String, we can take a slice of the string
    let s = String::from("Hello, world!");
    let _hello = &s[0..5];
    let _world = &s[7..];

    // in essence, we `hello` and `world` to `s` s.t. if some modification to `s` happens later,
    // the compiler catches it since hello and world are already existing references
    /*
    s.push('c');
    println!("{world}");
     */

    // slices exist in the context of other collections as well
    let arr = [1, 2, 3, 4, 5];
    let arr_slice = &arr[..3];
    for elem in arr_slice {
        println!("{elem}");
    }
}

fn print_fun(s: String) {
    println!("{s}");
}

fn use_no_ownership(s: &String) {
    println!("{s}");
    // we don't have ownership of the String, we just have a reference to that string
    // thus, when the reference s stops being used, `main` still has ownership of whatever it's referencing
    // we CANNOT modify an object we only have an immutable reference to - references are immutable by default
}

fn modify_string(s: &mut String) {
    // now we have a mutable reference - we can modify
    s.push_str("plusplusplus");
}
