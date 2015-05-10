#![feature(test)]
extern crate test;

/// a good hashing function is well-distributed & deterministic
/// to achieve those qualities, there can't be too many destructive operations
/// (we want to avoid situations that trend to a certain range of values)
/// and must reduce (fold) to take advantage of a string's uniqueness
/// hash_one will take the difference
pub fn hash_one(datum: &str) -> u8 {
    let nums: Vec<u8> = datum.bytes().collect();
    // we'll multiply each byte by the difference b/w it and the next byte
    // (as a decimal) and then clear out the decimals
    let mut hashed_datum: f32 = 0_f32;
    let mut iter = nums.iter().peekable();
    let mut total_difference: f32 = 0_f32;

    while let Some(num) = iter.next() {
        if iter.peek() != None {
            let next: u8 = **iter.peek().unwrap();
            let mut difference: f32 = *num as f32 - next as f32;
            while difference > 0_f32 {
                difference = difference / 10_f32;
            }
            hashed_datum += (*num as f32 * difference).abs();
            total_difference += difference;
        } else {
            hashed_datum += *num as f32 * total_difference;
        }
    }
    hashed_datum as u8
}

pub fn hash_two(datum: &str) -> u8 {
    let nums: Vec<u8> = datum.bytes().collect();

    // bit shift number 0-2 bits to left (yay ternary number system)
    // and add base 19 log of that^ to accumulator (hashed_datum)
    let mut hashed_datum: f32;
    hashed_datum = nums.iter().fold(0_f32, |hashed_datum: f32, num| {
        let hash: u8 = *num << (hashed_datum as u8 % 3);
        hashed_datum + (hash as f32).log(19_f32)
    });

    while hashed_datum.fract() != 0_f32 {
        hashed_datum = hashed_datum * 10_f32;
    }
    hashed_datum as u8
}
pub fn hash_three(datum: &str) -> u8 {
    let nums: Vec<u8> = datum.bytes().collect();

    // convert to radians
    let mut hashed_datum: f32 = nums.iter().fold(0_f32, |hash: f32, num| {
        hash * (num as f32).to_radians().sin()
    // trig-it
    // pwf
    // multiple

    nums[0]
}

#[cfg(test)]
mod tests {
    use super::hash_one;
    use super::hash_two;
    use test::Bencher;

    #[test]
    fn it_works() {
        // yes I cheated. well, this tests if fn is deterministic
        // see it_actually_works for more tests...

        assert_eq!(hash_one("Greystark"), 218);
        assert_eq!(hash_one("Greystark"), 218);
        assert_eq!(hash_two("Karstark"), 142);
        assert_eq!(hash_two("Karstark"), 142);
    }
    #[test]
    fn it_actually_works() {
        let north_houses = [
            "Amber" , "Bolton", "Condon", "Dustin", "Forrester",
            "Greystark", "Harclay", "Ironsmith", "Karstark", "Lake", "Marsh",
            "Norrey", "Overton", "Peat", "Pool", "Quagg", "Redbeard", "Reed",
            "Ryswell", "Slate", "Stane", "Stark", "Stout", "Thenn", "Umber",
            "Waterman", "Wells", "Whitehill", "Woodfoot", "Woods", "Woolfield",
            "Wull", "Wull"
            ];
        let mut northern_hash_one = Vec::new();
        let mut northern_hash_two = Vec::new();

        for house in north_houses.iter() {
            northern_hash_one.push(hash_one(house));
            northern_hash_two.push(hash_two(house));
        }

        // 33 elements and a range of 255 - a "well distributed" hash function
        // would hash one element every interval of 7.73
        // To test distribution we'll sort the  hashed values
        northern_hash_one.sort_by(|a, b| a.cmp(b));
        northern_hash_two.sort_by(|a, b| a.cmp(b));

        // & check there's a *relatively* consistent difference b/w elements
        // The better the hashing fn the smaller the interval
        // Let's start with 5 < x < 10 as the range - %30 away from "ideal"
        // ( 7.73 ± 2.27 )
        {
            let mut iter = northern_hash_one.iter().peekable();
            while let Some(num) = iter.next() {
                if iter.peek() != None {
                    let difference: u8 = *iter.peek().unwrap() - *num;
                    assert!(difference >= 5 || difference <= 10);
                }
            }
        }
        {
            let mut iter = northern_hash_two.iter().peekable();
            while let Some(num) = iter.next() {
                if iter.peek() != None {
                    let difference: u8 = *iter.peek().unwrap() - *num;
                    assert!(difference >= 5 || difference <= 10);
                }
            }
        }

        // to test for collisions we'll dedup and expect a difference of <= 2,
        // or 3% (33 elements with 1 intentional duplicate - Wull)
        let northern_one_len: usize = northern_hash_one.len();
        let northern_two_len: usize = northern_hash_two.len();
        northern_hash_one.dedup();
        northern_hash_two.dedup();

        assert!(northern_one_len - northern_hash_one.len() <= 2);
        assert!(northern_two_len - northern_hash_two.len() <= 2);

    }
    #[bench]
    fn hash_one_bench(b: &mut Bencher) {
        b.iter(|| hash_one("Great Danton"));
    }
    #[bench]
    fn hash_two_bench(b: &mut Bencher) {
        b.iter(|| hash_two("The Professor"));
    }
}

