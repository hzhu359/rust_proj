// here's how you make a struct
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    // here's how you instantiate one

    let user1 = User {
        active: true,
        username: String::from("farts30"),
        email: String::from("farts30@gmail.com"),
        sign_in_count: 1,
    };

    // access via .
    let email1 = user1.email;
    println!("{email1}");

    // they can be mutable

    let mut user2 = User {
        active: true,
        username: String::from("farts30"),
        email: String::from("farts30@gmail.com"),
        sign_in_count: 1,
    };

    user2.username = String::from("the farter, the son, the holy spirit");
    let email2 = user2.username;
    println!("{email2}");

    // here's some shorthand if you want to throw it into a function

    fn create_default_user(email: String, username: String) -> User {
        User {
            active: true,
            sign_in_count: 1,
            email,
            username,
        }
    }

    // spread/update syntax
    // you can make a struct out of another struct by spreading its contents into a new struct
    // define the properties you want to change, then spread

    let user3 = User {
        username: String::from("farter3"),
        active: true,
        email: String::from("email3@gmail.com"),
        sign_in_count: 3,
    };

    let user4 = User {
        sign_in_count: 5342,
        ..user3
    };

    println!(
        "active: {}, sign_in_count: {}, email: {}, username: {}",
        user4.active, user4.sign_in_count, user4.email, user4.username
    );

    // note that in the above, user3's non-copiable attributes now belong to user4

    // TUPLE structs - like structs, just without named parameters
    // i.e. just a tuple of defined types
    // credit: ChatGPT
    struct Color(u8, u8, u8);
    let azure_teal_lighter = Color(102, 204, 204);

    // these are just tuples, so index into them w/ .
    println!("azure_teal_lighter r value: {}", azure_teal_lighter.0);

    // if you want a "unit-like" struct (stores no data)
    struct FunTrait;
    // declare as such
    // we can use this to define traits:

}
