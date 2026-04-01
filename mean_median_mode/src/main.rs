use std::{collections::HashMap, io};

fn main() {
    println!(
        "I will find the mean, median, and mode of a random list of integers where you choose the size of the list and lower/upper value limits."
    );

    let size: u64 = loop {
        let a_size = get_input("List size:");
        if a_size > 0 {
            break a_size as u64;
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

    let list = build_list(size, min, max);
    assert_eq!(
        list.len(),
        size as usize,
        "list length must equal desired size!"
    );

    let mean = get_mean(&list);
    assert!(
        (mean > min as f64) & (mean < max as f64),
        "Mean must be within min/max range!"
    );
    println!("The mean is: {}", get_mean(&list));

    let mode = get_mode(&list);
    assert!(!mode.is_empty(), "mode is empty!");
    for i in &mode {
        assert!(
            (min..=max).contains(i),
            "Integer returned as a mode is outside lower/upper value limits!"
        );
    }
    println!("The mode(s) is/are (multiple if tied): {:?}", mode);
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

fn build_list(size: u64, min: i64, max: i64) -> Vec<i64> {
    assert!(size > 0, "Size must be greater than 0!");
    assert!(max > min, "Max must be greater than min!");

    let mut list: Vec<i64> = Vec::with_capacity(size as usize);
    for _ in 0..size {
        list.push(rand::random_range(min..=max));
    }
    list
}

fn get_mean(list: &Vec<i64>) -> f64 {
    assert!(!list.is_empty(), "Empty list!");

    let mut total: i128 = 0;
    for &i in list {
        total += i as i128;
    }
    total as f64 / list.len() as f64
}

fn get_mode(list: &Vec<i64>) -> Vec<i64> {
    assert!(!list.is_empty(), "Empty list!");

    let mut map = HashMap::new();

    for &i in list {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }

    let mut current_mode = vec![list[0]];
    for (&key, &count) in &map {
        let highest_count = *map.get(&current_mode[0]).unwrap();
        if count > highest_count {
            current_mode = vec![key];
        } else if count == highest_count {
            current_mode.push(key);
        }
    }
    current_mode
}

// fn get_median(list: &Vec<i64>) -> i64 {}
