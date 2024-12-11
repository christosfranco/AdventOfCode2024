use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read input from stdin
    let stdin = io::stdin();

    let mut arr_arr: Vec<Vec<i32>> = stdin
        .lock()
        .lines() // Read lines from stdin
        .map(|line| {
            line.unwrap() // Unwrap the `Result<String, _>`
                .split_whitespace() // Split by whitespace
                .map(|num| num.parse::<i32>().unwrap()) // Parse each number
                .collect() // Collect into a Vec<i32>
        })
        .collect(); // Collect all lines into a Vec<Vec<i32>>

    println!("{:?}", arr_arr);

    // Process and print results
    let result_part_one = part_one(&mut arr_arr);
    println!("Part 1: {}", result_part_one);
    let result_part_two = part_two(&mut arr_arr);
    println!("Part 2: {}", result_part_two);
    Ok(())
}

fn part_one(arr_arr: &mut Vec<Vec<i32>>) -> i64 {
    let mut number_valid = 0;
    for arr in (arr_arr) {
        let mut incr: bool = false;
        let mut valid: bool = false;
        for i in 0..arr.len() - 1 {
            let diff = arr[i] - arr[i + 1];
            if (i == 0) {
                if (4 > diff && diff > 0) {
                    incr = false;
                } else if (0 > diff && diff > -4) {
                    incr = true;
                } else {
                    break;
                }
            } else {
                if (incr && (0 > diff && diff > -4)) {
                    valid = true;
                } else if (!incr && (4 > diff && diff > 0)) {
                    valid = true;
                } else {
                    valid = false;
                    break;
                }
            }
            //            println!("{:?}", arr[i]);
        }
        if (valid) {
            number_valid += 1;
        }
    }
    return number_valid;
}

fn part_two(arr_arr: &mut Vec<Vec<i32>>) -> i64 {
    let mut number_valid = 0;
    let mut index = 0;
    let mut safe_used: bool = false;
    let mut valid: bool = false;
    for arr in (arr_arr) {
        (valid, index) = helper_part_two(arr);

        if (valid) {
            number_valid += 1;
        } else {
            for i in 0..arr.len() {
                let mut new_arr: Vec<i32> = arr[..i].iter().chain(&arr[i + 1..]).cloned().collect();
                (valid, index) = helper_part_two(&mut new_arr);
                if (valid) {
                    number_valid += 1;
                    break;
                }
            }
        }
    }
    return number_valid;
}

fn helper_part_two(arr: &mut Vec<i32>) -> (bool, usize) {
    let mut incr: i32 = 0;
    let mut decr: i32 = 0;
    let mut valid: bool = true;
    for i in 0..arr.len() - 1 {
        //            println!("{:?}", arr[i]);

        let diff = arr[i] - arr[i + 1];
        if (incr == 0 && (-1 >= diff && diff >= -3)) {
            decr += 1;
        } else if (decr == 0 && (3 >= diff && diff >= 1)) {
            incr += 1;
        } else {
            valid = false;

            return (valid, i);
        }
    }
    return (valid, 0);
}
