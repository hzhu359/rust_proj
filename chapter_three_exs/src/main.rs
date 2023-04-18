fn main() {
    christmas();

    println!(
        "324 fahrenheit to celsius: {}",
        fahrenheit_to_celsius(324.0)
    );
    println!(
        "324 celsius to fahrenheit: {}",
        celsius_to_fahrenheit(324.0)
    );

    print!("14th fibonacci: {}", fib(14));
}

// Convert temperatures between Fahrenheit and Celsius.
fn fahrenheit_to_celsius(fahrenheit: f32) -> f32 {
    (fahrenheit - 32.0) / 1.8
}

fn celsius_to_fahrenheit(celsius: f32) -> f32 {
    celsius * 1.8 + 32.0
}

// Generate the nth Fibonacci number.
// let's try dp
fn fib(n: i32) -> i32 {
    let mut first = 1;
    let mut second = 1;

    let mut i = 2;

    while i <= n {
        let temp = first + second;
        first = second;
        second = temp;
        i = i + 1;
    }

    second
}

fn christmas() {
    let ordinals = [
        "zeroth", "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth",
        "ninth", "tenth", "eleventh", "twelfth",
    ];

    let gifts = [
        "Twelve drummers drumming",
        "Eleven pipers piping",
        "Ten lords a-leaping",
        "Nine ladies dancing",
        "Eight maids a-milking",
        "Seven swans a-swimming",
        "Six geese a-laying",
        "Five gold rings (five golden rings)",
        "Four calling birds",
        "Three French hens",
        "Two turtledoves",
        "And a partridge in a pear tree",
        "",
    ];

    let mut day = 1;
    while day <= 12 {
        let ord = ordinals[day];
        println!("On the {ord} day of Christmas, my true love sent to me");

        if day == 1 {
            println!("A partridge in a pear tree");
        } else {
            for gift_idx in (1..=day).rev() {
                let gift = gifts[12 - gift_idx];
                println!("{gift}");
            }
        }

        println!("\n\n\n");
        day = day + 1;
    }
    let last_line = gifts[12 - 1];
    println!("{last_line}");
}
