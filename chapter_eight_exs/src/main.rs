use std::{
    collections::{HashMap, HashSet},
    vec,
};

fn main() {
    let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 7, 8, 7, 8, 8, 8, 8, 8];
    let (median, mode) = median_mode(&nums);
    println!("median: {}, mode: {}", median, mode);

    let words = ["ambulance", "beta", "", "guhguhguh"];
    for word in words {
        println!("{}", pig_latin(word).unwrap_or(String::from("INVALID")));
    }
}

fn median_mode(list_orig: &Vec<i32>) -> (f64, i32) {
    let mut list = list_orig.clone();
    list.sort();
    let len = list.len();
    let median = if len % 2 == 1 {
        f64::from(list[len / 2])
    } else {
        f64::from(list[len / 2] + list[(len / 2) - 1]) / 2.0
    };

    let mut counts: HashMap<i32, i32> = HashMap::new();

    let mut max_num = 0;
    let mut max_count = 0;

    for ele in list {
        let entry = counts.entry(ele).or_insert(0);
        *entry += 1;

        if *entry > max_count {
            max_num = ele;
            max_count = *entry;
        }
    }

    return (median, max_num);
}

fn pig_latin(input: &str) -> Option<String> {
    let first = input.chars().next();
    match first {
        Some('a' | 'e' | 'i' | 'o' | 'u') => Some(format!("{input}-hay")),
        Some(first_char) => Some(format!("{}-{}ay", input.get(1..).unwrap(), first_char)),
        None => None,
    }
}
