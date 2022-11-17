use anyhow::Result;
use bit_vec::BitVec;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::time::Instant;

/// Print an entire array
pub fn print_array(array: &[String]) {
    for c in array {
        println!("{c}");
    }
}

/// Search for the term using linear search
fn linear_search(array: &[String], term: String) -> bool {
    for c in array {
        if c == term.as_str() {
            return true;
        }
    }
    return false;
}

/// Search for the term using the worst search algorithm, bogo search
fn bogo_search(array: &[String], term: String) -> bool {
    let mut num: usize;
    loop {
        num = thread_rng().gen_range(0..array.len());
        if array[num] == term.as_str() {
            return true;
        }
    }
}

fn hash_1(s: String) -> i32 {
    let mut hash = 0;
    let size = s.len();

    for i in 0..size {
        hash = hash + (s.chars().nth(i)).unwrap() as i32 - 0x30;
    }
    hash
}

fn hash_2(s: String) -> i32 {
    let mut hash = 7;
    let size = s.len();

    for i in 0..size {
        hash = (hash * 31 + (s.chars().nth(i)).unwrap() as i32 - 0x30) % size as i32;
    }
    hash % size as i32
}

fn hash_3(s: String) -> i32 {
    (hash_2(s) + 7) * 3
}

struct BloomFilter {
    size: usize,
    hash_count: i8,
    bitvector: BitVec,
}

trait Filter {
    fn add(&mut self, value: String);
    fn check(&self, value: String) -> bool;
    fn hash(&self, s: String, i: usize) -> i32;
}

impl Filter for BloomFilter {
    fn add(&mut self, value: String) {
        for x in 0..self.hash_count {
            let v = self.hash(value.clone(), x.try_into().unwrap());
            let k = v as usize % self.size;

            self.bitvector.set(k, true);
        }
    }

    fn check(&self, value: String) -> bool {
        let mut acc = 0;
        for x in 0..self.hash_count {
            let v = self.hash(value.clone(), x.try_into().unwrap());
            let k = v as usize % self.size;

            if self.bitvector.get(k).unwrap_or(false) {
                acc += 1;
            }
        }
        return acc >= self.hash_count;
    }

    fn hash(&self, s: String, i: usize) -> i32 {
        let functions: [&dyn Fn(String) -> i32; 3] = [&hash_1, &hash_2, &hash_3];
        return functions[i](s);
    }
}

fn fill_array_and_bloom_filter(num_vec: &mut [String], bf: &mut BloomFilter) -> Result<()> {
    let file = File::open("english-words/words.txt")?;
    let reader = BufReader::new(file);

    let mut index = 0;
    for line in reader.lines() {
        if index < num_vec.len() {
            let l = line?;
            // Add word to array
            num_vec[index] = l.clone();
            // Add word to bloom filter
            bf.add(l);
        }
        index += 1;
    }

    Ok(())
}

fn main() {
    // Set up bloom filter
    let mut bf = BloomFilter {
        bitvector: BitVec::from_elem(10000, false),
        hash_count: 3,
        size: 10000,
    };

    // Set up num vec
    let mut num_vec: Vec<String> = vec![String::new(); 17000];

    fill_array_and_bloom_filter(&mut num_vec, &mut bf).unwrap();
    num_vec.shuffle(&mut thread_rng());

    let length = num_vec.len();

    // Pick random term
    let mut rng = rand::thread_rng();
    let x: usize = rng.gen_range(0..num_vec.len());
    let term: String = num_vec[x].clone();
    println!("The randomly selected term is '{term}'");

    // Calculate all of the hashes for the term
    let one = hash_1(term.clone());
    let two = hash_2(term.clone());
    let three = hash_3(term.clone());
    println!("The hashes for '{term}' are {one}, {two}, {three}\n");

    println!(
        "This will test how fast the word '{term}' can be found in the array of {length} words."
    );

    // Test the time it takes for linear search
    let before = Instant::now();
    for _ in 0..100 {
        linear_search(&mut num_vec, term.clone());
    }
    println!("Elapsed time for linear_search: {:.2?}", before.elapsed());

    // Test the time it takes for bloom filter check
    let before = Instant::now();
    for _ in 0..100 {
        bf.check(term.clone());
    }
    println!(
        "Elapsed time for bloom filter check: {:.2?}",
        before.elapsed()
    );

    // Test the time is takes for bogo search
    let before = Instant::now();
    for _ in 0..100 {
        bogo_search(&mut num_vec, term.clone());
    }
    println!("Elapsed time for bogo_search: {:.2?}", before.elapsed());
}
