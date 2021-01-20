use std::collections::HashMap;

fn main() {
    // DATA STRUCTURES
    let mut nums = vec![23, 26, 27, 27, 27, 29, 34, 42, 42, 43, 50, 60, 61];
    nums.sort();

    let mean = f64::from(nums.iter().sum::<i32>()) / nums.len() as f64;

    let median = match nums.len() {
        m if m % 2 == 0 => {
            let upper = nums.get((m / 2) as usize).unwrap();
            let lower = nums.get(((m / 2) - 1) as usize).unwrap();
            f64::from(upper + lower) / 2.0
        }
        n if n > 0 => f64::from(*nums.get((n / 2) as usize).unwrap()),
        _ => 0.0,
    };

    let mut nums_map = HashMap::new();
    for i in nums {
        let count = nums_map.entry(i).or_insert(0);
        *count += 1;
    }
    let mode = nums_map.into_iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap();

    println!("Mean: {}", mean);
    println!("Median: {}", median);
    println!("Mode: {:?}\n", mode);
}
