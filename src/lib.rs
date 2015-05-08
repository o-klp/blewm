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
    hashed_datum = nums.iter().fold(0_f32, |hashed_datum: f32, num| {
        let hash: u8 = *num << (hashed_datum as u8 % 3);
        hashed_datum + (hash as f32).log(19_f32)
    });

    while hashed_datum.fract() != 0_f32 {
        hashed_datum = hashed_datum * 10_f32;
    }
    println!("Angier? {:?} {:?}", hashed_datum, hashed_datum as u8);
    hashed_datum as u8
}

#[test]
fn it_works() {
    assert_eq!(hash_two("atest"), "atest".bytes().collect::<Vec<_>>()[0]);
}
