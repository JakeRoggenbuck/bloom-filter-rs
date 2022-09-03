use anyhow::Result;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::time::Instant;

fn get_array(num_vec: &mut [String]) -> Result<()> {
    let file = File::open("english-words/words.txt")?;
    let reader = BufReader::new(file);

    let mut index = 0;
    for line in reader.lines() {
        if index < num_vec.len() {
            num_vec[index] = line?;
        }
        index += 1;
    }

    Ok(())
}

#[allow(dead_code)]
fn print_array(array: &[String]) {
    for c in array {
        println!("{c}");
    }
}

fn linear_search(array: &[String], verbose: bool) {
    let mut index = 0;
    for c in array {
        if c == "anytime" {
            if verbose {
                println!("Found! {index}");
            }
        }
        index += 1;
    }
}

fn bogo_search(array: &[String], verbose: bool) {
    let mut times = 0;
    let mut num: usize;
    loop {
        num = thread_rng().gen_range(0..array.len());
        if array[num] == "anytime" {
            if verbose {
                println!("Found! {num} with {} times", times);
            }
            break;
        }
        times += 1;
    }
}

fn main() {
    let mut num_vec: Vec<String> = vec![String::new(); 17000];
    get_array(&mut num_vec).unwrap();
    num_vec.shuffle(&mut thread_rng());

    let before = Instant::now();
    for _ in 0..100 {
        linear_search(&mut num_vec, false);
    }
    println!("Elapsed time for linear_search: {:.2?}", before.elapsed());

    let before = Instant::now();
    for _ in 0..100 {
        bogo_search(&mut num_vec, false);
    }
    println!("Elapsed time for bogo_search: {:.2?}", before.elapsed());
}
