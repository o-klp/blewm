 ##bloom filter

Basic bloom filter with `.insert()` & `.contains` methods. A bloom filter sits
ontop of a data store and tells you the possibility that an element is actually
contained in the data store. A good bloom filter does this with very low false
positive rates. This one is pretty bad ~30%

There may be a few explanations for this:
The size of a bloom filter should probably be much much larger
than the amount of elements the data store will ever hold at one time. This
bloom filter is only 255 in length because the simple hashing functions used
are of size u8. That's another possibility for so many false positives. The
hashing functions are probably not that great, they just seem well-distributed
because they're returned as u8 (~ % 255).

### approximating rate
This is a m=255 k=3 bloom filter
(where m is available slots and k is hashing functions)
Approximation for bloom filters is `(1- e^(-kn/m))^k)`
Tests insert 105 elements, so (1 - e^(-315/255)) ^ 3) yields ~ 35%
This would suggest the hash fn's aren't *that* bad but that m is too small for
the given n
