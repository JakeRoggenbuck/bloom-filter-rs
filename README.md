# bloom-filter-rs
A bloom filters is a data structure that allows you to quickly identify if some data has been previously added to the structure.
What makes a bloom filter unique is that is that it gives up full accuracy for huge speed boost.
A bloom filter has small false positive rate, and this rate can be decreased by using more memory and more hash algorithms, however you can find an optimal amount of memory and hash alroithm count to achieve great speed while still maintaining lower memory than a normal list.
This specific implementation uses three different hashing algorithms.

## Words Submodule
```
git submodule init
git submodule update
```

## Results
- Elapsed time for linear_search: 122.46ms
- Elapsed time for bogo_search: 2.33s
