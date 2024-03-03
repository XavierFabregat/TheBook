use rand::Rng;
use std::collections::HashMap;

fn main() {
    // Exercise 1: Given a list of integers, use a vector and return the mean (the average value),
    // median (when sorted, the value in the middle position),
    // and mode (the value that occurs most often; a hash map will be helpful here) of the list.
    let mut rng = rand::thread_rng();
    let mut numbers = Vec::new();

    for _ in 0..10 {
        numbers.push(rng.gen_range(0..20));
    }

    numbers.sort();
    println!("{:?}", numbers);

    let mean = calc_avg(&numbers);

    match mean {
        None => println!("The array is empty."),
        Some(val) => println!("The mean of the list is {val}."),
    }

    let median = calc_median(&numbers);
    match median {
        None => println!("The array is empty."),
        Some(val) => println!("The median of this list is {val}."),
    };

    let mode = calc_mode(&numbers);
    match mode {
        None => println!("The array is empty."),
        Some(val) => println!("The mode of this list is {val}."),
    };

    // Exercise 2: Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!

    let s = "first";
    let pig_latin = pig_latin_translator(s);
    println!("The pig latin of {s} is {pig_latin}.");

    let s = "apple";
    let pig_latin = pig_latin_translator(s);
    println!("The pig latin of {s} is {pig_latin}.");
}

// Possible refactors:
// return Option<i32> to accommodate for empty arrays
// return a tuple (mean: f32, median: f32, mode: i32) or struct for better syntactic code

fn calc_avg(v: &Vec<i32>) -> Option<f32> {
    if v.len() == 0 {
        return None;
    };
    let mut sum: f32 = 0.0;
    for i in v {
        sum += *i as f32;
    }
    Some(sum / v.len() as f32)
}

fn calc_median(v: &Vec<i32>) -> Option<f32> {
    if v.len() == 0 {
        return None;
    };
    let len = v.len();
    if len % 2 == 0 {
        let mid = len / 2;
        let median = (v[mid] + v[mid - 1]) as f32 / 2.0;
        Some(median)
    } else {
        let mid = len / 2;
        let median = v[mid] as f32;
        Some(median)
    }
}

fn calc_mode(v: &Vec<i32>) -> Option<i32> {
    if v.len() == 0 {
        return None;
    };
    let mut el_to_return: i32 = 0;
    let mut highest_count: i32 = 0;
    let mut map = HashMap::new();

    for i in v {
        let count = map.entry(*i).or_insert(0); // if we find the value, get it, if not, set it at 0
        *count += 1; // if there is a value, add one to it.
    }

    for (key, value) in &map {
        if *value > highest_count {
            el_to_return = *key;
            highest_count = *value;
        }
    }

    Some(el_to_return)
}

// Exercise 2: Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!

fn pig_latin_translator(s: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let mut result = String::new();
    let first_char = s.chars().next().unwrap();
    let rest = s.chars().skip(1).collect::<String>();
    let is_vowel = vowels.contains(&first_char);
    if is_vowel {
        result.push_str(s);
        result.push_str("-hay");
    } else {
        result.push_str(&rest);
        result.push('-');
        result.push(first_char);
        result.push_str("ay");
    }
    result
}
