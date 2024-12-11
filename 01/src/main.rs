use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read input from stdin
    let stdin = io::stdin();
    let mut arr1: Vec<i64> = Vec::new();
    let mut arr2: Vec<i64> = Vec::new();

    for line in stdin.lock().lines() {
        let line = line?;
        let numbers: Vec<i64> = line
            .split_whitespace()
            .map(|num| num.parse::<i64>().expect("Invalid number"))
            .collect();

        if numbers.len() != 2 {
            return Err("Each line must have exactly two numbers".into());
        }

        // Add values to the vectors
        arr1.push(numbers[0]);
        arr2.push(numbers[1]);
    }

    if arr1.is_empty() || arr2.is_empty() {
        return Err("No input provided".into());
    }

    // Process and print results
    let result_part_one = sort_arrs(&mut arr1, &mut arr2);
    println!("Part 1: {}", result_part_one);
    let result_part_two = part_two(&mut arr1, &mut arr2);
    println!("Part 2: {}", result_part_two);
    Ok(())
}

fn part_two<'a, 'b>(arr1: &'a mut Vec<i64>, arr2: &'b mut Vec<i64>) -> i64 {
    let mut hash_map: HashMap<i64, (i64, i64)> = HashMap::new();

    let default_hash_val: i64 = 0;
    // populate the hashmap keys, with default 0
    for &mut item in arr1 {
        hash_map
            .entry(item)
            .and_modify(|(occ, _)| *occ += 1)
            .or_insert((1, default_hash_val));
    }

    for &mut item in arr2 {
        hash_map.entry(item).and_modify(|(_, value)| *value += item); // Accumulate if key exists
    }

    let mut acc: i64 = 0;

    for (_key, (occ, value)) in hash_map {
        acc += occ * value;
        //println!("key: {}, value: {}", key, value)
    }
    acc
}
fn sort_arrs<'a, 'b>(arr1: &'a mut Vec<i64>, arr2: &'b mut Vec<i64>) -> i64 {
    arr1.sort();
    arr2.sort();
    let mut acc: i64 = 0;

    for i in 0..arr1.len() {
        acc += (arr1[i] - arr2[i]).abs();
    }

    acc
}
