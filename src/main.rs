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

fn print_array(array: &[String]) {
    for c in array {
        println!("{c}");
    }
}

fn linear_search(array: &[String], verbose: bool, term: String) {
    let mut index = 0;
    for c in array {
        if c == term.as_str() {
            if verbose {
                println!("Found! {index}");
            }
        }
        index += 1;
    }
}

fn bogo_search(array: &[String], verbose: bool, term: String) {
    let mut times = 0;
    let mut num: usize;
    loop {
        num = thread_rng().gen_range(0..array.len());
        if array[num] == term.as_str() {
            if verbose {
                println!("Found! {num} with {} times", times);
            }
            break;
        }
        times += 1;
    }
}

fn hash_1(s: String) {
    let mut hash = 0;
    let size = s.len();

    for i in 0..size {
        hash = hash + (s.chars().nth(i)).unwrap() as i32 - 0x30;
    }

    println!("{hash}");
}

fn hash_2(s: String) {
    let mut hash = 7;
    let size = s.len();

    for i in 0..size {
        hash = (hash * 31 + (s.chars().nth(i)).unwrap() as i32 - 0x30) % size as i32;
    }

    hash = hash % size as i32;

    println!("{hash}");
}

fn hash_3(s: String) {
    let mut hash = 7;
    let p = 7;
    let size = s.len();

    for i in 0..size {
        hash = (hash * 7 + (s.chars().nth(i)).unwrap() as i64 - 0x30)
            * i64::pow(p, i.try_into().unwrap());
        hash = hash % size as i64;
    }

    println!("{hash}");
}

fn main() {
    let mut num_vec: Vec<String> = vec![String::new(); 17000];
    get_array(&mut num_vec).unwrap();
    num_vec.shuffle(&mut thread_rng());

    let mut rng = rand::thread_rng();

    let x: usize = rng.gen_range(0..num_vec.len());
    let term: String = num_vec[x].clone();

    println!("Term is {term}");

    hash_1(term.clone());
    hash_2(term.clone());
    hash_3(term.clone());

    let verbose = false;
    if verbose {
        print_array(&num_vec);
    }

    let before = Instant::now();
    for _ in 0..100 {
        linear_search(&mut num_vec, false, term.clone());
    }
    println!("Elapsed time for linear_search: {:.2?}", before.elapsed());

    let before = Instant::now();
    for _ in 0..100 {
        bogo_search(&mut num_vec, false, term.clone());
    }
    println!("Elapsed time for bogo_search: {:.2?}", before.elapsed());
}
