##bloom filter

Basic bloom filter with `.insert()` & `.contains()` methods. A bloom filter sits
ontop of a data store and tells you the possibility that an element is actually
contained in the data store. A good bloom filter does this with very low false
positive rates. False positive rate depends on the size of the bloom filter
See `approximating rate`

`src/bin.rs` creates a bloom filter of length 1000 and inserts 105 records with
a 0% false-positive rate (there is 1 intentional duplicate).

### approximating rate
This is a m = x, k = 3 bloom filter
(where m is available slots and k is hashing functions)
Approximation for bloom filters is `(1- e^(-kn/m))^k)`

`src/bin.rs` insert 105 elements, so `(1 - e^(-315/1000)) ^ 3)` yields ~ 2%
A smaller size would lead to a *much* greater false positive rate
(Inserting 105 elements into bloom filter with m = 255 lead to ~30% false
positive rate)
