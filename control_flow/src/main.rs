fn main() {
    // control flow looks similar to python
    // but the evaluation MUST be a bool and can't just be non-null/zero

    let x = 50;

    if x < 0 {
        println!("x is neg");
    } else if x < 10 {
        println!("x is small");
    } else {
        println!("x is big or somethin");
    }

    // ternaries
    let mystery = if x > 40 { "big" } else { "small" };
    // note that we use the {...} to evaluate these expressions
    // args must be of the same type, else we won't know type at compile time

    println!("mystery: {mystery}");

    // loops! look kinda of weird
    let mut cond = 10;
    loop {
        if cond < 0 {
            break;
        }
        println!("cond: {cond}");
        cond = cond - 1;
    }

    // we can also return a value from a loop:
    let mut loop_res = 1;
    let big_power_of_two = loop {
        loop_res = loop_res * 2;
        if loop_res > 1_000_000 {
            break loop_res; // this returns loop_res as an expression
        }
    };
    println!("here's a big power of two: {big_power_of_two}");

    // we can also label loops to break them selectively
    // label them starting with a single apostrophe:
    let mut i = 0;
    'outer: loop {
        let mut j = 0;
        'inner: loop {
            if j > 5 {
                if i > 5 {
                    break 'outer;
                }
                break 'inner;
            }
            println!("(i, j): ({i}, {j})");
            j = j + 1;
        }
        i = i + 1;
    }

    // a more familiar syntax: while
    let mut cond = 10;
    while cond > 0 {
        println!("{cond}");
        cond = cond - 1;
    }
    println!("happy new year!");

    // for(each) loops
    let tup = ["abc", "def", "ghi"];

    for e in tup {
        println!("{e}");
    }

    // so to mimic the for loop
    for num in (1..=10).rev() {
        println!("{num}");
    }
    println!("happy new year: iteratoer");
}
