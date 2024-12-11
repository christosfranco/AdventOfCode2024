use std::collections::HashMap;
use std::io::{self, BufRead};
fn main() {
    //    let mut hashmap = HashMap::new();

    // 53 can never come before 47
    //hashmap.insert(47, HashMap::from([(53, true)]));
    //let mut arr_arr = vec![vec![75, 47, 61, 53, 29]];

    let mut coordinates: HashMap<i64, HashMap<i64, bool>> = HashMap::new();
    let mut vectors = Vec::new();
    let mut is_parsing_coordinates = true;

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();

        // If we encounter a blank line, start parsing vectors
        if line.is_empty() {
            is_parsing_coordinates = false;
            continue;
        }

        if is_parsing_coordinates {
            // Parse x|y format and store the coordinates
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() == 2 {
                let x = parts[0].parse::<i64>().unwrap();
                let y = parts[1].parse::<i64>().unwrap();
                // Coordinates : HashMap< 47 , HashMap< 57 , true>
                if let Some(inner_hashmap) = coordinates.get_mut(&x) {
                    inner_hashmap.insert(y, true);
                } else {
                    coordinates.insert(x, HashMap::from([(y, true)]));
                }
                //                coordinates.insert((x, y)); // Store in a HashSet for uniqueness
            }
        } else {
            // Parse vectors in the format "a,b,c,..."
            let vector: Vec<i64> = line.split(',').map(|s| s.parse::<i64>().unwrap()).collect();
            vectors.push(vector);
        }
    }
    println!("{:?}, {:?}", vectors, coordinates);
    //let mut arr_arr = vec![vec![47, 53]];
    //75,47,61,53,29
    //let stdin = io::stdin();
    //let grid: Vec<Vec<char>> = stdin
    //    .lock()
    //    .lines()
    //    .map(|line| line.unwrap().chars().collect())
    //   .collect();
    // println!("{:?}", grid);
    println!("Occurrences: {}", part_one(coordinates, vectors));
    //println!("Occurrences: {}", part_two(grid));
}

// if it is correct the sum will be added
fn part_one(hashmap: HashMap<i64, HashMap<i64, bool>>, arr_arr: Vec<Vec<i64>>) -> i64 {
    let mut sum: i64 = 0;
    for arr in arr_arr {
        let mut fail: bool = false;
        //println!("arr: {:?}", arr);
        for i in 0..(arr.len()) {
            // 47 , 53
            // arr[0] -> 47
            // arr[1] -> 53
            if let Some(hash_bad_values) = hashmap.get(&arr[i]) {
                for j in 0..i {
                    // i = 29
                    //println!("i:  {},{:?}", i, hash_bad_values);
                    if let Some(_val) = hash_bad_values.get(&arr[j]) {
                        println!("failed");
                        fail = true;
                        break;
                    }
                }
            }
            if (fail) {
                break;
            }
        }
        if !fail {
            println!("Success: {:?}, Value: {:?}", arr, arr[arr.len() / 2]);
            sum += arr[arr.len() / 2];
        }
    }
    return sum;
}
