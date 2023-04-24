fn main() {
    // enums - enumerates a possible set of values (variants)
    // enums can have attributes
    // enums fall under one type!
    enum Color {
        Blue,
        Red,
        Green,
    };

    // attributes

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    };

    // we can also define methods on enums

    // the Option type - either can be Nothing or Something
    // so there's no null - we have to check the option explicitly

    let maybe_number_1 = Some(43);
    let maybe_number_2: Option<i32> = None;

    // so if we want to play with the option types, we can handle cases explicitly
    // ONLY returns Some value if both values are Some and not None
    fn add_option_i32(x: Option<i32>, y: Option<i32>) -> Option<i32> {
        match (x, y) {
            (Some(x_int), Some(y_int)) => Some(x_int + y_int),
            (_, _) => None,
        }
    }

    println!(
        "some + none: {:?}",
        add_option_i32(maybe_number_1, maybe_number_2)
    );
    println!("some + some: {:?}", add_option_i32(Some(32), Some(434)));

    // if let and else expressions

    let res: i32;

    if let Some(num) = maybe_number_1 {
        res = num;
    } else {
        res = 0;
    }

    println!("result of if let: {}", res);
}
