use std::{collections::HashMap, io};

fn main() {
    println!(
        "I will find the mean, median, and mode of a random list of integers where you choose the size of the list and lower/upper value limits."
    );

    let (size, min, max) = get_parameters();

    let mut list = build_list(size, min, max);
    assert_eq!(list.len(), size);

    let mean = get_mean(&list);
    assert!((min as f64..=max as f64).contains(&mean));
    println!("The mean is: {}", mean);

    let (count, mode) = get_mode(&list);
    assert!(count > 0);
    assert!(!mode.is_empty());
    for i in &mode {
        assert!((min..=max).contains(i));
    }
    println!("With {count} occurrences, the mode(s) is/are: {:?}", mode);

    let median = get_median(&mut list);
    assert!((min as f64..=max as f64).contains(&median));
    if !list.len().is_multiple_of(2) {
        assert!(median.fract() == 0.0);
    }
    println!("The median is: {}", median);
}

fn get_parameters() -> (usize, i64, i64) {
    let size: usize = loop {
        let a_size = get_input("List size:");
        if a_size > 0 {
            break a_size as usize;
        } else {
            println!("Size must be greater than 0");
        }
    };
    let min: i64 = loop {
        let val = get_input("Minimum allowed value:");
        if val < i64::MAX {
            break val;
        }
        println!("Minimum value must be less than {}", i64::MAX);
    };
    let max: i64 = loop {
        let val = get_input("Maximum allowed value:");
        if val > min {
            break val;
        }
        println!("Maximum value must be greater than minimum value");
    };
    (size, min, max)
}

fn get_input(prompt: &str) -> i64 {
    loop {
        println!("{}", prompt);
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let Ok(num) = input.trim().parse() else {
            println!("Must be an integer");
            continue;
        };
        break num;
    }
}

fn build_list(size: usize, min: i64, max: i64) -> Vec<i64> {
    assert!(size > 0);
    assert!(max > min);

    let mut list: Vec<i64> = Vec::with_capacity(size);
    for _ in 0..size {
        list.push(rand::random_range(min..=max));
    }
    list
}

fn get_mean(list: &[i64]) -> f64 {
    assert!(!list.is_empty());

    let mut total: i128 = 0;
    for &i in list {
        total += i as i128;
    }
    total as f64 / list.len() as f64
}

fn get_mode(list: &[i64]) -> (u64, Vec<i64>) {
    assert!(!list.is_empty());

    let mut map = HashMap::new();

    for &i in list {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }

    let mut current_mode = Vec::new();
    let mut highest_count = 0;
    for (&key, &count) in &map {
        if count > highest_count {
            current_mode = vec![key];
            highest_count = count;
        } else if count == highest_count {
            current_mode.push(key);
        }
    }
    (highest_count, current_mode)
}

fn get_median(list: &mut [i64]) -> f64 {
    assert!(!list.is_empty());

    let middle_index = list.len() / 2;

    if list.len().is_multiple_of(2) {
        (quickselect(list, middle_index - 1) + quickselect(list, middle_index)) as f64 / 2.0
    } else {
        quickselect(list, middle_index) as f64
    }
}

fn quickselect(list: &mut [i64], target: usize) -> i64 {
    assert!(!list.is_empty());
    assert!(target < list.len());

    let mut lo = 0;
    let mut hi = list.len() - 1;

    while lo < hi {
        let pivot_idx = partition(list, lo, hi);

        if target <= pivot_idx {
            hi = pivot_idx;
        } else {
            lo = pivot_idx + 1;
        }
    }
    list[lo]
}

fn partition(list: &mut [i64], lo: usize, hi: usize) -> usize {
    assert!(!list.is_empty());
    assert!(hi < list.len());
    assert!(lo <= hi);

    let pivot = list[lo];
    let mut i = lo;
    let mut j = hi;

    loop {
        while list[i] < pivot {
            i += 1;
        }

        while list[j] > pivot {
            j -= 1;
        }

        if i >= j {
            return j;
        }
        list.swap(i, j);
        i += 1;
        j -= 1;
    }
}
