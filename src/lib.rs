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
    println!("and here at the turn I leave you bourden {:?} {:?} {:?}",
             hashed_datum, hashed_datum as u8, datum);
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
    println!("Angier? {:?} {:?} {:?}", hashed_datum, hashed_datum as u8, datum);
    hashed_datum as u8
}

#[cfg(test)]
mod tests {
    use super::hash_one;
    use super::hash_two;
    use test::Bencher;

    #[test]
    fn it_works() {
        // yes I cheated
        // to actually test one should add lots of test data, hash it
        // then check for collisions/distribution
        // see below...

        assert_eq!(hash_one("Greystark"), 218);
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

        // 33 elements and a range of 255 - a wise one once said a 'good' hash
        // fn should have hmm... ~ (elements / range) collisions
        // we'll sort the hashes, dedup, and expect a difference of 1
        // (33 elements with 1 intentional duplicate - Wull)
        northern_hash_one.sort_by(|a, b| a.cmp(b));
        northern_hash_two.sort_by(|a, b| a.cmp(b));
        northern_hash_one.dedup();
        northern_hash_two.dedup();

        assert_eq!(32, northern_hash_one.len());
        assert_eq!(32, northern_hash_two.len());

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
