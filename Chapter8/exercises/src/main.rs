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
    println!("The mean of the list is {mean}.");

    let median = calc_median(&numbers);
    println!("The median of this list is {median}.");

    let mode = calc_mode(&numbers);
    println!("The mode of this list is {mode}.");
}

// Possible refactors:
// return Option<i32> to accommodate for empty arrays
// return a tuple (mean: f32, median: f32, mode: i32) or struct for better syntactic code

fn calc_avg(v: &Vec<i32>) -> f32 {
    let mut sum: f32 = 0.0;
    for i in v {
        sum += *i as f32;
    }
    sum / v.len() as f32
}

fn calc_median(v: &Vec<i32>) -> f32 {
    let len = v.len();
    if len % 2 == 0 {
        let mid = len / 2;
        let median = (v[mid] + v[mid - 1]) as f32 / 2.0;
        median
    } else {
        let mid = len / 2;
        let median = v[mid] as f32;
        median
    }
}

fn calc_mode(v: &Vec<i32>) -> i32 {
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

    el_to_return
}
