# bloom-filter-rs
A bloom filters is a data structure that allows you to quickly identify if some data has been previously added to the structure.
What makes a bloom filter unique is that is that it gives up full accuracy for huge speed boost.
A bloom filter has small false positive rate, and this rate can be decreased by using more memory and more hash algorithms, however you can find an optimal amount of memory and hash algorithm count to achieve great speed while still maintaining lower memory than a normal list.
This specific implementation uses three different hashing algorithms.

Bloom filters usually have two methods associated with the structure.
1. **add** an item to the structure
```rs
fn add(&mut self, value: String);
```
2. **check** if an item likely exists in the structure
```rs
fn check(&self, value: String) -> bool;
```

We will define a structure in Rust to represent the bloom filter.
```rs
struct BloomFilter {
    size: usize,
    hash_count: i8,
    bitvector: BitVec,
}
```

## Words Submodule
```
git submodule init
git submodule update
```

## Results
```
The randomly selected term is 'amicabilities'
The hashes for 'amicabilities' are 736, 11, 54

This will test how fast the word 'amicabilities' can be found in the array of 17000 words.
Elapsed time for linear_search: 220.81ms
Elapsed time for bloom filter check: 3.61ms
Elapsed time for bogo_search: 4.64s
```
```
The randomly selected term is 'Aldermaston'
The hashes for 'Aldermaston' are 618, 0, 21

This will test how fast the word 'Aldermaston' can be found in the array of 17000 words.
Elapsed time for linear_search: 245.29ms
Elapsed time for bloom filter check: 2.69ms
Elapsed time for bogo_search: 5.53s
```
```
The randomly selected term is 'Achille'
The hashes for 'Achille' are 354, 1, 24

This will test how fast the word 'Achille' can be found in the array of 17000 words.
Elapsed time for linear_search: 273.41ms
Elapsed time for bloom filter check: 1.30ms
Elapsed time for bogo_search: 6.68s
```


![Linear and Bloom filter (in ms)](https://user-images.githubusercontent.com/35516367/202334238-5aaa4157-d613-4ace-bcaa-c28c0cadb3a1.png)
![Linear, Bloom filter and Bogo (in ms)](https://user-images.githubusercontent.com/35516367/202334239-55aabb02-b24b-4091-99b3-b86ea67a9dfc.png)


