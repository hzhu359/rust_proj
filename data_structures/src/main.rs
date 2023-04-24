use std::{collections::HashMap, hash::Hash, vec};

fn main() {
    // some collections are included in std
    // Vec, VecDeque, LinkedLists, HashMaps, BTreeMaps, HashSet, BTreeSet, BinaryHeap

    // Vec(tor) - stores a dynamic aount of some data type
    // Vec ::new() creation - needs a type annotation to know what to store
    // to modify the Vec, we need to declare it as mut
    // in rust, a variable doesn't really refer to a pointer to some object
    // the variable IS the object, so in this case we need a mut since a change to the object
    // "changes" the variable

    let mut v = Vec::new();
    // compiler infers from what we push
    v.push(4);
    v.push(5);
    v.push(6);

    // a macro:
    let mut v_macro = vec![1, 2, 3];
    v_macro.push(4);

    for ele in &v_macro {
        println!("{}", ele);
    }

    // access either w/ [] or .get
    let v_elem_1 = &v_macro[3];
    let v_elem_2 = v_macro.get(3);

    /*
    let v_elem_1 = &v_macro[3];

    v_macro.push(5);

    println!("{}", v_elem_1);

    this situation fails
    immutable borrow:
        let v_elem_1 = &v_macro[3];

    mutable borrow:
        v_macro.push(5);
     */

    // more mutable borrowing (in loops !)
    for elem in &mut v_macro {
        *elem *= 10;
    }
    for elem in &v_macro {
        println!("{}", elem);
    }

    // String (capital S buddy)
    // we can create a mutable one via ::new()
    let mut s = String::new();
    // convert string literals to Strings via to_string or String::from
    s = "helloo".to_string();
    s = String::from("yooooooo");

    let s1 = String::from("abc");
    let s2 = String::from("123");

    // we can also format using string literals
    let s = format!("{s1}{s2}");

    // note that we cannot *index* into a string because
    // UTF-8 stores characters in different ways
    // i.e. Здравствуйте has 12 chars but is stored in 24 bits
    // so &word[0] would not be "З", but would be some byte representation (even though З is 2 bytes long)

    // so index through strings with .chars explicitly
    for c in "Здравствуйте".chars() {
        println!("{c}");
    }

    // hashmaps
    // k->v store

    let mut scores = HashMap::new();
    // put
    scores.insert(String::from("Yellow"), 50);
    scores.insert(String::from("Blue"), 30);
    let team_name_3 = String::from("Green");
    scores.insert(team_name_3, 500);

    // get
    let team_yellow = String::from("Yellow");
    let team_yellow_score = scores.get(&team_yellow).copied().unwrap_or(0);
    // note we need to copy the result of the Optional in order to map to an int and not an &int

    // hashmaps have an iterator
    // again - borrow the scores map here instead of taking ownership
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let t1 = scores.get("Yellow");

    // adding a key only if a key isn't present

    scores.entry(String::from("Grone")).or_insert(103);
    let grone = scores.get(&String::from("Grone")).copied().unwrap_or(0);
    scores.entry(String::from("Green")).or_insert(103);
    let green = scores.get(&String::from("Green")).copied().unwrap_or(0);
    println!("Grone: {grone}, Green: {green}");

    // updating a value based on the old value
    for (key, value) in &mut scores {
        *value += 1;
    }

    // or using .entry
    let sentence = "hello world wonderful world";
    let mut word_count: HashMap<String, i32> = HashMap::new();

    for word in sentence.split_ascii_whitespace() {
        let entry = word_count.entry(String::from(word)).or_insert(0);
        // entry is a mutable reference to the current value (init. as 0)
        // so we can go ahead and update it
        *entry += 1;
    }

    for (key, value) in word_count {
        println!("{key}: {value}");
    }
}
