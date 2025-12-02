use std::fs;
use std::collections::HashSet;

fn part_2(ranges: &Vec<&str>) {
    // --- Find the max number to determine max_reps ---
    let mut max_result: u64 = 0;

    for r in ranges {
        let (_, max_val_str) = r.split_once('-').expect("Invalid format");
        let max_val_int: u64 = max_val_str.parse().expect("Not a number");
        
        if max_val_int > max_result {
            max_result = max_val_int;
        }
    }

    // Python: max_reps = len(str(max_result))
    let max_reps = max_result.to_string().len();

    // Python: result = set()
    let mut result_set: HashSet<u64> = HashSet::new();

    // Python: for num_reps in range(2, max_reps+1):
    for num_reps in 2..=max_reps {
        
        for r in ranges {
            let (min_val, max_val) = r.split_once('-').unwrap();
            let min_val_int: u64 = min_val.parse().unwrap();
            let max_val_int: u64 = max_val.parse().unwrap();

            // --- Logic to calculate starting x1 ---
            let len_min = min_val.len();
            
            // Python: if (len(min_val)//num_reps > 1):
            let mut x1: u64 = if len_min / num_reps > 1 {
                // Python: x1 = int(min_val[:len(min_val)//num_reps]) - 1
                let slice_len = len_min / num_reps;
                let sub_str = &min_val[0..slice_len];
                sub_str.parse::<u64>().unwrap() - 1
            } else {
                0
            };

            // --- While True loop ---
            loop {
                x1 += 1;

                // Python: x = int(str(x1) * num_reps)
                // We convert x1 to string, repeat it num_reps times, parse back to u64
                let x_str = x1.to_string().repeat(num_reps);
                
                // Handle case where concatenation exceeds u64 limits (optional safety)
                let x: u64 = match x_str.parse() {
                    Ok(val) => val,
                    Err(_) => break, // If it's too big, it's definitely > max_val
                };

                if x >= min_val_int && x <= max_val_int {
                    result_set.insert(x);
                } else if x > max_val_int {
                    break;
                }
            }
        }
    }

    // Python: print(f'Day 2 Part 2: {sum(result)}')
    let sum_result: u64 = result_set.iter().sum();
    println!("Day 2 Part 2: {}", sum_result);
}

fn part_1(ranges: &Vec<&str>) {
    let mut result: u64 = 0;

    for r in ranges {
        // Python: min_val, max_val = r.split('-')
        // We use split_once, which returns an Option. We unwrap it assuming valid data.
        let (min_val, max_val) = r.split_once('-').expect("Invalid format");

        let min_val_int: u64 = min_val.parse().expect("Not a number");
        let max_val_int: u64 = max_val.parse().expect("Not a number");

        // Python: if (len(min_val)//2 > 1):
        let len_min = min_val.len();
        let mut x1: u64 = if len_min / 2 > 1 {
            // Python: x1 = int(min_val[:len(min_val)//2]) - 1
            let sub_str = &min_val[0..(len_min / 2)];
            sub_str.parse::<u64>().unwrap() - 1
        } else {
            0
        };

        // Python: while True:
        loop {
            x1 += 1;

            // Python: x = int(str(x1) + str(x1))
            // We format a new string combining x1 twice, then parse it back to int
            let x_str = format!("{}{}", x1, x1);
            let x: u64 = x_str.parse().unwrap_or(u64::MAX); // Safety if it exceeds u64

            if x >= min_val_int && x <= max_val_int {
                // println!("   {}", x); // Uncomment for debugging
                result += x;
            } else if x > max_val_int {
                break;
            }
        }
    }

    println!("Day 2 Part 1: {}", result);
}

fn main() {
    let data = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let ranges: Vec<&str> = data.split(',').map(|s| s.trim()).collect();

    part_1(&ranges);
    part_2(&ranges);
}
